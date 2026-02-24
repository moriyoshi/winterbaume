//! Serde-compatible view types for API Gateway V2 state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ApiGatewayV2Service;
use crate::state::ApiGatewayV2State;
use crate::types::{
    Api, ApiMapping, Authorizer, Deployment, DomainName, Integration, IntegrationResponse, Model,
    Route, RouteResponse, Stage, StoredDomainNameConfiguration, VpcLink,
};

/// Serializable view of the entire API Gateway V2 state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiGatewayV2StateView {
    /// APIs keyed by API ID.
    #[serde(default)]
    pub apis: HashMap<String, ApiView>,
    /// Stages keyed by "{api_id}/{stage_name}".
    #[serde(default)]
    pub stages: HashMap<String, StageView>,
    /// Integrations keyed by "{api_id}/{integration_id}".
    #[serde(default)]
    pub integrations: HashMap<String, IntegrationView>,
    /// Routes keyed by "{api_id}/{route_id}".
    #[serde(default)]
    pub routes: HashMap<String, RouteView>,
    /// Deployments keyed by "{api_id}/{deployment_id}".
    #[serde(default)]
    pub deployments: HashMap<String, DeploymentView>,
    /// Authorizers keyed by "{api_id}/{authorizer_id}".
    #[serde(default)]
    pub authorizers: HashMap<String, AuthorizerView>,
    /// Models keyed by "{api_id}/{model_id}".
    #[serde(default)]
    pub models: HashMap<String, ModelView>,
    /// VPC links keyed by vpc_link_id.
    #[serde(default)]
    pub vpc_links: HashMap<String, VpcLinkView>,
    /// Domain names keyed by domain_name.
    #[serde(default)]
    pub domain_names: HashMap<String, DomainNameView>,
    /// API mappings keyed by "{domain_name}/{mapping_id}".
    #[serde(default)]
    pub api_mappings: HashMap<String, ApiMappingView>,
    /// Integration responses keyed by "{api_id}/{integration_id}/{response_id}".
    #[serde(default)]
    pub integration_responses: HashMap<String, IntegrationResponseView>,
    /// Route responses keyed by "{api_id}/{route_id}/{response_id}".
    #[serde(default)]
    pub route_responses: HashMap<String, RouteResponseView>,
    /// Resource tags keyed by resource ARN.
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiView {
    pub api_id: String,
    pub name: String,
    pub protocol_type: String,
    pub route_selection_expression: Option<String>,
    pub description: Option<String>,
    pub api_endpoint: String,
    pub created_date: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageView {
    pub stage_name: String,
    pub api_id: String,
    pub description: Option<String>,
    pub deployment_id: Option<String>,
    pub auto_deploy: bool,
    pub created_date: String,
    pub last_updated_date: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationView {
    pub integration_id: String,
    pub api_id: String,
    pub integration_type: String,
    pub integration_uri: Option<String>,
    pub integration_method: Option<String>,
    pub description: Option<String>,
    pub payload_format_version: Option<String>,
    pub connection_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteView {
    pub route_id: String,
    pub api_id: String,
    pub route_key: String,
    pub target: Option<String>,
    pub authorization_type: Option<String>,
    pub authorizer_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentView {
    pub deployment_id: String,
    pub api_id: String,
    pub deployment_status: String,
    pub description: Option<String>,
    pub created_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizerView {
    pub authorizer_id: String,
    pub api_id: String,
    pub authorizer_type: String,
    pub authorizer_uri: Option<String>,
    pub authorizer_credentials_arn: Option<String>,
    pub authorizer_payload_format_version: Option<String>,
    pub authorizer_result_ttl_in_seconds: Option<i32>,
    pub identity_source: Option<Vec<String>>,
    pub identity_validation_expression: Option<String>,
    pub name: String,
    pub enable_simple_responses: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelView {
    pub model_id: String,
    pub api_id: String,
    pub name: String,
    pub content_type: Option<String>,
    pub description: Option<String>,
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcLinkView {
    pub vpc_link_id: String,
    pub name: String,
    pub security_group_ids: Vec<String>,
    pub subnet_ids: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub created_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredDomainNameConfigurationView {
    pub certificate_arn: Option<String>,
    pub endpoint_type: Option<String>,
    pub security_policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainNameView {
    pub domain_name: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub domain_name_configurations: Vec<StoredDomainNameConfigurationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMappingView {
    pub api_mapping_id: String,
    pub domain_name: String,
    pub api_id: String,
    pub stage: String,
    pub api_mapping_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationResponseView {
    pub integration_response_id: String,
    pub api_id: String,
    pub integration_id: String,
    pub integration_response_key: String,
    pub content_handling_strategy: Option<String>,
    #[serde(default)]
    pub response_parameters: Option<HashMap<String, String>>,
    #[serde(default)]
    pub response_templates: Option<HashMap<String, String>>,
    pub template_selection_expression: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResponseView {
    pub route_response_id: String,
    pub api_id: String,
    pub route_id: String,
    pub route_response_key: String,
    pub model_selection_expression: Option<String>,
}

// --- From internal types to view types ---

impl From<&ApiGatewayV2State> for ApiGatewayV2StateView {
    fn from(state: &ApiGatewayV2State) -> Self {
        ApiGatewayV2StateView {
            apis: state
                .apis
                .iter()
                .map(|(k, v)| (k.clone(), ApiView::from(v)))
                .collect(),
            stages: state
                .stages
                .iter()
                .map(|(k, v)| (k.clone(), StageView::from(v)))
                .collect(),
            integrations: state
                .integrations
                .iter()
                .map(|(k, v)| (k.clone(), IntegrationView::from(v)))
                .collect(),
            routes: state
                .routes
                .iter()
                .map(|(k, v)| (k.clone(), RouteView::from(v)))
                .collect(),
            deployments: state
                .deployments
                .iter()
                .map(|(k, v)| (k.clone(), DeploymentView::from(v)))
                .collect(),
            authorizers: state
                .authorizers
                .iter()
                .map(|(k, v)| (k.clone(), AuthorizerView::from(v)))
                .collect(),
            models: state
                .models
                .iter()
                .map(|(k, v)| (k.clone(), ModelView::from(v)))
                .collect(),
            vpc_links: state
                .vpc_links
                .iter()
                .map(|(k, v)| (k.clone(), VpcLinkView::from(v)))
                .collect(),
            domain_names: state
                .domain_names
                .iter()
                .map(|(k, v)| (k.clone(), DomainNameView::from(v)))
                .collect(),
            api_mappings: state
                .api_mappings
                .iter()
                .map(|(k, v)| (k.clone(), ApiMappingView::from(v)))
                .collect(),
            integration_responses: state
                .integration_responses
                .iter()
                .map(|(k, v)| (k.clone(), IntegrationResponseView::from(v)))
                .collect(),
            route_responses: state
                .route_responses
                .iter()
                .map(|(k, v)| (k.clone(), RouteResponseView::from(v)))
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

impl From<&Api> for ApiView {
    fn from(a: &Api) -> Self {
        ApiView {
            api_id: a.api_id.clone(),
            name: a.name.clone(),
            protocol_type: a.protocol_type.clone(),
            route_selection_expression: a.route_selection_expression.clone(),
            description: a.description.clone(),
            api_endpoint: a.api_endpoint.clone(),
            created_date: a.created_date.clone(),
            tags: a.tags.clone(),
        }
    }
}

impl From<&Stage> for StageView {
    fn from(s: &Stage) -> Self {
        StageView {
            stage_name: s.stage_name.clone(),
            api_id: s.api_id.clone(),
            description: s.description.clone(),
            deployment_id: s.deployment_id.clone(),
            auto_deploy: s.auto_deploy,
            created_date: s.created_date.clone(),
            last_updated_date: s.last_updated_date.clone(),
            tags: s.tags.clone(),
        }
    }
}

impl From<&Integration> for IntegrationView {
    fn from(i: &Integration) -> Self {
        IntegrationView {
            integration_id: i.integration_id.clone(),
            api_id: i.api_id.clone(),
            integration_type: i.integration_type.clone(),
            integration_uri: i.integration_uri.clone(),
            integration_method: i.integration_method.clone(),
            description: i.description.clone(),
            payload_format_version: i.payload_format_version.clone(),
            connection_type: i.connection_type.clone(),
        }
    }
}

impl From<&Route> for RouteView {
    fn from(r: &Route) -> Self {
        RouteView {
            route_id: r.route_id.clone(),
            api_id: r.api_id.clone(),
            route_key: r.route_key.clone(),
            target: r.target.clone(),
            authorization_type: r.authorization_type.clone(),
            authorizer_id: r.authorizer_id.clone(),
        }
    }
}

impl From<&Deployment> for DeploymentView {
    fn from(d: &Deployment) -> Self {
        DeploymentView {
            deployment_id: d.deployment_id.clone(),
            api_id: d.api_id.clone(),
            deployment_status: d.deployment_status.clone(),
            description: d.description.clone(),
            created_date: d.created_date.clone(),
        }
    }
}

impl From<&Authorizer> for AuthorizerView {
    fn from(a: &Authorizer) -> Self {
        AuthorizerView {
            authorizer_id: a.authorizer_id.clone(),
            api_id: a.api_id.clone(),
            authorizer_type: a.authorizer_type.clone(),
            authorizer_uri: a.authorizer_uri.clone(),
            authorizer_credentials_arn: a.authorizer_credentials_arn.clone(),
            authorizer_payload_format_version: a.authorizer_payload_format_version.clone(),
            authorizer_result_ttl_in_seconds: a.authorizer_result_ttl_in_seconds,
            identity_source: a.identity_source.clone(),
            identity_validation_expression: a.identity_validation_expression.clone(),
            name: a.name.clone(),
            enable_simple_responses: a.enable_simple_responses,
        }
    }
}

impl From<&Model> for ModelView {
    fn from(m: &Model) -> Self {
        ModelView {
            model_id: m.model_id.clone(),
            api_id: m.api_id.clone(),
            name: m.name.clone(),
            content_type: m.content_type.clone(),
            description: m.description.clone(),
            schema: m.schema.clone(),
        }
    }
}

impl From<&VpcLink> for VpcLinkView {
    fn from(v: &VpcLink) -> Self {
        VpcLinkView {
            vpc_link_id: v.vpc_link_id.clone(),
            name: v.name.clone(),
            security_group_ids: v.security_group_ids.clone(),
            subnet_ids: v.subnet_ids.clone(),
            tags: v.tags.clone(),
            created_date: v.created_date.clone(),
        }
    }
}

impl From<&StoredDomainNameConfiguration> for StoredDomainNameConfigurationView {
    fn from(c: &StoredDomainNameConfiguration) -> Self {
        StoredDomainNameConfigurationView {
            certificate_arn: c.certificate_arn.clone(),
            endpoint_type: c.endpoint_type.clone(),
            security_policy: c.security_policy.clone(),
        }
    }
}

impl From<&DomainName> for DomainNameView {
    fn from(d: &DomainName) -> Self {
        DomainNameView {
            domain_name: d.domain_name.clone(),
            tags: d.tags.clone(),
            domain_name_configurations: d
                .domain_name_configurations
                .iter()
                .map(StoredDomainNameConfigurationView::from)
                .collect(),
        }
    }
}

impl From<&ApiMapping> for ApiMappingView {
    fn from(m: &ApiMapping) -> Self {
        ApiMappingView {
            api_mapping_id: m.api_mapping_id.clone(),
            domain_name: m.domain_name.clone(),
            api_id: m.api_id.clone(),
            stage: m.stage.clone(),
            api_mapping_key: m.api_mapping_key.clone(),
        }
    }
}

impl From<&IntegrationResponse> for IntegrationResponseView {
    fn from(r: &IntegrationResponse) -> Self {
        IntegrationResponseView {
            integration_response_id: r.integration_response_id.clone(),
            api_id: r.api_id.clone(),
            integration_id: r.integration_id.clone(),
            integration_response_key: r.integration_response_key.clone(),
            content_handling_strategy: r.content_handling_strategy.clone(),
            response_parameters: r.response_parameters.clone(),
            response_templates: r.response_templates.clone(),
            template_selection_expression: r.template_selection_expression.clone(),
        }
    }
}

impl From<&RouteResponse> for RouteResponseView {
    fn from(r: &RouteResponse) -> Self {
        RouteResponseView {
            route_response_id: r.route_response_id.clone(),
            api_id: r.api_id.clone(),
            route_id: r.route_id.clone(),
            route_response_key: r.route_response_key.clone(),
            model_selection_expression: r.model_selection_expression.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<ApiGatewayV2StateView> for ApiGatewayV2State {
    fn from(view: ApiGatewayV2StateView) -> Self {
        ApiGatewayV2State {
            apis: view
                .apis
                .into_iter()
                .map(|(k, v)| (k, Api::from(v)))
                .collect(),
            stages: view
                .stages
                .into_iter()
                .map(|(k, v)| (k, Stage::from(v)))
                .collect(),
            integrations: view
                .integrations
                .into_iter()
                .map(|(k, v)| (k, Integration::from(v)))
                .collect(),
            routes: view
                .routes
                .into_iter()
                .map(|(k, v)| (k, Route::from(v)))
                .collect(),
            deployments: view
                .deployments
                .into_iter()
                .map(|(k, v)| (k, Deployment::from(v)))
                .collect(),
            authorizers: view
                .authorizers
                .into_iter()
                .map(|(k, v)| (k, Authorizer::from(v)))
                .collect(),
            models: view
                .models
                .into_iter()
                .map(|(k, v)| (k, Model::from(v)))
                .collect(),
            vpc_links: view
                .vpc_links
                .into_iter()
                .map(|(k, v)| (k, VpcLink::from(v)))
                .collect(),
            domain_names: view
                .domain_names
                .into_iter()
                .map(|(k, v)| (k, DomainName::from(v)))
                .collect(),
            api_mappings: view
                .api_mappings
                .into_iter()
                .map(|(k, v)| (k, ApiMapping::from(v)))
                .collect(),
            integration_responses: view
                .integration_responses
                .into_iter()
                .map(|(k, v)| (k, IntegrationResponse::from(v)))
                .collect(),
            route_responses: view
                .route_responses
                .into_iter()
                .map(|(k, v)| (k, RouteResponse::from(v)))
                .collect(),
            tags: view.tags,
        }
    }
}

impl From<ApiView> for Api {
    fn from(v: ApiView) -> Self {
        Api {
            api_id: v.api_id,
            name: v.name,
            protocol_type: v.protocol_type,
            route_selection_expression: v.route_selection_expression,
            description: v.description,
            api_endpoint: v.api_endpoint,
            created_date: v.created_date,
            tags: v.tags,
        }
    }
}

impl From<StageView> for Stage {
    fn from(v: StageView) -> Self {
        Stage {
            stage_name: v.stage_name,
            api_id: v.api_id,
            description: v.description,
            deployment_id: v.deployment_id,
            auto_deploy: v.auto_deploy,
            created_date: v.created_date,
            last_updated_date: v.last_updated_date,
            tags: v.tags,
        }
    }
}

impl From<IntegrationView> for Integration {
    fn from(v: IntegrationView) -> Self {
        Integration {
            integration_id: v.integration_id,
            api_id: v.api_id,
            integration_type: v.integration_type,
            integration_uri: v.integration_uri,
            integration_method: v.integration_method,
            description: v.description,
            payload_format_version: v.payload_format_version,
            connection_type: v.connection_type,
        }
    }
}

impl From<RouteView> for Route {
    fn from(v: RouteView) -> Self {
        Route {
            route_id: v.route_id,
            api_id: v.api_id,
            route_key: v.route_key,
            target: v.target,
            authorization_type: v.authorization_type,
            authorizer_id: v.authorizer_id,
        }
    }
}

impl From<DeploymentView> for Deployment {
    fn from(v: DeploymentView) -> Self {
        Deployment {
            deployment_id: v.deployment_id,
            api_id: v.api_id,
            deployment_status: v.deployment_status,
            description: v.description,
            created_date: v.created_date,
        }
    }
}

impl From<AuthorizerView> for Authorizer {
    fn from(v: AuthorizerView) -> Self {
        Authorizer {
            authorizer_id: v.authorizer_id,
            api_id: v.api_id,
            authorizer_type: v.authorizer_type,
            authorizer_uri: v.authorizer_uri,
            authorizer_credentials_arn: v.authorizer_credentials_arn,
            authorizer_payload_format_version: v.authorizer_payload_format_version,
            authorizer_result_ttl_in_seconds: v.authorizer_result_ttl_in_seconds,
            identity_source: v.identity_source,
            identity_validation_expression: v.identity_validation_expression,
            name: v.name,
            enable_simple_responses: v.enable_simple_responses,
        }
    }
}

impl From<ModelView> for Model {
    fn from(v: ModelView) -> Self {
        Model {
            model_id: v.model_id,
            api_id: v.api_id,
            name: v.name,
            content_type: v.content_type,
            description: v.description,
            schema: v.schema,
        }
    }
}

impl From<VpcLinkView> for VpcLink {
    fn from(v: VpcLinkView) -> Self {
        VpcLink {
            vpc_link_id: v.vpc_link_id,
            name: v.name,
            security_group_ids: v.security_group_ids,
            subnet_ids: v.subnet_ids,
            tags: v.tags,
            created_date: v.created_date,
        }
    }
}

impl From<StoredDomainNameConfigurationView> for StoredDomainNameConfiguration {
    fn from(v: StoredDomainNameConfigurationView) -> Self {
        StoredDomainNameConfiguration {
            certificate_arn: v.certificate_arn,
            endpoint_type: v.endpoint_type,
            security_policy: v.security_policy,
        }
    }
}

impl From<DomainNameView> for DomainName {
    fn from(v: DomainNameView) -> Self {
        DomainName {
            domain_name: v.domain_name,
            tags: v.tags,
            domain_name_configurations: v
                .domain_name_configurations
                .into_iter()
                .map(StoredDomainNameConfiguration::from)
                .collect(),
        }
    }
}

impl From<ApiMappingView> for ApiMapping {
    fn from(v: ApiMappingView) -> Self {
        ApiMapping {
            api_mapping_id: v.api_mapping_id,
            domain_name: v.domain_name,
            api_id: v.api_id,
            stage: v.stage,
            api_mapping_key: v.api_mapping_key,
        }
    }
}

impl From<IntegrationResponseView> for IntegrationResponse {
    fn from(v: IntegrationResponseView) -> Self {
        IntegrationResponse {
            integration_response_id: v.integration_response_id,
            api_id: v.api_id,
            integration_id: v.integration_id,
            integration_response_key: v.integration_response_key,
            content_handling_strategy: v.content_handling_strategy,
            response_parameters: v.response_parameters,
            response_templates: v.response_templates,
            template_selection_expression: v.template_selection_expression,
        }
    }
}

impl From<RouteResponseView> for RouteResponse {
    fn from(v: RouteResponseView) -> Self {
        RouteResponse {
            route_response_id: v.route_response_id,
            api_id: v.api_id,
            route_id: v.route_id,
            route_response_key: v.route_response_key,
            model_selection_expression: v.model_selection_expression,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for ApiGatewayV2Service {
    type StateView = ApiGatewayV2StateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ApiGatewayV2StateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = ApiGatewayV2State::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in view.apis {
                guard.apis.insert(k, Api::from(v));
            }
            for (k, v) in view.stages {
                guard.stages.insert(k, Stage::from(v));
            }
            for (k, v) in view.integrations {
                guard.integrations.insert(k, Integration::from(v));
            }
            for (k, v) in view.routes {
                guard.routes.insert(k, Route::from(v));
            }
            for (k, v) in view.deployments {
                guard.deployments.insert(k, Deployment::from(v));
            }
            for (k, v) in view.authorizers {
                guard.authorizers.insert(k, Authorizer::from(v));
            }
            for (k, v) in view.models {
                guard.models.insert(k, Model::from(v));
            }
            for (k, v) in view.vpc_links {
                guard.vpc_links.insert(k, VpcLink::from(v));
            }
            for (k, v) in view.domain_names {
                guard.domain_names.insert(k, DomainName::from(v));
            }
            for (k, v) in view.api_mappings {
                guard.api_mappings.insert(k, ApiMapping::from(v));
            }
            for (k, v) in view.integration_responses {
                guard
                    .integration_responses
                    .insert(k, IntegrationResponse::from(v));
            }
            for (k, v) in view.route_responses {
                guard.route_responses.insert(k, RouteResponse::from(v));
            }
            for (k, v) in view.tags {
                guard.tags.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
