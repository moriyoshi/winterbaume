//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-mediaconnect

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddBridgeOutputsRequest {
    #[serde(rename = "BridgeArn")]
    #[serde(default)]
    pub bridge_arn: String,
    #[serde(default)]
    pub outputs: Vec<AddBridgeOutputRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddBridgeOutputRequest {
    #[serde(rename = "networkOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_output: Option<AddBridgeNetworkOutputRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddBridgeNetworkOutputRequest {
    #[serde(rename = "ipAddress")]
    #[serde(default)]
    pub ip_address: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "networkName")]
    #[serde(default)]
    pub network_name: String,
    #[serde(default)]
    pub port: i32,
    #[serde(default)]
    pub protocol: String,
    #[serde(default)]
    pub ttl: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddBridgeOutputsResponse {
    #[serde(rename = "bridgeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<BridgeOutput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BridgeOutput {
    #[serde(rename = "flowOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_output: Option<BridgeFlowOutput>,
    #[serde(rename = "networkOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_output: Option<BridgeNetworkOutput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BridgeFlowOutput {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "flowSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_source_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BridgeNetworkOutput {
    #[serde(rename = "ipAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddBridgeSourcesRequest {
    #[serde(rename = "BridgeArn")]
    #[serde(default)]
    pub bridge_arn: String,
    #[serde(default)]
    pub sources: Vec<AddBridgeSourceRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddBridgeSourceRequest {
    #[serde(rename = "flowSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_source: Option<AddBridgeFlowSourceRequest>,
    #[serde(rename = "networkSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source: Option<AddBridgeNetworkSourceRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddBridgeFlowSourceRequest {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    pub flow_arn: String,
    #[serde(rename = "flowVpcInterfaceAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_vpc_interface_attachment: Option<VpcInterfaceAttachment>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcInterfaceAttachment {
    #[serde(rename = "vpcInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddBridgeNetworkSourceRequest {
    #[serde(rename = "multicastIp")]
    #[serde(default)]
    pub multicast_ip: String,
    #[serde(rename = "multicastSourceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicast_source_settings: Option<MulticastSourceSettings>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "networkName")]
    #[serde(default)]
    pub network_name: String,
    #[serde(default)]
    pub port: i32,
    #[serde(default)]
    pub protocol: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MulticastSourceSettings {
    #[serde(rename = "multicastSourceIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicast_source_ip: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddBridgeSourcesResponse {
    #[serde(rename = "bridgeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<BridgeSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BridgeSource {
    #[serde(rename = "flowSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_source: Option<BridgeFlowSource>,
    #[serde(rename = "networkSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source: Option<BridgeNetworkSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BridgeFlowSource {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "flowVpcInterfaceAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_vpc_interface_attachment: Option<VpcInterfaceAttachment>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "outputArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BridgeNetworkSource {
    #[serde(rename = "multicastIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicast_ip: Option<String>,
    #[serde(rename = "multicastSourceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicast_source_settings: Option<MulticastSourceSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddFlowMediaStreamsRequest {
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
    #[serde(rename = "mediaStreams")]
    #[serde(default)]
    pub media_streams: Vec<AddMediaStreamRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddMediaStreamRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<MediaStreamAttributesRequest>,
    #[serde(rename = "clockRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_rate: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "mediaStreamId")]
    #[serde(default)]
    pub media_stream_id: i32,
    #[serde(rename = "mediaStreamName")]
    #[serde(default)]
    pub media_stream_name: String,
    #[serde(rename = "mediaStreamTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "mediaStreamType")]
    #[serde(default)]
    pub media_stream_type: String,
    #[serde(rename = "videoFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_format: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaStreamAttributesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fmtp: Option<FmtpRequest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FmtpRequest {
    #[serde(rename = "channelOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_order: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colorimetry: Option<String>,
    #[serde(rename = "exactFramerate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_framerate: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(rename = "scanMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcs: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddFlowMediaStreamsResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "mediaStreams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_streams: Option<Vec<MediaStream>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaStream {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<MediaStreamAttributes>,
    #[serde(rename = "clockRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_rate: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fmt: Option<i32>,
    #[serde(rename = "mediaStreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_id: Option<i32>,
    #[serde(rename = "mediaStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_name: Option<String>,
    #[serde(rename = "mediaStreamType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_type: Option<String>,
    #[serde(rename = "videoFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_format: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaStreamAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fmtp: Option<Fmtp>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Fmtp {
    #[serde(rename = "channelOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_order: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colorimetry: Option<String>,
    #[serde(rename = "exactFramerate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_framerate: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(rename = "scanMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcs: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddFlowOutputsRequest {
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
    #[serde(default)]
    pub outputs: Vec<AddOutputRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddOutputRequest {
    #[serde(rename = "cidrAllowList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_allow_list: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "maxLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i32>,
    #[serde(rename = "mediaStreamOutputConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_output_configurations: Option<Vec<MediaStreamOutputConfigurationRequest>>,
    #[serde(rename = "minLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_latency: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ndiProgramName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndi_program_name: Option<String>,
    #[serde(rename = "ndiSpeedHqQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndi_speed_hq_quality: Option<i32>,
    #[serde(rename = "outputStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_status: Option<String>,
    #[serde(rename = "outputTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "remoteId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    #[serde(rename = "routerIntegrationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_integration_state: Option<String>,
    #[serde(rename = "routerIntegrationTransitEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_integration_transit_encryption: Option<FlowTransitEncryption>,
    #[serde(rename = "senderControlPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_control_port: Option<i32>,
    #[serde(rename = "smoothingLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoothing_latency: Option<i32>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "vpcInterfaceAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_attachment: Option<VpcInterfaceAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Encryption {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(rename = "constantInitializationVector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    #[serde(rename = "deviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "keyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaStreamOutputConfigurationRequest {
    #[serde(rename = "destinationConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_configurations: Option<Vec<DestinationConfigurationRequest>>,
    #[serde(rename = "encodingName")]
    #[serde(default)]
    pub encoding_name: String,
    #[serde(rename = "encodingParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_parameters: Option<EncodingParametersRequest>,
    #[serde(rename = "mediaStreamName")]
    #[serde(default)]
    pub media_stream_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationConfigurationRequest {
    #[serde(rename = "destinationIp")]
    #[serde(default)]
    pub destination_ip: String,
    #[serde(rename = "destinationPort")]
    #[serde(default)]
    pub destination_port: i32,
    #[serde(default)]
    pub interface: InterfaceRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InterfaceRequest {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncodingParametersRequest {
    #[serde(rename = "compressionFactor")]
    #[serde(default)]
    pub compression_factor: f64,
    #[serde(rename = "encoderProfile")]
    #[serde(default)]
    pub encoder_profile: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowTransitEncryption {
    #[serde(rename = "encryptionKeyConfiguration")]
    #[serde(default)]
    pub encryption_key_configuration: FlowTransitEncryptionKeyConfiguration,
    #[serde(rename = "encryptionKeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowTransitEncryptionKeyConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic: Option<AutomaticEncryptionKeyConfiguration>,
    #[serde(rename = "secretsManager")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager: Option<SecretsManagerEncryptionKeyConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomaticEncryptionKeyConfiguration {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecretsManagerEncryptionKeyConfiguration {
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    pub secret_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddFlowOutputsResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Output {
    #[serde(rename = "bridgeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_arn: Option<String>,
    #[serde(rename = "bridgePorts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_ports: Option<Vec<i32>>,
    #[serde(rename = "connectedRouterInputArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_router_input_arn: Option<String>,
    #[serde(rename = "dataTransferSubscriberFeePercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_subscriber_fee_percent: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "entitlementArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    #[serde(rename = "listenerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_address: Option<String>,
    #[serde(rename = "mediaLiveInputArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_live_input_arn: Option<String>,
    #[serde(rename = "mediaStreamOutputConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_output_configurations: Option<Vec<MediaStreamOutputConfiguration>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "outputArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_arn: Option<String>,
    #[serde(rename = "outputStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_status: Option<String>,
    #[serde(rename = "peerIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_ip_address: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "routerIntegrationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_integration_state: Option<String>,
    #[serde(rename = "routerIntegrationTransitEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_integration_transit_encryption: Option<FlowTransitEncryption>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
    #[serde(rename = "vpcInterfaceAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_attachment: Option<VpcInterfaceAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaStreamOutputConfiguration {
    #[serde(rename = "destinationConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_configurations: Option<Vec<DestinationConfiguration>>,
    #[serde(rename = "encodingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_name: Option<String>,
    #[serde(rename = "encodingParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_parameters: Option<EncodingParameters>,
    #[serde(rename = "mediaStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationConfiguration {
    #[serde(rename = "destinationIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ip: Option<String>,
    #[serde(rename = "destinationPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<Interface>,
    #[serde(rename = "outboundIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_ip: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Interface {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncodingParameters {
    #[serde(rename = "compressionFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_factor: Option<f64>,
    #[serde(rename = "encoderProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_profile: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Transport {
    #[serde(rename = "cidrAllowList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_allow_list: Option<Vec<String>>,
    #[serde(rename = "maxBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i32>,
    #[serde(rename = "maxLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i32>,
    #[serde(rename = "maxSyncBuffer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sync_buffer: Option<i32>,
    #[serde(rename = "minLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_latency: Option<i32>,
    #[serde(rename = "ndiProgramName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndi_program_name: Option<String>,
    #[serde(rename = "ndiSourceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndi_source_settings: Option<NdiSourceSettings>,
    #[serde(rename = "ndiSpeedHqQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndi_speed_hq_quality: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "remoteId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    #[serde(rename = "senderControlPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_control_port: Option<i32>,
    #[serde(rename = "senderIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_ip_address: Option<String>,
    #[serde(rename = "smoothingLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoothing_latency: Option<i32>,
    #[serde(rename = "sourceListenerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_listener_address: Option<String>,
    #[serde(rename = "sourceListenerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_listener_port: Option<i32>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NdiSourceSettings {
    #[serde(rename = "sourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddFlowSourcesRequest {
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
    #[serde(default)]
    pub sources: Vec<SetSourceRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetSourceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption: Option<Encryption>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "entitlementArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    #[serde(rename = "gatewayBridgeSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_bridge_source: Option<SetGatewayBridgeSourceRequest>,
    #[serde(rename = "ingestPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_port: Option<i32>,
    #[serde(rename = "maxBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i32>,
    #[serde(rename = "maxLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i32>,
    #[serde(rename = "maxSyncBuffer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sync_buffer: Option<i32>,
    #[serde(rename = "mediaStreamSourceConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_source_configurations: Option<Vec<MediaStreamSourceConfigurationRequest>>,
    #[serde(rename = "minLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_latency: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ndiSourceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndi_source_settings: Option<NdiSourceSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "routerIntegrationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_integration_state: Option<String>,
    #[serde(rename = "routerIntegrationTransitDecryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_integration_transit_decryption: Option<FlowTransitEncryption>,
    #[serde(rename = "senderControlPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_control_port: Option<i32>,
    #[serde(rename = "senderIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_ip_address: Option<String>,
    #[serde(rename = "sourceListenerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_listener_address: Option<String>,
    #[serde(rename = "sourceListenerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_listener_port: Option<i32>,
    #[serde(rename = "sourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "vpcInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_name: Option<String>,
    #[serde(rename = "whitelistCidr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_cidr: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetGatewayBridgeSourceRequest {
    #[serde(rename = "bridgeArn")]
    #[serde(default)]
    pub bridge_arn: String,
    #[serde(rename = "vpcInterfaceAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_attachment: Option<VpcInterfaceAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaStreamSourceConfigurationRequest {
    #[serde(rename = "encodingName")]
    #[serde(default)]
    pub encoding_name: String,
    #[serde(rename = "inputConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_configurations: Option<Vec<InputConfigurationRequest>>,
    #[serde(rename = "mediaStreamName")]
    #[serde(default)]
    pub media_stream_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputConfigurationRequest {
    #[serde(rename = "inputPort")]
    #[serde(default)]
    pub input_port: i32,
    #[serde(default)]
    pub interface: InterfaceRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddFlowSourcesResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Source {
    #[serde(rename = "connectedRouterOutputArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_router_output_arn: Option<String>,
    #[serde(rename = "dataTransferSubscriberFeePercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_subscriber_fee_percent: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption: Option<Encryption>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "entitlementArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    #[serde(rename = "gatewayBridgeSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_bridge_source: Option<GatewayBridgeSource>,
    #[serde(rename = "ingestIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_ip: Option<String>,
    #[serde(rename = "ingestPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_port: Option<i32>,
    #[serde(rename = "mediaStreamSourceConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_source_configurations: Option<Vec<MediaStreamSourceConfiguration>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "peerIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_ip_address: Option<String>,
    #[serde(rename = "routerIntegrationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_integration_state: Option<String>,
    #[serde(rename = "routerIntegrationTransitDecryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_integration_transit_decryption: Option<FlowTransitEncryption>,
    #[serde(rename = "senderControlPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_control_port: Option<i32>,
    #[serde(rename = "senderIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_ip_address: Option<String>,
    #[serde(rename = "sourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
    #[serde(rename = "vpcInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_name: Option<String>,
    #[serde(rename = "whitelistCidr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_cidr: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GatewayBridgeSource {
    #[serde(rename = "bridgeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_arn: Option<String>,
    #[serde(rename = "vpcInterfaceAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_attachment: Option<VpcInterfaceAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaStreamSourceConfiguration {
    #[serde(rename = "encodingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_name: Option<String>,
    #[serde(rename = "inputConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_configurations: Option<Vec<InputConfiguration>>,
    #[serde(rename = "mediaStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputConfiguration {
    #[serde(rename = "inputIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ip: Option<String>,
    #[serde(rename = "inputPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<Interface>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddFlowVpcInterfacesRequest {
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
    #[serde(rename = "vpcInterfaces")]
    #[serde(default)]
    pub vpc_interfaces: Vec<VpcInterfaceRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcInterfaceRequest {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "networkInterfaceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_type: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "subnetId")]
    #[serde(default)]
    pub subnet_id: String,
    #[serde(rename = "vpcInterfaceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddFlowVpcInterfacesResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "vpcInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interfaces: Option<Vec<VpcInterface>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcInterface {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkInterfaceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_ids: Option<Vec<String>>,
    #[serde(rename = "networkInterfaceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_type: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetRouterInputRequest {
    #[serde(rename = "Arns")]
    #[serde(default)]
    pub arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetRouterInputResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchGetRouterInputError>>,
    #[serde(rename = "routerInputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_inputs: Option<Vec<RouterInput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetRouterInputError {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<RouterInputConfiguration>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(rename = "ipAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "maintenanceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration: Option<MaintenanceConfiguration>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(rename = "maintenanceScheduleType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule_type: Option<String>,
    #[serde(rename = "maintenanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_type: Option<String>,
    #[serde(rename = "maximumBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<i64>,
    #[serde(rename = "maximumRoutedOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_routed_outputs: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<RouterInputMessage>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "regionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "routedOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routed_outputs: Option<i32>,
    #[serde(rename = "routingScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_scope: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "streamDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_details: Option<RouterInputStreamDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "transitEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption: Option<RouterInputTransitEncryption>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterInputConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover: Option<FailoverRouterInputConfiguration>,
    #[serde(rename = "mediaConnectFlow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_flow: Option<MediaConnectFlowRouterInputConfiguration>,
    #[serde(rename = "mediaLiveChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_live_channel: Option<MediaLiveChannelRouterInputConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge: Option<MergeRouterInputConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<StandardRouterInputConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailoverRouterInputConfiguration {
    #[serde(rename = "networkInterfaceArn")]
    #[serde(default)]
    pub network_interface_arn: String,
    #[serde(rename = "primarySourceIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_source_index: Option<i32>,
    #[serde(rename = "protocolConfigurations")]
    #[serde(default)]
    pub protocol_configurations: Vec<FailoverRouterInputProtocolConfiguration>,
    #[serde(rename = "sourcePriorityMode")]
    #[serde(default)]
    pub source_priority_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailoverRouterInputProtocolConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rist: Option<RistRouterInputConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtp: Option<RtpRouterInputConfiguration>,
    #[serde(rename = "srtCaller")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_caller: Option<SrtCallerRouterInputConfiguration>,
    #[serde(rename = "srtListener")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_listener: Option<SrtListenerRouterInputConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RistRouterInputConfiguration {
    #[serde(default)]
    pub port: i32,
    #[serde(rename = "recoveryLatencyMilliseconds")]
    #[serde(default)]
    pub recovery_latency_milliseconds: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RtpRouterInputConfiguration {
    #[serde(rename = "forwardErrorCorrection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_error_correction: Option<String>,
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtCallerRouterInputConfiguration {
    #[serde(rename = "decryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption_configuration: Option<SrtDecryptionConfiguration>,
    #[serde(rename = "minimumLatencyMilliseconds")]
    #[serde(default)]
    pub minimum_latency_milliseconds: i64,
    #[serde(rename = "sourceAddress")]
    #[serde(default)]
    pub source_address: String,
    #[serde(rename = "sourcePort")]
    #[serde(default)]
    pub source_port: i32,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtDecryptionConfiguration {
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    pub encryption_key: SecretsManagerEncryptionKeyConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtListenerRouterInputConfiguration {
    #[serde(rename = "decryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption_configuration: Option<SrtDecryptionConfiguration>,
    #[serde(rename = "minimumLatencyMilliseconds")]
    #[serde(default)]
    pub minimum_latency_milliseconds: i64,
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaConnectFlowRouterInputConfiguration {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "flowOutputArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_output_arn: Option<String>,
    #[serde(rename = "sourceTransitDecryption")]
    #[serde(default)]
    pub source_transit_decryption: FlowTransitEncryption,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaLiveChannelRouterInputConfiguration {
    #[serde(rename = "mediaLiveChannelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_live_channel_arn: Option<String>,
    #[serde(rename = "mediaLiveChannelOutputName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_live_channel_output_name: Option<String>,
    #[serde(rename = "mediaLivePipelineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_live_pipeline_id: Option<String>,
    #[serde(rename = "sourceTransitDecryption")]
    #[serde(default)]
    pub source_transit_decryption: MediaLiveTransitEncryption,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaLiveTransitEncryption {
    #[serde(rename = "encryptionKeyConfiguration")]
    #[serde(default)]
    pub encryption_key_configuration: MediaLiveTransitEncryptionKeyConfiguration,
    #[serde(rename = "encryptionKeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaLiveTransitEncryptionKeyConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic: Option<AutomaticEncryptionKeyConfiguration>,
    #[serde(rename = "secretsManager")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager: Option<SecretsManagerEncryptionKeyConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeRouterInputConfiguration {
    #[serde(rename = "mergeRecoveryWindowMilliseconds")]
    #[serde(default)]
    pub merge_recovery_window_milliseconds: i64,
    #[serde(rename = "networkInterfaceArn")]
    #[serde(default)]
    pub network_interface_arn: String,
    #[serde(rename = "protocolConfigurations")]
    #[serde(default)]
    pub protocol_configurations: Vec<MergeRouterInputProtocolConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeRouterInputProtocolConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rist: Option<RistRouterInputConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtp: Option<RtpRouterInputConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandardRouterInputConfiguration {
    #[serde(rename = "networkInterfaceArn")]
    #[serde(default)]
    pub network_interface_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "protocolConfiguration")]
    #[serde(default)]
    pub protocol_configuration: RouterInputProtocolConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterInputProtocolConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rist: Option<RistRouterInputConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtp: Option<RtpRouterInputConfiguration>,
    #[serde(rename = "srtCaller")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_caller: Option<SrtCallerRouterInputConfiguration>,
    #[serde(rename = "srtListener")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_listener: Option<SrtListenerRouterInputConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<DefaultMaintenanceConfiguration>,
    #[serde(rename = "preferredDayTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_day_time: Option<PreferredDayTimeMaintenanceConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultMaintenanceConfiguration {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PreferredDayTimeMaintenanceConfiguration {
    #[serde(default)]
    pub day: String,
    #[serde(default)]
    pub time: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceSchedule {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<WindowMaintenanceSchedule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WindowMaintenanceSchedule {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "scheduledTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterInputMessage {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterInputStreamDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover: Option<FailoverRouterInputStreamDetails>,
    #[serde(rename = "mediaConnectFlow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_flow: Option<MediaConnectFlowRouterInputStreamDetails>,
    #[serde(rename = "mediaLiveChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_live_channel: Option<MediaLiveChannelRouterInputStreamDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge: Option<MergeRouterInputStreamDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<StandardRouterInputStreamDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailoverRouterInputStreamDetails {
    #[serde(rename = "sourceIndexOneStreamDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_index_one_stream_details: Option<FailoverRouterInputIndexedStreamDetails>,
    #[serde(rename = "sourceIndexZeroStreamDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_index_zero_stream_details: Option<FailoverRouterInputIndexedStreamDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailoverRouterInputIndexedStreamDetails {
    #[serde(rename = "sourceIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_index: Option<i32>,
    #[serde(rename = "sourceIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaConnectFlowRouterInputStreamDetails {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaLiveChannelRouterInputStreamDetails {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeRouterInputStreamDetails {
    #[serde(rename = "sourceIndexOneStreamDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_index_one_stream_details: Option<MergeRouterInputIndexedStreamDetails>,
    #[serde(rename = "sourceIndexZeroStreamDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_index_zero_stream_details: Option<MergeRouterInputIndexedStreamDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeRouterInputIndexedStreamDetails {
    #[serde(rename = "sourceIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_index: Option<i32>,
    #[serde(rename = "sourceIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandardRouterInputStreamDetails {
    #[serde(rename = "sourceIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterInputTransitEncryption {
    #[serde(rename = "encryptionKeyConfiguration")]
    #[serde(default)]
    pub encryption_key_configuration: RouterInputTransitEncryptionKeyConfiguration,
    #[serde(rename = "encryptionKeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterInputTransitEncryptionKeyConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic: Option<AutomaticEncryptionKeyConfiguration>,
    #[serde(rename = "secretsManager")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager: Option<SecretsManagerEncryptionKeyConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetRouterNetworkInterfaceRequest {
    #[serde(rename = "Arns")]
    #[serde(default)]
    pub arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetRouterNetworkInterfaceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchGetRouterNetworkInterfaceError>>,
    #[serde(rename = "routerNetworkInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_network_interfaces: Option<Vec<RouterNetworkInterface>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetRouterNetworkInterfaceError {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterNetworkInterface {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "associatedInputCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_input_count: Option<i32>,
    #[serde(rename = "associatedOutputCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_output_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<RouterNetworkInterfaceConfiguration>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkInterfaceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_type: Option<String>,
    #[serde(rename = "regionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterNetworkInterfaceConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<PublicRouterNetworkInterfaceConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcRouterNetworkInterfaceConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublicRouterNetworkInterfaceConfiguration {
    #[serde(rename = "allowRules")]
    #[serde(default)]
    pub allow_rules: Vec<PublicRouterNetworkInterfaceRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublicRouterNetworkInterfaceRule {
    #[serde(default)]
    pub cidr: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcRouterNetworkInterfaceConfiguration {
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "subnetId")]
    #[serde(default)]
    pub subnet_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetRouterOutputRequest {
    #[serde(rename = "Arns")]
    #[serde(default)]
    pub arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetRouterOutputResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchGetRouterOutputError>>,
    #[serde(rename = "routerOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_outputs: Option<Vec<RouterOutput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetRouterOutputError {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<RouterOutputConfiguration>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ipAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "maintenanceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration: Option<MaintenanceConfiguration>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(rename = "maintenanceScheduleType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule_type: Option<String>,
    #[serde(rename = "maintenanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_type: Option<String>,
    #[serde(rename = "maximumBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<RouterOutputMessage>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "outputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<String>,
    #[serde(rename = "regionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "routedInputArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routed_input_arn: Option<String>,
    #[serde(rename = "routedState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routed_state: Option<String>,
    #[serde(rename = "routingScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_scope: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "streamDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_details: Option<RouterOutputStreamDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterOutputConfiguration {
    #[serde(rename = "mediaConnectFlow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_flow: Option<MediaConnectFlowRouterOutputConfiguration>,
    #[serde(rename = "mediaLiveInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_live_input: Option<MediaLiveInputRouterOutputConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<StandardRouterOutputConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaConnectFlowRouterOutputConfiguration {
    #[serde(rename = "destinationTransitEncryption")]
    #[serde(default)]
    pub destination_transit_encryption: FlowTransitEncryption,
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "flowSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_source_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaLiveInputRouterOutputConfiguration {
    #[serde(rename = "destinationTransitEncryption")]
    #[serde(default)]
    pub destination_transit_encryption: MediaLiveTransitEncryption,
    #[serde(rename = "mediaLiveInputArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_live_input_arn: Option<String>,
    #[serde(rename = "mediaLivePipelineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_live_pipeline_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandardRouterOutputConfiguration {
    #[serde(rename = "networkInterfaceArn")]
    #[serde(default)]
    pub network_interface_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "protocolConfiguration")]
    #[serde(default)]
    pub protocol_configuration: RouterOutputProtocolConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterOutputProtocolConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rist: Option<RistRouterOutputConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtp: Option<RtpRouterOutputConfiguration>,
    #[serde(rename = "srtCaller")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_caller: Option<SrtCallerRouterOutputConfiguration>,
    #[serde(rename = "srtListener")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_listener: Option<SrtListenerRouterOutputConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RistRouterOutputConfiguration {
    #[serde(rename = "destinationAddress")]
    #[serde(default)]
    pub destination_address: String,
    #[serde(rename = "destinationPort")]
    #[serde(default)]
    pub destination_port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RtpRouterOutputConfiguration {
    #[serde(rename = "destinationAddress")]
    #[serde(default)]
    pub destination_address: String,
    #[serde(rename = "destinationPort")]
    #[serde(default)]
    pub destination_port: i32,
    #[serde(rename = "forwardErrorCorrection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_error_correction: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtCallerRouterOutputConfiguration {
    #[serde(rename = "destinationAddress")]
    #[serde(default)]
    pub destination_address: String,
    #[serde(rename = "destinationPort")]
    #[serde(default)]
    pub destination_port: i32,
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<SrtEncryptionConfiguration>,
    #[serde(rename = "minimumLatencyMilliseconds")]
    #[serde(default)]
    pub minimum_latency_milliseconds: i64,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtEncryptionConfiguration {
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    pub encryption_key: SecretsManagerEncryptionKeyConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtListenerRouterOutputConfiguration {
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<SrtEncryptionConfiguration>,
    #[serde(rename = "minimumLatencyMilliseconds")]
    #[serde(default)]
    pub minimum_latency_milliseconds: i64,
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterOutputMessage {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterOutputStreamDetails {
    #[serde(rename = "mediaConnectFlow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_flow: Option<MediaConnectFlowRouterOutputStreamDetails>,
    #[serde(rename = "mediaLiveInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_live_input: Option<MediaLiveInputRouterOutputStreamDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<StandardRouterOutputStreamDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaConnectFlowRouterOutputStreamDetails {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaLiveInputRouterOutputStreamDetails {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandardRouterOutputStreamDetails {
    #[serde(rename = "destinationIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBridgeRequest {
    #[serde(rename = "egressGatewayBridge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_gateway_bridge: Option<AddEgressGatewayBridgeRequest>,
    #[serde(rename = "ingressGatewayBridge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_gateway_bridge: Option<AddIngressGatewayBridgeRequest>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<AddBridgeOutputRequest>>,
    #[serde(rename = "placementArn")]
    #[serde(default)]
    pub placement_arn: String,
    #[serde(rename = "sourceFailoverConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_failover_config: Option<FailoverConfig>,
    #[serde(default)]
    pub sources: Vec<AddBridgeSourceRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddEgressGatewayBridgeRequest {
    #[serde(rename = "maxBitrate")]
    #[serde(default)]
    pub max_bitrate: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddIngressGatewayBridgeRequest {
    #[serde(rename = "maxBitrate")]
    #[serde(default)]
    pub max_bitrate: i32,
    #[serde(rename = "maxOutputs")]
    #[serde(default)]
    pub max_outputs: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailoverConfig {
    #[serde(rename = "failoverMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_mode: Option<String>,
    #[serde(rename = "recoveryWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_window: Option<i32>,
    #[serde(rename = "sourcePriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_priority: Option<SourcePriority>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourcePriority {
    #[serde(rename = "primarySource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBridgeResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<Bridge>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Bridge {
    #[serde(rename = "bridgeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_arn: Option<String>,
    #[serde(rename = "bridgeMessages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_messages: Option<Vec<MessageDetail>>,
    #[serde(rename = "bridgeState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_state: Option<String>,
    #[serde(rename = "egressGatewayBridge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_gateway_bridge: Option<EgressGatewayBridge>,
    #[serde(rename = "ingressGatewayBridge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_gateway_bridge: Option<IngressGatewayBridge>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<BridgeOutput>>,
    #[serde(rename = "placementArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_arn: Option<String>,
    #[serde(rename = "sourceFailoverConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_failover_config: Option<FailoverConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<BridgeSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EgressGatewayBridge {
    #[serde(rename = "instanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "maxBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngressGatewayBridge {
    #[serde(rename = "instanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "maxBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i32>,
    #[serde(rename = "maxOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_outputs: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFlowRequest {
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "encodingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_config: Option<EncodingConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<GrantEntitlementRequest>>,
    #[serde(rename = "flowSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_size: Option<String>,
    #[serde(rename = "flowTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<AddMaintenance>,
    #[serde(rename = "mediaStreams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_streams: Option<Vec<AddMediaStreamRequest>>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ndiConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndi_config: Option<NdiConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<AddOutputRequest>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SetSourceRequest>,
    #[serde(rename = "sourceFailoverConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_failover_config: Option<FailoverConfig>,
    #[serde(rename = "sourceMonitoringConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_monitoring_config: Option<MonitoringConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<SetSourceRequest>>,
    #[serde(rename = "vpcInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interfaces: Option<Vec<VpcInterfaceRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncodingConfig {
    #[serde(rename = "encodingProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_profile: Option<String>,
    #[serde(rename = "videoMaxBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_max_bitrate: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrantEntitlementRequest {
    #[serde(rename = "dataTransferSubscriberFeePercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_subscriber_fee_percent: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "entitlementStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_status: Option<String>,
    #[serde(rename = "entitlementTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub subscribers: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddMaintenance {
    #[serde(rename = "maintenanceDay")]
    #[serde(default)]
    pub maintenance_day: String,
    #[serde(rename = "maintenanceStartHour")]
    #[serde(default)]
    pub maintenance_start_hour: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NdiConfig {
    #[serde(rename = "machineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[serde(rename = "ndiDiscoveryServers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndi_discovery_servers: Option<Vec<NdiDiscoveryServerConfig>>,
    #[serde(rename = "ndiState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndi_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NdiDiscoveryServerConfig {
    #[serde(rename = "discoveryServerAddress")]
    #[serde(default)]
    pub discovery_server_address: String,
    #[serde(rename = "discoveryServerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_server_port: Option<i32>,
    #[serde(rename = "vpcInterfaceAdapter")]
    #[serde(default)]
    pub vpc_interface_adapter: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonitoringConfig {
    #[serde(rename = "audioMonitoringSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_monitoring_settings: Option<Vec<AudioMonitoringSetting>>,
    #[serde(rename = "contentQualityAnalysisState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_quality_analysis_state: Option<String>,
    #[serde(rename = "thumbnailState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_state: Option<String>,
    #[serde(rename = "videoMonitoringSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_monitoring_settings: Option<Vec<VideoMonitoringSetting>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioMonitoringSetting {
    #[serde(rename = "silentAudio")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_audio: Option<SilentAudio>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SilentAudio {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "thresholdSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VideoMonitoringSetting {
    #[serde(rename = "blackFrames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_frames: Option<BlackFrames>,
    #[serde(rename = "frozenFrames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_frames: Option<FrozenFrames>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlackFrames {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "thresholdSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FrozenFrames {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "thresholdSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFlowResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<Flow>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Flow {
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "egressIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_ip: Option<String>,
    #[serde(rename = "encodingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_config: Option<EncodingConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<Entitlement>>,
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "flowSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_size: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<Maintenance>,
    #[serde(rename = "mediaStreams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_streams: Option<Vec<MediaStream>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ndiConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndi_config: Option<NdiConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(rename = "sourceFailoverConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_failover_config: Option<FailoverConfig>,
    #[serde(rename = "sourceMonitoringConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_monitoring_config: Option<MonitoringConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "vpcInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interfaces: Option<Vec<VpcInterface>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Entitlement {
    #[serde(rename = "dataTransferSubscriberFeePercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_subscriber_fee_percent: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "entitlementArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    #[serde(rename = "entitlementStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Maintenance {
    #[serde(rename = "maintenanceDay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_day: Option<String>,
    #[serde(rename = "maintenanceDeadline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_deadline: Option<String>,
    #[serde(rename = "maintenanceScheduledDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_scheduled_date: Option<String>,
    #[serde(rename = "maintenanceStartHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_start_hour: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGatewayRequest {
    #[serde(rename = "egressCidrBlocks")]
    #[serde(default)]
    pub egress_cidr_blocks: Vec<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub networks: Vec<GatewayNetwork>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GatewayNetwork {
    #[serde(rename = "cidrBlock")]
    #[serde(default)]
    pub cidr_block: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGatewayResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<Gateway>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Gateway {
    #[serde(rename = "egressCidrBlocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_cidr_blocks: Option<Vec<String>>,
    #[serde(rename = "gatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    #[serde(rename = "gatewayMessages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_messages: Option<Vec<MessageDetail>>,
    #[serde(rename = "gatewayState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<GatewayNetwork>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRouterInputRequest {
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub configuration: RouterInputConfiguration,
    #[serde(rename = "maintenanceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration: Option<MaintenanceConfiguration>,
    #[serde(rename = "maximumBitrate")]
    #[serde(default)]
    pub maximum_bitrate: i64,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "regionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "routingScope")]
    #[serde(default)]
    pub routing_scope: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub tier: String,
    #[serde(rename = "transitEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption: Option<RouterInputTransitEncryption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRouterInputResponse {
    #[serde(rename = "routerInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_input: Option<RouterInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRouterNetworkInterfaceRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub configuration: RouterNetworkInterfaceConfiguration,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "regionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRouterNetworkInterfaceResponse {
    #[serde(rename = "routerNetworkInterface")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_network_interface: Option<RouterNetworkInterface>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRouterOutputRequest {
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub configuration: RouterOutputConfiguration,
    #[serde(rename = "maintenanceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration: Option<MaintenanceConfiguration>,
    #[serde(rename = "maximumBitrate")]
    #[serde(default)]
    pub maximum_bitrate: i64,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "regionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "routingScope")]
    #[serde(default)]
    pub routing_scope: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub tier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRouterOutputResponse {
    #[serde(rename = "routerOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_output: Option<RouterOutput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBridgeRequest {
    #[serde(rename = "BridgeArn")]
    #[serde(default)]
    pub bridge_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBridgeResponse {
    #[serde(rename = "bridgeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFlowRequest {
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFlowResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGatewayRequest {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    pub gateway_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGatewayResponse {
    #[serde(rename = "gatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRouterInputRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRouterInputResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRouterNetworkInterfaceRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRouterNetworkInterfaceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRouterOutputRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRouterOutputResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterGatewayInstanceRequest {
    #[serde(rename = "Force")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "GatewayInstanceArn")]
    #[serde(default)]
    pub gateway_instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterGatewayInstanceResponse {
    #[serde(rename = "gatewayInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_instance_arn: Option<String>,
    #[serde(rename = "instanceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBridgeRequest {
    #[serde(rename = "BridgeArn")]
    #[serde(default)]
    pub bridge_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBridgeResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<Bridge>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFlowRequest {
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFlowResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<Flow>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Messages>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Messages {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFlowSourceMetadataRequest {
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFlowSourceMetadataResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<MessageDetail>>,
    #[serde(rename = "ndiInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndi_info: Option<NdiSourceMetadataInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "transportMediaInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_media_info: Option<TransportMediaInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NdiSourceMetadataInfo {
    #[serde(rename = "activeSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_source: Option<NdiSourceInfo>,
    #[serde(rename = "discoveredSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovered_sources: Option<Vec<NdiSourceInfo>>,
    #[serde(rename = "mediaInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_info: Option<NdiMediaInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<MessageDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NdiSourceInfo {
    #[serde(rename = "sourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NdiMediaInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streams: Option<Vec<NdiMediaStreamInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NdiMediaStreamInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(rename = "frameRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<String>,
    #[serde(rename = "frameResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_resolution: Option<FrameResolution>,
    #[serde(rename = "sampleRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i32>,
    #[serde(rename = "scanMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_mode: Option<String>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<i32>,
    #[serde(rename = "streamType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FrameResolution {
    #[serde(rename = "frameHeight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_height: Option<i32>,
    #[serde(rename = "frameWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_width: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransportMediaInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub programs: Option<Vec<TransportStreamProgram>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransportStreamProgram {
    #[serde(rename = "pcrPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_pid: Option<i32>,
    #[serde(rename = "programName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_name: Option<String>,
    #[serde(rename = "programNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_number: Option<i32>,
    #[serde(rename = "programPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_pid: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streams: Option<Vec<TransportStream>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransportStream {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(rename = "frameRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<String>,
    #[serde(rename = "frameResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_resolution: Option<FrameResolution>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    #[serde(rename = "sampleRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i32>,
    #[serde(rename = "sampleSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<i32>,
    #[serde(rename = "streamType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFlowSourceThumbnailRequest {
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFlowSourceThumbnailResponse {
    #[serde(rename = "thumbnailDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_details: Option<ThumbnailDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThumbnailDetails {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(rename = "thumbnailMessages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_messages: Option<Vec<MessageDetail>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGatewayInstanceRequest {
    #[serde(rename = "GatewayInstanceArn")]
    #[serde(default)]
    pub gateway_instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGatewayInstanceResponse {
    #[serde(rename = "gatewayInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_instance: Option<GatewayInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GatewayInstance {
    #[serde(rename = "bridgePlacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_placement: Option<String>,
    #[serde(rename = "connectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<String>,
    #[serde(rename = "gatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    #[serde(rename = "gatewayInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_instance_arn: Option<String>,
    #[serde(rename = "instanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "instanceMessages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_messages: Option<Vec<MessageDetail>>,
    #[serde(rename = "instanceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_state: Option<String>,
    #[serde(rename = "runningBridgeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_bridge_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGatewayRequest {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    pub gateway_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGatewayResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<Gateway>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOfferingRequest {
    #[serde(rename = "OfferingArn")]
    #[serde(default)]
    pub offering_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOfferingResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering: Option<Offering>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Offering {
    #[serde(rename = "currencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "durationUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_units: Option<String>,
    #[serde(rename = "offeringArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_arn: Option<String>,
    #[serde(rename = "offeringDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_description: Option<String>,
    #[serde(rename = "pricePerUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_per_unit: Option<String>,
    #[serde(rename = "priceUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_units: Option<String>,
    #[serde(rename = "resourceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ResourceSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceSpecification {
    #[serde(rename = "reservedBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_bitrate: Option<i32>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReservationRequest {
    #[serde(rename = "ReservationArn")]
    #[serde(default)]
    pub reservation_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReservationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<Reservation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Reservation {
    #[serde(rename = "currencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "durationUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_units: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "offeringArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_arn: Option<String>,
    #[serde(rename = "offeringDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_description: Option<String>,
    #[serde(rename = "pricePerUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_per_unit: Option<String>,
    #[serde(rename = "priceUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_units: Option<String>,
    #[serde(rename = "reservationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_arn: Option<String>,
    #[serde(rename = "reservationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_name: Option<String>,
    #[serde(rename = "reservationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_state: Option<String>,
    #[serde(rename = "resourceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ResourceSpecification>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouterInputRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouterInputResponse {
    #[serde(rename = "routerInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_input: Option<RouterInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouterInputSourceMetadataRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouterInputSourceMetadataResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sourceMetadataDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_metadata_details: Option<RouterInputSourceMetadataDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterInputSourceMetadataDetails {
    #[serde(rename = "routerInputMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_input_metadata: Option<RouterInputMetadata>,
    #[serde(rename = "sourceMetadataMessages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_metadata_messages: Option<Vec<RouterInputMessage>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterInputMetadata {
    #[serde(rename = "transportStreamMediaInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_media_info: Option<TransportMediaInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouterInputThumbnailRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouterInputThumbnailResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "thumbnailDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_details: Option<RouterInputThumbnailDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterInputThumbnailDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(rename = "thumbnailMessages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_messages: Option<Vec<RouterInputMessage>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouterNetworkInterfaceRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouterNetworkInterfaceResponse {
    #[serde(rename = "routerNetworkInterface")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_network_interface: Option<RouterNetworkInterface>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouterOutputRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouterOutputResponse {
    #[serde(rename = "routerOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_output: Option<RouterOutput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrantFlowEntitlementsRequest {
    #[serde(default)]
    pub entitlements: Vec<GrantEntitlementRequest>,
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrantFlowEntitlementsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<Entitlement>>,
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBridgesRequest {
    #[serde(rename = "FilterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
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
pub struct ListBridgesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridges: Option<Vec<ListedBridge>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedBridge {
    #[serde(rename = "bridgeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_arn: Option<String>,
    #[serde(rename = "bridgeState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_state: Option<String>,
    #[serde(rename = "bridgeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "placementArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEntitlementsRequest {
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
pub struct ListEntitlementsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<ListedEntitlement>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedEntitlement {
    #[serde(rename = "dataTransferSubscriberFeePercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_subscriber_fee_percent: Option<i32>,
    #[serde(rename = "entitlementArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    #[serde(rename = "entitlementName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlowsRequest {
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
pub struct ListFlowsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flows: Option<Vec<ListedFlow>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedFlow {
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<Maintenance>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGatewayInstancesRequest {
    #[serde(rename = "FilterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
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
pub struct ListGatewayInstancesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<ListedGatewayInstance>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedGatewayInstance {
    #[serde(rename = "gatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    #[serde(rename = "gatewayInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_instance_arn: Option<String>,
    #[serde(rename = "instanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "instanceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGatewaysRequest {
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
pub struct ListGatewaysResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<ListedGateway>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedGateway {
    #[serde(rename = "gatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    #[serde(rename = "gatewayState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOfferingsRequest {
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
pub struct ListOfferingsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offerings: Option<Vec<Offering>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReservationsRequest {
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
pub struct ListReservationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservations: Option<Vec<Reservation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRouterInputsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<RouterInputFilter>>,
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
pub struct RouterInputFilter {
    #[serde(rename = "inputTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_types: Option<Vec<String>>,
    #[serde(rename = "nameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<Vec<String>>,
    #[serde(rename = "networkInterfaceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_arns: Option<Vec<String>>,
    #[serde(rename = "regionNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_names: Option<Vec<String>>,
    #[serde(rename = "routingScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRouterInputsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "routerInputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_inputs: Option<Vec<ListedRouterInput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedRouterInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(rename = "maintenanceScheduleType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule_type: Option<String>,
    #[serde(rename = "maximumBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<i64>,
    #[serde(rename = "messageCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkInterfaceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_arn: Option<String>,
    #[serde(rename = "regionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "routedOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routed_outputs: Option<i32>,
    #[serde(rename = "routingScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_scope: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRouterNetworkInterfacesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<RouterNetworkInterfaceFilter>>,
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
pub struct RouterNetworkInterfaceFilter {
    #[serde(rename = "nameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<Vec<String>>,
    #[serde(rename = "networkInterfaceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_types: Option<Vec<String>>,
    #[serde(rename = "regionNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRouterNetworkInterfacesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "routerNetworkInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_network_interfaces: Option<Vec<ListedRouterNetworkInterface>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedRouterNetworkInterface {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "associatedInputCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_input_count: Option<i32>,
    #[serde(rename = "associatedOutputCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_output_count: Option<i32>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkInterfaceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_type: Option<String>,
    #[serde(rename = "regionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRouterOutputsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<RouterOutputFilter>>,
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
pub struct RouterOutputFilter {
    #[serde(rename = "nameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<Vec<String>>,
    #[serde(rename = "networkInterfaceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_arns: Option<Vec<String>>,
    #[serde(rename = "outputTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_types: Option<Vec<String>>,
    #[serde(rename = "regionNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_names: Option<Vec<String>>,
    #[serde(rename = "routedInputArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routed_input_arns: Option<Vec<String>>,
    #[serde(rename = "routingScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRouterOutputsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "routerOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_outputs: Option<Vec<ListedRouterOutput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedRouterOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(rename = "maintenanceScheduleType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule_type: Option<String>,
    #[serde(rename = "maximumBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<i64>,
    #[serde(rename = "messageCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkInterfaceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_arn: Option<String>,
    #[serde(rename = "outputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<String>,
    #[serde(rename = "regionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "routedInputArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routed_input_arn: Option<String>,
    #[serde(rename = "routedState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routed_state: Option<String>,
    #[serde(rename = "routingScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_scope: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForGlobalResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForGlobalResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
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
pub struct PurchaseOfferingRequest {
    #[serde(rename = "OfferingArn")]
    #[serde(default)]
    pub offering_arn: String,
    #[serde(rename = "reservationName")]
    #[serde(default)]
    pub reservation_name: String,
    #[serde(default)]
    pub start: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PurchaseOfferingResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<Reservation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveBridgeOutputRequest {
    #[serde(rename = "BridgeArn")]
    #[serde(default)]
    pub bridge_arn: String,
    #[serde(rename = "OutputName")]
    #[serde(default)]
    pub output_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveBridgeOutputResponse {
    #[serde(rename = "bridgeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_arn: Option<String>,
    #[serde(rename = "outputName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveBridgeSourceRequest {
    #[serde(rename = "BridgeArn")]
    #[serde(default)]
    pub bridge_arn: String,
    #[serde(rename = "SourceName")]
    #[serde(default)]
    pub source_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveBridgeSourceResponse {
    #[serde(rename = "bridgeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_arn: Option<String>,
    #[serde(rename = "sourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveFlowMediaStreamRequest {
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
    #[serde(rename = "MediaStreamName")]
    #[serde(default)]
    pub media_stream_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveFlowMediaStreamResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "mediaStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveFlowOutputRequest {
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
    #[serde(rename = "OutputArn")]
    #[serde(default)]
    pub output_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveFlowOutputResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "outputArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveFlowSourceRequest {
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    pub source_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveFlowSourceResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "sourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveFlowVpcInterfaceRequest {
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
    #[serde(rename = "VpcInterfaceName")]
    #[serde(default)]
    pub vpc_interface_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveFlowVpcInterfaceResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "nonDeletedNetworkInterfaceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_deleted_network_interface_ids: Option<Vec<String>>,
    #[serde(rename = "vpcInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestartRouterInputRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestartRouterInputResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestartRouterOutputRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestartRouterOutputResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokeFlowEntitlementRequest {
    #[serde(rename = "EntitlementArn")]
    #[serde(default)]
    pub entitlement_arn: String,
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokeFlowEntitlementResponse {
    #[serde(rename = "entitlementArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartFlowRequest {
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartFlowResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRouterInputRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRouterInputResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(rename = "maintenanceScheduleType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRouterOutputRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRouterOutputResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(rename = "maintenanceScheduleType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopFlowRequest {
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopFlowResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopRouterInputRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopRouterInputResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopRouterOutputRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopRouterOutputResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagGlobalResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TakeRouterInputRequest {
    #[serde(rename = "routerInputArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_input_arn: Option<String>,
    #[serde(rename = "RouterOutputArn")]
    #[serde(default)]
    pub router_output_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TakeRouterInputResponse {
    #[serde(rename = "routedState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routed_state: Option<String>,
    #[serde(rename = "routerInputArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_input_arn: Option<String>,
    #[serde(rename = "routerInputName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_input_name: Option<String>,
    #[serde(rename = "routerOutputArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_output_arn: Option<String>,
    #[serde(rename = "routerOutputName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_output_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagGlobalResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
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
pub struct UpdateBridgeOutputRequest {
    #[serde(rename = "BridgeArn")]
    #[serde(default)]
    pub bridge_arn: String,
    #[serde(rename = "networkOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_output: Option<UpdateBridgeNetworkOutputRequest>,
    #[serde(rename = "OutputName")]
    #[serde(default)]
    pub output_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBridgeNetworkOutputRequest {
    #[serde(rename = "ipAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "networkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBridgeOutputResponse {
    #[serde(rename = "bridgeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<BridgeOutput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBridgeRequest {
    #[serde(rename = "BridgeArn")]
    #[serde(default)]
    pub bridge_arn: String,
    #[serde(rename = "egressGatewayBridge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_gateway_bridge: Option<UpdateEgressGatewayBridgeRequest>,
    #[serde(rename = "ingressGatewayBridge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_gateway_bridge: Option<UpdateIngressGatewayBridgeRequest>,
    #[serde(rename = "sourceFailoverConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_failover_config: Option<UpdateFailoverConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEgressGatewayBridgeRequest {
    #[serde(rename = "maxBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIngressGatewayBridgeRequest {
    #[serde(rename = "maxBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i32>,
    #[serde(rename = "maxOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_outputs: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFailoverConfig {
    #[serde(rename = "failoverMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_mode: Option<String>,
    #[serde(rename = "recoveryWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_window: Option<i32>,
    #[serde(rename = "sourcePriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_priority: Option<SourcePriority>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBridgeResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<Bridge>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBridgeSourceRequest {
    #[serde(rename = "BridgeArn")]
    #[serde(default)]
    pub bridge_arn: String,
    #[serde(rename = "flowSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_source: Option<UpdateBridgeFlowSourceRequest>,
    #[serde(rename = "networkSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source: Option<UpdateBridgeNetworkSourceRequest>,
    #[serde(rename = "SourceName")]
    #[serde(default)]
    pub source_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBridgeFlowSourceRequest {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "flowVpcInterfaceAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_vpc_interface_attachment: Option<VpcInterfaceAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBridgeNetworkSourceRequest {
    #[serde(rename = "multicastIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicast_ip: Option<String>,
    #[serde(rename = "multicastSourceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicast_source_settings: Option<MulticastSourceSettings>,
    #[serde(rename = "networkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBridgeSourceResponse {
    #[serde(rename = "bridgeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<BridgeSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBridgeStateRequest {
    #[serde(rename = "BridgeArn")]
    #[serde(default)]
    pub bridge_arn: String,
    #[serde(rename = "desiredState")]
    #[serde(default)]
    pub desired_state: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBridgeStateResponse {
    #[serde(rename = "bridgeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_arn: Option<String>,
    #[serde(rename = "desiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowEntitlementRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<UpdateEncryption>,
    #[serde(rename = "EntitlementArn")]
    #[serde(default)]
    pub entitlement_arn: String,
    #[serde(rename = "entitlementStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_status: Option<String>,
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEncryption {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(rename = "constantInitializationVector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    #[serde(rename = "deviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "keyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowEntitlementResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement: Option<Entitlement>,
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowMediaStreamRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<MediaStreamAttributesRequest>,
    #[serde(rename = "clockRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_rate: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
    #[serde(rename = "MediaStreamName")]
    #[serde(default)]
    pub media_stream_name: String,
    #[serde(rename = "mediaStreamType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_type: Option<String>,
    #[serde(rename = "videoFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_format: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowMediaStreamResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "mediaStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream: Option<MediaStream>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowOutputRequest {
    #[serde(rename = "cidrAllowList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_allow_list: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<UpdateEncryption>,
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
    #[serde(rename = "maxLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i32>,
    #[serde(rename = "mediaStreamOutputConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_output_configurations: Option<Vec<MediaStreamOutputConfigurationRequest>>,
    #[serde(rename = "minLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_latency: Option<i32>,
    #[serde(rename = "ndiProgramName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndi_program_name: Option<String>,
    #[serde(rename = "ndiSpeedHqQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndi_speed_hq_quality: Option<i32>,
    #[serde(rename = "OutputArn")]
    #[serde(default)]
    pub output_arn: String,
    #[serde(rename = "outputStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "remoteId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    #[serde(rename = "routerIntegrationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_integration_state: Option<String>,
    #[serde(rename = "routerIntegrationTransitEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_integration_transit_encryption: Option<FlowTransitEncryption>,
    #[serde(rename = "senderControlPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_control_port: Option<i32>,
    #[serde(rename = "senderIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_ip_address: Option<String>,
    #[serde(rename = "smoothingLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoothing_latency: Option<i32>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "vpcInterfaceAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_attachment: Option<VpcInterfaceAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowOutputResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Output>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowRequest {
    #[serde(rename = "encodingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_config: Option<EncodingConfig>,
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
    #[serde(rename = "flowSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_size: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<UpdateMaintenance>,
    #[serde(rename = "ndiConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndi_config: Option<NdiConfig>,
    #[serde(rename = "sourceFailoverConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_failover_config: Option<UpdateFailoverConfig>,
    #[serde(rename = "sourceMonitoringConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_monitoring_config: Option<MonitoringConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMaintenance {
    #[serde(rename = "maintenanceDay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_day: Option<String>,
    #[serde(rename = "maintenanceScheduledDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_scheduled_date: Option<String>,
    #[serde(rename = "maintenanceStartHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_start_hour: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<Flow>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowSourceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption: Option<UpdateEncryption>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "entitlementArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    #[serde(rename = "FlowArn")]
    #[serde(default)]
    pub flow_arn: String,
    #[serde(rename = "gatewayBridgeSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_bridge_source: Option<UpdateGatewayBridgeSourceRequest>,
    #[serde(rename = "ingestPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_port: Option<i32>,
    #[serde(rename = "maxBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i32>,
    #[serde(rename = "maxLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i32>,
    #[serde(rename = "maxSyncBuffer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sync_buffer: Option<i32>,
    #[serde(rename = "mediaStreamSourceConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_source_configurations: Option<Vec<MediaStreamSourceConfigurationRequest>>,
    #[serde(rename = "minLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_latency: Option<i32>,
    #[serde(rename = "ndiSourceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndi_source_settings: Option<NdiSourceSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "routerIntegrationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_integration_state: Option<String>,
    #[serde(rename = "routerIntegrationTransitDecryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_integration_transit_decryption: Option<FlowTransitEncryption>,
    #[serde(rename = "senderControlPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_control_port: Option<i32>,
    #[serde(rename = "senderIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_ip_address: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    pub source_arn: String,
    #[serde(rename = "sourceListenerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_listener_address: Option<String>,
    #[serde(rename = "sourceListenerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_listener_port: Option<i32>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "vpcInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_name: Option<String>,
    #[serde(rename = "whitelistCidr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_cidr: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGatewayBridgeSourceRequest {
    #[serde(rename = "bridgeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_arn: Option<String>,
    #[serde(rename = "vpcInterfaceAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_attachment: Option<VpcInterfaceAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowSourceResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGatewayInstanceRequest {
    #[serde(rename = "bridgePlacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_placement: Option<String>,
    #[serde(rename = "GatewayInstanceArn")]
    #[serde(default)]
    pub gateway_instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGatewayInstanceResponse {
    #[serde(rename = "bridgePlacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_placement: Option<String>,
    #[serde(rename = "gatewayInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_instance_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRouterInputRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<RouterInputConfiguration>,
    #[serde(rename = "maintenanceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration: Option<MaintenanceConfiguration>,
    #[serde(rename = "maximumBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "routingScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_scope: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "transitEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption: Option<RouterInputTransitEncryption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRouterInputResponse {
    #[serde(rename = "routerInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_input: Option<RouterInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRouterNetworkInterfaceRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<RouterNetworkInterfaceConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRouterNetworkInterfaceResponse {
    #[serde(rename = "routerNetworkInterface")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_network_interface: Option<RouterNetworkInterface>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRouterOutputRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<RouterOutputConfiguration>,
    #[serde(rename = "maintenanceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration: Option<MaintenanceConfiguration>,
    #[serde(rename = "maximumBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "routingScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_scope: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRouterOutputResponse {
    #[serde(rename = "routerOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_output: Option<RouterOutput>,
}
