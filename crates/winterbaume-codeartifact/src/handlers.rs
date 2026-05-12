use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::model;
use crate::state::{CodeArtifactError, CodeArtifactState};
use crate::views::CodeArtifactStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct CodeArtifactService {
    pub(crate) state: Arc<BackendState<CodeArtifactState>>,
    pub(crate) notifier: StateChangeNotifier<CodeArtifactStateView>,
}

impl CodeArtifactService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CodeArtifactService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CodeArtifactService {
    fn service_name(&self) -> &str {
        "codeartifact"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://codeartifact\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl CodeArtifactService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        #[allow(clippy::result_large_err)]
        let parse_json_body = |body: &[u8]| -> Result<Value, MockResponse> {
            if body.is_empty() {
                return Ok(json!({}));
            }
            serde_json::from_slice(body)
                .map_err(|_| rest_json_error(400, "ValidationException", "Invalid JSON body"))
        };

        let response = match (method, path.as_str()) {
            // Domain operations — domain name is a query param
            ("POST", "/v1/domain") => {
                let domain_name = match extract_query_param(&request.uri, "domain") {
                    Some(n) => n,
                    None => {
                        return rest_json_error(
                            400,
                            "ValidationException",
                            "Missing 'domain' query param",
                        );
                    }
                };
                let body = match parse_json_body(&request.body) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                self.handle_create_domain(&state, &domain_name, account_id, &region, &body)
                    .await
            }
            ("GET", "/v1/domain") => {
                let domain_name = match extract_query_param(&request.uri, "domain") {
                    Some(n) => n,
                    None => {
                        return rest_json_error(
                            400,
                            "ValidationException",
                            "Missing 'domain' query param",
                        );
                    }
                };
                self.handle_describe_domain(&state, &domain_name).await
            }
            ("DELETE", "/v1/domain") => {
                let domain_name = match extract_query_param(&request.uri, "domain") {
                    Some(n) => n,
                    None => {
                        return rest_json_error(
                            400,
                            "ValidationException",
                            "Missing 'domain' query param",
                        );
                    }
                };
                self.handle_delete_domain(&state, &domain_name).await
            }
            ("POST", "/v1/domains") => self.handle_list_domains(&state).await,
            // Repository operations — domain and repository are query params
            ("POST", "/v1/repository") => {
                let domain_name = match extract_query_param(&request.uri, "domain") {
                    Some(n) => n,
                    None => {
                        return rest_json_error(
                            400,
                            "ValidationException",
                            "Missing 'domain' query param",
                        );
                    }
                };
                let repo_name = match extract_query_param(&request.uri, "repository") {
                    Some(n) => n,
                    None => {
                        return rest_json_error(
                            400,
                            "ValidationException",
                            "Missing 'repository' query param",
                        );
                    }
                };
                let body = match parse_json_body(&request.body) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                self.handle_create_repository(
                    &state,
                    &domain_name,
                    &repo_name,
                    account_id,
                    &region,
                    &body,
                )
                .await
            }
            ("GET", "/v1/repository") => {
                let domain_name = match extract_query_param(&request.uri, "domain") {
                    Some(n) => n,
                    None => {
                        return rest_json_error(
                            400,
                            "ValidationException",
                            "Missing 'domain' query param",
                        );
                    }
                };
                let repo_name = match extract_query_param(&request.uri, "repository") {
                    Some(n) => n,
                    None => {
                        return rest_json_error(
                            400,
                            "ValidationException",
                            "Missing 'repository' query param",
                        );
                    }
                };
                self.handle_describe_repository(&state, &domain_name, &repo_name)
                    .await
            }
            ("DELETE", "/v1/repository") => {
                let domain_name = match extract_query_param(&request.uri, "domain") {
                    Some(n) => n,
                    None => {
                        return rest_json_error(
                            400,
                            "ValidationException",
                            "Missing 'domain' query param",
                        );
                    }
                };
                let repo_name = match extract_query_param(&request.uri, "repository") {
                    Some(n) => n,
                    None => {
                        return rest_json_error(
                            400,
                            "ValidationException",
                            "Missing 'repository' query param",
                        );
                    }
                };
                self.handle_delete_repository(&state, &domain_name, &repo_name)
                    .await
            }
            ("POST", "/v1/repositories") => self.handle_list_repositories(&state).await,
            ("POST", "/v1/domain/repositories") => {
                let domain_name = match extract_query_param(&request.uri, "domain") {
                    Some(n) => n,
                    None => {
                        return rest_json_error(
                            400,
                            "ValidationException",
                            "Missing 'domain' query param",
                        );
                    }
                };
                self.handle_list_repositories_in_domain(&state, &domain_name)
                    .await
            }
            // --- Unimplemented operations ---
            // --- Unimplemented operations (auto-generated stubs) ---
            // POST /v1/repository/external-connection => AssociateExternalConnection (not implemented)
            // POST /v1/package/versions/copy => CopyPackageVersions (not implemented)
            // POST /v1/package-group => CreatePackageGroup (not implemented)
            // DELETE /v1/domain/permissions/policy => DeleteDomainPermissionsPolicy (not implemented)
            // DELETE /v1/package => DeletePackage (not implemented)
            // DELETE /v1/package-group => DeletePackageGroup (not implemented)
            // POST /v1/package/versions/delete => DeletePackageVersions (not implemented)
            // DELETE /v1/repository/permissions/policies => DeleteRepositoryPermissionsPolicy (not implemented)
            // GET /v1/package => DescribePackage (not implemented)
            // GET /v1/package-group => DescribePackageGroup (not implemented)
            // GET /v1/package/version => DescribePackageVersion (not implemented)
            // DELETE /v1/repository/external-connection => DisassociateExternalConnection (not implemented)
            // POST /v1/package/versions/dispose => DisposePackageVersions (not implemented)
            // GET /v1/get-associated-package-group => GetAssociatedPackageGroup (not implemented)
            // POST /v1/authorization-token => GetAuthorizationToken (not implemented)
            // GET /v1/domain/permissions/policy => GetDomainPermissionsPolicy (not implemented)
            // GET /v1/package/version/asset => GetPackageVersionAsset (not implemented)
            // GET /v1/package/version/readme => GetPackageVersionReadme (not implemented)
            // GET /v1/repository/endpoint => GetRepositoryEndpoint (not implemented)
            // GET /v1/repository/permissions/policy => GetRepositoryPermissionsPolicy (not implemented)
            // GET /v1/package-group-allowed-repositories => ListAllowedRepositoriesForGroup (not implemented)
            // GET /v1/list-associated-packages => ListAssociatedPackages (not implemented)
            // POST /v1/package-groups => ListPackageGroups (not implemented)
            // POST /v1/package/version/assets => ListPackageVersionAssets (not implemented)
            // POST /v1/package/version/dependencies => ListPackageVersionDependencies (not implemented)
            // POST /v1/package/versions => ListPackageVersions (not implemented)
            // POST /v1/packages => ListPackages (not implemented)
            // POST /v1/package-groups/sub-groups => ListSubPackageGroups (not implemented)
            // POST /v1/tags => ListTagsForResource (not implemented)
            // POST /v1/package/version/publish => PublishPackageVersion (not implemented)
            // PUT /v1/domain/permissions/policy => PutDomainPermissionsPolicy (not implemented)
            // POST /v1/package => PutPackageOriginConfiguration (not implemented)
            // PUT /v1/repository/permissions/policy => PutRepositoryPermissionsPolicy (not implemented)
            // POST /v1/tag => TagResource (not implemented)
            // POST /v1/untag => UntagResource (not implemented)
            // PUT /v1/package-group => UpdatePackageGroup (not implemented)
            // PUT /v1/package-group-origin-configuration => UpdatePackageGroupOriginConfiguration (not implemented)
            // POST /v1/package/versions/update_status => UpdatePackageVersionsStatus (not implemented)
            // PUT /v1/repository => UpdateRepository (not implemented)

            // 39 unimplemented operations above
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        let is_mutating = matches!(method, "POST" | "PUT" | "PATCH" | "DELETE")
            && !matches!(
                path.as_str(),
                "/v1/domains" | "/v1/repositories" | "/v1/domain/repositories"
            )
            && response.status / 100 == 2;
        if is_mutating {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeArtifactState>>,
        domain_name: &str,
        account_id: &str,
        region: &str,
        body: &Value,
    ) -> MockResponse {
        let encryption_key = body
            .get("encryptionKey")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let tags = extract_tags(body);
        let now = now_secs();
        let mut state = state.write().await;
        match state.create_domain(domain_name, account_id, region, encryption_key, tags, now) {
            Ok(domain) => wire::serialize_create_domain_response(&model::CreateDomainResult {
                domain: Some(domain_to_description(&domain)),
            }),
            Err(e) => codeartifact_error_response(&e),
        }
    }

    async fn handle_describe_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeArtifactState>>,
        domain_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_domain(domain_name) {
            Ok(domain) => wire::serialize_describe_domain_response(&model::DescribeDomainResult {
                domain: Some(domain_to_description(domain)),
            }),
            Err(e) => codeartifact_error_response(&e),
        }
    }

    async fn handle_delete_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeArtifactState>>,
        domain_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_domain(domain_name) {
            Ok(domain) => wire::serialize_delete_domain_response(&model::DeleteDomainResult {
                domain: Some(domain_to_description(&domain)),
            }),
            Err(e) => codeartifact_error_response(&e),
        }
    }

    async fn handle_list_domains(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeArtifactState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let domains: Vec<model::DomainSummary> = state
            .list_domains()
            .into_iter()
            .map(|d| model::DomainSummary {
                arn: Some(d.arn.clone()),
                name: Some(d.name.clone()),
                owner: Some(d.owner.clone()),
                status: Some(d.status.clone()),
                created_time: Some(d.created_time),
                encryption_key: d.encryption_key.clone(),
            })
            .collect();
        wire::serialize_list_domains_response(&model::ListDomainsResult {
            domains: Some(domains),
            ..Default::default()
        })
    }

    async fn handle_create_repository(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeArtifactState>>,
        domain_name: &str,
        repo_name: &str,
        account_id: &str,
        region: &str,
        body: &Value,
    ) -> MockResponse {
        let description = body
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let tags = extract_tags_codeartifact(body);
        let now = now_secs();
        let mut state = state.write().await;
        match state.create_repository(
            domain_name,
            repo_name,
            account_id,
            region,
            description,
            tags,
            now,
        ) {
            Ok(repo) => {
                wire::serialize_create_repository_response(&model::CreateRepositoryResult {
                    repository: Some(repo_to_description(&repo)),
                })
            }
            Err(e) => codeartifact_error_response(&e),
        }
    }

    async fn handle_describe_repository(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeArtifactState>>,
        domain_name: &str,
        repo_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_repository(domain_name, repo_name) {
            Ok(repo) => {
                wire::serialize_describe_repository_response(&model::DescribeRepositoryResult {
                    repository: Some(repo_to_description(repo)),
                })
            }
            Err(e) => codeartifact_error_response(&e),
        }
    }

    async fn handle_delete_repository(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeArtifactState>>,
        domain_name: &str,
        repo_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_repository(domain_name, repo_name) {
            Ok(repo) => {
                wire::serialize_delete_repository_response(&model::DeleteRepositoryResult {
                    repository: Some(repo_to_description(&repo)),
                })
            }
            Err(e) => codeartifact_error_response(&e),
        }
    }

    async fn handle_list_repositories(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeArtifactState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let repos: Vec<model::RepositorySummary> = state
            .list_repositories()
            .into_iter()
            .map(repo_to_summary)
            .collect();
        wire::serialize_list_repositories_response(&model::ListRepositoriesResult {
            repositories: Some(repos),
            ..Default::default()
        })
    }

    async fn handle_list_repositories_in_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeArtifactState>>,
        domain_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let repos: Vec<model::RepositorySummary> = state
            .list_repositories_in_domain(domain_name)
            .into_iter()
            .map(repo_to_summary)
            .collect();
        wire::serialize_list_repositories_in_domain_response(
            &model::ListRepositoriesInDomainResult {
                repositories: Some(repos),
                ..Default::default()
            },
        )
    }
}

fn domain_to_description(d: &crate::types::Domain) -> model::DomainDescription {
    model::DomainDescription {
        arn: Some(d.arn.clone()),
        name: Some(d.name.clone()),
        owner: Some(d.owner.clone()),
        status: Some(d.status.clone()),
        created_time: Some(d.created_time),
        encryption_key: d.encryption_key.clone(),
        ..Default::default()
    }
}

fn repo_to_description(r: &crate::types::Repository) -> model::RepositoryDescription {
    model::RepositoryDescription {
        arn: Some(r.arn.clone()),
        name: Some(r.name.clone()),
        domain_name: Some(r.domain_name.clone()),
        domain_owner: Some(r.domain_owner.clone()),
        description: r.description.clone(),
        created_time: Some(r.created_time),
        ..Default::default()
    }
}

fn repo_to_summary(r: &crate::types::Repository) -> model::RepositorySummary {
    model::RepositorySummary {
        arn: Some(r.arn.clone()),
        name: Some(r.name.clone()),
        domain_name: Some(r.domain_name.clone()),
        domain_owner: Some(r.domain_owner.clone()),
        description: r.description.clone(),
        created_time: Some(r.created_time),
        ..Default::default()
    }
}

fn extract_tags(body: &Value) -> std::collections::HashMap<String, String> {
    body.get("tags")
        .and_then(|v| v.as_object())
        .map(|m| {
            m.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect()
        })
        .unwrap_or_default()
}

fn extract_tags_codeartifact(body: &Value) -> std::collections::HashMap<String, String> {
    body.get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|t| {
                    let k = t.get("key").and_then(|v| v.as_str())?;
                    let v = t.get("value").and_then(|v| v.as_str())?;
                    Some((k.to_string(), v.to_string()))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn now_secs() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

fn codeartifact_error_response(e: &CodeArtifactError) -> MockResponse {
    let (status, error_type) = match e {
        CodeArtifactError::DomainAlreadyExists(_) => (409, "ConflictException"),
        CodeArtifactError::DomainNotFound(_) => (404, "ResourceNotFoundException"),
        CodeArtifactError::RepositoryAlreadyExists { .. } => (409, "ConflictException"),
        CodeArtifactError::RepositoryNotFound { .. } => (404, "ResourceNotFoundException"),
    };
    rest_json_error(status, error_type, &e.to_string())
}

fn extract_path(uri: &str) -> String {
    if let Some(idx) = uri.find("amazonaws.com") {
        let after_host = &uri[idx + "amazonaws.com".len()..];
        if let Some(q) = after_host.find('?') {
            after_host[..q].to_string()
        } else {
            after_host.to_string()
        }
    } else {
        uri.split('?').next().unwrap_or(uri).to_string()
    }
}

fn extract_query_param(uri: &str, param: &str) -> Option<String> {
    let query = uri.split('?').nth(1)?;
    for part in query.split('&') {
        if let Some((k, v)) = part.split_once('=')
            && k == param
        {
            return Some(percent_decode(v));
        }
    }
    None
}

fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                let hi = bytes.next().and_then(hex_val);
                let lo = bytes.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            b'+' => result.push(' '),
            _ => result.push(b as char),
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
