use std::collections::HashMap;

use chrono::Utc;
use uuid::Uuid;

use crate::types::{
    Account, ApiGatewayModel, ApiKey, Authorizer, BasePathMapping, ClientCertificate, Deployment,
    DocumentationPart, DocumentationVersion, DomainNameAccessAssociation, EndpointConfiguration,
    GatewayResponse, Integration, IntegrationResponse, Method, MethodResponse, QuotaSettings,
    RequestValidator, Resource, RestApi, Stage, ThrottleSettings, UsagePlan, UsagePlanApiStage,
    UsagePlanKey, VpcLink,
};

#[derive(Debug, Default)]
pub struct ApiGatewayState {
    /// Rest APIs keyed by API id
    pub rest_apis: HashMap<String, RestApi>,
    /// Resources keyed by (api_id, resource_id)
    pub resources: HashMap<(String, String), Resource>,
    /// Stages keyed by (api_id, stage_name)
    pub stages: HashMap<(String, String), Stage>,
    /// Deployments keyed by (api_id, deployment_id)
    pub deployments: HashMap<(String, String), Deployment>,
    /// Authorizers keyed by (api_id, authorizer_id)
    pub authorizers: HashMap<(String, String), Authorizer>,
    /// Models keyed by (api_id, model_name)
    pub models: HashMap<(String, String), ApiGatewayModel>,
    /// API Keys keyed by api_key_id
    pub api_keys: HashMap<String, ApiKey>,
    /// Base Path Mappings keyed by (domain_name, base_path)
    pub base_path_mappings: HashMap<(String, String), BasePathMapping>,
    /// Domain Names keyed by domain_name
    pub domain_names: HashMap<String, DomainName>,
    /// Gateway Responses keyed by (api_id, response_type)
    pub gateway_responses: HashMap<(String, String), GatewayResponse>,
    /// Request Validators keyed by (api_id, validator_id)
    pub request_validators: HashMap<(String, String), RequestValidator>,
    /// Usage Plans keyed by usage_plan_id
    pub usage_plans: HashMap<String, UsagePlan>,
    /// Usage Plan Keys keyed by (usage_plan_id, key_id)
    pub usage_plan_keys: HashMap<(String, String), UsagePlanKey>,
    /// VPC Links keyed by vpc_link_id
    pub vpc_links: HashMap<String, VpcLink>,
    /// Account settings (single account)
    pub account: Account,
    /// Client certificates keyed by client_certificate_id
    pub client_certificates: HashMap<String, ClientCertificate>,
    /// Documentation parts keyed by (api_id, part_id)
    pub documentation_parts: HashMap<(String, String), DocumentationPart>,
    /// Documentation versions keyed by (api_id, version)
    pub documentation_versions: HashMap<(String, String), DocumentationVersion>,
    /// Tags keyed by resource ARN
    pub tags: HashMap<String, HashMap<String, String>>,
    /// Domain name access associations keyed by ARN
    pub domain_name_access_associations: HashMap<String, DomainNameAccessAssociation>,
}

/// Local alias to avoid confusion with model::DomainName
pub type DomainName = crate::types::DomainName;

#[derive(Debug, thiserror::Error)]
pub enum ApiGatewayError {
    #[error("Invalid REST API identifier specified")]
    RestApiNotFound,
    #[error("Invalid Resource identifier specified")]
    ResourceNotFound,
    #[error("Invalid Stage identifier specified")]
    StageNotFound,
    #[error("Invalid Deployment identifier specified")]
    DeploymentNotFound,
    #[error("Invalid deployment identifier specified")]
    DeploymentNotFoundLower,
    #[error("Invalid Method identifier specified")]
    MethodNotFound,
    #[error("Invalid MethodResponse identifier specified")]
    MethodResponseNotFound,
    #[error("Invalid Integration identifier specified")]
    IntegrationNotFound,
    #[error("Invalid IntegrationResponse identifier specified")]
    IntegrationResponseNotFound,
    #[error("Invalid Model identifier specified")]
    ModelNotFound,
    #[error("Invalid Authorizer identifier specified")]
    AuthorizerNotFound,
    #[error("Invalid API Key identifier specified")]
    ApiKeyNotFound,
    #[error("Invalid domain name identifier specified")]
    DomainNameNotFound,
    #[error("Invalid base path mapping identifier specified")]
    BasePathMappingNotFound,
    #[error("Invalid gateway response type specified")]
    GatewayResponseNotFound,
    #[error("Invalid request validator identifier specified")]
    RequestValidatorNotFound,
    #[error("Invalid usage plan identifier specified")]
    UsagePlanNotFound,
    #[error("Invalid usage plan key identifier specified")]
    UsagePlanKeyNotFound,
    #[error("Invalid VPC link identifier specified")]
    VpcLinkNotFound,
    #[error("Invalid client certificate identifier specified")]
    ClientCertificateNotFound,
    #[error("Invalid documentation part identifier specified")]
    DocumentationPartNotFound,
    #[error("Invalid documentation version identifier specified")]
    DocumentationVersionNotFound,
    #[error("Invalid domain name access association identifier specified")]
    DomainNameAccessAssociationNotFound,
}

impl ApiGatewayState {
    #[allow(clippy::too_many_arguments)]
    pub fn create_rest_api(
        &mut self,
        name: &str,
        description: Option<&str>,
        version: Option<&str>,
        endpoint_configuration: Option<EndpointConfiguration>,
        tags: HashMap<String, String>,
        disable_execute_api_endpoint: Option<bool>,
        policy: Option<&str>,
        api_key_source: Option<&str>,
        binary_media_types: Vec<String>,
        minimum_compression_size: Option<i32>,
    ) -> &RestApi {
        let api_id = Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(10)
            .collect::<String>();
        let root_id = Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(10)
            .collect::<String>();
        let created_date = Utc::now().timestamp() as f64;

        let api = RestApi {
            id: api_id.clone(),
            name: name.to_string(),
            description: description.map(str::to_string),
            version: version.map(str::to_string),
            created_date,
            endpoint_configuration,
            tags,
            disable_execute_api_endpoint,
            policy: policy.map(str::to_string),
            api_key_source: api_key_source.map(str::to_string),
            binary_media_types,
            minimum_compression_size,
            root_resource_id: root_id.clone(),
        };

        self.rest_apis.insert(api_id.clone(), api);

        // Create the root "/" resource automatically
        let root = Resource {
            id: root_id.clone(),
            parent_id: None,
            path_part: None,
            path: "/".to_string(),
            methods: HashMap::new(),
        };
        self.resources.insert((api_id.clone(), root_id), root);

        self.rest_apis.get(&api_id).unwrap()
    }

    pub fn get_rest_api(&self, api_id: &str) -> Result<&RestApi, ApiGatewayError> {
        self.rest_apis
            .get(api_id)
            .ok_or(ApiGatewayError::RestApiNotFound)
    }

    pub fn list_rest_apis(&self) -> Vec<&RestApi> {
        self.rest_apis.values().collect()
    }

    pub fn delete_rest_api(&mut self, api_id: &str) -> Result<(), ApiGatewayError> {
        if self.rest_apis.remove(api_id).is_none() {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        // Remove all associated state for this API
        self.resources.retain(|(rid, _), _| rid != api_id);
        self.stages.retain(|(rid, _), _| rid != api_id);
        self.deployments.retain(|(rid, _), _| rid != api_id);
        self.authorizers.retain(|(rid, _), _| rid != api_id);
        self.models.retain(|(rid, _), _| rid != api_id);
        Ok(())
    }

    pub fn create_resource(
        &mut self,
        api_id: &str,
        parent_id: &str,
        path_part: &str,
    ) -> Result<Resource, ApiGatewayError> {
        // Verify the API exists
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        // Find parent to compute path
        let parent_path = match self
            .resources
            .get(&(api_id.to_string(), parent_id.to_string()))
        {
            Some(p) => p.path.clone(),
            None => {
                return Err(ApiGatewayError::ResourceNotFound);
            }
        };

        let resource_id = Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(10)
            .collect::<String>();

        let path = if parent_path == "/" {
            format!("/{path_part}")
        } else {
            format!("{parent_path}/{path_part}")
        };

        let resource = Resource {
            id: resource_id.clone(),
            parent_id: Some(parent_id.to_string()),
            path_part: Some(path_part.to_string()),
            path,
            methods: HashMap::new(),
        };

        self.resources
            .insert((api_id.to_string(), resource_id), resource.clone());
        Ok(resource)
    }

    pub fn get_resource(
        &self,
        api_id: &str,
        resource_id: &str,
    ) -> Result<&Resource, ApiGatewayError> {
        self.resources
            .get(&(api_id.to_string(), resource_id.to_string()))
            .ok_or(ApiGatewayError::ResourceNotFound)
    }

    pub fn list_resources(&self, api_id: &str) -> Result<Vec<&Resource>, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        let resources: Vec<&Resource> = self
            .resources
            .iter()
            .filter(|((rid, _), _)| rid == api_id)
            .map(|(_, r)| r)
            .collect();
        Ok(resources)
    }

    pub fn delete_resource(
        &mut self,
        api_id: &str,
        resource_id: &str,
    ) -> Result<(), ApiGatewayError> {
        if self
            .resources
            .remove(&(api_id.to_string(), resource_id.to_string()))
            .is_none()
        {
            return Err(ApiGatewayError::ResourceNotFound);
        }
        Ok(())
    }

    // ---- Stage operations ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_stage(
        &mut self,
        api_id: &str,
        stage_name: &str,
        deployment_id: &str,
        description: Option<&str>,
        tracing_enabled: Option<bool>,
        tags: HashMap<String, String>,
        variables: HashMap<String, String>,
        cache_cluster_enabled: Option<bool>,
        cache_cluster_size: Option<String>,
        documentation_version: Option<String>,
    ) -> Result<&Stage, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        let now = Utc::now().timestamp() as f64;
        let stage = Stage {
            rest_api_id: api_id.to_string(),
            stage_name: stage_name.to_string(),
            deployment_id: deployment_id.to_string(),
            description: description.map(str::to_string),
            created_date: now,
            last_updated_date: now,
            tags,
            tracing_enabled,
            variables,
            cache_cluster_enabled,
            cache_cluster_size,
            documentation_version,
        };
        self.stages
            .insert((api_id.to_string(), stage_name.to_string()), stage);
        Ok(self
            .stages
            .get(&(api_id.to_string(), stage_name.to_string()))
            .unwrap())
    }

    pub fn get_stage(&self, api_id: &str, stage_name: &str) -> Result<&Stage, ApiGatewayError> {
        self.stages
            .get(&(api_id.to_string(), stage_name.to_string()))
            .ok_or(ApiGatewayError::StageNotFound)
    }

    pub fn list_stages(&self, api_id: &str) -> Result<Vec<&Stage>, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        Ok(self
            .stages
            .iter()
            .filter(|((rid, _), _)| rid == api_id)
            .map(|(_, s)| s)
            .collect())
    }

    pub fn delete_stage(&mut self, api_id: &str, stage_name: &str) -> Result<(), ApiGatewayError> {
        if self
            .stages
            .remove(&(api_id.to_string(), stage_name.to_string()))
            .is_none()
        {
            return Err(ApiGatewayError::StageNotFound);
        }
        Ok(())
    }

    pub fn update_stage(
        &mut self,
        api_id: &str,
        stage_name: &str,
        patch_ops: &serde_json::Value,
    ) -> Result<&Stage, ApiGatewayError> {
        let stage = self
            .stages
            .get_mut(&(api_id.to_string(), stage_name.to_string()))
            .ok_or(ApiGatewayError::StageNotFound)?;
        if let Some(ops) = patch_ops.as_array() {
            for op in ops {
                let op_name = op.get("op").and_then(|v| v.as_str()).unwrap_or("");
                let path = op.get("path").and_then(|v| v.as_str()).unwrap_or("");
                let value = op.get("value").and_then(|v| v.as_str()).unwrap_or("");
                if op_name == "replace" || op_name == "add" {
                    match path {
                        "/description" => stage.description = Some(value.to_string()),
                        "/deploymentId" => stage.deployment_id = value.to_string(),
                        "/tracingEnabled" => {
                            stage.tracing_enabled = Some(value == "true" || value == "True")
                        }
                        "/cacheClusterEnabled" => {
                            stage.cache_cluster_enabled = Some(value == "true" || value == "True")
                        }
                        "/cacheClusterSize" => stage.cache_cluster_size = Some(value.to_string()),
                        "/documentationVersion" => {
                            stage.documentation_version = Some(value.to_string())
                        }
                        p if p.starts_with("/variables/") => {
                            let key = p.trim_start_matches("/variables/");
                            stage.variables.insert(key.to_string(), value.to_string());
                        }
                        _ => {}
                    }
                } else if op_name == "remove" {
                    match path {
                        "/description" => stage.description = None,
                        "/tracingEnabled" => stage.tracing_enabled = None,
                        p if p.starts_with("/variables/") => {
                            let key = p.trim_start_matches("/variables/");
                            stage.variables.remove(key);
                        }
                        _ => {}
                    }
                }
            }
        }
        stage.last_updated_date = Utc::now().timestamp() as f64;
        Ok(self
            .stages
            .get(&(api_id.to_string(), stage_name.to_string()))
            .unwrap())
    }

    // ---- Deployment operations ----

    pub fn create_deployment(
        &mut self,
        api_id: &str,
        description: Option<&str>,
    ) -> Result<&Deployment, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        let deployment_id = Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(10)
            .collect::<String>();
        let deployment = Deployment {
            id: deployment_id.clone(),
            description: description.map(str::to_string),
            created_date: Utc::now().timestamp() as f64,
        };
        self.deployments
            .insert((api_id.to_string(), deployment_id.clone()), deployment);
        Ok(self
            .deployments
            .get(&(api_id.to_string(), deployment_id))
            .unwrap())
    }

    pub fn get_deployment(
        &self,
        api_id: &str,
        deployment_id: &str,
    ) -> Result<&Deployment, ApiGatewayError> {
        self.deployments
            .get(&(api_id.to_string(), deployment_id.to_string()))
            .ok_or(ApiGatewayError::DeploymentNotFound)
    }

    pub fn list_deployments(&self, api_id: &str) -> Result<Vec<&Deployment>, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        Ok(self
            .deployments
            .iter()
            .filter(|((rid, _), _)| rid == api_id)
            .map(|(_, d)| d)
            .collect())
    }

    pub fn delete_deployment(
        &mut self,
        api_id: &str,
        deployment_id: &str,
    ) -> Result<(), ApiGatewayError> {
        if self
            .deployments
            .remove(&(api_id.to_string(), deployment_id.to_string()))
            .is_none()
        {
            return Err(ApiGatewayError::DeploymentNotFound);
        }
        Ok(())
    }

    // ---- Method operations ----

    #[allow(clippy::too_many_arguments)]
    pub fn put_method(
        &mut self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
        authorization_type: &str,
        authorizer_id: Option<&str>,
        api_key_required: Option<bool>,
        operation_name: Option<&str>,
        request_models: HashMap<String, String>,
        request_parameters: HashMap<String, bool>,
        request_validator_id: Option<&str>,
    ) -> Result<&Method, ApiGatewayError> {
        let resource = self
            .resources
            .get_mut(&(api_id.to_string(), resource_id.to_string()))
            .ok_or(ApiGatewayError::ResourceNotFound)?;
        let method = Method {
            http_method: http_method.to_string(),
            authorization_type: authorization_type.to_string(),
            authorizer_id: authorizer_id.map(str::to_string),
            api_key_required,
            operation_name: operation_name.map(str::to_string),
            request_models,
            request_parameters,
            request_validator_id: request_validator_id.map(str::to_string),
            method_responses: HashMap::new(),
            integration: None,
        };
        resource.methods.insert(http_method.to_uppercase(), method);
        Ok(resource.methods.get(&http_method.to_uppercase()).unwrap())
    }

    pub fn get_method(
        &self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
    ) -> Result<&Method, ApiGatewayError> {
        let resource = self
            .resources
            .get(&(api_id.to_string(), resource_id.to_string()))
            .ok_or(ApiGatewayError::ResourceNotFound)?;
        resource
            .methods
            .get(&http_method.to_uppercase())
            .ok_or(ApiGatewayError::MethodNotFound)
    }

    pub fn delete_method(
        &mut self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
    ) -> Result<(), ApiGatewayError> {
        let resource = self
            .resources
            .get_mut(&(api_id.to_string(), resource_id.to_string()))
            .ok_or(ApiGatewayError::ResourceNotFound)?;
        if resource
            .methods
            .remove(&http_method.to_uppercase())
            .is_none()
        {
            return Err(ApiGatewayError::MethodNotFound);
        }
        Ok(())
    }

    // ---- MethodResponse operations ----

    pub fn put_method_response(
        &mut self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
        status_code: &str,
        response_models: HashMap<String, String>,
        response_parameters: HashMap<String, bool>,
    ) -> Result<&MethodResponse, ApiGatewayError> {
        let resource = self
            .resources
            .get_mut(&(api_id.to_string(), resource_id.to_string()))
            .ok_or(ApiGatewayError::ResourceNotFound)?;
        let method = resource
            .methods
            .get_mut(&http_method.to_uppercase())
            .ok_or(ApiGatewayError::MethodNotFound)?;
        let mr = MethodResponse {
            status_code: status_code.to_string(),
            response_models,
            response_parameters,
        };
        method.method_responses.insert(status_code.to_string(), mr);
        Ok(method.method_responses.get(status_code).unwrap())
    }

    pub fn get_method_response(
        &self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
        status_code: &str,
    ) -> Result<&MethodResponse, ApiGatewayError> {
        let method = self.get_method(api_id, resource_id, http_method)?;
        method
            .method_responses
            .get(status_code)
            .ok_or(ApiGatewayError::MethodResponseNotFound)
    }

    pub fn delete_method_response(
        &mut self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
        status_code: &str,
    ) -> Result<(), ApiGatewayError> {
        let resource = self
            .resources
            .get_mut(&(api_id.to_string(), resource_id.to_string()))
            .ok_or(ApiGatewayError::ResourceNotFound)?;
        let method = resource
            .methods
            .get_mut(&http_method.to_uppercase())
            .ok_or(ApiGatewayError::MethodNotFound)?;
        if method.method_responses.remove(status_code).is_none() {
            return Err(ApiGatewayError::MethodResponseNotFound);
        }
        Ok(())
    }

    // ---- Integration operations ----

    pub fn put_integration(
        &mut self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
        integration: Integration,
    ) -> Result<&Integration, ApiGatewayError> {
        let resource = self
            .resources
            .get_mut(&(api_id.to_string(), resource_id.to_string()))
            .ok_or(ApiGatewayError::ResourceNotFound)?;
        let method = resource
            .methods
            .get_mut(&http_method.to_uppercase())
            .ok_or(ApiGatewayError::MethodNotFound)?;
        method.integration = Some(integration);
        Ok(method.integration.as_ref().unwrap())
    }

    pub fn get_integration(
        &self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
    ) -> Result<&Integration, ApiGatewayError> {
        let method = self.get_method(api_id, resource_id, http_method)?;
        method
            .integration
            .as_ref()
            .ok_or(ApiGatewayError::IntegrationNotFound)
    }

    pub fn delete_integration(
        &mut self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
    ) -> Result<(), ApiGatewayError> {
        let resource = self
            .resources
            .get_mut(&(api_id.to_string(), resource_id.to_string()))
            .ok_or(ApiGatewayError::ResourceNotFound)?;
        let method = resource
            .methods
            .get_mut(&http_method.to_uppercase())
            .ok_or(ApiGatewayError::MethodNotFound)?;
        if method.integration.take().is_none() {
            return Err(ApiGatewayError::IntegrationNotFound);
        }
        Ok(())
    }

    // ---- IntegrationResponse operations ----

    pub fn put_integration_response(
        &mut self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
        status_code: &str,
        ir: IntegrationResponse,
    ) -> Result<&IntegrationResponse, ApiGatewayError> {
        let resource = self
            .resources
            .get_mut(&(api_id.to_string(), resource_id.to_string()))
            .ok_or(ApiGatewayError::ResourceNotFound)?;
        let method = resource
            .methods
            .get_mut(&http_method.to_uppercase())
            .ok_or(ApiGatewayError::MethodNotFound)?;
        let integration = method
            .integration
            .as_mut()
            .ok_or(ApiGatewayError::IntegrationNotFound)?;
        integration
            .integration_responses
            .insert(status_code.to_string(), ir);
        Ok(integration.integration_responses.get(status_code).unwrap())
    }

    pub fn get_integration_response(
        &self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
        status_code: &str,
    ) -> Result<&IntegrationResponse, ApiGatewayError> {
        let integration = self.get_integration(api_id, resource_id, http_method)?;
        integration
            .integration_responses
            .get(status_code)
            .ok_or(ApiGatewayError::IntegrationResponseNotFound)
    }

    pub fn delete_integration_response(
        &mut self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
        status_code: &str,
    ) -> Result<(), ApiGatewayError> {
        let resource = self
            .resources
            .get_mut(&(api_id.to_string(), resource_id.to_string()))
            .ok_or(ApiGatewayError::ResourceNotFound)?;
        let method = resource
            .methods
            .get_mut(&http_method.to_uppercase())
            .ok_or(ApiGatewayError::MethodNotFound)?;
        let integration = method
            .integration
            .as_mut()
            .ok_or(ApiGatewayError::IntegrationNotFound)?;
        if integration
            .integration_responses
            .remove(status_code)
            .is_none()
        {
            return Err(ApiGatewayError::IntegrationResponseNotFound);
        }
        Ok(())
    }

    // ---- Model operations ----

    pub fn create_model(
        &mut self,
        api_id: &str,
        name: &str,
        description: Option<&str>,
        schema: Option<&str>,
        content_type: Option<&str>,
    ) -> Result<&ApiGatewayModel, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        let model_id = Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(10)
            .collect::<String>();
        let model = ApiGatewayModel {
            id: model_id,
            name: name.to_string(),
            description: description.map(str::to_string),
            schema: schema.map(str::to_string),
            content_type: content_type.map(str::to_string),
        };
        self.models
            .insert((api_id.to_string(), name.to_string()), model);
        Ok(self
            .models
            .get(&(api_id.to_string(), name.to_string()))
            .unwrap())
    }

    pub fn get_model(
        &self,
        api_id: &str,
        model_name: &str,
    ) -> Result<&ApiGatewayModel, ApiGatewayError> {
        self.models
            .get(&(api_id.to_string(), model_name.to_string()))
            .ok_or(ApiGatewayError::ModelNotFound)
    }

    pub fn list_models(&self, api_id: &str) -> Result<Vec<&ApiGatewayModel>, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        Ok(self
            .models
            .iter()
            .filter(|((rid, _), _)| rid == api_id)
            .map(|(_, m)| m)
            .collect())
    }

    pub fn delete_model(&mut self, api_id: &str, model_name: &str) -> Result<(), ApiGatewayError> {
        if self
            .models
            .remove(&(api_id.to_string(), model_name.to_string()))
            .is_none()
        {
            return Err(ApiGatewayError::ModelNotFound);
        }
        Ok(())
    }

    // ---- Authorizer operations ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_authorizer(
        &mut self,
        api_id: &str,
        name: &str,
        authorizer_type: &str,
        authorizer_uri: Option<&str>,
        authorizer_credentials: Option<&str>,
        identity_source: Option<&str>,
        identity_validation_expression: Option<&str>,
        authorizer_result_ttl_in_seconds: Option<i32>,
        auth_type: Option<&str>,
        provider_arns: Vec<String>,
    ) -> Result<&Authorizer, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        let authorizer_id = Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(10)
            .collect::<String>();
        let authorizer = Authorizer {
            id: authorizer_id.clone(),
            name: name.to_string(),
            authorizer_type: authorizer_type.to_string(),
            authorizer_uri: authorizer_uri.map(str::to_string),
            authorizer_credentials: authorizer_credentials.map(str::to_string),
            identity_source: identity_source.map(str::to_string),
            identity_validation_expression: identity_validation_expression.map(str::to_string),
            authorizer_result_ttl_in_seconds,
            auth_type: auth_type.map(str::to_string),
            provider_arns,
        };
        self.authorizers
            .insert((api_id.to_string(), authorizer_id.clone()), authorizer);
        Ok(self
            .authorizers
            .get(&(api_id.to_string(), authorizer_id))
            .unwrap())
    }

    pub fn get_authorizer(
        &self,
        api_id: &str,
        authorizer_id: &str,
    ) -> Result<&Authorizer, ApiGatewayError> {
        self.authorizers
            .get(&(api_id.to_string(), authorizer_id.to_string()))
            .ok_or(ApiGatewayError::AuthorizerNotFound)
    }

    pub fn list_authorizers(&self, api_id: &str) -> Result<Vec<&Authorizer>, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        Ok(self
            .authorizers
            .iter()
            .filter(|((rid, _), _)| rid == api_id)
            .map(|(_, a)| a)
            .collect())
    }

    pub fn delete_authorizer(
        &mut self,
        api_id: &str,
        authorizer_id: &str,
    ) -> Result<(), ApiGatewayError> {
        if self
            .authorizers
            .remove(&(api_id.to_string(), authorizer_id.to_string()))
            .is_none()
        {
            return Err(ApiGatewayError::AuthorizerNotFound);
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_authorizer(
        &mut self,
        api_id: &str,
        authorizer_id: &str,
        name: Option<&str>,
        authorizer_type: Option<&str>,
        authorizer_uri: Option<&str>,
        authorizer_credentials: Option<&str>,
        identity_source: Option<&str>,
        identity_validation_expression: Option<&str>,
        authorizer_result_ttl_in_seconds: Option<i32>,
        auth_type: Option<&str>,
        provider_arns: Option<Vec<String>>,
    ) -> Result<&Authorizer, ApiGatewayError> {
        let a = self
            .authorizers
            .get_mut(&(api_id.to_string(), authorizer_id.to_string()))
            .ok_or(ApiGatewayError::AuthorizerNotFound)?;
        if let Some(v) = name {
            a.name = v.to_string();
        }
        if let Some(v) = authorizer_type {
            a.authorizer_type = v.to_string();
        }
        if let Some(v) = authorizer_uri {
            a.authorizer_uri = Some(v.to_string());
        }
        if let Some(v) = authorizer_credentials {
            a.authorizer_credentials = Some(v.to_string());
        }
        if let Some(v) = identity_source {
            a.identity_source = Some(v.to_string());
        }
        if let Some(v) = identity_validation_expression {
            a.identity_validation_expression = Some(v.to_string());
        }
        if let Some(v) = authorizer_result_ttl_in_seconds {
            a.authorizer_result_ttl_in_seconds = Some(v);
        }
        if let Some(v) = auth_type {
            a.auth_type = Some(v.to_string());
        }
        if let Some(v) = provider_arns {
            a.provider_arns = v;
        }
        Ok(self
            .authorizers
            .get(&(api_id.to_string(), authorizer_id.to_string()))
            .unwrap())
    }

    // ---- API Key operations ----

    pub fn create_api_key(
        &mut self,
        name: &str,
        description: Option<&str>,
        enabled: bool,
        stage_keys: Vec<String>,
        tags: HashMap<String, String>,
    ) -> &ApiKey {
        let key_id = Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(10)
            .collect::<String>();
        let value = Uuid::new_v4().to_string().replace('-', "");
        let created_date = Utc::now().timestamp() as f64;
        let key = ApiKey {
            id: key_id.clone(),
            name: name.to_string(),
            value,
            description: description.map(str::to_string),
            enabled,
            stage_keys,
            tags,
            created_date,
        };
        self.api_keys.insert(key_id.clone(), key);
        self.api_keys.get(&key_id).unwrap()
    }

    pub fn get_api_key(&self, key_id: &str) -> Result<&ApiKey, ApiGatewayError> {
        self.api_keys
            .get(key_id)
            .ok_or(ApiGatewayError::ApiKeyNotFound)
    }

    pub fn list_api_keys(&self) -> Vec<&ApiKey> {
        self.api_keys.values().collect()
    }

    pub fn delete_api_key(&mut self, key_id: &str) -> Result<(), ApiGatewayError> {
        if self.api_keys.remove(key_id).is_none() {
            return Err(ApiGatewayError::ApiKeyNotFound);
        }
        Ok(())
    }

    pub fn update_api_key(
        &mut self,
        key_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        enabled: Option<bool>,
    ) -> Result<&ApiKey, ApiGatewayError> {
        let k = self
            .api_keys
            .get_mut(key_id)
            .ok_or(ApiGatewayError::ApiKeyNotFound)?;
        if let Some(v) = name {
            k.name = v.to_string();
        }
        if let Some(v) = description {
            k.description = Some(v.to_string());
        }
        if let Some(v) = enabled {
            k.enabled = v;
        }
        Ok(self.api_keys.get(key_id).unwrap())
    }

    // ---- Base Path Mapping operations ----

    pub fn create_base_path_mapping(
        &mut self,
        domain_name: &str,
        base_path: &str,
        rest_api_id: &str,
        stage: Option<&str>,
    ) -> Result<&BasePathMapping, ApiGatewayError> {
        if !self.domain_names.contains_key(domain_name) {
            return Err(ApiGatewayError::DomainNameNotFound);
        }
        let mapping = BasePathMapping {
            base_path: base_path.to_string(),
            rest_api_id: rest_api_id.to_string(),
            stage: stage.map(str::to_string),
        };
        self.base_path_mappings
            .insert((domain_name.to_string(), base_path.to_string()), mapping);
        Ok(self
            .base_path_mappings
            .get(&(domain_name.to_string(), base_path.to_string()))
            .unwrap())
    }

    pub fn get_base_path_mapping(
        &self,
        domain_name: &str,
        base_path: &str,
    ) -> Result<&BasePathMapping, ApiGatewayError> {
        self.base_path_mappings
            .get(&(domain_name.to_string(), base_path.to_string()))
            .ok_or(ApiGatewayError::BasePathMappingNotFound)
    }

    pub fn list_base_path_mappings(&self, domain_name: &str) -> Vec<&BasePathMapping> {
        self.base_path_mappings
            .iter()
            .filter(|((dn, _), _)| dn == domain_name)
            .map(|(_, m)| m)
            .collect()
    }

    pub fn delete_base_path_mapping(
        &mut self,
        domain_name: &str,
        base_path: &str,
    ) -> Result<(), ApiGatewayError> {
        if self
            .base_path_mappings
            .remove(&(domain_name.to_string(), base_path.to_string()))
            .is_none()
        {
            return Err(ApiGatewayError::BasePathMappingNotFound);
        }
        Ok(())
    }

    pub fn update_base_path_mapping(
        &mut self,
        domain_name: &str,
        base_path: &str,
        rest_api_id: Option<&str>,
        stage: Option<&str>,
        new_base_path: Option<&str>,
    ) -> Result<BasePathMapping, ApiGatewayError> {
        let mut mapping = self
            .base_path_mappings
            .remove(&(domain_name.to_string(), base_path.to_string()))
            .ok_or(ApiGatewayError::BasePathMappingNotFound)?;
        if let Some(v) = rest_api_id {
            mapping.rest_api_id = v.to_string();
        }
        if let Some(v) = stage {
            mapping.stage = Some(v.to_string());
        }
        let effective_base_path = new_base_path.unwrap_or(base_path).to_string();
        mapping.base_path = effective_base_path.clone();
        self.base_path_mappings.insert(
            (domain_name.to_string(), effective_base_path),
            mapping.clone(),
        );
        Ok(mapping)
    }

    // ---- Domain Name operations ----

    pub fn create_domain_name(
        &mut self,
        domain_name: &str,
        certificate_name: Option<&str>,
        distribution_domain_name: Option<&str>,
        tags: HashMap<String, String>,
    ) -> &DomainName {
        let dn = DomainName {
            domain_name: domain_name.to_string(),
            certificate_name: certificate_name.map(str::to_string),
            distribution_domain_name: distribution_domain_name.map(str::to_string),
            tags,
        };
        self.domain_names.insert(domain_name.to_string(), dn);
        self.domain_names.get(domain_name).unwrap()
    }

    pub fn get_domain_name(&self, domain_name: &str) -> Result<&DomainName, ApiGatewayError> {
        self.domain_names
            .get(domain_name)
            .ok_or(ApiGatewayError::DomainNameNotFound)
    }

    pub fn list_domain_names(&self) -> Vec<&DomainName> {
        self.domain_names.values().collect()
    }

    pub fn delete_domain_name(&mut self, domain_name: &str) -> Result<(), ApiGatewayError> {
        if self.domain_names.remove(domain_name).is_none() {
            return Err(ApiGatewayError::DomainNameNotFound);
        }
        Ok(())
    }

    // ---- Gateway Response operations ----

    pub fn put_gateway_response(
        &mut self,
        api_id: &str,
        response_type: &str,
        status_code: Option<&str>,
        response_parameters: HashMap<String, String>,
        response_templates: HashMap<String, String>,
    ) -> Result<&GatewayResponse, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        let gr = GatewayResponse {
            response_type: response_type.to_string(),
            status_code: status_code.map(str::to_string),
            response_parameters,
            response_templates,
        };
        self.gateway_responses
            .insert((api_id.to_string(), response_type.to_string()), gr);
        Ok(self
            .gateway_responses
            .get(&(api_id.to_string(), response_type.to_string()))
            .unwrap())
    }

    pub fn get_gateway_response(
        &self,
        api_id: &str,
        response_type: &str,
    ) -> Result<&GatewayResponse, ApiGatewayError> {
        self.gateway_responses
            .get(&(api_id.to_string(), response_type.to_string()))
            .ok_or(ApiGatewayError::GatewayResponseNotFound)
    }

    pub fn list_gateway_responses(
        &self,
        api_id: &str,
    ) -> Result<Vec<&GatewayResponse>, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        Ok(self
            .gateway_responses
            .iter()
            .filter(|((rid, _), _)| rid == api_id)
            .map(|(_, gr)| gr)
            .collect())
    }

    pub fn delete_gateway_response(
        &mut self,
        api_id: &str,
        response_type: &str,
    ) -> Result<(), ApiGatewayError> {
        if self
            .gateway_responses
            .remove(&(api_id.to_string(), response_type.to_string()))
            .is_none()
        {
            return Err(ApiGatewayError::GatewayResponseNotFound);
        }
        Ok(())
    }

    // ---- Request Validator operations ----

    pub fn create_request_validator(
        &mut self,
        api_id: &str,
        name: &str,
        validate_request_body: bool,
        validate_request_parameters: bool,
    ) -> Result<&RequestValidator, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        let validator_id = Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(10)
            .collect::<String>();
        let validator = RequestValidator {
            id: validator_id.clone(),
            name: name.to_string(),
            validate_request_body,
            validate_request_parameters,
        };
        self.request_validators
            .insert((api_id.to_string(), validator_id.clone()), validator);
        Ok(self
            .request_validators
            .get(&(api_id.to_string(), validator_id))
            .unwrap())
    }

    pub fn get_request_validator(
        &self,
        api_id: &str,
        validator_id: &str,
    ) -> Result<&RequestValidator, ApiGatewayError> {
        self.request_validators
            .get(&(api_id.to_string(), validator_id.to_string()))
            .ok_or(ApiGatewayError::RequestValidatorNotFound)
    }

    pub fn list_request_validators(
        &self,
        api_id: &str,
    ) -> Result<Vec<&RequestValidator>, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        Ok(self
            .request_validators
            .iter()
            .filter(|((rid, _), _)| rid == api_id)
            .map(|(_, v)| v)
            .collect())
    }

    pub fn delete_request_validator(
        &mut self,
        api_id: &str,
        validator_id: &str,
    ) -> Result<(), ApiGatewayError> {
        if self
            .request_validators
            .remove(&(api_id.to_string(), validator_id.to_string()))
            .is_none()
        {
            return Err(ApiGatewayError::RequestValidatorNotFound);
        }
        Ok(())
    }

    pub fn update_request_validator(
        &mut self,
        api_id: &str,
        validator_id: &str,
        name: Option<&str>,
        validate_request_body: Option<bool>,
        validate_request_parameters: Option<bool>,
    ) -> Result<&RequestValidator, ApiGatewayError> {
        let v = self
            .request_validators
            .get_mut(&(api_id.to_string(), validator_id.to_string()))
            .ok_or(ApiGatewayError::RequestValidatorNotFound)?;
        if let Some(n) = name {
            v.name = n.to_string();
        }
        if let Some(b) = validate_request_body {
            v.validate_request_body = b;
        }
        if let Some(b) = validate_request_parameters {
            v.validate_request_parameters = b;
        }
        Ok(self
            .request_validators
            .get(&(api_id.to_string(), validator_id.to_string()))
            .unwrap())
    }

    // ---- Usage Plan operations ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_usage_plan(
        &mut self,
        name: &str,
        description: Option<&str>,
        api_stages: Vec<UsagePlanApiStage>,
        throttle: Option<ThrottleSettings>,
        quota: Option<QuotaSettings>,
        product_code: Option<&str>,
        tags: HashMap<String, String>,
    ) -> &UsagePlan {
        let plan_id = Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(10)
            .collect::<String>();
        let plan = UsagePlan {
            id: plan_id.clone(),
            name: name.to_string(),
            description: description.map(str::to_string),
            api_stages,
            throttle,
            quota,
            product_code: product_code.map(str::to_string),
            tags,
        };
        self.usage_plans.insert(plan_id.clone(), plan);
        self.usage_plans.get(&plan_id).unwrap()
    }

    pub fn get_usage_plan(&self, plan_id: &str) -> Result<&UsagePlan, ApiGatewayError> {
        self.usage_plans
            .get(plan_id)
            .ok_or(ApiGatewayError::UsagePlanNotFound)
    }

    pub fn list_usage_plans(&self) -> Vec<&UsagePlan> {
        self.usage_plans.values().collect()
    }

    pub fn delete_usage_plan(&mut self, plan_id: &str) -> Result<(), ApiGatewayError> {
        if self.usage_plans.remove(plan_id).is_none() {
            return Err(ApiGatewayError::UsagePlanNotFound);
        }
        Ok(())
    }

    pub fn update_usage_plan(
        &mut self,
        plan_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        product_code: Option<&str>,
    ) -> Result<&UsagePlan, ApiGatewayError> {
        let p = self
            .usage_plans
            .get_mut(plan_id)
            .ok_or(ApiGatewayError::UsagePlanNotFound)?;
        if let Some(v) = name {
            p.name = v.to_string();
        }
        if let Some(v) = description {
            p.description = Some(v.to_string());
        }
        if let Some(v) = product_code {
            p.product_code = Some(v.to_string());
        }
        Ok(self.usage_plans.get(plan_id).unwrap())
    }

    // ---- Usage Plan Key operations ----

    pub fn create_usage_plan_key(
        &mut self,
        plan_id: &str,
        key_id: &str,
        key_type: &str,
    ) -> Result<&UsagePlanKey, ApiGatewayError> {
        if !self.usage_plans.contains_key(plan_id) {
            return Err(ApiGatewayError::UsagePlanNotFound);
        }
        // Look up the API key to get name/value
        let (name, value) = if let Some(ak) = self.api_keys.get(key_id) {
            (Some(ak.name.clone()), Some(ak.value.clone()))
        } else {
            (None, None)
        };
        let upk = UsagePlanKey {
            id: key_id.to_string(),
            key_type: key_type.to_string(),
            name,
            value,
        };
        self.usage_plan_keys
            .insert((plan_id.to_string(), key_id.to_string()), upk);
        Ok(self
            .usage_plan_keys
            .get(&(plan_id.to_string(), key_id.to_string()))
            .unwrap())
    }

    pub fn get_usage_plan_key(
        &self,
        plan_id: &str,
        key_id: &str,
    ) -> Result<&UsagePlanKey, ApiGatewayError> {
        self.usage_plan_keys
            .get(&(plan_id.to_string(), key_id.to_string()))
            .ok_or(ApiGatewayError::UsagePlanKeyNotFound)
    }

    pub fn list_usage_plan_keys(
        &self,
        plan_id: &str,
    ) -> Result<Vec<&UsagePlanKey>, ApiGatewayError> {
        if !self.usage_plans.contains_key(plan_id) {
            return Err(ApiGatewayError::UsagePlanNotFound);
        }
        Ok(self
            .usage_plan_keys
            .iter()
            .filter(|((pid, _), _)| pid == plan_id)
            .map(|(_, k)| k)
            .collect())
    }

    pub fn delete_usage_plan_key(
        &mut self,
        plan_id: &str,
        key_id: &str,
    ) -> Result<(), ApiGatewayError> {
        if self
            .usage_plan_keys
            .remove(&(plan_id.to_string(), key_id.to_string()))
            .is_none()
        {
            return Err(ApiGatewayError::UsagePlanKeyNotFound);
        }
        Ok(())
    }

    // ---- VPC Link operations ----

    pub fn create_vpc_link(
        &mut self,
        name: &str,
        description: Option<&str>,
        target_arns: Vec<String>,
        tags: HashMap<String, String>,
    ) -> &VpcLink {
        let link_id = Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(10)
            .collect::<String>();
        let link = VpcLink {
            id: link_id.clone(),
            name: name.to_string(),
            description: description.map(str::to_string),
            target_arns,
            status: "AVAILABLE".to_string(),
            status_message: None,
            tags,
        };
        self.vpc_links.insert(link_id.clone(), link);
        self.vpc_links.get(&link_id).unwrap()
    }

    pub fn get_vpc_link(&self, link_id: &str) -> Result<&VpcLink, ApiGatewayError> {
        self.vpc_links
            .get(link_id)
            .ok_or(ApiGatewayError::VpcLinkNotFound)
    }

    pub fn list_vpc_links(&self) -> Vec<&VpcLink> {
        self.vpc_links.values().collect()
    }

    pub fn delete_vpc_link(&mut self, link_id: &str) -> Result<(), ApiGatewayError> {
        if self.vpc_links.remove(link_id).is_none() {
            return Err(ApiGatewayError::VpcLinkNotFound);
        }
        Ok(())
    }

    // ---- Account operations ----

    pub fn get_account(&self) -> &Account {
        &self.account
    }

    pub fn update_account(&mut self, cloudwatch_role_arn: Option<&str>) -> &Account {
        if let Some(v) = cloudwatch_role_arn {
            self.account.cloudwatch_role_arn = Some(v.to_string());
        }
        &self.account
    }

    // ---- Client Certificate operations ----

    pub fn generate_client_certificate(
        &mut self,
        description: Option<&str>,
        tags: HashMap<String, String>,
    ) -> &ClientCertificate {
        let cert_id = Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(10)
            .collect::<String>();
        let now = Utc::now().timestamp() as f64;
        let cert = ClientCertificate {
            client_certificate_id: cert_id.clone(),
            description: description.map(str::to_string),
            pem_encoded_certificate: format!(
                "-----BEGIN CERTIFICATE-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA{}\n-----END CERTIFICATE-----",
                cert_id
            ),
            created_date: now,
            expiration_date: now + 365.0 * 24.0 * 3600.0,
            tags,
        };
        self.client_certificates.insert(cert_id.clone(), cert);
        self.client_certificates.get(&cert_id).unwrap()
    }

    pub fn get_client_certificate(
        &self,
        cert_id: &str,
    ) -> Result<&ClientCertificate, ApiGatewayError> {
        self.client_certificates
            .get(cert_id)
            .ok_or(ApiGatewayError::ClientCertificateNotFound)
    }

    pub fn list_client_certificates(&self) -> Vec<&ClientCertificate> {
        self.client_certificates.values().collect()
    }

    pub fn delete_client_certificate(&mut self, cert_id: &str) -> Result<(), ApiGatewayError> {
        if self.client_certificates.remove(cert_id).is_none() {
            return Err(ApiGatewayError::ClientCertificateNotFound);
        }
        Ok(())
    }

    pub fn update_client_certificate(
        &mut self,
        cert_id: &str,
        description: Option<&str>,
    ) -> Result<&ClientCertificate, ApiGatewayError> {
        let cert = self
            .client_certificates
            .get_mut(cert_id)
            .ok_or(ApiGatewayError::ClientCertificateNotFound)?;
        if let Some(d) = description {
            cert.description = Some(d.to_string());
        }
        Ok(self.client_certificates.get(cert_id).unwrap())
    }

    // ---- Documentation Part operations ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_documentation_part(
        &mut self,
        api_id: &str,
        location_type: &str,
        location_path: Option<&str>,
        location_method: Option<&str>,
        location_status_code: Option<&str>,
        location_name: Option<&str>,
        properties: &str,
    ) -> Result<&DocumentationPart, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        let part_id = Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(10)
            .collect::<String>();
        let part = DocumentationPart {
            id: part_id.clone(),
            location_type: location_type.to_string(),
            location_path: location_path.map(str::to_string),
            location_method: location_method.map(str::to_string),
            location_status_code: location_status_code.map(str::to_string),
            location_name: location_name.map(str::to_string),
            properties: properties.to_string(),
        };
        self.documentation_parts
            .insert((api_id.to_string(), part_id.clone()), part);
        Ok(self
            .documentation_parts
            .get(&(api_id.to_string(), part_id))
            .unwrap())
    }

    pub fn get_documentation_part(
        &self,
        api_id: &str,
        part_id: &str,
    ) -> Result<&DocumentationPart, ApiGatewayError> {
        self.documentation_parts
            .get(&(api_id.to_string(), part_id.to_string()))
            .ok_or(ApiGatewayError::DocumentationPartNotFound)
    }

    pub fn list_documentation_parts(
        &self,
        api_id: &str,
    ) -> Result<Vec<&DocumentationPart>, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        Ok(self
            .documentation_parts
            .iter()
            .filter(|((rid, _), _)| rid == api_id)
            .map(|(_, p)| p)
            .collect())
    }

    pub fn delete_documentation_part(
        &mut self,
        api_id: &str,
        part_id: &str,
    ) -> Result<(), ApiGatewayError> {
        if self
            .documentation_parts
            .remove(&(api_id.to_string(), part_id.to_string()))
            .is_none()
        {
            return Err(ApiGatewayError::DocumentationPartNotFound);
        }
        Ok(())
    }

    pub fn update_documentation_part(
        &mut self,
        api_id: &str,
        part_id: &str,
        properties: Option<&str>,
    ) -> Result<&DocumentationPart, ApiGatewayError> {
        let part = self
            .documentation_parts
            .get_mut(&(api_id.to_string(), part_id.to_string()))
            .ok_or(ApiGatewayError::DocumentationPartNotFound)?;
        if let Some(p) = properties {
            part.properties = p.to_string();
        }
        Ok(self
            .documentation_parts
            .get(&(api_id.to_string(), part_id.to_string()))
            .unwrap())
    }

    // ---- Documentation Version operations ----

    pub fn create_documentation_version(
        &mut self,
        api_id: &str,
        version: &str,
        description: Option<&str>,
    ) -> Result<&DocumentationVersion, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        let now = Utc::now().timestamp() as f64;
        let dv = DocumentationVersion {
            version: version.to_string(),
            created_date: now,
            description: description.map(str::to_string),
        };
        self.documentation_versions
            .insert((api_id.to_string(), version.to_string()), dv);
        Ok(self
            .documentation_versions
            .get(&(api_id.to_string(), version.to_string()))
            .unwrap())
    }

    pub fn get_documentation_version(
        &self,
        api_id: &str,
        version: &str,
    ) -> Result<&DocumentationVersion, ApiGatewayError> {
        self.documentation_versions
            .get(&(api_id.to_string(), version.to_string()))
            .ok_or(ApiGatewayError::DocumentationVersionNotFound)
    }

    pub fn list_documentation_versions(
        &self,
        api_id: &str,
    ) -> Result<Vec<&DocumentationVersion>, ApiGatewayError> {
        if !self.rest_apis.contains_key(api_id) {
            return Err(ApiGatewayError::RestApiNotFound);
        }
        Ok(self
            .documentation_versions
            .iter()
            .filter(|((rid, _), _)| rid == api_id)
            .map(|(_, v)| v)
            .collect())
    }

    pub fn delete_documentation_version(
        &mut self,
        api_id: &str,
        version: &str,
    ) -> Result<(), ApiGatewayError> {
        if self
            .documentation_versions
            .remove(&(api_id.to_string(), version.to_string()))
            .is_none()
        {
            return Err(ApiGatewayError::DocumentationVersionNotFound);
        }
        Ok(())
    }

    pub fn update_documentation_version(
        &mut self,
        api_id: &str,
        version: &str,
        description: Option<&str>,
    ) -> Result<&DocumentationVersion, ApiGatewayError> {
        let dv = self
            .documentation_versions
            .get_mut(&(api_id.to_string(), version.to_string()))
            .ok_or(ApiGatewayError::DocumentationVersionNotFound)?;
        if let Some(d) = description {
            dv.description = Some(d.to_string());
        }
        Ok(self
            .documentation_versions
            .get(&(api_id.to_string(), version.to_string()))
            .unwrap())
    }

    // ---- Tag operations ----

    pub fn get_tags(&self, resource_arn: &str) -> HashMap<String, String> {
        self.tags.get(resource_arn).cloned().unwrap_or_default()
    }

    pub fn tag_resource(&mut self, resource_arn: &str, tags: HashMap<String, String>) {
        let entry = self.tags.entry(resource_arn.to_string()).or_default();
        entry.extend(tags);
    }

    pub fn untag_resource(&mut self, resource_arn: &str, tag_keys: &[&str]) {
        if let Some(tags) = self.tags.get_mut(resource_arn) {
            for key in tag_keys {
                tags.remove(*key);
            }
        }
    }

    // ---- VPC Link update ----

    pub fn update_vpc_link(
        &mut self,
        link_id: &str,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<&VpcLink, ApiGatewayError> {
        let l = self
            .vpc_links
            .get_mut(link_id)
            .ok_or(ApiGatewayError::VpcLinkNotFound)?;
        if let Some(v) = name {
            l.name = v.to_string();
        }
        if let Some(v) = description {
            l.description = Some(v.to_string());
        }
        Ok(self.vpc_links.get(link_id).unwrap())
    }

    // ---- Deployment update ----

    pub fn update_deployment(
        &mut self,
        api_id: &str,
        deployment_id: &str,
        description: Option<&str>,
    ) -> Result<&Deployment, ApiGatewayError> {
        let d = self
            .deployments
            .get_mut(&(api_id.to_string(), deployment_id.to_string()))
            .ok_or(ApiGatewayError::DeploymentNotFoundLower)?;
        if let Some(v) = description {
            d.description = Some(v.to_string());
        }
        Ok(self
            .deployments
            .get(&(api_id.to_string(), deployment_id.to_string()))
            .unwrap())
    }

    // ---- Domain Name update ----

    pub fn update_domain_name(
        &mut self,
        domain_name: &str,
        certificate_name: Option<&str>,
    ) -> Result<&crate::types::DomainName, ApiGatewayError> {
        let dn = self
            .domain_names
            .get_mut(domain_name)
            .ok_or(ApiGatewayError::DomainNameNotFound)?;
        if let Some(v) = certificate_name {
            dn.certificate_name = Some(v.to_string());
        }
        Ok(self.domain_names.get(domain_name).unwrap())
    }

    // ---- Gateway Response update ----

    pub fn update_gateway_response(
        &mut self,
        api_id: &str,
        response_type: &str,
        status_code: Option<&str>,
        response_parameters: HashMap<String, String>,
        response_templates: HashMap<String, String>,
    ) -> Result<&GatewayResponse, ApiGatewayError> {
        let gr = self
            .gateway_responses
            .get_mut(&(api_id.to_string(), response_type.to_string()))
            .ok_or(ApiGatewayError::GatewayResponseNotFound)?;
        if let Some(v) = status_code {
            gr.status_code = Some(v.to_string());
        }
        if !response_parameters.is_empty() {
            gr.response_parameters.extend(response_parameters);
        }
        if !response_templates.is_empty() {
            gr.response_templates.extend(response_templates);
        }
        Ok(self
            .gateway_responses
            .get(&(api_id.to_string(), response_type.to_string()))
            .unwrap())
    }

    // ---- Method update ----

    #[allow(clippy::too_many_arguments)]
    pub fn update_method(
        &mut self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
        authorization_type: Option<&str>,
        authorizer_id: Option<&str>,
        api_key_required: Option<bool>,
        operation_name: Option<&str>,
    ) -> Result<&Method, ApiGatewayError> {
        let resource = self
            .resources
            .get_mut(&(api_id.to_string(), resource_id.to_string()))
            .ok_or(ApiGatewayError::ResourceNotFound)?;
        let method = resource
            .methods
            .get_mut(&http_method.to_uppercase())
            .ok_or(ApiGatewayError::MethodNotFound)?;
        if let Some(v) = authorization_type {
            method.authorization_type = v.to_string();
        }
        if let Some(v) = authorizer_id {
            method.authorizer_id = Some(v.to_string());
        }
        if let Some(v) = api_key_required {
            method.api_key_required = Some(v);
        }
        if let Some(v) = operation_name {
            method.operation_name = Some(v.to_string());
        }
        let m = resource.methods.get(&http_method.to_uppercase()).unwrap();
        Ok(m)
    }

    // ---- Resource update ----

    pub fn update_resource(
        &mut self,
        api_id: &str,
        resource_id: &str,
        path_part: Option<&str>,
    ) -> Result<&Resource, ApiGatewayError> {
        let resource = self
            .resources
            .get_mut(&(api_id.to_string(), resource_id.to_string()))
            .ok_or(ApiGatewayError::ResourceNotFound)?;
        if let Some(v) = path_part {
            resource.path_part = Some(v.to_string());
        }
        Ok(self
            .resources
            .get(&(api_id.to_string(), resource_id.to_string()))
            .unwrap())
    }

    // ---- Integration update ----

    pub fn update_integration(
        &mut self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
        uri: Option<&str>,
        passthrough_behavior: Option<&str>,
        timeout_in_millis: Option<i32>,
    ) -> Result<&Integration, ApiGatewayError> {
        let resource = self
            .resources
            .get_mut(&(api_id.to_string(), resource_id.to_string()))
            .ok_or(ApiGatewayError::ResourceNotFound)?;
        let method = resource
            .methods
            .get_mut(&http_method.to_uppercase())
            .ok_or(ApiGatewayError::MethodNotFound)?;
        let integration = method
            .integration
            .as_mut()
            .ok_or(ApiGatewayError::IntegrationNotFound)?;
        if let Some(v) = uri {
            integration.uri = Some(v.to_string());
        }
        if let Some(v) = passthrough_behavior {
            integration.passthrough_behavior = Some(v.to_string());
        }
        if let Some(v) = timeout_in_millis {
            integration.timeout_in_millis = Some(v);
        }
        Ok(method.integration.as_ref().unwrap())
    }

    // ---- IntegrationResponse update ----

    pub fn update_integration_response(
        &mut self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
        status_code: &str,
        selection_pattern: Option<&str>,
        content_handling: Option<&str>,
    ) -> Result<&IntegrationResponse, ApiGatewayError> {
        let resource = self
            .resources
            .get_mut(&(api_id.to_string(), resource_id.to_string()))
            .ok_or(ApiGatewayError::ResourceNotFound)?;
        let method = resource
            .methods
            .get_mut(&http_method.to_uppercase())
            .ok_or(ApiGatewayError::MethodNotFound)?;
        let integration = method
            .integration
            .as_mut()
            .ok_or(ApiGatewayError::IntegrationNotFound)?;
        let ir = integration
            .integration_responses
            .get_mut(status_code)
            .ok_or(ApiGatewayError::IntegrationResponseNotFound)?;
        if let Some(v) = selection_pattern {
            ir.selection_pattern = Some(v.to_string());
        }
        if let Some(v) = content_handling {
            ir.content_handling = Some(v.to_string());
        }
        Ok(method
            .integration
            .as_ref()
            .unwrap()
            .integration_responses
            .get(status_code)
            .unwrap())
    }

    // ---- MethodResponse update ----

    pub fn update_method_response(
        &mut self,
        api_id: &str,
        resource_id: &str,
        http_method: &str,
        status_code: &str,
    ) -> Result<&MethodResponse, ApiGatewayError> {
        let method = self.get_method(api_id, resource_id, http_method)?;
        method
            .method_responses
            .get(status_code)
            .ok_or(ApiGatewayError::MethodResponseNotFound)
    }

    // ---- Model update ----

    pub fn update_model(
        &mut self,
        api_id: &str,
        model_name: &str,
        description: Option<&str>,
        schema: Option<&str>,
    ) -> Result<&ApiGatewayModel, ApiGatewayError> {
        let m = self
            .models
            .get_mut(&(api_id.to_string(), model_name.to_string()))
            .ok_or(ApiGatewayError::ModelNotFound)?;
        if let Some(v) = description {
            m.description = Some(v.to_string());
        }
        if let Some(v) = schema {
            m.schema = Some(v.to_string());
        }
        Ok(self
            .models
            .get(&(api_id.to_string(), model_name.to_string()))
            .unwrap())
    }

    // ---- Domain Name Access Association operations ----

    pub fn create_domain_name_access_association(
        &mut self,
        domain_name_arn: &str,
        access_association_source: &str,
        access_association_source_type: &str,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> &DomainNameAccessAssociation {
        let arn = format!(
            "arn:aws:apigateway:{region}::/domainnames/{domain_name_arn}/accessassociations/{access_association_source}"
        );
        let assoc = DomainNameAccessAssociation {
            arn: arn.clone(),
            domain_name_arn: domain_name_arn.to_string(),
            access_association_source: access_association_source.to_string(),
            access_association_source_type: access_association_source_type.to_string(),
            tags,
        };
        let _ = account_id;
        let _ = region;
        self.domain_name_access_associations
            .insert(arn.clone(), assoc);
        self.domain_name_access_associations.get(&arn).unwrap()
    }

    pub fn list_domain_name_access_associations(&self) -> Vec<&DomainNameAccessAssociation> {
        self.domain_name_access_associations.values().collect()
    }

    pub fn delete_domain_name_access_association(
        &mut self,
        arn: &str,
    ) -> Result<(), ApiGatewayError> {
        if self.domain_name_access_associations.remove(arn).is_none() {
            return Err(ApiGatewayError::DomainNameAccessAssociationNotFound);
        }
        Ok(())
    }

    // ---- RestApi update ----

    pub fn update_rest_api(
        &mut self,
        api_id: &str,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<&RestApi, ApiGatewayError> {
        let api = self
            .rest_apis
            .get_mut(api_id)
            .ok_or(ApiGatewayError::RestApiNotFound)?;
        if let Some(v) = name {
            api.name = v.to_string();
        }
        if let Some(v) = description {
            api.description = Some(v.to_string());
        }
        Ok(self.rest_apis.get(api_id).unwrap())
    }
}
