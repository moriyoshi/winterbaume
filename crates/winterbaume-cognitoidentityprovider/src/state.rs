use std::collections::HashMap;

use chrono::Utc;
#[allow(unused_imports)]
use serde_json;

use crate::types::*;

#[derive(Debug, Default)]
pub struct CognitoIdpState {
    pub user_pools: HashMap<String, UserPool>,
}

#[derive(Debug, thiserror::Error)]
pub enum CognitoIdpError {
    #[error("User pool {0} does not exist.")]
    PoolNotFound(String),
    #[error("User does not exist.")]
    UserNotFound,
    #[error("Client {0} does not exist.")]
    ClientNotFound(String),
    #[error("User account already exists.")]
    UsernameExists,
    #[error("Missing USERNAME")]
    MissingUsername,
    #[error("Missing PASSWORD")]
    MissingPassword,
    #[error("User is disabled.")]
    UserDisabled,
    #[error("Incorrect username or password.")]
    IncorrectPassword,
    #[error("Unsupported auth flow: {0}")]
    UnsupportedAuthFlow(String),
    #[error("User already exists.")]
    UserAlreadyExists,
    #[error("A group with the name '{0}' already exists.")]
    GroupExists(String),
    #[error("Group '{0}' not found.")]
    GroupNotFound(String),
    #[error("Provider '{0}' already exists.")]
    DuplicateProvider(String),
    #[error("Identity provider '{0}' not found.")]
    IdentityProviderNotFound(String),
    #[error("{0} already exists in user pool {1}.")]
    ResourceServerAlreadyExists(String, String),
    #[error("Resource server {0} does not exist.")]
    ResourceServerNotFound(String),
    #[error("Resource server '{0}' not found.")]
    ResourceServerNotFoundAlt(String),
    #[error("A domain already exists for this user pool.")]
    DomainAlreadyExists,
    #[error("Domain {0} does not match.")]
    DomainMismatch(String),
    #[error("No domain exists for this user pool.")]
    NoDomainExists,
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    #[error("Import job {0} not found.")]
    ImportJobNotFound(String),
    #[error("Identity provider with identifier '{0}' not found.")]
    IdentityProviderByIdentifierNotFound(String),
}

fn pool_not_found(id: &str) -> CognitoIdpError {
    CognitoIdpError::PoolNotFound(id.to_string())
}

fn user_not_found() -> CognitoIdpError {
    CognitoIdpError::UserNotFound
}

impl CognitoIdpState {
    fn get_pool(&self, id: &str) -> Result<&UserPool, CognitoIdpError> {
        self.user_pools.get(id).ok_or_else(|| pool_not_found(id))
    }

    fn get_pool_mut(&mut self, id: &str) -> Result<&mut UserPool, CognitoIdpError> {
        self.user_pools
            .get_mut(id)
            .ok_or_else(|| pool_not_found(id))
    }

    // --- User Pool ---

    pub fn create_user_pool(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&UserPool, CognitoIdpError> {
        let pool_id = format!(
            "{region}_{}",
            &uuid::Uuid::new_v4().simple().to_string()[..9]
        );
        let arn = format!("arn:aws:cognito-idp:{region}:{account_id}:userpool/{pool_id}");

        let pool = UserPool {
            id: pool_id.clone(),
            name: name.to_string(),
            arn,
            status: "Enabled".to_string(),
            created_date: Utc::now(),
            clients: HashMap::new(),
            users: HashMap::new(),
            groups: HashMap::new(),
            identity_providers: HashMap::new(),
            resource_servers: HashMap::new(),
            domain: None,
            mfa_config: None,
            custom_attributes: Vec::new(),
            tags,
            devices: HashMap::new(),
            ui_customization: None,
            managed_login_brandings: HashMap::new(),
            risk_configuration: None,
            log_delivery_config: None,
            user_import_jobs: HashMap::new(),
            terms: HashMap::new(),
        };

        self.user_pools.insert(pool_id.clone(), pool);
        Ok(self.user_pools.get(&pool_id).unwrap())
    }

    pub fn describe_user_pool(&self, id: &str) -> Result<&UserPool, CognitoIdpError> {
        self.get_pool(id)
    }

    pub fn update_user_pool(&mut self, id: &str) -> Result<&UserPool, CognitoIdpError> {
        // Verify it exists, actual updates would modify pool properties
        let _ = self.get_pool(id)?;
        Ok(self.user_pools.get(id).unwrap())
    }

    pub fn delete_user_pool(&mut self, id: &str) -> Result<(), CognitoIdpError> {
        if self.user_pools.remove(id).is_none() {
            return Err(pool_not_found(id));
        }
        Ok(())
    }

    pub fn list_user_pools(&self) -> Vec<&UserPool> {
        self.user_pools.values().collect()
    }

    // --- User Pool Client ---

    pub fn create_user_pool_client(
        &mut self,
        user_pool_id: &str,
        client_name: &str,
        generate_secret: bool,
        explicit_auth_flows: Vec<String>,
        allowed_oauth_flows: Vec<String>,
        allowed_oauth_scopes: Vec<String>,
        callback_urls: Vec<String>,
        logout_urls: Vec<String>,
        allowed_oauth_flows_user_pool_client: bool,
        refresh_token_validity: Option<i32>,
        supported_identity_providers: Vec<String>,
    ) -> Result<&UserPoolClient, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let client_id = uuid::Uuid::new_v4().to_string().replace('-', "")[..26].to_string();
        let client_secret = if generate_secret {
            Some(uuid::Uuid::new_v4().to_string().replace('-', ""))
        } else {
            None
        };

        let client = UserPoolClient {
            id: client_id.clone(),
            name: client_name.to_string(),
            user_pool_id: user_pool_id.to_string(),
            client_secret,
            explicit_auth_flows,
            allowed_oauth_flows,
            allowed_oauth_scopes,
            callback_urls,
            logout_urls,
            allowed_oauth_flows_user_pool_client,
            refresh_token_validity,
            supported_identity_providers,
        };

        pool.clients.insert(client_id.clone(), client);
        Ok(pool.clients.get(&client_id).unwrap())
    }

    pub fn describe_user_pool_client(
        &self,
        user_pool_id: &str,
        client_id: &str,
    ) -> Result<&UserPoolClient, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        pool.clients
            .get(client_id)
            .ok_or_else(|| CognitoIdpError::ClientNotFound(client_id.to_string()))
    }

    pub fn update_user_pool_client(
        &mut self,
        user_pool_id: &str,
        client_id: &str,
        client_name: Option<&str>,
    ) -> Result<&UserPoolClient, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let client = pool
            .clients
            .get_mut(client_id)
            .ok_or_else(|| CognitoIdpError::ClientNotFound(client_id.to_string()))?;
        if let Some(name) = client_name {
            client.name = name.to_string();
        }
        Ok(client)
    }

    pub fn delete_user_pool_client(
        &mut self,
        user_pool_id: &str,
        client_id: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        pool.clients
            .remove(client_id)
            .ok_or_else(|| CognitoIdpError::ClientNotFound(client_id.to_string()))?;
        Ok(())
    }

    pub fn list_user_pool_clients(
        &self,
        user_pool_id: &str,
    ) -> Result<Vec<&UserPoolClient>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        Ok(pool.clients.values().collect())
    }

    // --- User Admin operations ---

    pub fn admin_create_user(
        &mut self,
        user_pool_id: &str,
        username: &str,
        attributes: HashMap<String, String>,
    ) -> Result<&CognitoUser, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        if pool.users.contains_key(username) {
            return Err(CognitoIdpError::UsernameExists);
        }

        let mut attributes = attributes;
        // Auto-generate a sub attribute if not provided
        if !attributes.contains_key("sub") {
            attributes.insert("sub".to_string(), uuid::Uuid::new_v4().to_string());
        }

        let user = CognitoUser {
            username: username.to_string(),
            status: "FORCE_CHANGE_PASSWORD".to_string(),
            created_date: Utc::now(),
            attributes,
            enabled: true,
            password: None,
            confirmed: false,
            groups: Vec::new(),
            linked_providers: Vec::new(),
        };

        pool.users.insert(username.to_string(), user);
        Ok(pool.users.get(username).unwrap())
    }

    pub fn admin_get_user(
        &self,
        user_pool_id: &str,
        username: &str,
    ) -> Result<&CognitoUser, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        pool.users.get(username).ok_or_else(user_not_found)
    }

    pub fn admin_delete_user(
        &mut self,
        user_pool_id: &str,
        username: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        pool.users.remove(username).ok_or_else(user_not_found)?;
        Ok(())
    }

    pub fn admin_update_user_attributes(
        &mut self,
        user_pool_id: &str,
        username: &str,
        attributes: HashMap<String, String>,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let user = pool.users.get_mut(username).ok_or_else(user_not_found)?;
        user.attributes.extend(attributes);
        Ok(())
    }

    pub fn admin_enable_user(
        &mut self,
        user_pool_id: &str,
        username: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let user = pool.users.get_mut(username).ok_or_else(user_not_found)?;
        user.enabled = true;
        Ok(())
    }

    pub fn admin_disable_user(
        &mut self,
        user_pool_id: &str,
        username: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let user = pool.users.get_mut(username).ok_or_else(user_not_found)?;
        user.enabled = false;
        Ok(())
    }

    pub fn admin_set_user_password(
        &mut self,
        user_pool_id: &str,
        username: &str,
        password: &str,
        permanent: bool,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let user = pool.users.get_mut(username).ok_or_else(user_not_found)?;
        user.password = Some(password.to_string());
        if permanent {
            user.status = "CONFIRMED".to_string();
        }
        Ok(())
    }

    pub fn admin_confirm_sign_up(
        &mut self,
        user_pool_id: &str,
        username: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let user = pool.users.get_mut(username).ok_or_else(user_not_found)?;
        user.status = "CONFIRMED".to_string();
        user.confirmed = true;
        Ok(())
    }

    pub fn admin_reset_user_password(
        &mut self,
        user_pool_id: &str,
        username: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let user = pool.users.get_mut(username).ok_or_else(user_not_found)?;
        user.status = "RESET_REQUIRED".to_string();
        Ok(())
    }

    pub fn list_users(&self, user_pool_id: &str) -> Result<Vec<&CognitoUser>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        Ok(pool.users.values().collect())
    }

    pub fn admin_add_user_to_group(
        &mut self,
        user_pool_id: &str,
        username: &str,
        group_name: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let user = pool.users.get_mut(username).ok_or_else(user_not_found)?;
        if !user.groups.contains(&group_name.to_string()) {
            user.groups.push(group_name.to_string());
        }
        Ok(())
    }

    pub fn admin_remove_user_from_group(
        &mut self,
        user_pool_id: &str,
        username: &str,
        group_name: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let user = pool.users.get_mut(username).ok_or_else(user_not_found)?;
        user.groups.retain(|g| g != group_name);
        Ok(())
    }

    pub fn admin_list_groups_for_user(
        &self,
        user_pool_id: &str,
        username: &str,
    ) -> Result<Vec<&Group>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        let user = pool.users.get(username).ok_or_else(user_not_found)?;
        let groups: Vec<&Group> = user
            .groups
            .iter()
            .filter_map(|gn| pool.groups.get(gn))
            .collect();
        Ok(groups)
    }

    // --- Sign Up / Confirm ---

    pub fn sign_up(
        &mut self,
        user_pool_id: &str,
        username: &str,
        password: &str,
        attributes: HashMap<String, String>,
    ) -> Result<&CognitoUser, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        if pool.users.contains_key(username) {
            return Err(CognitoIdpError::UserAlreadyExists);
        }
        let mut attributes = attributes;
        if !attributes.contains_key("sub") {
            attributes.insert("sub".to_string(), uuid::Uuid::new_v4().to_string());
        }
        let user = CognitoUser {
            username: username.to_string(),
            status: "UNCONFIRMED".to_string(),
            created_date: Utc::now(),
            attributes,
            enabled: true,
            password: Some(password.to_string()),
            confirmed: false,
            groups: Vec::new(),
            linked_providers: Vec::new(),
        };
        pool.users.insert(username.to_string(), user);
        Ok(pool.users.get(username).unwrap())
    }

    pub fn confirm_sign_up(
        &mut self,
        user_pool_id: &str,
        username: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let user = pool.users.get_mut(username).ok_or_else(user_not_found)?;
        user.status = "CONFIRMED".to_string();
        user.confirmed = true;
        Ok(())
    }

    // --- Auth ---

    pub fn admin_initiate_auth(
        &self,
        user_pool_id: &str,
        _client_id: &str,
        auth_flow: &str,
        auth_parameters: &HashMap<String, String>,
    ) -> Result<serde_json::Value, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;

        match auth_flow {
            "ADMIN_USER_PASSWORD_AUTH" | "ADMIN_NO_SRP_AUTH" | "USER_PASSWORD_AUTH" => {
                let username = auth_parameters
                    .get("USERNAME")
                    .ok_or(CognitoIdpError::MissingUsername)?;
                let password = auth_parameters
                    .get("PASSWORD")
                    .ok_or(CognitoIdpError::MissingPassword)?;

                let user = pool
                    .users
                    .get(username.as_str())
                    .ok_or_else(user_not_found)?;
                if !user.enabled {
                    return Err(CognitoIdpError::UserDisabled);
                }
                if user.password.as_deref() != Some(password) {
                    return Err(CognitoIdpError::IncorrectPassword);
                }

                Ok(serde_json::json!({
                    "AuthenticationResult": {
                        "AccessToken": format!("access-token-{}", uuid::Uuid::new_v4()),
                        "IdToken": format!("id-token-{}", uuid::Uuid::new_v4()),
                        "RefreshToken": format!("refresh-token-{}", uuid::Uuid::new_v4()),
                        "ExpiresIn": 3600,
                        "TokenType": "Bearer",
                    }
                }))
            }
            "REFRESH_TOKEN_AUTH" | "REFRESH_TOKEN" => Ok(serde_json::json!({
                "AuthenticationResult": {
                    "AccessToken": format!("access-token-{}", uuid::Uuid::new_v4()),
                    "IdToken": format!("id-token-{}", uuid::Uuid::new_v4()),
                    "ExpiresIn": 3600,
                    "TokenType": "Bearer",
                }
            })),
            _ => Err(CognitoIdpError::UnsupportedAuthFlow(auth_flow.to_string())),
        }
    }

    pub fn initiate_auth(
        &self,
        user_pool_id: &str,
        client_id: &str,
        auth_flow: &str,
        auth_parameters: &HashMap<String, String>,
    ) -> Result<serde_json::Value, CognitoIdpError> {
        // For USER_PASSWORD_AUTH, same as admin but different flow name
        self.admin_initiate_auth(user_pool_id, client_id, auth_flow, auth_parameters)
    }

    pub fn respond_to_auth_challenge(
        &self,
        _user_pool_id: &str,
        _client_id: &str,
        _challenge_name: &str,
        _challenge_responses: &HashMap<String, String>,
    ) -> Result<serde_json::Value, CognitoIdpError> {
        // Return successful auth result
        Ok(serde_json::json!({
            "AuthenticationResult": {
                "AccessToken": format!("access-token-{}", uuid::Uuid::new_v4()),
                "IdToken": format!("id-token-{}", uuid::Uuid::new_v4()),
                "RefreshToken": format!("refresh-token-{}", uuid::Uuid::new_v4()),
                "ExpiresIn": 3600,
                "TokenType": "Bearer",
            }
        }))
    }

    pub fn admin_respond_to_auth_challenge(
        &self,
        user_pool_id: &str,
        client_id: &str,
        challenge_name: &str,
        challenge_responses: &HashMap<String, String>,
    ) -> Result<serde_json::Value, CognitoIdpError> {
        self.respond_to_auth_challenge(user_pool_id, client_id, challenge_name, challenge_responses)
    }

    // --- Groups ---

    pub fn create_group(
        &mut self,
        user_pool_id: &str,
        group_name: &str,
        description: Option<&str>,
        role_arn: Option<&str>,
        precedence: Option<i32>,
    ) -> Result<&Group, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        if pool.groups.contains_key(group_name) {
            return Err(CognitoIdpError::GroupExists(group_name.to_string()));
        }

        let now = Utc::now();
        let group = Group {
            group_name: group_name.to_string(),
            user_pool_id: user_pool_id.to_string(),
            description: description.map(|s| s.to_string()),
            role_arn: role_arn.map(|s| s.to_string()),
            precedence,
            created_date: now,
            last_modified_date: now,
        };

        pool.groups.insert(group_name.to_string(), group);
        Ok(pool.groups.get(group_name).unwrap())
    }

    pub fn get_group(
        &self,
        user_pool_id: &str,
        group_name: &str,
    ) -> Result<&Group, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        pool.groups
            .get(group_name)
            .ok_or_else(|| CognitoIdpError::GroupNotFound(group_name.to_string()))
    }

    pub fn update_group(
        &mut self,
        user_pool_id: &str,
        group_name: &str,
        description: Option<&str>,
        role_arn: Option<&str>,
        precedence: Option<i32>,
    ) -> Result<&Group, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let group = pool
            .groups
            .get_mut(group_name)
            .ok_or_else(|| CognitoIdpError::GroupNotFound(group_name.to_string()))?;
        if let Some(d) = description {
            group.description = Some(d.to_string());
        }
        if let Some(r) = role_arn {
            group.role_arn = Some(r.to_string());
        }
        if let Some(p) = precedence {
            group.precedence = Some(p);
        }
        group.last_modified_date = Utc::now();
        Ok(group)
    }

    pub fn delete_group(
        &mut self,
        user_pool_id: &str,
        group_name: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        pool.groups
            .remove(group_name)
            .ok_or_else(|| CognitoIdpError::GroupNotFound(group_name.to_string()))?;
        Ok(())
    }

    pub fn list_groups(&self, user_pool_id: &str) -> Result<Vec<&Group>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        Ok(pool.groups.values().collect())
    }

    pub fn list_users_in_group(
        &self,
        user_pool_id: &str,
        group_name: &str,
    ) -> Result<Vec<&CognitoUser>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        Ok(pool
            .users
            .values()
            .filter(|u| u.groups.contains(&group_name.to_string()))
            .collect())
    }

    // --- Identity Providers ---

    pub fn create_identity_provider(
        &mut self,
        user_pool_id: &str,
        provider_name: &str,
        provider_type: &str,
        provider_details: HashMap<String, String>,
        attribute_mapping: HashMap<String, String>,
        idp_identifiers: Vec<String>,
    ) -> Result<&IdentityProvider, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        if pool.identity_providers.contains_key(provider_name) {
            return Err(CognitoIdpError::DuplicateProvider(
                provider_name.to_string(),
            ));
        }

        let now = Utc::now();
        let idp = IdentityProvider {
            provider_name: provider_name.to_string(),
            provider_type: provider_type.to_string(),
            user_pool_id: user_pool_id.to_string(),
            provider_details,
            attribute_mapping,
            idp_identifiers,
            created_date: now,
            last_modified_date: now,
        };

        pool.identity_providers
            .insert(provider_name.to_string(), idp);
        Ok(pool.identity_providers.get(provider_name).unwrap())
    }

    pub fn describe_identity_provider(
        &self,
        user_pool_id: &str,
        provider_name: &str,
    ) -> Result<&IdentityProvider, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        pool.identity_providers
            .get(provider_name)
            .ok_or_else(|| CognitoIdpError::IdentityProviderNotFound(provider_name.to_string()))
    }

    pub fn update_identity_provider(
        &mut self,
        user_pool_id: &str,
        provider_name: &str,
        provider_details: Option<HashMap<String, String>>,
        attribute_mapping: Option<HashMap<String, String>>,
        idp_identifiers: Option<Vec<String>>,
    ) -> Result<&IdentityProvider, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let idp = pool
            .identity_providers
            .get_mut(provider_name)
            .ok_or_else(|| CognitoIdpError::IdentityProviderNotFound(provider_name.to_string()))?;
        if let Some(details) = provider_details {
            idp.provider_details = details;
        }
        if let Some(mapping) = attribute_mapping {
            idp.attribute_mapping = mapping;
        }
        if let Some(identifiers) = idp_identifiers {
            idp.idp_identifiers = identifiers;
        }
        idp.last_modified_date = Utc::now();
        Ok(idp)
    }

    pub fn delete_identity_provider(
        &mut self,
        user_pool_id: &str,
        provider_name: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        pool.identity_providers
            .remove(provider_name)
            .ok_or_else(|| CognitoIdpError::IdentityProviderNotFound(provider_name.to_string()))?;
        Ok(())
    }

    pub fn list_identity_providers(
        &self,
        user_pool_id: &str,
    ) -> Result<Vec<&IdentityProvider>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        Ok(pool.identity_providers.values().collect())
    }

    // --- Resource Servers ---

    pub fn create_resource_server(
        &mut self,
        user_pool_id: &str,
        identifier: &str,
        name: &str,
        scopes: Vec<ResourceServerScope>,
    ) -> Result<&ResourceServer, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        if pool.resource_servers.contains_key(identifier) {
            return Err(CognitoIdpError::ResourceServerAlreadyExists(
                identifier.to_string(),
                user_pool_id.to_string(),
            ));
        }

        let rs = ResourceServer {
            identifier: identifier.to_string(),
            name: name.to_string(),
            user_pool_id: user_pool_id.to_string(),
            scopes,
        };

        pool.resource_servers.insert(identifier.to_string(), rs);
        Ok(pool.resource_servers.get(identifier).unwrap())
    }

    pub fn describe_resource_server(
        &self,
        user_pool_id: &str,
        identifier: &str,
    ) -> Result<&ResourceServer, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        pool.resource_servers
            .get(identifier)
            .ok_or_else(|| CognitoIdpError::ResourceServerNotFound(identifier.to_string()))
    }

    pub fn update_resource_server(
        &mut self,
        user_pool_id: &str,
        identifier: &str,
        name: Option<&str>,
        scopes: Option<Vec<ResourceServerScope>>,
    ) -> Result<&ResourceServer, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let rs = pool
            .resource_servers
            .get_mut(identifier)
            .ok_or_else(|| CognitoIdpError::ResourceServerNotFoundAlt(identifier.to_string()))?;
        if let Some(n) = name {
            rs.name = n.to_string();
        }
        if let Some(s) = scopes {
            rs.scopes = s;
        }
        Ok(rs)
    }

    pub fn delete_resource_server(
        &mut self,
        user_pool_id: &str,
        identifier: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        pool.resource_servers
            .remove(identifier)
            .ok_or_else(|| CognitoIdpError::ResourceServerNotFoundAlt(identifier.to_string()))?;
        Ok(())
    }

    pub fn list_resource_servers(
        &self,
        user_pool_id: &str,
    ) -> Result<Vec<&ResourceServer>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        Ok(pool.resource_servers.values().collect())
    }

    // --- Custom Attributes ---

    pub fn add_custom_attributes(
        &mut self,
        user_pool_id: &str,
        attributes: Vec<String>,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        pool.custom_attributes.extend(attributes);
        Ok(())
    }

    // --- Admin Delete User Attributes ---

    pub fn admin_delete_user_attributes(
        &mut self,
        user_pool_id: &str,
        username: &str,
        attribute_names: &[&str],
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let user = pool.users.get_mut(username).ok_or_else(user_not_found)?;
        for name in attribute_names {
            user.attributes.remove(*name);
        }
        Ok(())
    }

    // --- User Pool Domain ---

    pub fn create_user_pool_domain(
        &mut self,
        user_pool_id: &str,
        domain: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        if pool.domain.is_some() {
            return Err(CognitoIdpError::DomainAlreadyExists);
        }
        pool.domain = Some(UserPoolDomain {
            domain: domain.to_string(),
            user_pool_id: user_pool_id.to_string(),
            status: "ACTIVE".to_string(),
            cloud_front_distribution: None,
            custom_domain_config: None,
        });
        Ok(())
    }

    pub fn describe_user_pool_domain(&self, domain: &str) -> Option<&UserPoolDomain> {
        for pool in self.user_pools.values() {
            if let Some(ref d) = pool.domain
                && d.domain == domain
            {
                return Some(d);
            }
        }
        None
    }

    pub fn update_user_pool_domain(
        &mut self,
        user_pool_id: &str,
        domain: &str,
        custom_domain_config: Option<CustomDomainConfig>,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        if let Some(ref mut d) = pool.domain {
            if d.domain != domain {
                return Err(CognitoIdpError::DomainMismatch(domain.to_string()));
            }
            d.custom_domain_config = custom_domain_config;
            Ok(())
        } else {
            Err(CognitoIdpError::NoDomainExists)
        }
    }

    pub fn delete_user_pool_domain(
        &mut self,
        user_pool_id: &str,
        domain: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        if let Some(ref d) = pool.domain {
            if d.domain != domain {
                return Err(CognitoIdpError::DomainMismatch(domain.to_string()));
            }
            pool.domain = None;
            Ok(())
        } else {
            Err(CognitoIdpError::NoDomainExists)
        }
    }

    // --- MFA Config ---

    pub fn get_user_pool_mfa_config(
        &self,
        user_pool_id: &str,
    ) -> Result<Option<&MfaConfig>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        Ok(pool.mfa_config.as_ref())
    }

    pub fn set_user_pool_mfa_config(
        &mut self,
        user_pool_id: &str,
        mfa_configuration: &str,
        sms_config: Option<SmsMfaConfig>,
        software_token_config: Option<SoftwareTokenMfaConfig>,
    ) -> Result<&MfaConfig, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        pool.mfa_config = Some(MfaConfig {
            mfa_configuration: mfa_configuration.to_string(),
            sms_mfa_configuration: sms_config,
            software_token_mfa_configuration: software_token_config,
        });
        Ok(pool.mfa_config.as_ref().unwrap())
    }

    // --- Tags ---

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.find_pool_by_arn_mut(resource_arn)?;
        pool.tags.extend(tags);
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), CognitoIdpError> {
        let pool = self.find_pool_by_arn_mut(resource_arn)?;
        for key in tag_keys {
            pool.tags.remove(key);
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<&HashMap<String, String>, CognitoIdpError> {
        let pool = self.find_pool_by_arn(resource_arn)?;
        Ok(&pool.tags)
    }

    fn find_pool_by_arn(&self, arn: &str) -> Result<&UserPool, CognitoIdpError> {
        self.user_pools
            .values()
            .find(|p| p.arn == arn)
            .ok_or_else(|| CognitoIdpError::ResourceNotFound(arn.to_string()))
    }

    fn find_pool_by_arn_mut(&mut self, arn: &str) -> Result<&mut UserPool, CognitoIdpError> {
        self.user_pools
            .values_mut()
            .find(|p| p.arn == arn)
            .ok_or_else(|| CognitoIdpError::ResourceNotFound(arn.to_string()))
    }

    // --- Misc: find user pool by client_id ---

    pub fn find_pool_id_by_client_id(&self, client_id: &str) -> Option<String> {
        for pool in self.user_pools.values() {
            if pool.clients.contains_key(client_id) {
                return Some(pool.id.clone());
            }
        }
        None
    }

    pub fn admin_user_global_sign_out(
        &self,
        user_pool_id: &str,
        username: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        pool.users.get(username).ok_or_else(user_not_found)?;
        Ok(())
    }

    pub fn global_sign_out(&self) -> Result<(), CognitoIdpError> {
        // Token-based, always succeeds
        Ok(())
    }

    // --- Device tracking ---

    pub fn admin_forget_device(
        &mut self,
        user_pool_id: &str,
        username: &str,
        device_key: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        pool.users.get(username).ok_or_else(user_not_found)?;
        let k = format!("{username}::{device_key}");
        pool.devices.remove(&k);
        Ok(())
    }

    pub fn admin_get_device(
        &self,
        user_pool_id: &str,
        username: &str,
        device_key: &str,
    ) -> Result<Option<&Device>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        pool.users.get(username).ok_or_else(user_not_found)?;
        let k = format!("{username}::{device_key}");
        Ok(pool.devices.get(&k))
    }

    pub fn admin_list_devices(
        &self,
        user_pool_id: &str,
        username: &str,
    ) -> Result<Vec<&Device>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        pool.users.get(username).ok_or_else(user_not_found)?;
        let prefix = format!("{username}::");
        Ok(pool
            .devices
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(_, v)| v)
            .collect())
    }

    pub fn admin_update_device_status(
        &mut self,
        user_pool_id: &str,
        username: &str,
        _device_key: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        pool.users.get(username).ok_or_else(user_not_found)?;
        Ok(())
    }

    pub fn confirm_device(
        &mut self,
        device_key: &str,
        device_name: Option<&str>,
    ) -> Result<(), CognitoIdpError> {
        // Token-based: store device globally in any pool that has a user with access token
        // For mock purposes just create a device in the first pool found
        for pool in self.user_pools.values_mut() {
            if let Some(user) = pool.users.values().next() {
                let username = user.username.clone();
                let k = format!("{username}::{device_key}");
                let mut attrs = HashMap::new();
                if let Some(name) = device_name {
                    attrs.insert("device_name".to_string(), name.to_string());
                }
                let now = Utc::now();
                pool.devices.insert(
                    k,
                    Device {
                        device_key: device_key.to_string(),
                        device_attributes: attrs,
                        device_created_date: now,
                        device_last_modified_date: now,
                    },
                );
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn forget_device(&mut self, _device_key: &str) -> Result<(), CognitoIdpError> {
        // Token-based stub
        Ok(())
    }

    pub fn get_device(
        &self,
        _device_key: &str,
    ) -> Result<Option<serde_json::Value>, CognitoIdpError> {
        // Token-based: return a stub
        Ok(None)
    }

    pub fn list_devices(&self) -> Result<Vec<serde_json::Value>, CognitoIdpError> {
        // Token-based: return empty list
        Ok(vec![])
    }

    pub fn update_device_status(&mut self, _device_key: &str) -> Result<(), CognitoIdpError> {
        // Token-based stub
        Ok(())
    }

    // --- User import jobs ---

    pub fn create_user_import_job(
        &mut self,
        user_pool_id: &str,
        job_name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&UserImportJob, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let job_id = format!("import-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let job_arn = format!(
            "arn:aws:cognito-idp:{region}:{account_id}:userpool/{user_pool_id}/importJob/{job_id}"
        );
        let job = UserImportJob {
            job_id: job_id.clone(),
            job_name: job_name.to_string(),
            job_arn,
            user_pool_id: user_pool_id.to_string(),
            pre_signed_url: Some(format!(
                "https://mock-presigned-url.s3.amazonaws.com/{job_id}"
            )),
            created_date: Utc::now(),
            status: "Created".to_string(),
        };
        pool.user_import_jobs.insert(job_id.clone(), job);
        Ok(pool.user_import_jobs.get(&job_id).unwrap())
    }

    pub fn describe_user_import_job(
        &self,
        user_pool_id: &str,
        job_id: &str,
    ) -> Result<&UserImportJob, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        pool.user_import_jobs
            .get(job_id)
            .ok_or_else(|| CognitoIdpError::ImportJobNotFound(job_id.to_string()))
    }

    pub fn list_user_import_jobs(
        &self,
        user_pool_id: &str,
    ) -> Result<Vec<&UserImportJob>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        Ok(pool.user_import_jobs.values().collect())
    }

    pub fn start_user_import_job(
        &mut self,
        user_pool_id: &str,
        job_id: &str,
    ) -> Result<&UserImportJob, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let job = pool
            .user_import_jobs
            .get_mut(job_id)
            .ok_or_else(|| CognitoIdpError::ImportJobNotFound(job_id.to_string()))?;
        job.status = "Pending".to_string();
        Ok(job)
    }

    pub fn stop_user_import_job(
        &mut self,
        user_pool_id: &str,
        job_id: &str,
    ) -> Result<&UserImportJob, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let job = pool
            .user_import_jobs
            .get_mut(job_id)
            .ok_or_else(|| CognitoIdpError::ImportJobNotFound(job_id.to_string()))?;
        job.status = "Stopped".to_string();
        Ok(job)
    }

    // --- UI Customization ---

    pub fn get_ui_customization(
        &self,
        user_pool_id: &str,
    ) -> Result<Option<&UICustomization>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        Ok(pool.ui_customization.as_ref())
    }

    pub fn set_ui_customization(
        &mut self,
        user_pool_id: &str,
        client_id: Option<&str>,
        css: Option<&str>,
        image_url: Option<&str>,
    ) -> Result<&UICustomization, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let now = Utc::now();
        let existing_creation_date = pool
            .ui_customization
            .as_ref()
            .map(|u| u.creation_date)
            .unwrap_or(now);
        pool.ui_customization = Some(UICustomization {
            user_pool_id: user_pool_id.to_string(),
            client_id: client_id.map(|s| s.to_string()),
            css: css.map(|s| s.to_string()),
            css_version: None,
            image_url: image_url.map(|s| s.to_string()),
            creation_date: existing_creation_date,
            last_modified_date: now,
        });
        Ok(pool.ui_customization.as_ref().unwrap())
    }

    // --- Managed Login Branding ---

    pub fn create_managed_login_branding(
        &mut self,
        user_pool_id: &str,
        settings: Option<serde_json::Value>,
    ) -> Result<&ManagedLoginBranding, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let branding_id = format!("branding-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let now = Utc::now();
        let branding = ManagedLoginBranding {
            branding_id: branding_id.clone(),
            user_pool_id: user_pool_id.to_string(),
            settings,
            creation_date: now,
            last_modified_date: now,
        };
        pool.managed_login_brandings
            .insert(branding_id.clone(), branding);
        Ok(pool.managed_login_brandings.get(&branding_id).unwrap())
    }

    pub fn delete_managed_login_branding(
        &mut self,
        user_pool_id: &str,
        branding_id: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        pool.managed_login_brandings.remove(branding_id);
        Ok(())
    }

    pub fn describe_managed_login_branding(
        &self,
        user_pool_id: &str,
        branding_id: &str,
    ) -> Result<Option<&ManagedLoginBranding>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        Ok(pool.managed_login_brandings.get(branding_id))
    }

    pub fn describe_managed_login_branding_by_client(
        &self,
        user_pool_id: &str,
        _client_id: &str,
    ) -> Result<Option<&ManagedLoginBranding>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        Ok(pool.managed_login_brandings.values().next())
    }

    pub fn update_managed_login_branding(
        &mut self,
        user_pool_id: &str,
        branding_id: &str,
        settings: Option<serde_json::Value>,
    ) -> Result<&ManagedLoginBranding, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let branding = pool
            .managed_login_brandings
            .entry(branding_id.to_string())
            .or_insert_with(|| {
                let now = Utc::now();
                ManagedLoginBranding {
                    branding_id: branding_id.to_string(),
                    user_pool_id: user_pool_id.to_string(),
                    settings: None,
                    creation_date: now,
                    last_modified_date: now,
                }
            });
        branding.settings = settings;
        branding.last_modified_date = Utc::now();
        Ok(branding)
    }

    // --- Risk Configuration ---

    pub fn get_risk_configuration(
        &self,
        user_pool_id: &str,
    ) -> Result<Option<&serde_json::Value>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        Ok(pool.risk_configuration.as_ref())
    }

    pub fn set_risk_configuration(
        &mut self,
        user_pool_id: &str,
        config: serde_json::Value,
    ) -> Result<&serde_json::Value, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        pool.risk_configuration = Some(config);
        Ok(pool.risk_configuration.as_ref().unwrap())
    }

    // --- Log Delivery Configuration ---

    pub fn get_log_delivery_configuration(
        &self,
        user_pool_id: &str,
    ) -> Result<Option<&Vec<serde_json::Value>>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        Ok(pool.log_delivery_config.as_ref())
    }

    pub fn set_log_delivery_configuration(
        &mut self,
        user_pool_id: &str,
        configs: Vec<serde_json::Value>,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        pool.log_delivery_config = Some(configs);
        Ok(())
    }

    // --- Identity Provider operations ---

    pub fn get_identity_provider_by_identifier(
        &self,
        user_pool_id: &str,
        idp_identifier: &str,
    ) -> Result<&IdentityProvider, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        pool.identity_providers
            .values()
            .find(|idp| idp.idp_identifiers.contains(&idp_identifier.to_string()))
            .ok_or_else(|| {
                CognitoIdpError::IdentityProviderByIdentifierNotFound(idp_identifier.to_string())
            })
    }

    pub fn admin_link_provider_for_user(
        &mut self,
        user_pool_id: &str,
        destination_user_username: &str,
        source_provider_name: Option<String>,
        source_provider_attribute_name: Option<String>,
        source_provider_attribute_value: Option<String>,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        let user = pool
            .users
            .get_mut(destination_user_username)
            .ok_or_else(user_not_found)?;
        user.linked_providers.push(LinkedProvider {
            provider_name: source_provider_name,
            provider_attribute_name: source_provider_attribute_name,
            provider_attribute_value: source_provider_attribute_value,
        });
        Ok(())
    }

    pub fn admin_disable_provider_for_user(
        &mut self,
        user_pool_id: &str,
        user_provider_name: Option<&str>,
        user_provider_attribute_value: Option<&str>,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        if let (Some(pn), Some(pv)) = (user_provider_name, user_provider_attribute_value) {
            for user in pool.users.values_mut() {
                user.linked_providers.retain(|lp| {
                    lp.provider_name.as_deref() != Some(pn)
                        || lp.provider_attribute_value.as_deref() != Some(pv)
                });
            }
        }
        Ok(())
    }

    // --- Terms ---

    pub fn create_terms(
        &mut self,
        user_pool_id: &str,
        terms_id: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        pool.terms.insert(terms_id.to_string(), data.clone());
        Ok(data)
    }

    pub fn delete_terms(
        &mut self,
        user_pool_id: &str,
        terms_id: &str,
    ) -> Result<(), CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        pool.terms.remove(terms_id);
        Ok(())
    }

    pub fn describe_terms(
        &self,
        user_pool_id: &str,
        terms_id: &str,
    ) -> Result<Option<&serde_json::Value>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        Ok(pool.terms.get(terms_id))
    }

    pub fn list_terms(
        &self,
        user_pool_id: &str,
    ) -> Result<Vec<&serde_json::Value>, CognitoIdpError> {
        let pool = self.get_pool(user_pool_id)?;
        Ok(pool.terms.values().collect())
    }

    pub fn update_terms(
        &mut self,
        user_pool_id: &str,
        terms_id: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value, CognitoIdpError> {
        let pool = self.get_pool_mut(user_pool_id)?;
        pool.terms.insert(terms_id.to_string(), data.clone());
        Ok(data)
    }

    // --- User Attributes (token-based) ---

    pub fn delete_user_attributes(
        &mut self,
        _access_token: &str,
        attribute_names: &[String],
    ) -> Result<(), CognitoIdpError> {
        // Token-based: remove attributes from first user found
        for pool in self.user_pools.values_mut() {
            if let Some(user) = pool.users.values_mut().next() {
                for name in attribute_names {
                    user.attributes.remove(name);
                }
                return Ok(());
            }
        }
        Ok(())
    }

    // --- Resend Confirmation Code ---

    pub fn resend_confirmation_code(
        &self,
        client_id: &str,
        _username: &str,
    ) -> Result<(), CognitoIdpError> {
        // Verify pool exists
        let _pool_id = self
            .find_pool_id_by_client_id(client_id)
            .ok_or_else(|| CognitoIdpError::ClientNotFound(client_id.to_string()))?;
        Ok(())
    }
}
