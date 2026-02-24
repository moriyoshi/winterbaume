//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-s3-control

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize void response for restXml protocol.
pub fn serialize_associate_access_grants_identity_center_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_create_access_grant_response(result: &CreateAccessGrantResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_create_access_grants_instance_response(
    result: &CreateAccessGrantsInstanceResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_create_access_grants_location_response(
    result: &CreateAccessGrantsLocationResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_create_access_point_response(result: &CreateAccessPointResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_create_access_point_for_object_lambda_response(
    result: &CreateAccessPointForObjectLambdaResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_create_bucket_response(result: &CreateBucketResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_job_response(result: &CreateJobResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_create_multi_region_access_point_response(
    result: &CreateMultiRegionAccessPointResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize void response for restXml protocol.
pub fn serialize_create_storage_lens_group_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_access_grant_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_access_grants_instance_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_access_grants_instance_resource_policy_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_access_grants_location_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_access_point_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_access_point_for_object_lambda_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_access_point_policy_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_access_point_policy_for_object_lambda_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_access_point_scope_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_lifecycle_configuration_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_policy_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_replication_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_tagging_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_job_tagging_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_multi_region_access_point_response(
    result: &DeleteMultiRegionAccessPointResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_public_access_block_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_storage_lens_configuration_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_storage_lens_configuration_tagging_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_storage_lens_group_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_describe_job_response(result: &DescribeJobResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_describe_multi_region_access_point_operation_response(
    result: &DescribeMultiRegionAccessPointOperationResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize void response for restXml protocol.
pub fn serialize_dissociate_access_grants_identity_center_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_get_access_grant_response(result: &GetAccessGrantResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_access_grants_instance_response(
    result: &GetAccessGrantsInstanceResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_access_grants_instance_for_prefix_response(
    result: &GetAccessGrantsInstanceForPrefixResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_access_grants_instance_resource_policy_response(
    result: &GetAccessGrantsInstanceResourcePolicyResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_access_grants_location_response(
    result: &GetAccessGrantsLocationResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_access_point_response(result: &GetAccessPointResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_access_point_configuration_for_object_lambda_response(
    result: &GetAccessPointConfigurationForObjectLambdaResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_access_point_for_object_lambda_response(
    result: &GetAccessPointForObjectLambdaResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_access_point_policy_response(
    result: &GetAccessPointPolicyResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_access_point_policy_for_object_lambda_response(
    result: &GetAccessPointPolicyForObjectLambdaResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_access_point_policy_status_response(
    result: &GetAccessPointPolicyStatusResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_access_point_policy_status_for_object_lambda_response(
    result: &GetAccessPointPolicyStatusForObjectLambdaResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_access_point_scope_response(
    result: &GetAccessPointScopeResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_response(result: &GetBucketResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_lifecycle_configuration_response(
    result: &GetBucketLifecycleConfigurationResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_policy_response(result: &GetBucketPolicyResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_replication_response(
    result: &GetBucketReplicationResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_tagging_response(result: &GetBucketTaggingResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_versioning_response(
    result: &GetBucketVersioningResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_data_access_response(result: &GetDataAccessResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_job_tagging_response(result: &GetJobTaggingResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_multi_region_access_point_response(
    result: &GetMultiRegionAccessPointResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_multi_region_access_point_policy_response(
    result: &GetMultiRegionAccessPointPolicyResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_multi_region_access_point_policy_status_response(
    result: &GetMultiRegionAccessPointPolicyStatusResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_multi_region_access_point_routes_response(
    result: &GetMultiRegionAccessPointRoutesResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_public_access_block_response(
    result: &PublicAccessBlockConfiguration,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_storage_lens_configuration_response(
    result: &StorageLensConfiguration,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_storage_lens_configuration_tagging_response(
    result: &GetStorageLensConfigurationTaggingResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_storage_lens_group_response(result: &StorageLensGroup) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_access_grants_response(result: &ListAccessGrantsResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_access_grants_instances_response(
    result: &ListAccessGrantsInstancesResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_access_grants_locations_response(
    result: &ListAccessGrantsLocationsResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_access_points_response(result: &ListAccessPointsResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_access_points_for_directory_buckets_response(
    result: &ListAccessPointsForDirectoryBucketsResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_access_points_for_object_lambda_response(
    result: &ListAccessPointsForObjectLambdaResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_caller_access_grants_response(
    result: &ListCallerAccessGrantsResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_jobs_response(result: &ListJobsResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_multi_region_access_points_response(
    result: &ListMultiRegionAccessPointsResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_regional_buckets_response(
    result: &ListRegionalBucketsResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_storage_lens_configurations_response(
    result: &ListStorageLensConfigurationsResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_storage_lens_groups_response(
    result: &ListStorageLensGroupsResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_put_access_grants_instance_resource_policy_response(
    result: &PutAccessGrantsInstanceResourcePolicyResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_access_point_configuration_for_object_lambda_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_access_point_policy_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_access_point_policy_for_object_lambda_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_access_point_scope_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_lifecycle_configuration_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_policy_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_replication_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_tagging_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_versioning_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_put_job_tagging_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_put_multi_region_access_point_policy_response(
    result: &PutMultiRegionAccessPointPolicyResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_public_access_block_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_storage_lens_configuration_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_put_storage_lens_configuration_tagging_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_submit_multi_region_access_point_routes_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(204, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(204, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_update_access_grants_location_response(
    result: &UpdateAccessGrantsLocationResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_update_job_priority_response(result: &UpdateJobPriorityResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_update_job_status_response(result: &UpdateJobStatusResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize void response for restXml protocol.
pub fn serialize_update_storage_lens_group_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Deserialize request for restXml protocol.
pub fn deserialize_associate_access_grants_identity_center_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateAccessGrantsIdentityCenterRequest, String> {
    let mut input = AssociateAccessGrantsIdentityCenterRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<AssociateAccessGrantsIdentityCenterRequest>(body)
            .map_err(|err| {
                format!("failed to deserialize AssociateAccessGrantsIdentityCenter request: {err}")
            })?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_access_grant_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAccessGrantRequest, String> {
    let mut input = CreateAccessGrantRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateAccessGrantRequest>(body)
            .map_err(|err| format!("failed to deserialize CreateAccessGrant request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_access_grants_instance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAccessGrantsInstanceRequest, String> {
    let mut input = CreateAccessGrantsInstanceRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<CreateAccessGrantsInstanceRequest>(body).map_err(|err| {
                format!("failed to deserialize CreateAccessGrantsInstance request: {err}")
            })?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_access_grants_location_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAccessGrantsLocationRequest, String> {
    let mut input = CreateAccessGrantsLocationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<CreateAccessGrantsLocationRequest>(body).map_err(|err| {
                format!("failed to deserialize CreateAccessGrantsLocation request: {err}")
            })?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_access_point_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAccessPointRequest, String> {
    let mut input = CreateAccessPointRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateAccessPointRequest>(body)
            .map_err(|err| format!("failed to deserialize CreateAccessPoint request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_access_point_for_object_lambda_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAccessPointForObjectLambdaRequest, String> {
    let mut input = CreateAccessPointForObjectLambdaRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateAccessPointForObjectLambdaRequest>(body).map_err(
            |err| format!("failed to deserialize CreateAccessPointForObjectLambda request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_bucket_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBucketRequest, String> {
    let mut input = CreateBucketRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<CreateBucketConfiguration>(body)
            .map_err(|err| format!("failed to deserialize CreateBucket payload: {err}"))?;
        input.create_bucket_configuration = Some(payload);
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-acl")
        .and_then(|value| value.to_str().ok())
    {
        input.a_c_l = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-full-control")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_full_control = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-read")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_read = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-read-acp")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_read_a_c_p = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-write")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_write = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-write-acp")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_write_a_c_p = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-bucket-object-lock-enabled")
        .and_then(|value| value.to_str().ok())
    {
        input.object_lock_enabled_for_bucket = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-outpost-id")
        .and_then(|value| value.to_str().ok())
    {
        input.outpost_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateJobRequest, String> {
    let mut input = CreateJobRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateJobRequest>(body)
            .map_err(|err| format!("failed to deserialize CreateJob request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_multi_region_access_point_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateMultiRegionAccessPointRequest, String> {
    let mut input = CreateMultiRegionAccessPointRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateMultiRegionAccessPointRequest>(body).map_err(
            |err| format!("failed to deserialize CreateMultiRegionAccessPoint request: {err}"),
        )?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_storage_lens_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateStorageLensGroupRequest, String> {
    let mut input = CreateStorageLensGroupRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateStorageLensGroupRequest>(body).map_err(|err| {
            format!("failed to deserialize CreateStorageLensGroup request: {err}")
        })?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_access_grant_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccessGrantRequest, String> {
    let mut input = DeleteAccessGrantRequest::default();
    for (name, value) in labels {
        match *name {
            "AccessGrantId" => {
                input.access_grant_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_access_grants_instance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccessGrantsInstanceRequest, String> {
    let mut input = DeleteAccessGrantsInstanceRequest::default();
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_access_grants_instance_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccessGrantsInstanceResourcePolicyRequest, String> {
    let mut input = DeleteAccessGrantsInstanceResourcePolicyRequest::default();
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_access_grants_location_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccessGrantsLocationRequest, String> {
    let mut input = DeleteAccessGrantsLocationRequest::default();
    for (name, value) in labels {
        match *name {
            "AccessGrantsLocationId" => {
                input.access_grants_location_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_access_point_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccessPointRequest, String> {
    let mut input = DeleteAccessPointRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_access_point_for_object_lambda_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccessPointForObjectLambdaRequest, String> {
    let mut input = DeleteAccessPointForObjectLambdaRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_access_point_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccessPointPolicyRequest, String> {
    let mut input = DeleteAccessPointPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_access_point_policy_for_object_lambda_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccessPointPolicyForObjectLambdaRequest, String> {
    let mut input = DeleteAccessPointPolicyForObjectLambdaRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_access_point_scope_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccessPointScopeRequest, String> {
    let mut input = DeleteAccessPointScopeRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketRequest, String> {
    let mut input = DeleteBucketRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_lifecycle_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketLifecycleConfigurationRequest, String> {
    let mut input = DeleteBucketLifecycleConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketPolicyRequest, String> {
    let mut input = DeleteBucketPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_replication_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketReplicationRequest, String> {
    let mut input = DeleteBucketReplicationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_tagging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketTaggingRequest, String> {
    let mut input = DeleteBucketTaggingRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_job_tagging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteJobTaggingRequest, String> {
    let mut input = DeleteJobTaggingRequest::default();
    for (name, value) in labels {
        match *name {
            "JobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_multi_region_access_point_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteMultiRegionAccessPointRequest, String> {
    let mut input = DeleteMultiRegionAccessPointRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<DeleteMultiRegionAccessPointRequest>(body).map_err(
            |err| format!("failed to deserialize DeleteMultiRegionAccessPoint request: {err}"),
        )?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_public_access_block_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePublicAccessBlockRequest, String> {
    let mut input = DeletePublicAccessBlockRequest::default();
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_storage_lens_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteStorageLensConfigurationRequest, String> {
    let mut input = DeleteStorageLensConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "ConfigId" => {
                input.config_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_storage_lens_configuration_tagging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteStorageLensConfigurationTaggingRequest, String> {
    let mut input = DeleteStorageLensConfigurationTaggingRequest::default();
    for (name, value) in labels {
        match *name {
            "ConfigId" => {
                input.config_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_storage_lens_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteStorageLensGroupRequest, String> {
    let mut input = DeleteStorageLensGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_describe_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeJobRequest, String> {
    let mut input = DescribeJobRequest::default();
    for (name, value) in labels {
        match *name {
            "JobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_describe_multi_region_access_point_operation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeMultiRegionAccessPointOperationRequest, String> {
    let mut input = DescribeMultiRegionAccessPointOperationRequest::default();
    for (name, value) in labels {
        match *name {
            "RequestTokenARN" => {
                input.request_token_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_dissociate_access_grants_identity_center_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DissociateAccessGrantsIdentityCenterRequest, String> {
    let mut input = DissociateAccessGrantsIdentityCenterRequest::default();
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_access_grant_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccessGrantRequest, String> {
    let mut input = GetAccessGrantRequest::default();
    for (name, value) in labels {
        match *name {
            "AccessGrantId" => {
                input.access_grant_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_access_grants_instance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccessGrantsInstanceRequest, String> {
    let mut input = GetAccessGrantsInstanceRequest::default();
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_access_grants_instance_for_prefix_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccessGrantsInstanceForPrefixRequest, String> {
    let mut input = GetAccessGrantsInstanceForPrefixRequest::default();
    if let Some(value) = query.get("s3prefix") {
        input.s3_prefix = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_access_grants_instance_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccessGrantsInstanceResourcePolicyRequest, String> {
    let mut input = GetAccessGrantsInstanceResourcePolicyRequest::default();
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_access_grants_location_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccessGrantsLocationRequest, String> {
    let mut input = GetAccessGrantsLocationRequest::default();
    for (name, value) in labels {
        match *name {
            "AccessGrantsLocationId" => {
                input.access_grants_location_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_access_point_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccessPointRequest, String> {
    let mut input = GetAccessPointRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_access_point_configuration_for_object_lambda_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccessPointConfigurationForObjectLambdaRequest, String> {
    let mut input = GetAccessPointConfigurationForObjectLambdaRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_access_point_for_object_lambda_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccessPointForObjectLambdaRequest, String> {
    let mut input = GetAccessPointForObjectLambdaRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_access_point_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccessPointPolicyRequest, String> {
    let mut input = GetAccessPointPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_access_point_policy_for_object_lambda_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccessPointPolicyForObjectLambdaRequest, String> {
    let mut input = GetAccessPointPolicyForObjectLambdaRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_access_point_policy_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccessPointPolicyStatusRequest, String> {
    let mut input = GetAccessPointPolicyStatusRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_access_point_policy_status_for_object_lambda_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccessPointPolicyStatusForObjectLambdaRequest, String> {
    let mut input = GetAccessPointPolicyStatusForObjectLambdaRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_access_point_scope_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccessPointScopeRequest, String> {
    let mut input = GetAccessPointScopeRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketRequest, String> {
    let mut input = GetBucketRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_lifecycle_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketLifecycleConfigurationRequest, String> {
    let mut input = GetBucketLifecycleConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketPolicyRequest, String> {
    let mut input = GetBucketPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_replication_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketReplicationRequest, String> {
    let mut input = GetBucketReplicationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_tagging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketTaggingRequest, String> {
    let mut input = GetBucketTaggingRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_versioning_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketVersioningRequest, String> {
    let mut input = GetBucketVersioningRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_data_access_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDataAccessRequest, String> {
    let mut input = GetDataAccessRequest::default();
    if let Some(value) = query.get("durationSeconds") {
        input.duration_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("permission") {
        input.permission = value.to_string();
    }
    if let Some(value) = query.get("privilege") {
        input.privilege = Some(value.to_string());
    }
    if let Some(value) = query.get("target") {
        input.target = value.to_string();
    }
    if let Some(value) = query.get("targetType") {
        input.target_type = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_job_tagging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetJobTaggingRequest, String> {
    let mut input = GetJobTaggingRequest::default();
    for (name, value) in labels {
        match *name {
            "JobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_multi_region_access_point_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMultiRegionAccessPointRequest, String> {
    let mut input = GetMultiRegionAccessPointRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_multi_region_access_point_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMultiRegionAccessPointPolicyRequest, String> {
    let mut input = GetMultiRegionAccessPointPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_multi_region_access_point_policy_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMultiRegionAccessPointPolicyStatusRequest, String> {
    let mut input = GetMultiRegionAccessPointPolicyStatusRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_multi_region_access_point_routes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMultiRegionAccessPointRoutesRequest, String> {
    let mut input = GetMultiRegionAccessPointRoutesRequest::default();
    for (name, value) in labels {
        match *name {
            "Mrap" => {
                input.mrap = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_public_access_block_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPublicAccessBlockRequest, String> {
    let mut input = GetPublicAccessBlockRequest::default();
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_storage_lens_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetStorageLensConfigurationRequest, String> {
    let mut input = GetStorageLensConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "ConfigId" => {
                input.config_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_storage_lens_configuration_tagging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetStorageLensConfigurationTaggingRequest, String> {
    let mut input = GetStorageLensConfigurationTaggingRequest::default();
    for (name, value) in labels {
        match *name {
            "ConfigId" => {
                input.config_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_storage_lens_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetStorageLensGroupRequest, String> {
    let mut input = GetStorageLensGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_access_grants_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAccessGrantsRequest, String> {
    let mut input = ListAccessGrantsRequest::default();
    if let Some(value) = query.get("application_arn") {
        input.application_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("grantscope") {
        input.grant_scope = Some(value.to_string());
    }
    if let Some(value) = query.get("granteeidentifier") {
        input.grantee_identifier = Some(value.to_string());
    }
    if let Some(value) = query.get("granteetype") {
        input.grantee_type = Some(value.to_string());
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
    if let Some(value) = query.get("permission") {
        input.permission = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_access_grants_instances_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAccessGrantsInstancesRequest, String> {
    let mut input = ListAccessGrantsInstancesRequest::default();
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
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_access_grants_locations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAccessGrantsLocationsRequest, String> {
    let mut input = ListAccessGrantsLocationsRequest::default();
    if let Some(value) = query.get("locationscope") {
        input.location_scope = Some(value.to_string());
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
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_access_points_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAccessPointsRequest, String> {
    let mut input = ListAccessPointsRequest::default();
    if let Some(value) = query.get("bucket") {
        input.bucket = Some(value.to_string());
    }
    if let Some(value) = query.get("dataSourceId") {
        input.data_source_id = Some(value.to_string());
    }
    if let Some(value) = query.get("dataSourceType") {
        input.data_source_type = Some(value.to_string());
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
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_access_points_for_directory_buckets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAccessPointsForDirectoryBucketsRequest, String> {
    let mut input = ListAccessPointsForDirectoryBucketsRequest::default();
    if let Some(value) = query.get("directoryBucket") {
        input.directory_bucket = Some(value.to_string());
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
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_access_points_for_object_lambda_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAccessPointsForObjectLambdaRequest, String> {
    let mut input = ListAccessPointsForObjectLambdaRequest::default();
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
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_caller_access_grants_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCallerAccessGrantsRequest, String> {
    let mut input = ListCallerAccessGrantsRequest::default();
    if let Some(value) = query.get("allowedByApplication") {
        input.allowed_by_application = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("grantscope") {
        input.grant_scope = Some(value.to_string());
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
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListJobsRequest, String> {
    let mut input = ListJobsRequest::default();
    if let Some(value) = query.get("jobStatuses") {
        input.job_statuses = Some(JobStatusList {
            items: value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        });
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
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_multi_region_access_points_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMultiRegionAccessPointsRequest, String> {
    let mut input = ListMultiRegionAccessPointsRequest::default();
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
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_regional_buckets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRegionalBucketsRequest, String> {
    let mut input = ListRegionalBucketsRequest::default();
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
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-outpost-id")
        .and_then(|value| value.to_str().ok())
    {
        input.outpost_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_storage_lens_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListStorageLensConfigurationsRequest, String> {
    let mut input = ListStorageLensConfigurationsRequest::default();
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_storage_lens_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListStorageLensGroupsRequest, String> {
    let mut input = ListStorageLensGroupsRequest::default();
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceRequest, String> {
    let mut input = ListTagsForResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_access_grants_instance_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAccessGrantsInstanceResourcePolicyRequest, String> {
    let mut input = PutAccessGrantsInstanceResourcePolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<PutAccessGrantsInstanceResourcePolicyRequest>(body)
            .map_err(|err| {
                format!(
                    "failed to deserialize PutAccessGrantsInstanceResourcePolicy request: {err}"
                )
            })?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_access_point_configuration_for_object_lambda_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAccessPointConfigurationForObjectLambdaRequest, String> {
    let mut input = PutAccessPointConfigurationForObjectLambdaRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<PutAccessPointConfigurationForObjectLambdaRequest>(body)
            .map_err(|err| format!("failed to deserialize PutAccessPointConfigurationForObjectLambda request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_access_point_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAccessPointPolicyRequest, String> {
    let mut input = PutAccessPointPolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<PutAccessPointPolicyRequest>(body)
            .map_err(|err| format!("failed to deserialize PutAccessPointPolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_access_point_policy_for_object_lambda_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAccessPointPolicyForObjectLambdaRequest, String> {
    let mut input = PutAccessPointPolicyForObjectLambdaRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<PutAccessPointPolicyForObjectLambdaRequest>(body)
            .map_err(|err| {
                format!("failed to deserialize PutAccessPointPolicyForObjectLambda request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_access_point_scope_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAccessPointScopeRequest, String> {
    let mut input = PutAccessPointScopeRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<PutAccessPointScopeRequest>(body)
            .map_err(|err| format!("failed to deserialize PutAccessPointScope request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_lifecycle_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketLifecycleConfigurationRequest, String> {
    let mut input = PutBucketLifecycleConfigurationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<LifecycleConfiguration>(body).map_err(|err| {
            format!("failed to deserialize PutBucketLifecycleConfiguration payload: {err}")
        })?;
        input.lifecycle_configuration = Some(payload);
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketPolicyRequest, String> {
    let mut input = PutBucketPolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<PutBucketPolicyRequest>(body)
            .map_err(|err| format!("failed to deserialize PutBucketPolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-confirm-remove-self-bucket-access")
        .and_then(|value| value.to_str().ok())
    {
        input.confirm_remove_self_bucket_access = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_replication_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketReplicationRequest, String> {
    let mut input = PutBucketReplicationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<ReplicationConfiguration>(body)
            .map_err(|err| format!("failed to deserialize PutBucketReplication payload: {err}"))?;
        input.replication_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_tagging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketTaggingRequest, String> {
    let mut input = PutBucketTaggingRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<Tagging>(body)
            .map_err(|err| format!("failed to deserialize PutBucketTagging payload: {err}"))?;
        input.tagging = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_versioning_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketVersioningRequest, String> {
    let mut input = PutBucketVersioningRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<VersioningConfiguration>(body)
            .map_err(|err| format!("failed to deserialize PutBucketVersioning payload: {err}"))?;
        input.versioning_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-mfa")
        .and_then(|value| value.to_str().ok())
    {
        input.m_f_a = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_job_tagging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutJobTaggingRequest, String> {
    let mut input = PutJobTaggingRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<PutJobTaggingRequest>(body)
            .map_err(|err| format!("failed to deserialize PutJobTagging request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "JobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_multi_region_access_point_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutMultiRegionAccessPointPolicyRequest, String> {
    let mut input = PutMultiRegionAccessPointPolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<PutMultiRegionAccessPointPolicyRequest>(body).map_err(
            |err| format!("failed to deserialize PutMultiRegionAccessPointPolicy request: {err}"),
        )?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_public_access_block_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutPublicAccessBlockRequest, String> {
    let mut input = PutPublicAccessBlockRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<PublicAccessBlockConfiguration>(body)
            .map_err(|err| format!("failed to deserialize PutPublicAccessBlock payload: {err}"))?;
        input.public_access_block_configuration = payload;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_storage_lens_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutStorageLensConfigurationRequest, String> {
    let mut input = PutStorageLensConfigurationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<PutStorageLensConfigurationRequest>(body).map_err(|err| {
                format!("failed to deserialize PutStorageLensConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ConfigId" => {
                input.config_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_storage_lens_configuration_tagging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutStorageLensConfigurationTaggingRequest, String> {
    let mut input = PutStorageLensConfigurationTaggingRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<PutStorageLensConfigurationTaggingRequest>(body)
            .map_err(|err| {
                format!("failed to deserialize PutStorageLensConfigurationTagging request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ConfigId" => {
                input.config_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_submit_multi_region_access_point_routes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SubmitMultiRegionAccessPointRoutesRequest, String> {
    let mut input = SubmitMultiRegionAccessPointRoutesRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<SubmitMultiRegionAccessPointRoutesRequest>(body)
            .map_err(|err| {
                format!("failed to deserialize SubmitMultiRegionAccessPointRoutes request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "Mrap" => {
                input.mrap = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceRequest, String> {
    let mut input = TagResourceRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<TagResourceRequest>(body)
            .map_err(|err| format!("failed to deserialize TagResource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("tagKeys") {
        input.tag_keys = TagKeyList {
            items: value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        };
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_access_grants_location_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAccessGrantsLocationRequest, String> {
    let mut input = UpdateAccessGrantsLocationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<UpdateAccessGrantsLocationRequest>(body).map_err(|err| {
                format!("failed to deserialize UpdateAccessGrantsLocation request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AccessGrantsLocationId" => {
                input.access_grants_location_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_job_priority_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateJobPriorityRequest, String> {
    let mut input = UpdateJobPriorityRequest::default();
    for (name, value) in labels {
        match *name {
            "JobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("priority") {
        input.priority = value
            .parse::<i32>()
            .map_err(|err| format!("failed to parse integer: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_job_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateJobStatusRequest, String> {
    let mut input = UpdateJobStatusRequest::default();
    for (name, value) in labels {
        match *name {
            "JobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("requestedJobStatus") {
        input.requested_job_status = value.to_string();
    }
    if let Some(value) = query.get("statusUpdateReason") {
        input.status_update_reason = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_storage_lens_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateStorageLensGroupRequest, String> {
    let mut input = UpdateStorageLensGroupRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<UpdateStorageLensGroupRequest>(body).map_err(|err| {
            format!("failed to deserialize UpdateStorageLensGroup request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-account-id")
        .and_then(|value| value.to_str().ok())
    {
        input.account_id = value.to_string();
    }
    Ok(input)
}
