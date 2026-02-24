//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-kafka

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
pub fn serialize_batch_associate_scram_secret_response(
    result: &BatchAssociateScramSecretResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_disassociate_scram_secret_response(
    result: &BatchDisassociateScramSecretResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_cluster_response(result: &CreateClusterResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_cluster_v2_response(result: &CreateClusterV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_configuration_response(
    result: &CreateConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_replicator_response(result: &CreateReplicatorResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_topic_response(result: &CreateTopicResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_vpc_connection_response(
    result: &CreateVpcConnectionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_cluster_response(result: &DeleteClusterResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_cluster_policy_response(
    result: &DeleteClusterPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_configuration_response(
    result: &DeleteConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_replicator_response(result: &DeleteReplicatorResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_topic_response(result: &DeleteTopicResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_vpc_connection_response(
    result: &DeleteVpcConnectionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_cluster_response(result: &DescribeClusterResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_cluster_operation_response(
    result: &DescribeClusterOperationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_cluster_operation_v2_response(
    result: &DescribeClusterOperationV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_cluster_v2_response(result: &DescribeClusterV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_configuration_response(
    result: &DescribeConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_configuration_revision_response(
    result: &DescribeConfigurationRevisionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_replicator_response(result: &DescribeReplicatorResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_topic_response(result: &DescribeTopicResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_topic_partitions_response(
    result: &DescribeTopicPartitionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_vpc_connection_response(
    result: &DescribeVpcConnectionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_bootstrap_brokers_response(
    result: &GetBootstrapBrokersResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_cluster_policy_response(result: &GetClusterPolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_compatible_kafka_versions_response(
    result: &GetCompatibleKafkaVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_client_vpc_connections_response(
    result: &ListClientVpcConnectionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_cluster_operations_response(
    result: &ListClusterOperationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_cluster_operations_v2_response(
    result: &ListClusterOperationsV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_clusters_response(result: &ListClustersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_clusters_v2_response(result: &ListClustersV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_configuration_revisions_response(
    result: &ListConfigurationRevisionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_configurations_response(result: &ListConfigurationsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_kafka_versions_response(result: &ListKafkaVersionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_nodes_response(result: &ListNodesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_replicators_response(result: &ListReplicatorsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_scram_secrets_response(result: &ListScramSecretsResponse) -> MockResponse {
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
pub fn serialize_list_topics_response(result: &ListTopicsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_vpc_connections_response(
    result: &ListVpcConnectionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_cluster_policy_response(result: &PutClusterPolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_reboot_broker_response(result: &RebootBrokerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_reject_client_vpc_connection_response(
    result: &RejectClientVpcConnectionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_broker_count_response(result: &UpdateBrokerCountResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_broker_storage_response(
    result: &UpdateBrokerStorageResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_broker_type_response(result: &UpdateBrokerTypeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_cluster_configuration_response(
    result: &UpdateClusterConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_cluster_kafka_version_response(
    result: &UpdateClusterKafkaVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_configuration_response(
    result: &UpdateConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_connectivity_response(result: &UpdateConnectivityResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_monitoring_response(result: &UpdateMonitoringResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_rebalancing_response(result: &UpdateRebalancingResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_replication_info_response(
    result: &UpdateReplicationInfoResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_security_response(result: &UpdateSecurityResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_storage_response(result: &UpdateStorageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_topic_response(result: &UpdateTopicResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_associate_scram_secret_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchAssociateScramSecretRequest, String> {
    let mut input = BatchAssociateScramSecretRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchAssociateScramSecretRequest>(&request.body).map_err(
            |err| format!("failed to deserialize BatchAssociateScramSecret request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_disassociate_scram_secret_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchDisassociateScramSecretRequest, String> {
    let mut input = BatchDisassociateScramSecretRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchDisassociateScramSecretRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize BatchDisassociateScramSecret request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_cluster_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateClusterRequest, String> {
    let mut input = CreateClusterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateClusterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateCluster request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_cluster_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateClusterV2Request, String> {
    let mut input = CreateClusterV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateClusterV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateClusterV2 request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConfigurationRequest, String> {
    let mut input = CreateConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateConfigurationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateConfiguration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_replicator_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateReplicatorRequest, String> {
    let mut input = CreateReplicatorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateReplicatorRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateReplicator request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_topic_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTopicRequest, String> {
    let mut input = CreateTopicRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTopicRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTopic request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_vpc_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateVpcConnectionRequest, String> {
    let mut input = CreateVpcConnectionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateVpcConnectionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateVpcConnection request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_cluster_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteClusterRequest, String> {
    let mut input = DeleteClusterRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("currentVersion") {
        input.current_version = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_cluster_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteClusterPolicyRequest, String> {
    let mut input = DeleteClusterPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteConfigurationRequest, String> {
    let mut input = DeleteConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_replicator_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteReplicatorRequest, String> {
    let mut input = DeleteReplicatorRequest::default();
    for (name, value) in labels {
        match *name {
            "ReplicatorArn" => {
                input.replicator_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("currentVersion") {
        input.current_version = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_topic_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTopicRequest, String> {
    let mut input = DeleteTopicRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            "TopicName" => {
                input.topic_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_vpc_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteVpcConnectionRequest, String> {
    let mut input = DeleteVpcConnectionRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_cluster_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeClusterRequest, String> {
    let mut input = DescribeClusterRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_cluster_operation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeClusterOperationRequest, String> {
    let mut input = DescribeClusterOperationRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterOperationArn" => {
                input.cluster_operation_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_cluster_operation_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeClusterOperationV2Request, String> {
    let mut input = DescribeClusterOperationV2Request::default();
    for (name, value) in labels {
        match *name {
            "ClusterOperationArn" => {
                input.cluster_operation_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_cluster_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeClusterV2Request, String> {
    let mut input = DescribeClusterV2Request::default();
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeConfigurationRequest, String> {
    let mut input = DescribeConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_configuration_revision_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeConfigurationRevisionRequest, String> {
    let mut input = DescribeConfigurationRevisionRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            "Revision" => {
                input.revision = value
                    .parse::<i64>()
                    .map_err(|err| format!("failed to parse long: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_replicator_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeReplicatorRequest, String> {
    let mut input = DescribeReplicatorRequest::default();
    for (name, value) in labels {
        match *name {
            "ReplicatorArn" => {
                input.replicator_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_topic_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTopicRequest, String> {
    let mut input = DescribeTopicRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            "TopicName" => {
                input.topic_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_topic_partitions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTopicPartitionsRequest, String> {
    let mut input = DescribeTopicPartitionsRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            "TopicName" => {
                input.topic_name = value.to_string();
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
pub fn deserialize_describe_vpc_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeVpcConnectionRequest, String> {
    let mut input = DescribeVpcConnectionRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_bootstrap_brokers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBootstrapBrokersRequest, String> {
    let mut input = GetBootstrapBrokersRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_cluster_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetClusterPolicyRequest, String> {
    let mut input = GetClusterPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_compatible_kafka_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCompatibleKafkaVersionsRequest, String> {
    let mut input = GetCompatibleKafkaVersionsRequest::default();
    if let Some(value) = query.get("clusterArn") {
        input.cluster_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_client_vpc_connections_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListClientVpcConnectionsRequest, String> {
    let mut input = ListClientVpcConnectionsRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
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
pub fn deserialize_list_cluster_operations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListClusterOperationsRequest, String> {
    let mut input = ListClusterOperationsRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
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
pub fn deserialize_list_cluster_operations_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListClusterOperationsV2Request, String> {
    let mut input = ListClusterOperationsV2Request::default();
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
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
pub fn deserialize_list_clusters_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListClustersRequest, String> {
    let mut input = ListClustersRequest::default();
    if let Some(value) = query.get("clusterNameFilter") {
        input.cluster_name_filter = Some(value.to_string());
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
pub fn deserialize_list_clusters_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListClustersV2Request, String> {
    let mut input = ListClustersV2Request::default();
    if let Some(value) = query.get("clusterNameFilter") {
        input.cluster_name_filter = Some(value.to_string());
    }
    if let Some(value) = query.get("clusterTypeFilter") {
        input.cluster_type_filter = Some(value.to_string());
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
pub fn deserialize_list_configuration_revisions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConfigurationRevisionsRequest, String> {
    let mut input = ListConfigurationRevisionsRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
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
pub fn deserialize_list_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConfigurationsRequest, String> {
    let mut input = ListConfigurationsRequest::default();
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
pub fn deserialize_list_kafka_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListKafkaVersionsRequest, String> {
    let mut input = ListKafkaVersionsRequest::default();
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
pub fn deserialize_list_nodes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListNodesRequest, String> {
    let mut input = ListNodesRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
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
pub fn deserialize_list_replicators_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListReplicatorsRequest, String> {
    let mut input = ListReplicatorsRequest::default();
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
    if let Some(value) = query.get("replicatorNameFilter") {
        input.replicator_name_filter = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_scram_secrets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListScramSecretsRequest, String> {
    let mut input = ListScramSecretsRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
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
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_topics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTopicsRequest, String> {
    let mut input = ListTopicsRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
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
    if let Some(value) = query.get("topicNameFilter") {
        input.topic_name_filter = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_vpc_connections_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVpcConnectionsRequest, String> {
    let mut input = ListVpcConnectionsRequest::default();
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
pub fn deserialize_put_cluster_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutClusterPolicyRequest, String> {
    let mut input = PutClusterPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutClusterPolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutClusterPolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_reboot_broker_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RebootBrokerRequest, String> {
    let mut input = RebootBrokerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RebootBrokerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RebootBroker request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_reject_client_vpc_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RejectClientVpcConnectionRequest, String> {
    let mut input = RejectClientVpcConnectionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RejectClientVpcConnectionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize RejectClientVpcConnection request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
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
            "ResourceArn" => {
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
            "ResourceArn" => {
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
pub fn deserialize_update_broker_count_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBrokerCountRequest, String> {
    let mut input = UpdateBrokerCountRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBrokerCountRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBrokerCount request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_broker_storage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBrokerStorageRequest, String> {
    let mut input = UpdateBrokerStorageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBrokerStorageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBrokerStorage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_broker_type_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBrokerTypeRequest, String> {
    let mut input = UpdateBrokerTypeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBrokerTypeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBrokerType request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_cluster_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateClusterConfigurationRequest, String> {
    let mut input = UpdateClusterConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateClusterConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateClusterConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_cluster_kafka_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateClusterKafkaVersionRequest, String> {
    let mut input = UpdateClusterKafkaVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateClusterKafkaVersionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateClusterKafkaVersion request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConfigurationRequest, String> {
    let mut input = UpdateConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateConfigurationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateConfiguration request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_connectivity_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConnectivityRequest, String> {
    let mut input = UpdateConnectivityRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateConnectivityRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateConnectivity request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_monitoring_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateMonitoringRequest, String> {
    let mut input = UpdateMonitoringRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateMonitoringRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateMonitoring request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_rebalancing_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRebalancingRequest, String> {
    let mut input = UpdateRebalancingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRebalancingRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateRebalancing request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_replication_info_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateReplicationInfoRequest, String> {
    let mut input = UpdateReplicationInfoRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateReplicationInfoRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateReplicationInfo request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ReplicatorArn" => {
                input.replicator_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_security_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSecurityRequest, String> {
    let mut input = UpdateSecurityRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSecurityRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateSecurity request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_storage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateStorageRequest, String> {
    let mut input = UpdateStorageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateStorageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateStorage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_topic_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTopicRequest, String> {
    let mut input = UpdateTopicRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTopicRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateTopic request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterArn" => {
                input.cluster_arn = value.to_string();
            }
            "TopicName" => {
                input.topic_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
