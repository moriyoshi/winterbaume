use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{DsError, DsState};
use crate::views::DsStateView;
use crate::wire;

/// Directory Service handler that processes awsJson1.1 protocol requests.
pub struct DirectoryService {
    pub(crate) state: Arc<BackendState<DsState>>,
    pub(crate) notifier: StateChangeNotifier<DsStateView>,
}

impl DirectoryService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for DirectoryService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for DirectoryService {
    fn service_name(&self) -> &str {
        "ds"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://ds\..*\.amazonaws\.com",
            r"https?://ds\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl DirectoryService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "DirectoryService_20150416.CreateDirectory"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        use winterbaume_core::StatefulService;
        let response = match action.as_str() {
            "CreateDirectory" => {
                self.handle_create_directory(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeDirectories" => self.handle_describe_directories(&state, body_bytes).await,
            "DeleteDirectory" => self.handle_delete_directory(&state, body_bytes).await,
            "ConnectDirectory" => {
                self.handle_connect_directory(&state, body_bytes, account_id, &region)
                    .await
            }
            // --- Unimplemented operations (auto-generated stubs) ---
            "AcceptSharedDirectory" => json_error_response(
                501,
                "NotImplementedError",
                "AcceptSharedDirectory is not yet implemented in winterbaume-ds",
            ),
            "AddIpRoutes" => json_error_response(
                501,
                "NotImplementedError",
                "AddIpRoutes is not yet implemented in winterbaume-ds",
            ),
            "AddRegion" => json_error_response(
                501,
                "NotImplementedError",
                "AddRegion is not yet implemented in winterbaume-ds",
            ),
            "AddTagsToResource" => json_error_response(
                501,
                "NotImplementedError",
                "AddTagsToResource is not yet implemented in winterbaume-ds",
            ),
            "CancelSchemaExtension" => json_error_response(
                501,
                "NotImplementedError",
                "CancelSchemaExtension is not yet implemented in winterbaume-ds",
            ),
            "CreateAlias" => json_error_response(
                501,
                "NotImplementedError",
                "CreateAlias is not yet implemented in winterbaume-ds",
            ),
            "CreateComputer" => json_error_response(
                501,
                "NotImplementedError",
                "CreateComputer is not yet implemented in winterbaume-ds",
            ),
            "CreateConditionalForwarder" => json_error_response(
                501,
                "NotImplementedError",
                "CreateConditionalForwarder is not yet implemented in winterbaume-ds",
            ),
            "CreateHybridAD" => json_error_response(
                501,
                "NotImplementedError",
                "CreateHybridAD is not yet implemented in winterbaume-ds",
            ),
            "CreateLogSubscription" => json_error_response(
                501,
                "NotImplementedError",
                "CreateLogSubscription is not yet implemented in winterbaume-ds",
            ),
            "CreateMicrosoftAD" => json_error_response(
                501,
                "NotImplementedError",
                "CreateMicrosoftAD is not yet implemented in winterbaume-ds",
            ),
            "CreateSnapshot" => json_error_response(
                501,
                "NotImplementedError",
                "CreateSnapshot is not yet implemented in winterbaume-ds",
            ),
            "CreateTrust" => json_error_response(
                501,
                "NotImplementedError",
                "CreateTrust is not yet implemented in winterbaume-ds",
            ),
            "DeleteADAssessment" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteADAssessment is not yet implemented in winterbaume-ds",
            ),
            "DeleteConditionalForwarder" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteConditionalForwarder is not yet implemented in winterbaume-ds",
            ),
            "DeleteLogSubscription" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteLogSubscription is not yet implemented in winterbaume-ds",
            ),
            "DeleteSnapshot" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteSnapshot is not yet implemented in winterbaume-ds",
            ),
            "DeleteTrust" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteTrust is not yet implemented in winterbaume-ds",
            ),
            "DeregisterCertificate" => json_error_response(
                501,
                "NotImplementedError",
                "DeregisterCertificate is not yet implemented in winterbaume-ds",
            ),
            "DeregisterEventTopic" => json_error_response(
                501,
                "NotImplementedError",
                "DeregisterEventTopic is not yet implemented in winterbaume-ds",
            ),
            "DescribeADAssessment" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeADAssessment is not yet implemented in winterbaume-ds",
            ),
            "DescribeCAEnrollmentPolicy" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeCAEnrollmentPolicy is not yet implemented in winterbaume-ds",
            ),
            "DescribeCertificate" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeCertificate is not yet implemented in winterbaume-ds",
            ),
            "DescribeClientAuthenticationSettings" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeClientAuthenticationSettings is not yet implemented in winterbaume-ds",
            ),
            "DescribeConditionalForwarders" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeConditionalForwarders is not yet implemented in winterbaume-ds",
            ),
            "DescribeDirectoryDataAccess" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeDirectoryDataAccess is not yet implemented in winterbaume-ds",
            ),
            "DescribeDomainControllers" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeDomainControllers is not yet implemented in winterbaume-ds",
            ),
            "DescribeEventTopics" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeEventTopics is not yet implemented in winterbaume-ds",
            ),
            "DescribeHybridADUpdate" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeHybridADUpdate is not yet implemented in winterbaume-ds",
            ),
            "DescribeLDAPSSettings" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeLDAPSSettings is not yet implemented in winterbaume-ds",
            ),
            "DescribeRegions" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeRegions is not yet implemented in winterbaume-ds",
            ),
            "DescribeSettings" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeSettings is not yet implemented in winterbaume-ds",
            ),
            "DescribeSharedDirectories" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeSharedDirectories is not yet implemented in winterbaume-ds",
            ),
            "DescribeSnapshots" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeSnapshots is not yet implemented in winterbaume-ds",
            ),
            "DescribeTrusts" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeTrusts is not yet implemented in winterbaume-ds",
            ),
            "DescribeUpdateDirectory" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeUpdateDirectory is not yet implemented in winterbaume-ds",
            ),
            "DisableCAEnrollmentPolicy" => json_error_response(
                501,
                "NotImplementedError",
                "DisableCAEnrollmentPolicy is not yet implemented in winterbaume-ds",
            ),
            "DisableClientAuthentication" => json_error_response(
                501,
                "NotImplementedError",
                "DisableClientAuthentication is not yet implemented in winterbaume-ds",
            ),
            "DisableDirectoryDataAccess" => json_error_response(
                501,
                "NotImplementedError",
                "DisableDirectoryDataAccess is not yet implemented in winterbaume-ds",
            ),
            "DisableLDAPS" => json_error_response(
                501,
                "NotImplementedError",
                "DisableLDAPS is not yet implemented in winterbaume-ds",
            ),
            "DisableRadius" => json_error_response(
                501,
                "NotImplementedError",
                "DisableRadius is not yet implemented in winterbaume-ds",
            ),
            "DisableSso" => json_error_response(
                501,
                "NotImplementedError",
                "DisableSso is not yet implemented in winterbaume-ds",
            ),
            "EnableCAEnrollmentPolicy" => json_error_response(
                501,
                "NotImplementedError",
                "EnableCAEnrollmentPolicy is not yet implemented in winterbaume-ds",
            ),
            "EnableClientAuthentication" => json_error_response(
                501,
                "NotImplementedError",
                "EnableClientAuthentication is not yet implemented in winterbaume-ds",
            ),
            "EnableDirectoryDataAccess" => json_error_response(
                501,
                "NotImplementedError",
                "EnableDirectoryDataAccess is not yet implemented in winterbaume-ds",
            ),
            "EnableLDAPS" => json_error_response(
                501,
                "NotImplementedError",
                "EnableLDAPS is not yet implemented in winterbaume-ds",
            ),
            "EnableRadius" => json_error_response(
                501,
                "NotImplementedError",
                "EnableRadius is not yet implemented in winterbaume-ds",
            ),
            "EnableSso" => json_error_response(
                501,
                "NotImplementedError",
                "EnableSso is not yet implemented in winterbaume-ds",
            ),
            "GetDirectoryLimits" => json_error_response(
                501,
                "NotImplementedError",
                "GetDirectoryLimits is not yet implemented in winterbaume-ds",
            ),
            "GetSnapshotLimits" => json_error_response(
                501,
                "NotImplementedError",
                "GetSnapshotLimits is not yet implemented in winterbaume-ds",
            ),
            "ListADAssessments" => json_error_response(
                501,
                "NotImplementedError",
                "ListADAssessments is not yet implemented in winterbaume-ds",
            ),
            "ListCertificates" => json_error_response(
                501,
                "NotImplementedError",
                "ListCertificates is not yet implemented in winterbaume-ds",
            ),
            "ListIpRoutes" => json_error_response(
                501,
                "NotImplementedError",
                "ListIpRoutes is not yet implemented in winterbaume-ds",
            ),
            "ListLogSubscriptions" => json_error_response(
                501,
                "NotImplementedError",
                "ListLogSubscriptions is not yet implemented in winterbaume-ds",
            ),
            "ListSchemaExtensions" => json_error_response(
                501,
                "NotImplementedError",
                "ListSchemaExtensions is not yet implemented in winterbaume-ds",
            ),
            "ListTagsForResource" => json_error_response(
                501,
                "NotImplementedError",
                "ListTagsForResource is not yet implemented in winterbaume-ds",
            ),
            "RegisterCertificate" => json_error_response(
                501,
                "NotImplementedError",
                "RegisterCertificate is not yet implemented in winterbaume-ds",
            ),
            "RegisterEventTopic" => json_error_response(
                501,
                "NotImplementedError",
                "RegisterEventTopic is not yet implemented in winterbaume-ds",
            ),
            "RejectSharedDirectory" => json_error_response(
                501,
                "NotImplementedError",
                "RejectSharedDirectory is not yet implemented in winterbaume-ds",
            ),
            "RemoveIpRoutes" => json_error_response(
                501,
                "NotImplementedError",
                "RemoveIpRoutes is not yet implemented in winterbaume-ds",
            ),
            "RemoveRegion" => json_error_response(
                501,
                "NotImplementedError",
                "RemoveRegion is not yet implemented in winterbaume-ds",
            ),
            "RemoveTagsFromResource" => json_error_response(
                501,
                "NotImplementedError",
                "RemoveTagsFromResource is not yet implemented in winterbaume-ds",
            ),
            "ResetUserPassword" => json_error_response(
                501,
                "NotImplementedError",
                "ResetUserPassword is not yet implemented in winterbaume-ds",
            ),
            "RestoreFromSnapshot" => json_error_response(
                501,
                "NotImplementedError",
                "RestoreFromSnapshot is not yet implemented in winterbaume-ds",
            ),
            "ShareDirectory" => json_error_response(
                501,
                "NotImplementedError",
                "ShareDirectory is not yet implemented in winterbaume-ds",
            ),
            "StartADAssessment" => json_error_response(
                501,
                "NotImplementedError",
                "StartADAssessment is not yet implemented in winterbaume-ds",
            ),
            "StartSchemaExtension" => json_error_response(
                501,
                "NotImplementedError",
                "StartSchemaExtension is not yet implemented in winterbaume-ds",
            ),
            "UnshareDirectory" => json_error_response(
                501,
                "NotImplementedError",
                "UnshareDirectory is not yet implemented in winterbaume-ds",
            ),
            "UpdateConditionalForwarder" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateConditionalForwarder is not yet implemented in winterbaume-ds",
            ),
            "UpdateDirectorySetup" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateDirectorySetup is not yet implemented in winterbaume-ds",
            ),
            "UpdateHybridAD" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateHybridAD is not yet implemented in winterbaume-ds",
            ),
            "UpdateNumberOfDomainControllers" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateNumberOfDomainControllers is not yet implemented in winterbaume-ds",
            ),
            "UpdateRadius" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateRadius is not yet implemented in winterbaume-ds",
            ),
            "UpdateSettings" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateSettings is not yet implemented in winterbaume-ds",
            ),
            "UpdateTrust" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateTrust is not yet implemented in winterbaume-ds",
            ),
            "VerifyTrust" => json_error_response(
                501,
                "NotImplementedError",
                "VerifyTrust is not yet implemented in winterbaume-ds",
            ),
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for DirectoryService"),
            ),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_directory(
        &self,
        state: &Arc<tokio::sync::RwLock<DsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_directory_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        if input.size.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Size'");
        }
        let name = input.name.as_str();
        let size = input.size.as_str();
        let password = if input.password.is_empty() {
            None
        } else {
            Some(input.password.as_str())
        };
        let description = input.description.as_deref();
        let short_name = input.short_name.as_deref();

        let vpc_settings = input.vpc_settings.as_ref().map(|vs| {
            let vpc_id: &str = vs.vpc_id.as_str();
            let subnet_ids: &[String] = vs.subnet_ids.as_slice();
            (vpc_id, subnet_ids)
        });

        let mut state = state.write().await;

        match state.create_directory(
            account_id,
            region,
            name,
            size,
            password,
            description,
            short_name,
            vpc_settings,
        ) {
            Ok(directory_id) => {
                wire::serialize_create_directory_response(&wire::CreateDirectoryResult {
                    directory_id: Some(directory_id),
                })
            }
            Err(e) => ds_error_response(&e),
        }
    }

    async fn handle_connect_directory(
        &self,
        state: &Arc<tokio::sync::RwLock<DsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_connect_directory_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        if input.size.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Size'");
        }
        let name = input.name.as_str();
        let size = input.size.as_str();
        let password = if input.password.is_empty() {
            None
        } else {
            Some(input.password.as_str())
        };
        let short_name = input.short_name.as_deref();
        let description = input.description.as_deref();
        let connect_settings = &input.connect_settings;

        if connect_settings.vpc_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'ConnectSettings.VpcId'",
            );
        }
        let vpc_id = connect_settings.vpc_id.as_str();
        let subnet_ids: &[String] = connect_settings.subnet_ids.as_slice();
        let customer_dns_ips: Vec<String> = connect_settings
            .customer_dns_ips
            .clone()
            .unwrap_or_default();

        if connect_settings.customer_user_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'ConnectSettings.CustomerUserName'",
            );
        }
        let customer_user_name = connect_settings.customer_user_name.as_str();

        let mut state = state.write().await;
        match state.connect_directory(
            account_id,
            region,
            name,
            size,
            password,
            short_name,
            description,
            vpc_id,
            subnet_ids,
            &customer_dns_ips,
            customer_user_name,
        ) {
            Ok(directory_id) => {
                wire::serialize_connect_directory_response(&wire::ConnectDirectoryResult {
                    directory_id: Some(directory_id),
                })
            }
            Err(e) => ds_error_response(&e),
        }
    }

    async fn handle_describe_directories(
        &self,
        state: &Arc<tokio::sync::RwLock<DsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_directories_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let directory_ids: Option<Vec<String>> = input.directory_ids;

        if let Some(ref ids) = directory_ids {
            if ids.is_empty() {
                return json_error_response(
                    400,
                    "InvalidParameterException",
                    "Invalid value for DirectoryIds: must not be empty",
                );
            }
        }

        let state = state.read().await;
        match state.describe_directories(directory_ids.as_deref()) {
            Ok(dirs) => {
                let entries: Vec<wire::DirectoryDescription> =
                    dirs.iter().map(|d| directory_to_wire(d)).collect();
                wire::serialize_describe_directories_response(&wire::DescribeDirectoriesResult {
                    directory_descriptions: Some(entries),
                    ..Default::default()
                })
            }
            Err(e) => ds_error_response(&e),
        }
    }

    async fn handle_delete_directory(
        &self,
        state: &Arc<tokio::sync::RwLock<DsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_directory_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.directory_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DirectoryId'");
        }
        let directory_id = input.directory_id.as_str();

        let mut state = state.write().await;
        match state.delete_directory(directory_id) {
            Ok(id) => wire::serialize_delete_directory_response(&wire::DeleteDirectoryResult {
                directory_id: Some(id),
            }),
            Err(e) => ds_error_response(&e),
        }
    }
}

// --- Utility functions ---

fn directory_to_wire(d: &crate::types::Directory) -> wire::DirectoryDescription {
    let vpc_settings = d
        .vpc_settings
        .as_ref()
        .map(|vpc| wire::DirectoryVpcSettingsDescription {
            vpc_id: Some(vpc.vpc_id.clone()),
            subnet_ids: Some(vpc.subnet_ids.clone()),
            security_group_id: Some(vpc.security_group_id.clone()),
            availability_zones: Some(vpc.availability_zones.clone()),
        });

    let connect_settings =
        d.connect_settings
            .as_ref()
            .map(|cs| wire::DirectoryConnectSettingsDescription {
                vpc_id: Some(cs.vpc_id.clone()),
                subnet_ids: Some(cs.subnet_ids.clone()),
                customer_user_name: Some(cs.customer_user_name.clone()),
                security_group_id: Some(cs.security_group_id.clone()),
                availability_zones: Some(cs.availability_zones.clone()),
                connect_ips: Some(cs.connect_ips.clone()),
                ..Default::default()
            });

    wire::DirectoryDescription {
        directory_id: Some(d.directory_id.clone()),
        name: Some(d.name.clone()),
        short_name: d.short_name.clone(),
        description: d.description.clone(),
        size: Some(d.size.clone()),
        r#type: Some(d.directory_type.as_str().to_string()),
        alias: Some(d.alias.clone()),
        access_url: Some(d.access_url.clone()),
        stage: Some(d.stage.as_str().to_string()),
        launch_time: Some(d.launch_time.timestamp() as f64),
        stage_last_updated_date_time: Some(d.stage_last_updated_date_time.timestamp() as f64),
        dns_ip_addrs: Some(d.dns_ip_addrs.clone()),
        sso_enabled: Some(d.ssoid_enabled),
        vpc_settings,
        connect_settings,
        ..Default::default()
    }
}

fn ds_error_response(err: &DsError) -> MockResponse {
    let (error_type, status) = match err {
        DsError::NameEmpty => ("ValidationException", 400),
        DsError::PasswordEmpty => ("ValidationException", 400),
        DsError::InvalidDirectorySize { .. } => ("InvalidParameterException", 400),
        DsError::DnsIpsEmpty => ("ValidationException", 400),
        DsError::DirectoryNotFound { .. } => ("EntityDoesNotExistException", 400),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use http::HeaderMap;

    use super::*;

    async fn make_request(target: &str, body: serde_json::Value) -> MockRequest {
        let mut headers = HeaderMap::new();
        headers.insert("x-amz-target", target.parse().unwrap());
        MockRequest {
            method: "POST".to_string(),
            uri: "https://ds.us-east-1.amazonaws.com/".to_string(),
            headers,
            body: Bytes::from(body.to_string()),
        }
    }

    #[tokio::test]
    async fn test_describe_directories_empty_array_returns_invalid_parameter_exception() {
        let service = DirectoryService::new();
        let req = make_request(
            "DirectoryService_20150416.DescribeDirectories",
            json!({ "DirectoryIds": [] }),
        )
        .await;
        let resp = service.dispatch(req).await;
        assert_eq!(resp.status, 400, "should return 400");
        let body: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
        assert_eq!(
            body["__type"].as_str(),
            Some("InvalidParameterException"),
            "error type should be InvalidParameterException, got: {body}"
        );
    }

    #[tokio::test]
    async fn test_describe_directories_null_ids_returns_all() {
        let service = DirectoryService::new();
        // No DirectoryIds key at all — should succeed and return empty list
        let req = make_request("DirectoryService_20150416.DescribeDirectories", json!({})).await;
        let resp = service.dispatch(req).await;
        assert_eq!(
            resp.status, 200,
            "should return 200 when DirectoryIds is absent"
        );
    }
}
