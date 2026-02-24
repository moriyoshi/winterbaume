//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-s3vectors

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
pub fn serialize_create_index_response(result: &CreateIndexOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_vector_bucket_response(result: &CreateVectorBucketOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_index_response(result: &DeleteIndexOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_vector_bucket_response(result: &DeleteVectorBucketOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_vector_bucket_policy_response(
    result: &DeleteVectorBucketPolicyOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_vectors_response(result: &DeleteVectorsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_index_response(result: &GetIndexOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_vector_bucket_response(result: &GetVectorBucketOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_vector_bucket_policy_response(
    result: &GetVectorBucketPolicyOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_vectors_response(result: &GetVectorsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_indexes_response(result: &ListIndexesOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_vector_buckets_response(result: &ListVectorBucketsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_vectors_response(result: &ListVectorsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_vector_bucket_policy_response(
    result: &PutVectorBucketPolicyOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_vectors_response(result: &PutVectorsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_query_vectors_response(result: &QueryVectorsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_index_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateIndexInput, String> {
    let mut input = CreateIndexInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateIndexInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateIndex request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_vector_bucket_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateVectorBucketInput, String> {
    let mut input = CreateVectorBucketInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateVectorBucketInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateVectorBucket request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_index_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteIndexInput, String> {
    let mut input = DeleteIndexInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteIndexInput>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteIndex request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_vector_bucket_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteVectorBucketInput, String> {
    let mut input = DeleteVectorBucketInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteVectorBucketInput>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteVectorBucket request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_vector_bucket_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteVectorBucketPolicyInput, String> {
    let mut input = DeleteVectorBucketPolicyInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteVectorBucketPolicyInput>(&request.body).map_err(
            |err| format!("failed to deserialize DeleteVectorBucketPolicy request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_vectors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteVectorsInput, String> {
    let mut input = DeleteVectorsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteVectorsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteVectors request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_index_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetIndexInput, String> {
    let mut input = GetIndexInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetIndexInput>(&request.body)
            .map_err(|err| format!("failed to deserialize GetIndex request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_vector_bucket_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetVectorBucketInput, String> {
    let mut input = GetVectorBucketInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetVectorBucketInput>(&request.body)
            .map_err(|err| format!("failed to deserialize GetVectorBucket request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_vector_bucket_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetVectorBucketPolicyInput, String> {
    let mut input = GetVectorBucketPolicyInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetVectorBucketPolicyInput>(&request.body)
            .map_err(|err| format!("failed to deserialize GetVectorBucketPolicy request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_vectors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetVectorsInput, String> {
    let mut input = GetVectorsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetVectorsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize GetVectors request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_indexes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIndexesInput, String> {
    let mut input = ListIndexesInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListIndexesInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListIndexes request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceInput, String> {
    let mut input = ListTagsForResourceInput::default();
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
pub fn deserialize_list_vector_buckets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVectorBucketsInput, String> {
    let mut input = ListVectorBucketsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListVectorBucketsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListVectorBuckets request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_vectors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVectorsInput, String> {
    let mut input = ListVectorsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListVectorsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListVectors request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_vector_bucket_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutVectorBucketPolicyInput, String> {
    let mut input = PutVectorBucketPolicyInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutVectorBucketPolicyInput>(&request.body)
            .map_err(|err| format!("failed to deserialize PutVectorBucketPolicy request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_vectors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutVectorsInput, String> {
    let mut input = PutVectorsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutVectorsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize PutVectors request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_query_vectors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<QueryVectorsInput, String> {
    let mut input = QueryVectorsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<QueryVectorsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize QueryVectors request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceInput, String> {
    let mut input = TagResourceInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceInput>(&request.body)
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
) -> Result<UntagResourceInput, String> {
    let mut input = UntagResourceInput::default();
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
