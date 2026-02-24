//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-appmesh

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGatewayRouteInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "gatewayRouteName")]
    #[serde(default)]
    pub gateway_route_name: String,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(default)]
    pub spec: GatewayRouteSpec,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    #[serde(rename = "virtualGatewayName")]
    #[serde(default)]
    pub virtual_gateway_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GatewayRouteSpec {
    #[serde(rename = "grpcRoute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_route: Option<GrpcGatewayRoute>,
    #[serde(rename = "http2Route")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2_route: Option<HttpGatewayRoute>,
    #[serde(rename = "httpRoute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_route: Option<HttpGatewayRoute>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrpcGatewayRoute {
    #[serde(default)]
    pub action: GrpcGatewayRouteAction,
    #[serde(rename = "match")]
    #[serde(default)]
    pub r#match: GrpcGatewayRouteMatch,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrpcGatewayRouteAction {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<GrpcGatewayRouteRewrite>,
    #[serde(default)]
    pub target: GatewayRouteTarget,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrpcGatewayRouteRewrite {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<GatewayRouteHostnameRewrite>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GatewayRouteHostnameRewrite {
    #[serde(rename = "defaultTargetHostname")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_target_hostname: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GatewayRouteTarget {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "virtualService")]
    #[serde(default)]
    pub virtual_service: GatewayRouteVirtualService,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GatewayRouteVirtualService {
    #[serde(rename = "virtualServiceName")]
    #[serde(default)]
    pub virtual_service_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrpcGatewayRouteMatch {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<GatewayRouteHostnameMatch>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<GrpcGatewayRouteMetadata>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GatewayRouteHostnameMatch {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrpcGatewayRouteMetadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
    #[serde(rename = "match")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<GrpcMetadataMatchMethod>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrpcMetadataMatchMethod {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<MatchRange>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MatchRange {
    #[serde(default)]
    pub end: i64,
    #[serde(default)]
    pub start: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpGatewayRoute {
    #[serde(default)]
    pub action: HttpGatewayRouteAction,
    #[serde(rename = "match")]
    #[serde(default)]
    pub r#match: HttpGatewayRouteMatch,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpGatewayRouteAction {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<HttpGatewayRouteRewrite>,
    #[serde(default)]
    pub target: GatewayRouteTarget,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpGatewayRouteRewrite {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<GatewayRouteHostnameRewrite>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<HttpGatewayRoutePathRewrite>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<HttpGatewayRoutePrefixRewrite>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpGatewayRoutePathRewrite {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpGatewayRoutePrefixRewrite {
    #[serde(rename = "defaultPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_prefix: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpGatewayRouteMatch {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HttpGatewayRouteHeader>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<GatewayRouteHostnameMatch>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<HttpPathMatch>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "queryParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<Vec<HttpQueryParameter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpGatewayRouteHeader {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
    #[serde(rename = "match")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<HeaderMatchMethod>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HeaderMatchMethod {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<MatchRange>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpPathMatch {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpQueryParameter {
    #[serde(rename = "match")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<QueryParameterMatch>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryParameterMatch {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagRef {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGatewayRouteOutput {
    #[serde(rename = "gatewayRoute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_route: Option<GatewayRouteData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GatewayRouteData {
    #[serde(rename = "gatewayRouteName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_route_name: Option<String>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ResourceMetadata>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<GatewayRouteSpec>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GatewayRouteStatus>,
    #[serde(rename = "virtualGatewayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceMetadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "resourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GatewayRouteStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMeshInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<MeshSpec>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeshSpec {
    #[serde(rename = "egressFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_filter: Option<EgressFilter>,
    #[serde(rename = "serviceDiscovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_discovery: Option<MeshServiceDiscovery>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EgressFilter {
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeshServiceDiscovery {
    #[serde(rename = "ipPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_preference: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMeshOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<MeshData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeshData {
    #[serde(rename = "meshName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ResourceMetadata>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<MeshSpec>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MeshStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeshStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRouteInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "routeName")]
    #[serde(default)]
    pub route_name: String,
    #[serde(default)]
    pub spec: RouteSpec,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    #[serde(rename = "virtualRouterName")]
    #[serde(default)]
    pub virtual_router_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteSpec {
    #[serde(rename = "grpcRoute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_route: Option<GrpcRoute>,
    #[serde(rename = "http2Route")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2_route: Option<HttpRoute>,
    #[serde(rename = "httpRoute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_route: Option<HttpRoute>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "tcpRoute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_route: Option<TcpRoute>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrpcRoute {
    #[serde(default)]
    pub action: GrpcRouteAction,
    #[serde(rename = "match")]
    #[serde(default)]
    pub r#match: GrpcRouteMatch,
    #[serde(rename = "retryPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<GrpcRetryPolicy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<GrpcTimeout>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrpcRouteAction {
    #[serde(rename = "weightedTargets")]
    #[serde(default)]
    pub weighted_targets: Vec<WeightedTarget>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WeightedTarget {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "virtualNode")]
    #[serde(default)]
    pub virtual_node: String,
    #[serde(default)]
    pub weight: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrpcRouteMatch {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<GrpcRouteMetadata>>,
    #[serde(rename = "methodName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrpcRouteMetadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
    #[serde(rename = "match")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<GrpcRouteMetadataMatchMethod>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrpcRouteMetadataMatchMethod {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<MatchRange>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrpcRetryPolicy {
    #[serde(rename = "grpcRetryEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_retry_events: Option<Vec<String>>,
    #[serde(rename = "httpRetryEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_retry_events: Option<Vec<String>>,
    #[serde(rename = "maxRetries")]
    #[serde(default)]
    pub max_retries: i64,
    #[serde(rename = "perRetryTimeout")]
    #[serde(default)]
    pub per_retry_timeout: Duration,
    #[serde(rename = "tcpRetryEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_retry_events: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Duration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrpcTimeout {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<Duration>,
    #[serde(rename = "perRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_request: Option<Duration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpRoute {
    #[serde(default)]
    pub action: HttpRouteAction,
    #[serde(rename = "match")]
    #[serde(default)]
    pub r#match: HttpRouteMatch,
    #[serde(rename = "retryPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<HttpRetryPolicy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<HttpTimeout>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpRouteAction {
    #[serde(rename = "weightedTargets")]
    #[serde(default)]
    pub weighted_targets: Vec<WeightedTarget>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpRouteMatch {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HttpRouteHeader>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<HttpPathMatch>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "queryParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<Vec<HttpQueryParameter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpRouteHeader {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
    #[serde(rename = "match")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<HeaderMatchMethod>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpRetryPolicy {
    #[serde(rename = "httpRetryEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_retry_events: Option<Vec<String>>,
    #[serde(rename = "maxRetries")]
    #[serde(default)]
    pub max_retries: i64,
    #[serde(rename = "perRetryTimeout")]
    #[serde(default)]
    pub per_retry_timeout: Duration,
    #[serde(rename = "tcpRetryEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_retry_events: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpTimeout {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<Duration>,
    #[serde(rename = "perRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_request: Option<Duration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TcpRoute {
    #[serde(default)]
    pub action: TcpRouteAction,
    #[serde(rename = "match")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<TcpRouteMatch>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<TcpTimeout>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TcpRouteAction {
    #[serde(rename = "weightedTargets")]
    #[serde(default)]
    pub weighted_targets: Vec<WeightedTarget>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TcpRouteMatch {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TcpTimeout {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<Duration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRouteOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<RouteData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteData {
    #[serde(rename = "meshName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ResourceMetadata>,
    #[serde(rename = "routeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<RouteSpec>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RouteStatus>,
    #[serde(rename = "virtualRouterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_router_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVirtualGatewayInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(default)]
    pub spec: VirtualGatewaySpec,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    #[serde(rename = "virtualGatewayName")]
    #[serde(default)]
    pub virtual_gateway_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewaySpec {
    #[serde(rename = "backendDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_defaults: Option<VirtualGatewayBackendDefaults>,
    #[serde(default)]
    pub listeners: Vec<VirtualGatewayListener>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<VirtualGatewayLogging>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayBackendDefaults {
    #[serde(rename = "clientPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_policy: Option<VirtualGatewayClientPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayClientPolicy {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<VirtualGatewayClientPolicyTls>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayClientPolicyTls {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<VirtualGatewayClientTlsCertificate>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<i32>>,
    #[serde(default)]
    pub validation: VirtualGatewayTlsValidationContext,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayClientTlsCertificate {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<VirtualGatewayListenerTlsFileCertificate>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sds: Option<VirtualGatewayListenerTlsSdsCertificate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayListenerTlsFileCertificate {
    #[serde(rename = "certificateChain")]
    #[serde(default)]
    pub certificate_chain: String,
    #[serde(rename = "privateKey")]
    #[serde(default)]
    pub private_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayListenerTlsSdsCertificate {
    #[serde(rename = "secretName")]
    #[serde(default)]
    pub secret_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayTlsValidationContext {
    #[serde(rename = "subjectAlternativeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<SubjectAlternativeNames>,
    #[serde(default)]
    pub trust: VirtualGatewayTlsValidationContextTrust,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubjectAlternativeNames {
    #[serde(rename = "match")]
    #[serde(default)]
    pub r#match: SubjectAlternativeNameMatchers,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubjectAlternativeNameMatchers {
    #[serde(default)]
    pub exact: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayTlsValidationContextTrust {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm: Option<VirtualGatewayTlsValidationContextAcmTrust>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<VirtualGatewayTlsValidationContextFileTrust>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sds: Option<VirtualGatewayTlsValidationContextSdsTrust>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayTlsValidationContextAcmTrust {
    #[serde(rename = "certificateAuthorityArns")]
    #[serde(default)]
    pub certificate_authority_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayTlsValidationContextFileTrust {
    #[serde(rename = "certificateChain")]
    #[serde(default)]
    pub certificate_chain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayTlsValidationContextSdsTrust {
    #[serde(rename = "secretName")]
    #[serde(default)]
    pub secret_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayListener {
    #[serde(rename = "connectionPool")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_pool: Option<VirtualGatewayConnectionPool>,
    #[serde(rename = "healthCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<VirtualGatewayHealthCheckPolicy>,
    #[serde(rename = "portMapping")]
    #[serde(default)]
    pub port_mapping: VirtualGatewayPortMapping,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<VirtualGatewayListenerTls>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayConnectionPool {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc: Option<VirtualGatewayGrpcConnectionPool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<VirtualGatewayHttpConnectionPool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2: Option<VirtualGatewayHttp2ConnectionPool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayGrpcConnectionPool {
    #[serde(rename = "maxRequests")]
    #[serde(default)]
    pub max_requests: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayHttpConnectionPool {
    #[serde(rename = "maxConnections")]
    #[serde(default)]
    pub max_connections: i32,
    #[serde(rename = "maxPendingRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pending_requests: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayHttp2ConnectionPool {
    #[serde(rename = "maxRequests")]
    #[serde(default)]
    pub max_requests: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayHealthCheckPolicy {
    #[serde(rename = "healthyThreshold")]
    #[serde(default)]
    pub healthy_threshold: i32,
    #[serde(rename = "intervalMillis")]
    #[serde(default)]
    pub interval_millis: i64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    pub protocol: String,
    #[serde(rename = "timeoutMillis")]
    #[serde(default)]
    pub timeout_millis: i64,
    #[serde(rename = "unhealthyThreshold")]
    #[serde(default)]
    pub unhealthy_threshold: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayPortMapping {
    #[serde(default)]
    pub port: i32,
    #[serde(default)]
    pub protocol: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayListenerTls {
    #[serde(default)]
    pub certificate: VirtualGatewayListenerTlsCertificate,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<VirtualGatewayListenerTlsValidationContext>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayListenerTlsCertificate {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm: Option<VirtualGatewayListenerTlsAcmCertificate>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<VirtualGatewayListenerTlsFileCertificate>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sds: Option<VirtualGatewayListenerTlsSdsCertificate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayListenerTlsAcmCertificate {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayListenerTlsValidationContext {
    #[serde(rename = "subjectAlternativeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<SubjectAlternativeNames>,
    #[serde(default)]
    pub trust: VirtualGatewayListenerTlsValidationContextTrust,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayListenerTlsValidationContextTrust {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<VirtualGatewayTlsValidationContextFileTrust>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sds: Option<VirtualGatewayTlsValidationContextSdsTrust>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayLogging {
    #[serde(rename = "accessLog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log: Option<VirtualGatewayAccessLog>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayAccessLog {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<VirtualGatewayFileAccessLog>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayFileAccessLog {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<LoggingFormat>,
    #[serde(default)]
    pub path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingFormat {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<Vec<JsonFormatRef>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JsonFormatRef {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVirtualGatewayOutput {
    #[serde(rename = "virtualGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway: Option<VirtualGatewayData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayData {
    #[serde(rename = "meshName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ResourceMetadata>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<VirtualGatewaySpec>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VirtualGatewayStatus>,
    #[serde(rename = "virtualGatewayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVirtualNodeInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(default)]
    pub spec: VirtualNodeSpec,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    #[serde(rename = "virtualNodeName")]
    #[serde(default)]
    pub virtual_node_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualNodeSpec {
    #[serde(rename = "backendDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_defaults: Option<BackendDefaults>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<Backend>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<Listener>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    #[serde(rename = "serviceDiscovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_discovery: Option<ServiceDiscovery>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackendDefaults {
    #[serde(rename = "clientPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_policy: Option<ClientPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientPolicy {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<ClientPolicyTls>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientPolicyTls {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<ClientTlsCertificate>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<i32>>,
    #[serde(default)]
    pub validation: TlsValidationContext,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientTlsCertificate {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<ListenerTlsFileCertificate>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sds: Option<ListenerTlsSdsCertificate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListenerTlsFileCertificate {
    #[serde(rename = "certificateChain")]
    #[serde(default)]
    pub certificate_chain: String,
    #[serde(rename = "privateKey")]
    #[serde(default)]
    pub private_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListenerTlsSdsCertificate {
    #[serde(rename = "secretName")]
    #[serde(default)]
    pub secret_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TlsValidationContext {
    #[serde(rename = "subjectAlternativeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<SubjectAlternativeNames>,
    #[serde(default)]
    pub trust: TlsValidationContextTrust,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TlsValidationContextTrust {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm: Option<TlsValidationContextAcmTrust>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<TlsValidationContextFileTrust>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sds: Option<TlsValidationContextSdsTrust>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TlsValidationContextAcmTrust {
    #[serde(rename = "certificateAuthorityArns")]
    #[serde(default)]
    pub certificate_authority_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TlsValidationContextFileTrust {
    #[serde(rename = "certificateChain")]
    #[serde(default)]
    pub certificate_chain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TlsValidationContextSdsTrust {
    #[serde(rename = "secretName")]
    #[serde(default)]
    pub secret_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Backend {
    #[serde(rename = "virtualService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_service: Option<VirtualServiceBackend>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualServiceBackend {
    #[serde(rename = "clientPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_policy: Option<ClientPolicy>,
    #[serde(rename = "virtualServiceName")]
    #[serde(default)]
    pub virtual_service_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Listener {
    #[serde(rename = "connectionPool")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_pool: Option<VirtualNodeConnectionPool>,
    #[serde(rename = "healthCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheckPolicy>,
    #[serde(rename = "outlierDetection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlier_detection: Option<OutlierDetection>,
    #[serde(rename = "portMapping")]
    #[serde(default)]
    pub port_mapping: PortMapping,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<ListenerTimeout>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<ListenerTls>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualNodeConnectionPool {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc: Option<VirtualNodeGrpcConnectionPool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<VirtualNodeHttpConnectionPool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2: Option<VirtualNodeHttp2ConnectionPool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp: Option<VirtualNodeTcpConnectionPool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualNodeGrpcConnectionPool {
    #[serde(rename = "maxRequests")]
    #[serde(default)]
    pub max_requests: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualNodeHttpConnectionPool {
    #[serde(rename = "maxConnections")]
    #[serde(default)]
    pub max_connections: i32,
    #[serde(rename = "maxPendingRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pending_requests: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualNodeHttp2ConnectionPool {
    #[serde(rename = "maxRequests")]
    #[serde(default)]
    pub max_requests: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualNodeTcpConnectionPool {
    #[serde(rename = "maxConnections")]
    #[serde(default)]
    pub max_connections: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HealthCheckPolicy {
    #[serde(rename = "healthyThreshold")]
    #[serde(default)]
    pub healthy_threshold: i32,
    #[serde(rename = "intervalMillis")]
    #[serde(default)]
    pub interval_millis: i64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    pub protocol: String,
    #[serde(rename = "timeoutMillis")]
    #[serde(default)]
    pub timeout_millis: i64,
    #[serde(rename = "unhealthyThreshold")]
    #[serde(default)]
    pub unhealthy_threshold: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutlierDetection {
    #[serde(rename = "baseEjectionDuration")]
    #[serde(default)]
    pub base_ejection_duration: Duration,
    #[serde(default)]
    pub interval: Duration,
    #[serde(rename = "maxEjectionPercent")]
    #[serde(default)]
    pub max_ejection_percent: i32,
    #[serde(rename = "maxServerErrors")]
    #[serde(default)]
    pub max_server_errors: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortMapping {
    #[serde(default)]
    pub port: i32,
    #[serde(default)]
    pub protocol: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListenerTimeout {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc: Option<GrpcTimeout>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpTimeout>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2: Option<HttpTimeout>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp: Option<TcpTimeout>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListenerTls {
    #[serde(default)]
    pub certificate: ListenerTlsCertificate,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<ListenerTlsValidationContext>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListenerTlsCertificate {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm: Option<ListenerTlsAcmCertificate>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<ListenerTlsFileCertificate>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sds: Option<ListenerTlsSdsCertificate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListenerTlsAcmCertificate {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListenerTlsValidationContext {
    #[serde(rename = "subjectAlternativeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<SubjectAlternativeNames>,
    #[serde(default)]
    pub trust: ListenerTlsValidationContextTrust,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListenerTlsValidationContextTrust {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<TlsValidationContextFileTrust>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sds: Option<TlsValidationContextSdsTrust>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Logging {
    #[serde(rename = "accessLog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log: Option<AccessLog>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessLog {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileAccessLog>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileAccessLog {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<LoggingFormat>,
    #[serde(default)]
    pub path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceDiscovery {
    #[serde(rename = "awsCloudMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_cloud_map: Option<AwsCloudMapServiceDiscovery>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<DnsServiceDiscovery>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudMapServiceDiscovery {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AwsCloudMapInstanceAttribute>>,
    #[serde(rename = "ipPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_preference: Option<String>,
    #[serde(rename = "namespaceName")]
    #[serde(default)]
    pub namespace_name: String,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    pub service_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudMapInstanceAttribute {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnsServiceDiscovery {
    #[serde(default)]
    pub hostname: String,
    #[serde(rename = "ipPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_preference: Option<String>,
    #[serde(rename = "responseType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVirtualNodeOutput {
    #[serde(rename = "virtualNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node: Option<VirtualNodeData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualNodeData {
    #[serde(rename = "meshName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ResourceMetadata>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<VirtualNodeSpec>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VirtualNodeStatus>,
    #[serde(rename = "virtualNodeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualNodeStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVirtualRouterInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(default)]
    pub spec: VirtualRouterSpec,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    #[serde(rename = "virtualRouterName")]
    #[serde(default)]
    pub virtual_router_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualRouterSpec {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<VirtualRouterListener>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualRouterListener {
    #[serde(rename = "portMapping")]
    #[serde(default)]
    pub port_mapping: PortMapping,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVirtualRouterOutput {
    #[serde(rename = "virtualRouter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_router: Option<VirtualRouterData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualRouterData {
    #[serde(rename = "meshName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ResourceMetadata>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<VirtualRouterSpec>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VirtualRouterStatus>,
    #[serde(rename = "virtualRouterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_router_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualRouterStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVirtualServiceInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(default)]
    pub spec: VirtualServiceSpec,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    #[serde(rename = "virtualServiceName")]
    #[serde(default)]
    pub virtual_service_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualServiceSpec {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<VirtualServiceProvider>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualServiceProvider {
    #[serde(rename = "virtualNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node: Option<VirtualNodeServiceProvider>,
    #[serde(rename = "virtualRouter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_router: Option<VirtualRouterServiceProvider>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualNodeServiceProvider {
    #[serde(rename = "virtualNodeName")]
    #[serde(default)]
    pub virtual_node_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualRouterServiceProvider {
    #[serde(rename = "virtualRouterName")]
    #[serde(default)]
    pub virtual_router_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVirtualServiceOutput {
    #[serde(rename = "virtualService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_service: Option<VirtualServiceData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualServiceData {
    #[serde(rename = "meshName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ResourceMetadata>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<VirtualServiceSpec>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VirtualServiceStatus>,
    #[serde(rename = "virtualServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualServiceStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGatewayRouteInput {
    #[serde(rename = "gatewayRouteName")]
    #[serde(default)]
    pub gateway_route_name: String,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "virtualGatewayName")]
    #[serde(default)]
    pub virtual_gateway_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGatewayRouteOutput {
    #[serde(rename = "gatewayRoute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_route: Option<GatewayRouteData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMeshInput {
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMeshOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<MeshData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRouteInput {
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "routeName")]
    #[serde(default)]
    pub route_name: String,
    #[serde(rename = "virtualRouterName")]
    #[serde(default)]
    pub virtual_router_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRouteOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<RouteData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVirtualGatewayInput {
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "virtualGatewayName")]
    #[serde(default)]
    pub virtual_gateway_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVirtualGatewayOutput {
    #[serde(rename = "virtualGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway: Option<VirtualGatewayData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVirtualNodeInput {
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "virtualNodeName")]
    #[serde(default)]
    pub virtual_node_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVirtualNodeOutput {
    #[serde(rename = "virtualNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node: Option<VirtualNodeData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVirtualRouterInput {
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "virtualRouterName")]
    #[serde(default)]
    pub virtual_router_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVirtualRouterOutput {
    #[serde(rename = "virtualRouter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_router: Option<VirtualRouterData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVirtualServiceInput {
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "virtualServiceName")]
    #[serde(default)]
    pub virtual_service_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVirtualServiceOutput {
    #[serde(rename = "virtualService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_service: Option<VirtualServiceData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGatewayRouteInput {
    #[serde(rename = "gatewayRouteName")]
    #[serde(default)]
    pub gateway_route_name: String,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "virtualGatewayName")]
    #[serde(default)]
    pub virtual_gateway_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGatewayRouteOutput {
    #[serde(rename = "gatewayRoute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_route: Option<GatewayRouteData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMeshInput {
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMeshOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<MeshData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRouteInput {
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "routeName")]
    #[serde(default)]
    pub route_name: String,
    #[serde(rename = "virtualRouterName")]
    #[serde(default)]
    pub virtual_router_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRouteOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<RouteData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVirtualGatewayInput {
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "virtualGatewayName")]
    #[serde(default)]
    pub virtual_gateway_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVirtualGatewayOutput {
    #[serde(rename = "virtualGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway: Option<VirtualGatewayData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVirtualNodeInput {
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "virtualNodeName")]
    #[serde(default)]
    pub virtual_node_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVirtualNodeOutput {
    #[serde(rename = "virtualNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node: Option<VirtualNodeData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVirtualRouterInput {
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "virtualRouterName")]
    #[serde(default)]
    pub virtual_router_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVirtualRouterOutput {
    #[serde(rename = "virtualRouter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_router: Option<VirtualRouterData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVirtualServiceInput {
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "virtualServiceName")]
    #[serde(default)]
    pub virtual_service_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVirtualServiceOutput {
    #[serde(rename = "virtualService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_service: Option<VirtualServiceData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGatewayRoutesInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "virtualGatewayName")]
    #[serde(default)]
    pub virtual_gateway_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGatewayRoutesOutput {
    #[serde(rename = "gatewayRoutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_routes: Option<Vec<GatewayRouteRef>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GatewayRouteRef {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "gatewayRouteName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_route_name: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_name: Option<String>,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "resourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "virtualGatewayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMeshesInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMeshesOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meshes: Option<Vec<MeshRef>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeshRef {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_name: Option<String>,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "resourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRoutesInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "virtualRouterName")]
    #[serde(default)]
    pub virtual_router_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRoutesOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<RouteRef>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteRef {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_name: Option<String>,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "resourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(rename = "routeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "virtualRouterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_router_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVirtualGatewaysInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVirtualGatewaysOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "virtualGateways")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateways: Option<Vec<VirtualGatewayRef>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGatewayRef {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_name: Option<String>,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "resourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "virtualGatewayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVirtualNodesInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVirtualNodesOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "virtualNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_nodes: Option<Vec<VirtualNodeRef>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualNodeRef {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_name: Option<String>,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "resourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "virtualNodeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVirtualRoutersInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVirtualRoutersOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "virtualRouters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_routers: Option<Vec<VirtualRouterRef>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualRouterRef {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_name: Option<String>,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "resourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "virtualRouterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_router_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVirtualServicesInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVirtualServicesOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "virtualServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_services: Option<Vec<VirtualServiceRef>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualServiceRef {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_name: Option<String>,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "resourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "virtualServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: Vec<TagRef>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGatewayRouteInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "gatewayRouteName")]
    #[serde(default)]
    pub gateway_route_name: String,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(default)]
    pub spec: GatewayRouteSpec,
    #[serde(rename = "virtualGatewayName")]
    #[serde(default)]
    pub virtual_gateway_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGatewayRouteOutput {
    #[serde(rename = "gatewayRoute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_route: Option<GatewayRouteData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMeshInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<MeshSpec>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMeshOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<MeshData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRouteInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(rename = "routeName")]
    #[serde(default)]
    pub route_name: String,
    #[serde(default)]
    pub spec: RouteSpec,
    #[serde(rename = "virtualRouterName")]
    #[serde(default)]
    pub virtual_router_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRouteOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<RouteData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVirtualGatewayInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(default)]
    pub spec: VirtualGatewaySpec,
    #[serde(rename = "virtualGatewayName")]
    #[serde(default)]
    pub virtual_gateway_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVirtualGatewayOutput {
    #[serde(rename = "virtualGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway: Option<VirtualGatewayData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVirtualNodeInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(default)]
    pub spec: VirtualNodeSpec,
    #[serde(rename = "virtualNodeName")]
    #[serde(default)]
    pub virtual_node_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVirtualNodeOutput {
    #[serde(rename = "virtualNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node: Option<VirtualNodeData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVirtualRouterInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(default)]
    pub spec: VirtualRouterSpec,
    #[serde(rename = "virtualRouterName")]
    #[serde(default)]
    pub virtual_router_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVirtualRouterOutput {
    #[serde(rename = "virtualRouter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_router: Option<VirtualRouterData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVirtualServiceInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "meshName")]
    #[serde(default)]
    pub mesh_name: String,
    #[serde(rename = "meshOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    #[serde(default)]
    pub spec: VirtualServiceSpec,
    #[serde(rename = "virtualServiceName")]
    #[serde(default)]
    pub virtual_service_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVirtualServiceOutput {
    #[serde(rename = "virtualService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_service: Option<VirtualServiceData>,
}
