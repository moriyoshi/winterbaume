use std::collections::HashMap;

use crate::types::{
    Api, ApiMapping, Authorizer, Deployment, DomainName, Integration, IntegrationResponse, Model,
    Route, RouteResponse, Stage, VpcLink,
};

#[derive(Debug, Default)]
pub struct ApiGatewayV2State {
    pub apis: HashMap<String, Api>,
    pub stages: HashMap<String, Stage>, // key: "{api_id}/{stage_name}"
    pub integrations: HashMap<String, Integration>, // key: "{api_id}/{integration_id}"
    pub routes: HashMap<String, Route>, // key: "{api_id}/{route_id}"
    pub deployments: HashMap<String, Deployment>, // key: "{api_id}/{deployment_id}"
    pub authorizers: HashMap<String, Authorizer>, // key: "{api_id}/{authorizer_id}"
    pub models: HashMap<String, Model>, // key: "{api_id}/{model_id}"
    pub vpc_links: HashMap<String, VpcLink>, // key: vpc_link_id
    pub domain_names: HashMap<String, DomainName>, // key: domain_name
    pub api_mappings: HashMap<String, ApiMapping>, // key: "{domain_name}/{mapping_id}"
    pub integration_responses: HashMap<String, IntegrationResponse>, // key: "{api_id}/{integration_id}/{response_id}"
    pub route_responses: HashMap<String, RouteResponse>, // key: "{api_id}/{route_id}/{response_id}"
    pub tags: HashMap<String, HashMap<String, String>>,  // key: resource_arn
}

#[derive(Debug, thiserror::Error)]
pub enum ApiGatewayV2Error {
    #[error("Name is required")]
    NameRequired,
    #[error("ProtocolType is required")]
    ProtocolTypeRequired,
    #[error("API not found: {0}")]
    ApiNotFound(String),
    #[error("Stage not found: {0}")]
    StageNotFound(String),
    #[error("Integration not found: {0}")]
    IntegrationNotFound(String),
    #[error("Route not found: {0}")]
    RouteNotFound(String),
    #[error("Deployment not found: {0}")]
    DeploymentNotFound(String),
    #[error("Authorizer not found: {0}")]
    AuthorizerNotFound(String),
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    #[error("VPC Link not found: {0}")]
    VpcLinkNotFound(String),
    #[error("Domain name not found: {0}")]
    DomainNameNotFound(String),
    #[error("API mapping not found: {0}")]
    ApiMappingNotFound(String),
    #[error("Integration response not found: {0}")]
    IntegrationResponseNotFound(String),
    #[error("Route response not found: {0}")]
    RouteResponseNotFound(String),
}

impl ApiGatewayV2State {
    // -----------------------------------------------------------------------
    // APIs
    // -----------------------------------------------------------------------

    pub fn create_api(
        &mut self,
        name: &str,
        protocol_type: &str,
        route_selection_expression: Option<&str>,
        description: Option<&str>,
        tags: HashMap<String, String>,
    ) -> Result<&Api, ApiGatewayV2Error> {
        if name.is_empty() {
            return Err(ApiGatewayV2Error::NameRequired);
        }
        if protocol_type.is_empty() {
            return Err(ApiGatewayV2Error::ProtocolTypeRequired);
        }
        let api_id = uuid::Uuid::new_v4().to_string();
        let api = Api {
            api_id: api_id.clone(),
            name: name.to_string(),
            protocol_type: protocol_type.to_string(),
            route_selection_expression: route_selection_expression.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
            api_endpoint: format!("https://{api_id}.execute-api.us-east-1.amazonaws.com"),
            created_date: chrono_now_rfc3339(),
            tags,
        };
        self.apis.insert(api_id.clone(), api);
        Ok(self.apis.get(&api_id).unwrap())
    }

    pub fn get_api(&self, api_id: &str) -> Result<&Api, ApiGatewayV2Error> {
        self.apis
            .get(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))
    }

    pub fn list_apis(&self) -> Vec<&Api> {
        self.apis.values().collect()
    }

    pub fn delete_api(&mut self, api_id: &str) -> Result<(), ApiGatewayV2Error> {
        if self.apis.remove(api_id).is_none() {
            return Err(ApiGatewayV2Error::ApiNotFound(api_id.to_string()));
        }
        // Cascade: remove all associated resources
        let prefix = format!("{api_id}/");
        self.stages.retain(|k, _| !k.starts_with(&prefix));
        self.integrations.retain(|k, _| !k.starts_with(&prefix));
        self.routes.retain(|k, _| !k.starts_with(&prefix));
        self.deployments.retain(|k, _| !k.starts_with(&prefix));
        self.authorizers.retain(|k, _| !k.starts_with(&prefix));
        self.models.retain(|k, _| !k.starts_with(&prefix));
        self.integration_responses
            .retain(|k, _| !k.starts_with(&prefix));
        self.route_responses.retain(|k, _| !k.starts_with(&prefix));
        Ok(())
    }

    pub fn update_api(
        &mut self,
        api_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        route_selection_expression: Option<&str>,
    ) -> Result<&Api, ApiGatewayV2Error> {
        let api = self
            .apis
            .get_mut(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))?;
        if let Some(n) = name {
            api.name = n.to_string();
        }
        if let Some(d) = description {
            api.description = Some(d.to_string());
        }
        if let Some(r) = route_selection_expression {
            api.route_selection_expression = Some(r.to_string());
        }
        Ok(self.apis.get(api_id).unwrap())
    }

    // -----------------------------------------------------------------------
    // Stages
    // -----------------------------------------------------------------------

    pub fn create_stage(
        &mut self,
        api_id: &str,
        stage_name: &str,
        description: Option<&str>,
        deployment_id: Option<&str>,
        auto_deploy: bool,
        tags: HashMap<String, String>,
    ) -> Result<&Stage, ApiGatewayV2Error> {
        self.apis
            .get(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))?;
        let key = format!("{api_id}/{stage_name}");
        let now = chrono_now_rfc3339();
        let stage = Stage {
            stage_name: stage_name.to_string(),
            api_id: api_id.to_string(),
            description: description.map(|s| s.to_string()),
            deployment_id: deployment_id.map(|s| s.to_string()),
            auto_deploy,
            created_date: now.clone(),
            last_updated_date: now,
            tags,
        };
        self.stages.insert(key.clone(), stage);
        Ok(self.stages.get(&key).unwrap())
    }

    pub fn get_stage(&self, api_id: &str, stage_name: &str) -> Result<&Stage, ApiGatewayV2Error> {
        let key = format!("{api_id}/{stage_name}");
        self.stages
            .get(&key)
            .ok_or_else(|| ApiGatewayV2Error::StageNotFound(stage_name.to_string()))
    }

    pub fn list_stages(&self, api_id: &str) -> Result<Vec<&Stage>, ApiGatewayV2Error> {
        self.apis
            .get(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))?;
        let prefix = format!("{api_id}/");
        Ok(self
            .stages
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(_, v)| v)
            .collect())
    }

    pub fn delete_stage(
        &mut self,
        api_id: &str,
        stage_name: &str,
    ) -> Result<(), ApiGatewayV2Error> {
        let key = format!("{api_id}/{stage_name}");
        if self.stages.remove(&key).is_none() {
            return Err(ApiGatewayV2Error::StageNotFound(stage_name.to_string()));
        }
        Ok(())
    }

    pub fn update_stage(
        &mut self,
        api_id: &str,
        stage_name: &str,
        description: Option<&str>,
        deployment_id: Option<&str>,
        auto_deploy: Option<bool>,
    ) -> Result<&Stage, ApiGatewayV2Error> {
        let key = format!("{api_id}/{stage_name}");
        let stage = self
            .stages
            .get_mut(&key)
            .ok_or_else(|| ApiGatewayV2Error::StageNotFound(stage_name.to_string()))?;
        if let Some(d) = description {
            stage.description = Some(d.to_string());
        }
        if let Some(dep) = deployment_id {
            stage.deployment_id = Some(dep.to_string());
        }
        if let Some(ad) = auto_deploy {
            stage.auto_deploy = ad;
        }
        stage.last_updated_date = chrono_now_rfc3339();
        Ok(self.stages.get(&key).unwrap())
    }

    // -----------------------------------------------------------------------
    // Integrations
    // -----------------------------------------------------------------------

    pub fn create_integration(
        &mut self,
        api_id: &str,
        integration_type: &str,
        integration_uri: Option<&str>,
        integration_method: Option<&str>,
        description: Option<&str>,
        payload_format_version: Option<&str>,
        connection_type: Option<&str>,
    ) -> Result<&Integration, ApiGatewayV2Error> {
        self.apis
            .get(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))?;
        let integration_id = uuid::Uuid::new_v4().to_string();
        let integration = Integration {
            integration_id: integration_id.clone(),
            api_id: api_id.to_string(),
            integration_type: integration_type.to_string(),
            integration_uri: integration_uri.map(|s| s.to_string()),
            integration_method: integration_method.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
            payload_format_version: payload_format_version.map(|s| s.to_string()),
            connection_type: connection_type.map(|s| s.to_string()),
        };
        let key = format!("{api_id}/{integration_id}");
        self.integrations.insert(key.clone(), integration);
        Ok(self.integrations.get(&key).unwrap())
    }

    pub fn get_integration(
        &self,
        api_id: &str,
        integration_id: &str,
    ) -> Result<&Integration, ApiGatewayV2Error> {
        let key = format!("{api_id}/{integration_id}");
        self.integrations
            .get(&key)
            .ok_or_else(|| ApiGatewayV2Error::IntegrationNotFound(integration_id.to_string()))
    }

    pub fn list_integrations(&self, api_id: &str) -> Result<Vec<&Integration>, ApiGatewayV2Error> {
        self.apis
            .get(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))?;
        let prefix = format!("{api_id}/");
        Ok(self
            .integrations
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(_, v)| v)
            .collect())
    }

    pub fn delete_integration(
        &mut self,
        api_id: &str,
        integration_id: &str,
    ) -> Result<(), ApiGatewayV2Error> {
        let key = format!("{api_id}/{integration_id}");
        if self.integrations.remove(&key).is_none() {
            return Err(ApiGatewayV2Error::IntegrationNotFound(
                integration_id.to_string(),
            ));
        }
        Ok(())
    }

    pub fn update_integration(
        &mut self,
        api_id: &str,
        integration_id: &str,
        integration_type: Option<&str>,
        integration_uri: Option<&str>,
        integration_method: Option<&str>,
        description: Option<&str>,
        payload_format_version: Option<&str>,
        connection_type: Option<&str>,
    ) -> Result<&Integration, ApiGatewayV2Error> {
        let key = format!("{api_id}/{integration_id}");
        let integration = self
            .integrations
            .get_mut(&key)
            .ok_or_else(|| ApiGatewayV2Error::IntegrationNotFound(integration_id.to_string()))?;
        if let Some(t) = integration_type {
            integration.integration_type = t.to_string();
        }
        if let Some(u) = integration_uri {
            integration.integration_uri = Some(u.to_string());
        }
        if let Some(m) = integration_method {
            integration.integration_method = Some(m.to_string());
        }
        if let Some(d) = description {
            integration.description = Some(d.to_string());
        }
        if let Some(p) = payload_format_version {
            integration.payload_format_version = Some(p.to_string());
        }
        if let Some(c) = connection_type {
            integration.connection_type = Some(c.to_string());
        }
        Ok(self.integrations.get(&key).unwrap())
    }

    // -----------------------------------------------------------------------
    // Routes
    // -----------------------------------------------------------------------

    pub fn create_route(
        &mut self,
        api_id: &str,
        route_key: &str,
        target: Option<&str>,
        authorization_type: Option<&str>,
        authorizer_id: Option<&str>,
    ) -> Result<&Route, ApiGatewayV2Error> {
        self.apis
            .get(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))?;
        let route_id = uuid::Uuid::new_v4().to_string();
        let route = Route {
            route_id: route_id.clone(),
            api_id: api_id.to_string(),
            route_key: route_key.to_string(),
            target: target.map(|s| s.to_string()),
            authorization_type: authorization_type.map(|s| s.to_string()),
            authorizer_id: authorizer_id.map(|s| s.to_string()),
        };
        let key = format!("{api_id}/{route_id}");
        self.routes.insert(key.clone(), route);
        Ok(self.routes.get(&key).unwrap())
    }

    pub fn get_route(&self, api_id: &str, route_id: &str) -> Result<&Route, ApiGatewayV2Error> {
        let key = format!("{api_id}/{route_id}");
        self.routes
            .get(&key)
            .ok_or_else(|| ApiGatewayV2Error::RouteNotFound(route_id.to_string()))
    }

    pub fn list_routes(&self, api_id: &str) -> Result<Vec<&Route>, ApiGatewayV2Error> {
        self.apis
            .get(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))?;
        let prefix = format!("{api_id}/");
        Ok(self
            .routes
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(_, v)| v)
            .collect())
    }

    pub fn delete_route(&mut self, api_id: &str, route_id: &str) -> Result<(), ApiGatewayV2Error> {
        let key = format!("{api_id}/{route_id}");
        if self.routes.remove(&key).is_none() {
            return Err(ApiGatewayV2Error::RouteNotFound(route_id.to_string()));
        }
        Ok(())
    }

    pub fn update_route(
        &mut self,
        api_id: &str,
        route_id: &str,
        route_key: Option<&str>,
        target: Option<&str>,
        authorization_type: Option<&str>,
        authorizer_id: Option<&str>,
    ) -> Result<&Route, ApiGatewayV2Error> {
        let key = format!("{api_id}/{route_id}");
        let route = self
            .routes
            .get_mut(&key)
            .ok_or_else(|| ApiGatewayV2Error::RouteNotFound(route_id.to_string()))?;
        if let Some(rk) = route_key {
            route.route_key = rk.to_string();
        }
        if let Some(t) = target {
            route.target = Some(t.to_string());
        }
        if let Some(at) = authorization_type {
            route.authorization_type = Some(at.to_string());
        }
        if let Some(aid) = authorizer_id {
            route.authorizer_id = Some(aid.to_string());
        }
        Ok(self.routes.get(&key).unwrap())
    }

    // -----------------------------------------------------------------------
    // Deployments
    // -----------------------------------------------------------------------

    pub fn create_deployment(
        &mut self,
        api_id: &str,
        description: Option<&str>,
    ) -> Result<&Deployment, ApiGatewayV2Error> {
        self.apis
            .get(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))?;
        let deployment_id = uuid::Uuid::new_v4().to_string();
        let deployment = Deployment {
            deployment_id: deployment_id.clone(),
            api_id: api_id.to_string(),
            deployment_status: "DEPLOYED".to_string(),
            description: description.map(|s| s.to_string()),
            created_date: chrono_now_rfc3339(),
        };
        let key = format!("{api_id}/{deployment_id}");
        self.deployments.insert(key.clone(), deployment);
        Ok(self.deployments.get(&key).unwrap())
    }

    pub fn get_deployment(
        &self,
        api_id: &str,
        deployment_id: &str,
    ) -> Result<&Deployment, ApiGatewayV2Error> {
        let key = format!("{api_id}/{deployment_id}");
        self.deployments
            .get(&key)
            .ok_or_else(|| ApiGatewayV2Error::DeploymentNotFound(deployment_id.to_string()))
    }

    pub fn list_deployments(&self, api_id: &str) -> Result<Vec<&Deployment>, ApiGatewayV2Error> {
        self.apis
            .get(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))?;
        let prefix = format!("{api_id}/");
        Ok(self
            .deployments
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(_, v)| v)
            .collect())
    }

    pub fn delete_deployment(
        &mut self,
        api_id: &str,
        deployment_id: &str,
    ) -> Result<(), ApiGatewayV2Error> {
        let key = format!("{api_id}/{deployment_id}");
        if self.deployments.remove(&key).is_none() {
            return Err(ApiGatewayV2Error::DeploymentNotFound(
                deployment_id.to_string(),
            ));
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Authorizers
    // -----------------------------------------------------------------------

    pub fn create_authorizer(
        &mut self,
        api_id: &str,
        name: &str,
        authorizer_type: &str,
        authorizer_uri: Option<&str>,
        authorizer_credentials_arn: Option<&str>,
        authorizer_payload_format_version: Option<&str>,
        authorizer_result_ttl_in_seconds: Option<i32>,
        identity_source: Option<Vec<String>>,
        identity_validation_expression: Option<&str>,
        enable_simple_responses: Option<bool>,
    ) -> Result<&Authorizer, ApiGatewayV2Error> {
        self.apis
            .get(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))?;
        let authorizer_id = uuid::Uuid::new_v4().to_string();
        let authorizer = Authorizer {
            authorizer_id: authorizer_id.clone(),
            api_id: api_id.to_string(),
            name: name.to_string(),
            authorizer_type: authorizer_type.to_string(),
            authorizer_uri: authorizer_uri.map(|s| s.to_string()),
            authorizer_credentials_arn: authorizer_credentials_arn.map(|s| s.to_string()),
            authorizer_payload_format_version: authorizer_payload_format_version
                .map(|s| s.to_string()),
            authorizer_result_ttl_in_seconds,
            identity_source,
            identity_validation_expression: identity_validation_expression.map(|s| s.to_string()),
            enable_simple_responses,
        };
        let key = format!("{api_id}/{authorizer_id}");
        self.authorizers.insert(key.clone(), authorizer);
        Ok(self.authorizers.get(&key).unwrap())
    }

    pub fn get_authorizer(
        &self,
        api_id: &str,
        authorizer_id: &str,
    ) -> Result<&Authorizer, ApiGatewayV2Error> {
        let key = format!("{api_id}/{authorizer_id}");
        self.authorizers
            .get(&key)
            .ok_or_else(|| ApiGatewayV2Error::AuthorizerNotFound(authorizer_id.to_string()))
    }

    pub fn list_authorizers(&self, api_id: &str) -> Result<Vec<&Authorizer>, ApiGatewayV2Error> {
        self.apis
            .get(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))?;
        let prefix = format!("{api_id}/");
        Ok(self
            .authorizers
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(_, v)| v)
            .collect())
    }

    pub fn delete_authorizer(
        &mut self,
        api_id: &str,
        authorizer_id: &str,
    ) -> Result<(), ApiGatewayV2Error> {
        let key = format!("{api_id}/{authorizer_id}");
        if self.authorizers.remove(&key).is_none() {
            return Err(ApiGatewayV2Error::AuthorizerNotFound(
                authorizer_id.to_string(),
            ));
        }
        Ok(())
    }

    pub fn update_authorizer(
        &mut self,
        api_id: &str,
        authorizer_id: &str,
        name: Option<&str>,
        authorizer_type: Option<&str>,
        authorizer_uri: Option<&str>,
        authorizer_credentials_arn: Option<&str>,
        authorizer_payload_format_version: Option<&str>,
        authorizer_result_ttl_in_seconds: Option<i32>,
        identity_source: Option<Vec<String>>,
        identity_validation_expression: Option<&str>,
        enable_simple_responses: Option<bool>,
    ) -> Result<&Authorizer, ApiGatewayV2Error> {
        let key = format!("{api_id}/{authorizer_id}");
        let auth = self
            .authorizers
            .get_mut(&key)
            .ok_or_else(|| ApiGatewayV2Error::AuthorizerNotFound(authorizer_id.to_string()))?;
        if let Some(n) = name {
            auth.name = n.to_string();
        }
        if let Some(t) = authorizer_type {
            auth.authorizer_type = t.to_string();
        }
        if let Some(u) = authorizer_uri {
            auth.authorizer_uri = Some(u.to_string());
        }
        if let Some(c) = authorizer_credentials_arn {
            auth.authorizer_credentials_arn = Some(c.to_string());
        }
        if let Some(p) = authorizer_payload_format_version {
            auth.authorizer_payload_format_version = Some(p.to_string());
        }
        if let Some(ttl) = authorizer_result_ttl_in_seconds {
            auth.authorizer_result_ttl_in_seconds = Some(ttl);
        }
        if let Some(is) = identity_source {
            auth.identity_source = Some(is);
        }
        if let Some(ive) = identity_validation_expression {
            auth.identity_validation_expression = Some(ive.to_string());
        }
        if let Some(esr) = enable_simple_responses {
            auth.enable_simple_responses = Some(esr);
        }
        Ok(self.authorizers.get(&key).unwrap())
    }

    // -----------------------------------------------------------------------
    // Models
    // -----------------------------------------------------------------------

    pub fn create_model(
        &mut self,
        api_id: &str,
        name: &str,
        content_type: Option<&str>,
        description: Option<&str>,
        schema: Option<&str>,
    ) -> Result<&Model, ApiGatewayV2Error> {
        self.apis
            .get(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))?;
        let model_id = uuid::Uuid::new_v4().to_string();
        let model = Model {
            model_id: model_id.clone(),
            api_id: api_id.to_string(),
            name: name.to_string(),
            content_type: content_type.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
            schema: schema.map(|s| s.to_string()),
        };
        let key = format!("{api_id}/{model_id}");
        self.models.insert(key.clone(), model);
        Ok(self.models.get(&key).unwrap())
    }

    pub fn get_model(&self, api_id: &str, model_id: &str) -> Result<&Model, ApiGatewayV2Error> {
        let key = format!("{api_id}/{model_id}");
        self.models
            .get(&key)
            .ok_or_else(|| ApiGatewayV2Error::ModelNotFound(model_id.to_string()))
    }

    pub fn list_models(&self, api_id: &str) -> Result<Vec<&Model>, ApiGatewayV2Error> {
        self.apis
            .get(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))?;
        let prefix = format!("{api_id}/");
        Ok(self
            .models
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(_, v)| v)
            .collect())
    }

    pub fn delete_model(&mut self, api_id: &str, model_id: &str) -> Result<(), ApiGatewayV2Error> {
        let key = format!("{api_id}/{model_id}");
        if self.models.remove(&key).is_none() {
            return Err(ApiGatewayV2Error::ModelNotFound(model_id.to_string()));
        }
        Ok(())
    }

    pub fn update_model(
        &mut self,
        api_id: &str,
        model_id: &str,
        name: Option<&str>,
        content_type: Option<&str>,
        description: Option<&str>,
        schema: Option<&str>,
    ) -> Result<&Model, ApiGatewayV2Error> {
        let key = format!("{api_id}/{model_id}");
        let model = self
            .models
            .get_mut(&key)
            .ok_or_else(|| ApiGatewayV2Error::ModelNotFound(model_id.to_string()))?;
        if let Some(n) = name {
            model.name = n.to_string();
        }
        if let Some(ct) = content_type {
            model.content_type = Some(ct.to_string());
        }
        if let Some(d) = description {
            model.description = Some(d.to_string());
        }
        if let Some(s) = schema {
            model.schema = Some(s.to_string());
        }
        Ok(self.models.get(&key).unwrap())
    }

    // -----------------------------------------------------------------------
    // VPC Links
    // -----------------------------------------------------------------------

    pub fn create_vpc_link(
        &mut self,
        name: &str,
        security_group_ids: Vec<String>,
        subnet_ids: Vec<String>,
        tags: HashMap<String, String>,
    ) -> &VpcLink {
        let vpc_link_id = uuid::Uuid::new_v4().to_string();
        let vpc_link = VpcLink {
            vpc_link_id: vpc_link_id.clone(),
            name: name.to_string(),
            security_group_ids,
            subnet_ids,
            tags,
            created_date: chrono_now_rfc3339(),
        };
        self.vpc_links.insert(vpc_link_id.clone(), vpc_link);
        self.vpc_links.get(&vpc_link_id).unwrap()
    }

    pub fn get_vpc_link(&self, vpc_link_id: &str) -> Result<&VpcLink, ApiGatewayV2Error> {
        self.vpc_links
            .get(vpc_link_id)
            .ok_or_else(|| ApiGatewayV2Error::VpcLinkNotFound(vpc_link_id.to_string()))
    }

    pub fn list_vpc_links(&self) -> Vec<&VpcLink> {
        self.vpc_links.values().collect()
    }

    pub fn delete_vpc_link(&mut self, vpc_link_id: &str) -> Result<(), ApiGatewayV2Error> {
        if self.vpc_links.remove(vpc_link_id).is_none() {
            return Err(ApiGatewayV2Error::VpcLinkNotFound(vpc_link_id.to_string()));
        }
        Ok(())
    }

    pub fn update_vpc_link(
        &mut self,
        vpc_link_id: &str,
        name: Option<&str>,
    ) -> Result<&VpcLink, ApiGatewayV2Error> {
        let link = self
            .vpc_links
            .get_mut(vpc_link_id)
            .ok_or_else(|| ApiGatewayV2Error::VpcLinkNotFound(vpc_link_id.to_string()))?;
        if let Some(n) = name {
            link.name = n.to_string();
        }
        Ok(self.vpc_links.get(vpc_link_id).unwrap())
    }

    // -----------------------------------------------------------------------
    // Domain Names
    // -----------------------------------------------------------------------

    pub fn create_domain_name(
        &mut self,
        domain_name: &str,
        tags: HashMap<String, String>,
        domain_name_configurations: Vec<crate::types::StoredDomainNameConfiguration>,
    ) -> &DomainName {
        let dn = DomainName {
            domain_name: domain_name.to_string(),
            tags,
            domain_name_configurations,
        };
        self.domain_names.insert(domain_name.to_string(), dn);
        self.domain_names.get(domain_name).unwrap()
    }

    pub fn get_domain_name(&self, domain_name: &str) -> Result<&DomainName, ApiGatewayV2Error> {
        self.domain_names
            .get(domain_name)
            .ok_or_else(|| ApiGatewayV2Error::DomainNameNotFound(domain_name.to_string()))
    }

    pub fn list_domain_names(&self) -> Vec<&DomainName> {
        self.domain_names.values().collect()
    }

    pub fn delete_domain_name(&mut self, domain_name: &str) -> Result<(), ApiGatewayV2Error> {
        if self.domain_names.remove(domain_name).is_none() {
            return Err(ApiGatewayV2Error::DomainNameNotFound(
                domain_name.to_string(),
            ));
        }
        // Cascade: remove all api mappings for this domain
        let prefix = format!("{domain_name}/");
        self.api_mappings.retain(|k, _| !k.starts_with(&prefix));
        Ok(())
    }

    // -----------------------------------------------------------------------
    // API Mappings
    // -----------------------------------------------------------------------

    pub fn create_api_mapping(
        &mut self,
        domain_name: &str,
        api_id: &str,
        stage: &str,
        api_mapping_key: Option<&str>,
    ) -> Result<&ApiMapping, ApiGatewayV2Error> {
        self.domain_names
            .get(domain_name)
            .ok_or_else(|| ApiGatewayV2Error::DomainNameNotFound(domain_name.to_string()))?;
        self.apis
            .get(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))?;
        let api_mapping_id = uuid::Uuid::new_v4().to_string();
        let mapping = ApiMapping {
            api_mapping_id: api_mapping_id.clone(),
            domain_name: domain_name.to_string(),
            api_id: api_id.to_string(),
            stage: stage.to_string(),
            api_mapping_key: api_mapping_key.map(|s| s.to_string()),
        };
        let key = format!("{domain_name}/{api_mapping_id}");
        self.api_mappings.insert(key.clone(), mapping);
        Ok(self.api_mappings.get(&key).unwrap())
    }

    pub fn get_api_mapping(
        &self,
        domain_name: &str,
        api_mapping_id: &str,
    ) -> Result<&ApiMapping, ApiGatewayV2Error> {
        let key = format!("{domain_name}/{api_mapping_id}");
        self.api_mappings
            .get(&key)
            .ok_or_else(|| ApiGatewayV2Error::ApiMappingNotFound(api_mapping_id.to_string()))
    }

    pub fn list_api_mappings(
        &self,
        domain_name: &str,
    ) -> Result<Vec<&ApiMapping>, ApiGatewayV2Error> {
        self.domain_names
            .get(domain_name)
            .ok_or_else(|| ApiGatewayV2Error::DomainNameNotFound(domain_name.to_string()))?;
        let prefix = format!("{domain_name}/");
        Ok(self
            .api_mappings
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(_, v)| v)
            .collect())
    }

    pub fn delete_api_mapping(
        &mut self,
        domain_name: &str,
        api_mapping_id: &str,
    ) -> Result<(), ApiGatewayV2Error> {
        let key = format!("{domain_name}/{api_mapping_id}");
        if self.api_mappings.remove(&key).is_none() {
            return Err(ApiGatewayV2Error::ApiMappingNotFound(
                api_mapping_id.to_string(),
            ));
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Integration Responses
    // -----------------------------------------------------------------------

    pub fn create_integration_response(
        &mut self,
        api_id: &str,
        integration_id: &str,
        integration_response_key: &str,
        content_handling_strategy: Option<&str>,
        response_parameters: Option<HashMap<String, String>>,
        response_templates: Option<HashMap<String, String>>,
        template_selection_expression: Option<&str>,
    ) -> Result<&IntegrationResponse, ApiGatewayV2Error> {
        // Validate integration exists
        let int_key = format!("{api_id}/{integration_id}");
        if !self.integrations.contains_key(&int_key) {
            return Err(ApiGatewayV2Error::IntegrationNotFound(
                integration_id.to_string(),
            ));
        }
        let response_id = uuid::Uuid::new_v4().to_string();
        let resp = IntegrationResponse {
            integration_response_id: response_id.clone(),
            api_id: api_id.to_string(),
            integration_id: integration_id.to_string(),
            integration_response_key: integration_response_key.to_string(),
            content_handling_strategy: content_handling_strategy.map(|s| s.to_string()),
            response_parameters,
            response_templates,
            template_selection_expression: template_selection_expression.map(|s| s.to_string()),
        };
        let key = format!("{api_id}/{integration_id}/{response_id}");
        self.integration_responses.insert(key.clone(), resp);
        Ok(self.integration_responses.get(&key).unwrap())
    }

    pub fn get_integration_response(
        &self,
        api_id: &str,
        integration_id: &str,
        response_id: &str,
    ) -> Result<&IntegrationResponse, ApiGatewayV2Error> {
        let key = format!("{api_id}/{integration_id}/{response_id}");
        self.integration_responses
            .get(&key)
            .ok_or_else(|| ApiGatewayV2Error::IntegrationResponseNotFound(response_id.to_string()))
    }

    pub fn list_integration_responses(
        &self,
        api_id: &str,
        integration_id: &str,
    ) -> Result<Vec<&IntegrationResponse>, ApiGatewayV2Error> {
        let int_key = format!("{api_id}/{integration_id}");
        if !self.integrations.contains_key(&int_key) {
            return Err(ApiGatewayV2Error::IntegrationNotFound(
                integration_id.to_string(),
            ));
        }
        let prefix = format!("{api_id}/{integration_id}/");
        Ok(self
            .integration_responses
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(_, v)| v)
            .collect())
    }

    pub fn delete_integration_response(
        &mut self,
        api_id: &str,
        integration_id: &str,
        response_id: &str,
    ) -> Result<(), ApiGatewayV2Error> {
        let key = format!("{api_id}/{integration_id}/{response_id}");
        if self.integration_responses.remove(&key).is_none() {
            return Err(ApiGatewayV2Error::IntegrationResponseNotFound(
                response_id.to_string(),
            ));
        }
        Ok(())
    }

    pub fn update_integration_response(
        &mut self,
        api_id: &str,
        integration_id: &str,
        response_id: &str,
        integration_response_key: Option<&str>,
        content_handling_strategy: Option<&str>,
        response_parameters: Option<HashMap<String, String>>,
        response_templates: Option<HashMap<String, String>>,
        template_selection_expression: Option<&str>,
    ) -> Result<&IntegrationResponse, ApiGatewayV2Error> {
        let key = format!("{api_id}/{integration_id}/{response_id}");
        let resp = self.integration_responses.get_mut(&key).ok_or_else(|| {
            ApiGatewayV2Error::IntegrationResponseNotFound(response_id.to_string())
        })?;
        if let Some(k) = integration_response_key {
            resp.integration_response_key = k.to_string();
        }
        if let Some(chs) = content_handling_strategy {
            resp.content_handling_strategy = Some(chs.to_string());
        }
        if let Some(rp) = response_parameters {
            resp.response_parameters = Some(rp);
        }
        if let Some(rt) = response_templates {
            resp.response_templates = Some(rt);
        }
        if let Some(tse) = template_selection_expression {
            resp.template_selection_expression = Some(tse.to_string());
        }
        Ok(self.integration_responses.get(&key).unwrap())
    }

    // -----------------------------------------------------------------------
    // Route Responses
    // -----------------------------------------------------------------------

    pub fn create_route_response(
        &mut self,
        api_id: &str,
        route_id: &str,
        route_response_key: &str,
        model_selection_expression: Option<&str>,
    ) -> Result<&RouteResponse, ApiGatewayV2Error> {
        let route_key = format!("{api_id}/{route_id}");
        if !self.routes.contains_key(&route_key) {
            return Err(ApiGatewayV2Error::RouteNotFound(route_id.to_string()));
        }
        let response_id = uuid::Uuid::new_v4().to_string();
        let resp = RouteResponse {
            route_response_id: response_id.clone(),
            api_id: api_id.to_string(),
            route_id: route_id.to_string(),
            route_response_key: route_response_key.to_string(),
            model_selection_expression: model_selection_expression.map(|s| s.to_string()),
        };
        let key = format!("{api_id}/{route_id}/{response_id}");
        self.route_responses.insert(key.clone(), resp);
        Ok(self.route_responses.get(&key).unwrap())
    }

    pub fn get_route_response(
        &self,
        api_id: &str,
        route_id: &str,
        response_id: &str,
    ) -> Result<&RouteResponse, ApiGatewayV2Error> {
        let key = format!("{api_id}/{route_id}/{response_id}");
        self.route_responses
            .get(&key)
            .ok_or_else(|| ApiGatewayV2Error::RouteResponseNotFound(response_id.to_string()))
    }

    pub fn list_route_responses(
        &self,
        api_id: &str,
        route_id: &str,
    ) -> Result<Vec<&RouteResponse>, ApiGatewayV2Error> {
        let route_key = format!("{api_id}/{route_id}");
        if !self.routes.contains_key(&route_key) {
            return Err(ApiGatewayV2Error::RouteNotFound(route_id.to_string()));
        }
        let prefix = format!("{api_id}/{route_id}/");
        Ok(self
            .route_responses
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(_, v)| v)
            .collect())
    }

    pub fn delete_route_response(
        &mut self,
        api_id: &str,
        route_id: &str,
        response_id: &str,
    ) -> Result<(), ApiGatewayV2Error> {
        let key = format!("{api_id}/{route_id}/{response_id}");
        if self.route_responses.remove(&key).is_none() {
            return Err(ApiGatewayV2Error::RouteResponseNotFound(
                response_id.to_string(),
            ));
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // CORS Configuration
    // -----------------------------------------------------------------------

    pub fn delete_cors_configuration(&mut self, api_id: &str) -> Result<(), ApiGatewayV2Error> {
        self.apis
            .get(api_id)
            .ok_or_else(|| ApiGatewayV2Error::ApiNotFound(api_id.to_string()))?;
        // CORS config is part of the API; just succeed (no separate storage needed)
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Tags
    // -----------------------------------------------------------------------

    pub fn get_tags(&self, resource_arn: &str) -> HashMap<String, String> {
        self.tags.get(resource_arn).cloned().unwrap_or_default()
    }

    pub fn tag_resource(&mut self, resource_arn: &str, new_tags: HashMap<String, String>) {
        self.tags
            .entry(resource_arn.to_string())
            .or_default()
            .extend(new_tags);
    }

    pub fn untag_resource(&mut self, resource_arn: &str, tag_keys: &[&str]) {
        if let Some(tags) = self.tags.get_mut(resource_arn) {
            for k in tag_keys {
                tags.remove(*k);
            }
        }
    }
}

fn chrono_now_rfc3339() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Format as a simple ISO-8601 date-time string
    let (y, mo, d, h, mi, s) = epoch_to_datetime(secs);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{s:02}Z")
}

fn epoch_to_datetime(epoch: u64) -> (u64, u64, u64, u64, u64, u64) {
    let s = epoch % 60;
    let m = (epoch / 60) % 60;
    let h = (epoch / 3600) % 24;
    let days = epoch / 86400;

    // Days since 1970-01-01
    let mut year = 1970u64;
    let mut remaining = days;
    loop {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        year += 1;
    }
    let months = [
        31u64,
        if is_leap(year) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut month = 1u64;
    for days_in_month in &months {
        if remaining < *days_in_month {
            break;
        }
        remaining -= days_in_month;
        month += 1;
    }
    (year, month, remaining + 1, h, m, s)
}

fn is_leap(year: u64) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}
