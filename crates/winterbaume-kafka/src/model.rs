//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-kafka

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAssociateScramSecretRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "secretArnList")]
    #[serde(default)]
    pub secret_arn_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAssociateScramSecretResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "unprocessedScramSecrets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_scram_secrets: Option<Vec<UnprocessedScramSecret>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedScramSecret {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDisassociateScramSecretRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "secretArnList")]
    #[serde(default)]
    pub secret_arn_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDisassociateScramSecretResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "unprocessedScramSecrets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_scram_secrets: Option<Vec<UnprocessedScramSecret>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterRequest {
    #[serde(rename = "brokerNodeGroupInfo")]
    #[serde(default)]
    pub broker_node_group_info: BrokerNodeGroupInfo,
    #[serde(rename = "clientAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication: Option<ClientAuthentication>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "configurationInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_info: Option<ConfigurationInfo>,
    #[serde(rename = "encryptionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_info: Option<EncryptionInfo>,
    #[serde(rename = "enhancedMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring: Option<String>,
    #[serde(rename = "kafkaVersion")]
    #[serde(default)]
    pub kafka_version: String,
    #[serde(rename = "loggingInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    #[serde(rename = "numberOfBrokerNodes")]
    #[serde(default)]
    pub number_of_broker_nodes: i32,
    #[serde(rename = "openMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_monitoring: Option<OpenMonitoringInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebalancing: Option<Rebalancing>,
    #[serde(rename = "storageMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrokerNodeGroupInfo {
    #[serde(rename = "brokerAZDistribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_a_z_distribution: Option<String>,
    #[serde(rename = "clientSubnets")]
    #[serde(default)]
    pub client_subnets: Vec<String>,
    #[serde(rename = "connectivityInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_info: Option<ConnectivityInfo>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    pub instance_type: String,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "storageInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_info: Option<StorageInfo>,
    #[serde(rename = "zoneIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectivityInfo {
    #[serde(rename = "networkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "publicAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access: Option<PublicAccess>,
    #[serde(rename = "vpcConnectivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connectivity: Option<VpcConnectivity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublicAccess {
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConnectivity {
    #[serde(rename = "clientAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication: Option<VpcConnectivityClientAuthentication>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConnectivityClientAuthentication {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl: Option<VpcConnectivitySasl>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<VpcConnectivityTls>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConnectivitySasl {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam: Option<VpcConnectivityIam>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scram: Option<VpcConnectivityScram>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConnectivityIam {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConnectivityScram {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConnectivityTls {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageInfo {
    #[serde(rename = "ebsStorageInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_storage_info: Option<EBSStorageInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EBSStorageInfo {
    #[serde(rename = "provisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "volumeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionedThroughput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "volumeThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_throughput: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientAuthentication {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl: Option<Sasl>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<Tls>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unauthenticated: Option<Unauthenticated>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Sasl {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam: Option<Iam>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scram: Option<Scram>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Iam {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scram {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tls {
    #[serde(rename = "certificateAuthorityArnList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn_list: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Unauthenticated {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationInfo {
    #[serde(default)]
    pub arn: String,
    #[serde(default)]
    pub revision: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionInfo {
    #[serde(rename = "encryptionAtRest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest: Option<EncryptionAtRest>,
    #[serde(rename = "encryptionInTransit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_in_transit: Option<EncryptionInTransit>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionAtRest {
    #[serde(rename = "dataVolumeKMSKeyId")]
    #[serde(default)]
    pub data_volume_k_m_s_key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionInTransit {
    #[serde(rename = "clientBroker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_broker: Option<String>,
    #[serde(rename = "inCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_cluster: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingInfo {
    #[serde(rename = "brokerLogs")]
    #[serde(default)]
    pub broker_logs: BrokerLogs,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrokerLogs {
    #[serde(rename = "cloudWatchLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs: Option<CloudWatchLogs>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose: Option<Firehose>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLogs {
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "logGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Firehose {
    #[serde(rename = "deliveryStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream: Option<String>,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3 {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenMonitoringInfo {
    #[serde(default)]
    pub prometheus: PrometheusInfo,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrometheusInfo {
    #[serde(rename = "jmxExporter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jmx_exporter: Option<JmxExporterInfo>,
    #[serde(rename = "nodeExporter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_exporter: Option<NodeExporterInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JmxExporterInfo {
    #[serde(rename = "enabledInBroker")]
    #[serde(default)]
    pub enabled_in_broker: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeExporterInfo {
    #[serde(rename = "enabledInBroker")]
    #[serde(default)]
    pub enabled_in_broker: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Rebalancing {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterV2Request {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned: Option<ProvisionedRequest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless: Option<ServerlessRequest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionedRequest {
    #[serde(rename = "brokerNodeGroupInfo")]
    #[serde(default)]
    pub broker_node_group_info: BrokerNodeGroupInfo,
    #[serde(rename = "clientAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication: Option<ClientAuthentication>,
    #[serde(rename = "configurationInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_info: Option<ConfigurationInfo>,
    #[serde(rename = "encryptionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_info: Option<EncryptionInfo>,
    #[serde(rename = "enhancedMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring: Option<String>,
    #[serde(rename = "kafkaVersion")]
    #[serde(default)]
    pub kafka_version: String,
    #[serde(rename = "loggingInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    #[serde(rename = "numberOfBrokerNodes")]
    #[serde(default)]
    pub number_of_broker_nodes: i32,
    #[serde(rename = "openMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_monitoring: Option<OpenMonitoringInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebalancing: Option<Rebalancing>,
    #[serde(rename = "storageMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServerlessRequest {
    #[serde(rename = "clientAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication: Option<ServerlessClientAuthentication>,
    #[serde(rename = "vpcConfigs")]
    #[serde(default)]
    pub vpc_configs: Vec<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServerlessClientAuthentication {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl: Option<ServerlessSasl>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServerlessSasl {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam: Option<Iam>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfig {
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterV2Response {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "clusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "kafkaVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_versions: Option<Vec<String>>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "serverProperties")]
    #[serde(default)]
    pub server_properties: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "latestRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<ConfigurationRevision>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationRevision {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReplicatorRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "kafkaClusters")]
    #[serde(default)]
    pub kafka_clusters: Vec<KafkaCluster>,
    #[serde(rename = "logDelivery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery: Option<LogDelivery>,
    #[serde(rename = "replicationInfoList")]
    #[serde(default)]
    pub replication_info_list: Vec<ReplicationInfo>,
    #[serde(rename = "replicatorName")]
    #[serde(default)]
    pub replicator_name: String,
    #[serde(rename = "serviceExecutionRoleArn")]
    #[serde(default)]
    pub service_execution_role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KafkaCluster {
    #[serde(rename = "amazonMskCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_msk_cluster: Option<AmazonMskCluster>,
    #[serde(rename = "apacheKafkaCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apache_kafka_cluster: Option<ApacheKafkaCluster>,
    #[serde(rename = "clientAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication: Option<KafkaClusterClientAuthentication>,
    #[serde(rename = "encryptionInTransit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_in_transit: Option<KafkaClusterEncryptionInTransit>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<KafkaClusterClientVpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonMskCluster {
    #[serde(rename = "mskClusterArn")]
    #[serde(default)]
    pub msk_cluster_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApacheKafkaCluster {
    #[serde(rename = "apacheKafkaClusterId")]
    #[serde(default)]
    pub apache_kafka_cluster_id: String,
    #[serde(rename = "bootstrapBrokerString")]
    #[serde(default)]
    pub bootstrap_broker_string: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KafkaClusterClientAuthentication {
    #[serde(rename = "saslScram")]
    #[serde(default)]
    pub sasl_scram: KafkaClusterSaslScramAuthentication,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KafkaClusterSaslScramAuthentication {
    #[serde(default)]
    pub mechanism: String,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    pub secret_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KafkaClusterEncryptionInTransit {
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    pub encryption_type: String,
    #[serde(rename = "rootCaCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_ca_certificate: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KafkaClusterClientVpcConfig {
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogDelivery {
    #[serde(rename = "replicatorLogDelivery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_log_delivery: Option<ReplicatorLogDelivery>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicatorLogDelivery {
    #[serde(rename = "cloudWatchLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs: Option<ReplicatorCloudWatchLogs>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose: Option<ReplicatorFirehose>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<ReplicatorS3>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicatorCloudWatchLogs {
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "logGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicatorFirehose {
    #[serde(rename = "deliveryStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream: Option<String>,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicatorS3 {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationInfo {
    #[serde(rename = "consumerGroupReplication")]
    #[serde(default)]
    pub consumer_group_replication: ConsumerGroupReplication,
    #[serde(rename = "sourceKafkaClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_kafka_cluster_arn: Option<String>,
    #[serde(rename = "sourceKafkaClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_kafka_cluster_id: Option<String>,
    #[serde(rename = "targetCompressionType")]
    #[serde(default)]
    pub target_compression_type: String,
    #[serde(rename = "targetKafkaClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_kafka_cluster_arn: Option<String>,
    #[serde(rename = "targetKafkaClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_kafka_cluster_id: Option<String>,
    #[serde(rename = "topicReplication")]
    #[serde(default)]
    pub topic_replication: TopicReplication,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConsumerGroupReplication {
    #[serde(rename = "consumerGroupOffsetSyncMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_offset_sync_mode: Option<String>,
    #[serde(rename = "consumerGroupsToExclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_groups_to_exclude: Option<Vec<String>>,
    #[serde(rename = "consumerGroupsToReplicate")]
    #[serde(default)]
    pub consumer_groups_to_replicate: Vec<String>,
    #[serde(rename = "detectAndCopyNewConsumerGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_and_copy_new_consumer_groups: Option<bool>,
    #[serde(rename = "synchroniseConsumerGroupOffsets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronise_consumer_group_offsets: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicReplication {
    #[serde(rename = "copyAccessControlListsForTopics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_access_control_lists_for_topics: Option<bool>,
    #[serde(rename = "copyTopicConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_topic_configurations: Option<bool>,
    #[serde(rename = "detectAndCopyNewTopics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_and_copy_new_topics: Option<bool>,
    #[serde(rename = "startingPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position: Option<ReplicationStartingPosition>,
    #[serde(rename = "topicNameConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name_configuration: Option<ReplicationTopicNameConfiguration>,
    #[serde(rename = "topicsToExclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics_to_exclude: Option<Vec<String>>,
    #[serde(rename = "topicsToReplicate")]
    #[serde(default)]
    pub topics_to_replicate: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationStartingPosition {
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationTopicNameConfiguration {
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReplicatorResponse {
    #[serde(rename = "replicatorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_arn: Option<String>,
    #[serde(rename = "replicatorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_name: Option<String>,
    #[serde(rename = "replicatorState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTopicRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<String>,
    #[serde(rename = "partitionCount")]
    #[serde(default)]
    pub partition_count: i32,
    #[serde(rename = "replicationFactor")]
    #[serde(default)]
    pub replication_factor: i32,
    #[serde(rename = "topicName")]
    #[serde(default)]
    pub topic_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTopicResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "topicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "topicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcConnectionRequest {
    #[serde(default)]
    pub authentication: String,
    #[serde(rename = "clientSubnets")]
    #[serde(default)]
    pub client_subnets: Vec<String>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    pub security_groups: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "targetClusterArn")]
    #[serde(default)]
    pub target_cluster_arn: String,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    pub vpc_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcConnectionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
    #[serde(rename = "clientSubnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_subnets: Option<Vec<String>>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "vpcConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_arn: Option<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterPolicyRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "CurrentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReplicatorRequest {
    #[serde(rename = "CurrentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(rename = "ReplicatorArn")]
    #[serde(default)]
    pub replicator_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReplicatorResponse {
    #[serde(rename = "replicatorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_arn: Option<String>,
    #[serde(rename = "replicatorState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTopicRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    pub topic_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTopicResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "topicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "topicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVpcConnectionRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVpcConnectionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "vpcConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterOperationRequest {
    #[serde(rename = "ClusterOperationArn")]
    #[serde(default)]
    pub cluster_operation_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterOperationResponse {
    #[serde(rename = "clusterOperationInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_info: Option<ClusterOperationInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterOperationInfo {
    #[serde(rename = "clientRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "errorInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<ErrorInfo>,
    #[serde(rename = "operationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_arn: Option<String>,
    #[serde(rename = "operationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_state: Option<String>,
    #[serde(rename = "operationSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_steps: Option<Vec<ClusterOperationStep>>,
    #[serde(rename = "operationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    #[serde(rename = "sourceClusterInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_cluster_info: Option<MutableClusterInfo>,
    #[serde(rename = "targetClusterInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cluster_info: Option<MutableClusterInfo>,
    #[serde(rename = "vpcConnectionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_info: Option<VpcConnectionInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorInfo {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterOperationStep {
    #[serde(rename = "stepInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_info: Option<ClusterOperationStepInfo>,
    #[serde(rename = "stepName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterOperationStepInfo {
    #[serde(rename = "stepStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MutableClusterInfo {
    #[serde(rename = "brokerCountUpdateInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_count_update_info: Option<BrokerCountUpdateInfo>,
    #[serde(rename = "brokerEBSVolumeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_e_b_s_volume_info: Option<Vec<BrokerEBSVolumeInfo>>,
    #[serde(rename = "clientAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication: Option<ClientAuthentication>,
    #[serde(rename = "configurationInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_info: Option<ConfigurationInfo>,
    #[serde(rename = "connectivityInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_info: Option<ConnectivityInfo>,
    #[serde(rename = "encryptionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_info: Option<EncryptionInfo>,
    #[serde(rename = "enhancedMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring: Option<String>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "kafkaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_version: Option<String>,
    #[serde(rename = "loggingInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    #[serde(rename = "numberOfBrokerNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_broker_nodes: Option<i32>,
    #[serde(rename = "openMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_monitoring: Option<OpenMonitoring>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebalancing: Option<Rebalancing>,
    #[serde(rename = "storageMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_mode: Option<String>,
    #[serde(rename = "zookeeperAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_access: Option<ZookeeperAccess>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrokerCountUpdateInfo {
    #[serde(rename = "createdBrokerIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_broker_ids: Option<Vec<f64>>,
    #[serde(rename = "deletedBrokerIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_broker_ids: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrokerEBSVolumeInfo {
    #[serde(rename = "kafkaBrokerNodeId")]
    #[serde(default)]
    pub kafka_broker_node_id: String,
    #[serde(rename = "provisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "volumeSizeGB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_g_b: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenMonitoring {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus: Option<Prometheus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Prometheus {
    #[serde(rename = "jmxExporter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jmx_exporter: Option<JmxExporter>,
    #[serde(rename = "nodeExporter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_exporter: Option<NodeExporter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JmxExporter {
    #[serde(rename = "enabledInBroker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_in_broker: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeExporter {
    #[serde(rename = "enabledInBroker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_in_broker: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZookeeperAccess {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConnectionInfo {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "userIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<UserIdentity>,
    #[serde(rename = "vpcConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserIdentity {
    #[serde(rename = "principalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterOperationV2Request {
    #[serde(rename = "ClusterOperationArn")]
    #[serde(default)]
    pub cluster_operation_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterOperationV2Response {
    #[serde(rename = "clusterOperationInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_info: Option<ClusterOperationV2>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterOperationV2 {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "errorInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<ErrorInfo>,
    #[serde(rename = "operationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_arn: Option<String>,
    #[serde(rename = "operationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_state: Option<String>,
    #[serde(rename = "operationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned: Option<ClusterOperationV2Provisioned>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless: Option<ClusterOperationV2Serverless>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterOperationV2Provisioned {
    #[serde(rename = "operationSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_steps: Option<Vec<ClusterOperationStep>>,
    #[serde(rename = "sourceClusterInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_cluster_info: Option<MutableClusterInfo>,
    #[serde(rename = "targetClusterInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cluster_info: Option<MutableClusterInfo>,
    #[serde(rename = "vpcConnectionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_info: Option<VpcConnectionInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterOperationV2Serverless {
    #[serde(rename = "sourceClusterInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_cluster_info: Option<ServerlessConnectivityInfo>,
    #[serde(rename = "targetClusterInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cluster_info: Option<ServerlessConnectivityInfo>,
    #[serde(rename = "vpcConnectionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_info: Option<VpcConnectionInfoServerless>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServerlessConnectivityInfo {
    #[serde(rename = "networkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConnectionInfoServerless {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "userIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<UserIdentity>,
    #[serde(rename = "vpcConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterResponse {
    #[serde(rename = "clusterInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_info: Option<ClusterInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterInfo {
    #[serde(rename = "activeOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_operation_arn: Option<String>,
    #[serde(rename = "brokerNodeGroupInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_node_group_info: Option<BrokerNodeGroupInfo>,
    #[serde(rename = "clientAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication: Option<ClientAuthentication>,
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "currentBrokerSoftwareInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_broker_software_info: Option<BrokerSoftwareInfo>,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(rename = "customerActionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_action_status: Option<String>,
    #[serde(rename = "encryptionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_info: Option<EncryptionInfo>,
    #[serde(rename = "enhancedMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring: Option<String>,
    #[serde(rename = "loggingInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    #[serde(rename = "numberOfBrokerNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_broker_nodes: Option<i32>,
    #[serde(rename = "openMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_monitoring: Option<OpenMonitoring>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebalancing: Option<Rebalancing>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "stateInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_info: Option<StateInfo>,
    #[serde(rename = "storageMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "zookeeperConnectString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_connect_string: Option<String>,
    #[serde(rename = "zookeeperConnectStringTls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_connect_string_tls: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrokerSoftwareInfo {
    #[serde(rename = "configurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_arn: Option<String>,
    #[serde(rename = "configurationRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_revision: Option<i64>,
    #[serde(rename = "kafkaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StateInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterV2Request {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterV2Response {
    #[serde(rename = "clusterInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_info: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cluster {
    #[serde(rename = "activeOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_operation_arn: Option<String>,
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "clusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned: Option<Provisioned>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless: Option<Serverless>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "stateInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_info: Option<StateInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Provisioned {
    #[serde(rename = "brokerNodeGroupInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_node_group_info: Option<BrokerNodeGroupInfo>,
    #[serde(rename = "clientAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication: Option<ClientAuthentication>,
    #[serde(rename = "currentBrokerSoftwareInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_broker_software_info: Option<BrokerSoftwareInfo>,
    #[serde(rename = "customerActionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_action_status: Option<String>,
    #[serde(rename = "encryptionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_info: Option<EncryptionInfo>,
    #[serde(rename = "enhancedMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring: Option<String>,
    #[serde(rename = "loggingInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    #[serde(rename = "numberOfBrokerNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_broker_nodes: Option<i32>,
    #[serde(rename = "openMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_monitoring: Option<OpenMonitoringInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebalancing: Option<Rebalancing>,
    #[serde(rename = "storageMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_mode: Option<String>,
    #[serde(rename = "zookeeperConnectString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_connect_string: Option<String>,
    #[serde(rename = "zookeeperConnectStringTls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_connect_string_tls: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Serverless {
    #[serde(rename = "clientAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication: Option<ServerlessClientAuthentication>,
    #[serde(rename = "connectivityInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_info: Option<ServerlessConnectivityInfo>,
    #[serde(rename = "vpcConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configs: Option<Vec<VpcConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "kafkaVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_versions: Option<Vec<String>>,
    #[serde(rename = "latestRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<ConfigurationRevision>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationRevisionRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Revision")]
    #[serde(default)]
    pub revision: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationRevisionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[serde(rename = "serverProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_properties: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicatorRequest {
    #[serde(rename = "ReplicatorArn")]
    #[serde(default)]
    pub replicator_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicatorResponse {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(rename = "isReplicatorReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_replicator_reference: Option<bool>,
    #[serde(rename = "kafkaClusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_clusters: Option<Vec<KafkaClusterDescription>>,
    #[serde(rename = "logDelivery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery: Option<LogDelivery>,
    #[serde(rename = "replicationInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_info_list: Option<Vec<ReplicationInfoDescription>>,
    #[serde(rename = "replicatorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_arn: Option<String>,
    #[serde(rename = "replicatorDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_description: Option<String>,
    #[serde(rename = "replicatorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_name: Option<String>,
    #[serde(rename = "replicatorResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_resource_arn: Option<String>,
    #[serde(rename = "replicatorState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_state: Option<String>,
    #[serde(rename = "serviceExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_execution_role_arn: Option<String>,
    #[serde(rename = "stateInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_info: Option<ReplicationStateInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KafkaClusterDescription {
    #[serde(rename = "amazonMskCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_msk_cluster: Option<AmazonMskCluster>,
    #[serde(rename = "apacheKafkaCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apache_kafka_cluster: Option<ApacheKafkaCluster>,
    #[serde(rename = "clientAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication: Option<KafkaClusterClientAuthentication>,
    #[serde(rename = "encryptionInTransit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_in_transit: Option<KafkaClusterEncryptionInTransit>,
    #[serde(rename = "kafkaClusterAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_cluster_alias: Option<String>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<KafkaClusterClientVpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationInfoDescription {
    #[serde(rename = "consumerGroupReplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_replication: Option<ConsumerGroupReplication>,
    #[serde(rename = "sourceKafkaClusterAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_kafka_cluster_alias: Option<String>,
    #[serde(rename = "targetCompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_compression_type: Option<String>,
    #[serde(rename = "targetKafkaClusterAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_kafka_cluster_alias: Option<String>,
    #[serde(rename = "topicReplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_replication: Option<TopicReplication>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationStateInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTopicPartitionsRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    pub topic_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTopicPartitionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<TopicPartitionInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicPartitionInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isr: Option<Vec<i32>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTopicRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    pub topic_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTopicResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<String>,
    #[serde(rename = "partitionCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_count: Option<i32>,
    #[serde(rename = "replicationFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "topicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "topicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVpcConnectionRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVpcConnectionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "targetClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cluster_arn: Option<String>,
    #[serde(rename = "vpcConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_arn: Option<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBootstrapBrokersRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBootstrapBrokersResponse {
    #[serde(rename = "bootstrapBrokerString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string: Option<String>,
    #[serde(rename = "bootstrapBrokerStringIpv6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_ipv6: Option<String>,
    #[serde(rename = "bootstrapBrokerStringPublicSaslIam")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_public_sasl_iam: Option<String>,
    #[serde(rename = "bootstrapBrokerStringPublicSaslScram")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_public_sasl_scram: Option<String>,
    #[serde(rename = "bootstrapBrokerStringPublicTls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_public_tls: Option<String>,
    #[serde(rename = "bootstrapBrokerStringSaslIam")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_sasl_iam: Option<String>,
    #[serde(rename = "bootstrapBrokerStringSaslIamIpv6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_sasl_iam_ipv6: Option<String>,
    #[serde(rename = "bootstrapBrokerStringSaslScram")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_sasl_scram: Option<String>,
    #[serde(rename = "bootstrapBrokerStringSaslScramIpv6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_sasl_scram_ipv6: Option<String>,
    #[serde(rename = "bootstrapBrokerStringTls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_tls: Option<String>,
    #[serde(rename = "bootstrapBrokerStringTlsIpv6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_tls_ipv6: Option<String>,
    #[serde(rename = "bootstrapBrokerStringVpcConnectivitySaslIam")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_vpc_connectivity_sasl_iam: Option<String>,
    #[serde(rename = "bootstrapBrokerStringVpcConnectivitySaslScram")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_vpc_connectivity_sasl_scram: Option<String>,
    #[serde(rename = "bootstrapBrokerStringVpcConnectivityTls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_vpc_connectivity_tls: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClusterPolicyRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClusterPolicyResponse {
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCompatibleKafkaVersionsRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCompatibleKafkaVersionsResponse {
    #[serde(rename = "compatibleKafkaVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_kafka_versions: Option<Vec<CompatibleKafkaVersion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompatibleKafkaVersion {
    #[serde(rename = "sourceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    #[serde(rename = "targetVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_versions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClientVpcConnectionsRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
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
pub struct ListClientVpcConnectionsResponse {
    #[serde(rename = "clientVpcConnections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_vpc_connections: Option<Vec<ClientVpcConnection>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientVpcConnection {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "vpcConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClusterOperationsRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
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
pub struct ListClusterOperationsResponse {
    #[serde(rename = "clusterOperationInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_info_list: Option<Vec<ClusterOperationInfo>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClusterOperationsV2Request {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
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
pub struct ListClusterOperationsV2Response {
    #[serde(rename = "clusterOperationInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_info_list: Option<Vec<ClusterOperationV2Summary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterOperationV2Summary {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "operationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_arn: Option<String>,
    #[serde(rename = "operationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_state: Option<String>,
    #[serde(rename = "operationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClustersRequest {
    #[serde(rename = "ClusterNameFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name_filter: Option<String>,
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
pub struct ListClustersResponse {
    #[serde(rename = "clusterInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_info_list: Option<Vec<ClusterInfo>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClustersV2Request {
    #[serde(rename = "ClusterNameFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name_filter: Option<String>,
    #[serde(rename = "ClusterTypeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type_filter: Option<String>,
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
pub struct ListClustersV2Response {
    #[serde(rename = "clusterInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_info_list: Option<Vec<Cluster>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationRevisionsRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
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
pub struct ListConfigurationRevisionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revisions: Option<Vec<ConfigurationRevision>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationsRequest {
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
pub struct ListConfigurationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Configuration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "kafkaVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_versions: Option<Vec<String>>,
    #[serde(rename = "latestRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<ConfigurationRevision>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKafkaVersionsRequest {
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
pub struct ListKafkaVersionsResponse {
    #[serde(rename = "kafkaVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_versions: Option<Vec<KafkaVersion>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KafkaVersion {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNodesRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
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
pub struct ListNodesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "nodeInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_info_list: Option<Vec<NodeInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeInfo {
    #[serde(rename = "addedToClusterTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_cluster_time: Option<String>,
    #[serde(rename = "brokerNodeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_node_info: Option<BrokerNodeInfo>,
    #[serde(rename = "controllerNodeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_node_info: Option<ControllerNodeInfo>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "nodeARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_a_r_n: Option<String>,
    #[serde(rename = "nodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "zookeeperNodeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_node_info: Option<ZookeeperNodeInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrokerNodeInfo {
    #[serde(rename = "attachedENIId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_e_n_i_id: Option<String>,
    #[serde(rename = "brokerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<f64>,
    #[serde(rename = "clientSubnet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_subnet: Option<String>,
    #[serde(rename = "clientVpcIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_vpc_ip_address: Option<String>,
    #[serde(rename = "currentBrokerSoftwareInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_broker_software_info: Option<BrokerSoftwareInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControllerNodeInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZookeeperNodeInfo {
    #[serde(rename = "attachedENIId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_e_n_i_id: Option<String>,
    #[serde(rename = "clientVpcIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_vpc_ip_address: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<String>>,
    #[serde(rename = "zookeeperId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_id: Option<f64>,
    #[serde(rename = "zookeeperVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReplicatorsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReplicatorNameFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_name_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReplicatorsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicators: Option<Vec<ReplicatorSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicatorSummary {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(rename = "isReplicatorReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_replicator_reference: Option<bool>,
    #[serde(rename = "kafkaClustersSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_clusters_summary: Option<Vec<KafkaClusterSummary>>,
    #[serde(rename = "replicationInfoSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_info_summary_list: Option<Vec<ReplicationInfoSummary>>,
    #[serde(rename = "replicatorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_arn: Option<String>,
    #[serde(rename = "replicatorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_name: Option<String>,
    #[serde(rename = "replicatorResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_resource_arn: Option<String>,
    #[serde(rename = "replicatorState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KafkaClusterSummary {
    #[serde(rename = "amazonMskCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_msk_cluster: Option<AmazonMskCluster>,
    #[serde(rename = "apacheKafkaCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apache_kafka_cluster: Option<ApacheKafkaCluster>,
    #[serde(rename = "kafkaClusterAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_cluster_alias: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationInfoSummary {
    #[serde(rename = "sourceKafkaClusterAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_kafka_cluster_alias: Option<String>,
    #[serde(rename = "targetKafkaClusterAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_kafka_cluster_alias: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListScramSecretsRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
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
pub struct ListScramSecretsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "secretArnList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
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
pub struct ListTopicsRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TopicNameFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTopicsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<TopicInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicInfo {
    #[serde(rename = "outOfSyncReplicaCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_sync_replica_count: Option<i32>,
    #[serde(rename = "partitionCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_count: Option<i32>,
    #[serde(rename = "replicationFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<i32>,
    #[serde(rename = "topicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "topicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVpcConnectionsRequest {
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
pub struct ListVpcConnectionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "vpcConnections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connections: Option<Vec<VpcConnection>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConnection {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "targetClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cluster_arn: Option<String>,
    #[serde(rename = "vpcConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_arn: Option<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutClusterPolicyRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(default)]
    pub policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutClusterPolicyResponse {
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebootBrokerRequest {
    #[serde(rename = "brokerIds")]
    #[serde(default)]
    pub broker_ids: Vec<String>,
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebootBrokerResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectClientVpcConnectionRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "vpcConnectionArn")]
    #[serde(default)]
    pub vpc_connection_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectClientVpcConnectionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

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
pub struct UpdateBrokerCountRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    pub current_version: String,
    #[serde(rename = "targetNumberOfBrokerNodes")]
    #[serde(default)]
    pub target_number_of_broker_nodes: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBrokerCountResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBrokerStorageRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    pub current_version: String,
    #[serde(rename = "targetBrokerEBSVolumeInfo")]
    #[serde(default)]
    pub target_broker_e_b_s_volume_info: Vec<BrokerEBSVolumeInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBrokerStorageResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBrokerTypeRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    pub current_version: String,
    #[serde(rename = "targetInstanceType")]
    #[serde(default)]
    pub target_instance_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBrokerTypeResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterConfigurationRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "configurationInfo")]
    #[serde(default)]
    pub configuration_info: ConfigurationInfo,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    pub current_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterConfigurationResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterKafkaVersionRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "configurationInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_info: Option<ConfigurationInfo>,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    pub current_version: String,
    #[serde(rename = "targetKafkaVersion")]
    #[serde(default)]
    pub target_kafka_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterKafkaVersionResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "serverProperties")]
    #[serde(default)]
    pub server_properties: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "latestRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<ConfigurationRevision>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectivityRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "connectivityInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_info: Option<ConnectivityInfo>,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    pub current_version: String,
    #[serde(rename = "zookeeperAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_access: Option<ZookeeperAccess>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectivityResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMonitoringRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    pub current_version: String,
    #[serde(rename = "enhancedMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring: Option<String>,
    #[serde(rename = "loggingInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    #[serde(rename = "openMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_monitoring: Option<OpenMonitoringInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMonitoringResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRebalancingRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    pub current_version: String,
    #[serde(default)]
    pub rebalancing: Rebalancing,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRebalancingResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReplicationInfoRequest {
    #[serde(rename = "consumerGroupReplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_replication: Option<ConsumerGroupReplicationUpdate>,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    pub current_version: String,
    #[serde(rename = "logDelivery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery: Option<LogDelivery>,
    #[serde(rename = "ReplicatorArn")]
    #[serde(default)]
    pub replicator_arn: String,
    #[serde(rename = "sourceKafkaClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_kafka_cluster_arn: Option<String>,
    #[serde(rename = "sourceKafkaClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_kafka_cluster_id: Option<String>,
    #[serde(rename = "targetKafkaClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_kafka_cluster_arn: Option<String>,
    #[serde(rename = "targetKafkaClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_kafka_cluster_id: Option<String>,
    #[serde(rename = "topicReplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_replication: Option<TopicReplicationUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConsumerGroupReplicationUpdate {
    #[serde(rename = "consumerGroupsToExclude")]
    #[serde(default)]
    pub consumer_groups_to_exclude: Vec<String>,
    #[serde(rename = "consumerGroupsToReplicate")]
    #[serde(default)]
    pub consumer_groups_to_replicate: Vec<String>,
    #[serde(rename = "detectAndCopyNewConsumerGroups")]
    #[serde(default)]
    pub detect_and_copy_new_consumer_groups: bool,
    #[serde(rename = "synchroniseConsumerGroupOffsets")]
    #[serde(default)]
    pub synchronise_consumer_group_offsets: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicReplicationUpdate {
    #[serde(rename = "copyAccessControlListsForTopics")]
    #[serde(default)]
    pub copy_access_control_lists_for_topics: bool,
    #[serde(rename = "copyTopicConfigurations")]
    #[serde(default)]
    pub copy_topic_configurations: bool,
    #[serde(rename = "detectAndCopyNewTopics")]
    #[serde(default)]
    pub detect_and_copy_new_topics: bool,
    #[serde(rename = "topicsToExclude")]
    #[serde(default)]
    pub topics_to_exclude: Vec<String>,
    #[serde(rename = "topicsToReplicate")]
    #[serde(default)]
    pub topics_to_replicate: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReplicationInfoResponse {
    #[serde(rename = "replicatorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_arn: Option<String>,
    #[serde(rename = "replicatorState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicator_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecurityRequest {
    #[serde(rename = "clientAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication: Option<ClientAuthentication>,
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    pub current_version: String,
    #[serde(rename = "encryptionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_info: Option<EncryptionInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecurityResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStorageRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    pub current_version: String,
    #[serde(rename = "provisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "storageMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_mode: Option<String>,
    #[serde(rename = "volumeSizeGB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_g_b: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStorageResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTopicRequest {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<String>,
    #[serde(rename = "partitionCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_count: Option<i32>,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    pub topic_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTopicResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "topicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "topicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}
