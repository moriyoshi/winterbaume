use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    json_error_response,
};

use crate::state::{CodeCommitError, CodeCommitState};
use crate::views::CodeCommitStateView;
use crate::wire;

pub struct CodeCommitService {
    pub(crate) state: Arc<BackendState<CodeCommitState>>,
    pub(crate) notifier: StateChangeNotifier<CodeCommitStateView>,
}

impl CodeCommitService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CodeCommitService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CodeCommitService {
    fn service_name(&self) -> &str {
        "codecommit"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://codecommit\..*\.amazonaws\.com",
            r"https?://codecommit\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl CodeCommitService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "CodeCommit_20150413.CreateRepository"
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

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        match action.as_str() {
            "CreateRepository" => {
                self.handle_create_repository(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetRepository" => self.handle_get_repository(&state, body_bytes).await,
            "DeleteRepository" => self.handle_delete_repository(&state, body_bytes).await,
            "ListRepositories" => self.handle_list_repositories(&state, body_bytes).await,
            "UpdateRepositoryDescription" => {
                self.handle_update_repository_description(&state, body_bytes)
                    .await
            }
            "UpdateRepositoryName" => {
                self.handle_update_repository_name(&state, body_bytes, account_id, &region)
                    .await
            }
            // Branches
            "CreateBranch" => self.handle_create_branch(&state, body_bytes).await,
            "GetBranch" => self.handle_get_branch(&state, body_bytes).await,
            "ListBranches" => self.handle_list_branches(&state, body_bytes).await,
            "DeleteBranch" => self.handle_delete_branch(&state, body_bytes).await,
            "UpdateDefaultBranch" => self.handle_update_default_branch(&state, body_bytes).await,
            // Commits & files
            "CreateCommit" => self.handle_create_commit(&state, body_bytes).await,
            "GetCommit" => self.handle_get_commit(&state, body_bytes).await,
            "GetDifferences" => self.handle_get_differences(&state, body_bytes).await,
            "GetFile" => self.handle_get_file(&state, body_bytes).await,
            "GetFolder" => self.handle_get_folder(&state, body_bytes).await,
            "PutFile" => self.handle_put_file(&state, body_bytes).await,
            "DeleteFile" => self.handle_delete_file(&state, body_bytes).await,
            // Pull Requests
            "CreatePullRequest" => self.handle_create_pull_request(&state, body_bytes).await,
            "GetPullRequest" => self.handle_get_pull_request(&state, body_bytes).await,
            "ListPullRequests" => self.handle_list_pull_requests(&state, body_bytes).await,
            "UpdatePullRequestStatus" => {
                self.handle_update_pull_request_status(&state, body_bytes)
                    .await
            }
            // Tags
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            // --- Unimplemented operations ---
            "AssociateApprovalRuleTemplateWithRepository" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateApprovalRuleTemplateWithRepository is not yet implemented in winterbaume-codecommit",
            ),
            "BatchAssociateApprovalRuleTemplateWithRepositories" => json_error_response(
                501,
                "NotImplementedError",
                "BatchAssociateApprovalRuleTemplateWithRepositories is not yet implemented in winterbaume-codecommit",
            ),
            "BatchDescribeMergeConflicts" => json_error_response(
                501,
                "NotImplementedError",
                "BatchDescribeMergeConflicts is not yet implemented in winterbaume-codecommit",
            ),
            "BatchDisassociateApprovalRuleTemplateFromRepositories" => json_error_response(
                501,
                "NotImplementedError",
                "BatchDisassociateApprovalRuleTemplateFromRepositories is not yet implemented in winterbaume-codecommit",
            ),
            "BatchGetCommits" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetCommits is not yet implemented in winterbaume-codecommit",
            ),
            "BatchGetRepositories" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetRepositories is not yet implemented in winterbaume-codecommit",
            ),
            "CreateApprovalRuleTemplate" => json_error_response(
                501,
                "NotImplementedError",
                "CreateApprovalRuleTemplate is not yet implemented in winterbaume-codecommit",
            ),
            "CreatePullRequestApprovalRule" => json_error_response(
                501,
                "NotImplementedError",
                "CreatePullRequestApprovalRule is not yet implemented in winterbaume-codecommit",
            ),
            "CreateUnreferencedMergeCommit" => json_error_response(
                501,
                "NotImplementedError",
                "CreateUnreferencedMergeCommit is not yet implemented in winterbaume-codecommit",
            ),
            "DeleteApprovalRuleTemplate" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteApprovalRuleTemplate is not yet implemented in winterbaume-codecommit",
            ),
            "DeleteCommentContent" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteCommentContent is not yet implemented in winterbaume-codecommit",
            ),
            "DeletePullRequestApprovalRule" => json_error_response(
                501,
                "NotImplementedError",
                "DeletePullRequestApprovalRule is not yet implemented in winterbaume-codecommit",
            ),
            "DescribeMergeConflicts" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeMergeConflicts is not yet implemented in winterbaume-codecommit",
            ),
            "DescribePullRequestEvents" => json_error_response(
                501,
                "NotImplementedError",
                "DescribePullRequestEvents is not yet implemented in winterbaume-codecommit",
            ),
            "DisassociateApprovalRuleTemplateFromRepository" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociateApprovalRuleTemplateFromRepository is not yet implemented in winterbaume-codecommit",
            ),
            "EvaluatePullRequestApprovalRules" => json_error_response(
                501,
                "NotImplementedError",
                "EvaluatePullRequestApprovalRules is not yet implemented in winterbaume-codecommit",
            ),
            "GetApprovalRuleTemplate" => json_error_response(
                501,
                "NotImplementedError",
                "GetApprovalRuleTemplate is not yet implemented in winterbaume-codecommit",
            ),
            "GetBlob" => json_error_response(
                501,
                "NotImplementedError",
                "GetBlob is not yet implemented in winterbaume-codecommit",
            ),
            "GetComment" => json_error_response(
                501,
                "NotImplementedError",
                "GetComment is not yet implemented in winterbaume-codecommit",
            ),
            "GetCommentReactions" => json_error_response(
                501,
                "NotImplementedError",
                "GetCommentReactions is not yet implemented in winterbaume-codecommit",
            ),
            "GetCommentsForComparedCommit" => json_error_response(
                501,
                "NotImplementedError",
                "GetCommentsForComparedCommit is not yet implemented in winterbaume-codecommit",
            ),
            "GetCommentsForPullRequest" => json_error_response(
                501,
                "NotImplementedError",
                "GetCommentsForPullRequest is not yet implemented in winterbaume-codecommit",
            ),
            "GetMergeCommit" => json_error_response(
                501,
                "NotImplementedError",
                "GetMergeCommit is not yet implemented in winterbaume-codecommit",
            ),
            "GetMergeConflicts" => json_error_response(
                501,
                "NotImplementedError",
                "GetMergeConflicts is not yet implemented in winterbaume-codecommit",
            ),
            "GetMergeOptions" => json_error_response(
                501,
                "NotImplementedError",
                "GetMergeOptions is not yet implemented in winterbaume-codecommit",
            ),
            "GetPullRequestApprovalStates" => json_error_response(
                501,
                "NotImplementedError",
                "GetPullRequestApprovalStates is not yet implemented in winterbaume-codecommit",
            ),
            "GetPullRequestOverrideState" => json_error_response(
                501,
                "NotImplementedError",
                "GetPullRequestOverrideState is not yet implemented in winterbaume-codecommit",
            ),
            "GetRepositoryTriggers" => json_error_response(
                501,
                "NotImplementedError",
                "GetRepositoryTriggers is not yet implemented in winterbaume-codecommit",
            ),
            "ListApprovalRuleTemplates" => json_error_response(
                501,
                "NotImplementedError",
                "ListApprovalRuleTemplates is not yet implemented in winterbaume-codecommit",
            ),
            "ListAssociatedApprovalRuleTemplatesForRepository" => json_error_response(
                501,
                "NotImplementedError",
                "ListAssociatedApprovalRuleTemplatesForRepository is not yet implemented in winterbaume-codecommit",
            ),
            "ListFileCommitHistory" => json_error_response(
                501,
                "NotImplementedError",
                "ListFileCommitHistory is not yet implemented in winterbaume-codecommit",
            ),
            "ListRepositoriesForApprovalRuleTemplate" => json_error_response(
                501,
                "NotImplementedError",
                "ListRepositoriesForApprovalRuleTemplate is not yet implemented in winterbaume-codecommit",
            ),
            "MergeBranchesByFastForward" => json_error_response(
                501,
                "NotImplementedError",
                "MergeBranchesByFastForward is not yet implemented in winterbaume-codecommit",
            ),
            "MergeBranchesBySquash" => json_error_response(
                501,
                "NotImplementedError",
                "MergeBranchesBySquash is not yet implemented in winterbaume-codecommit",
            ),
            "MergeBranchesByThreeWay" => json_error_response(
                501,
                "NotImplementedError",
                "MergeBranchesByThreeWay is not yet implemented in winterbaume-codecommit",
            ),
            "MergePullRequestByFastForward" => json_error_response(
                501,
                "NotImplementedError",
                "MergePullRequestByFastForward is not yet implemented in winterbaume-codecommit",
            ),
            "MergePullRequestBySquash" => json_error_response(
                501,
                "NotImplementedError",
                "MergePullRequestBySquash is not yet implemented in winterbaume-codecommit",
            ),
            "MergePullRequestByThreeWay" => json_error_response(
                501,
                "NotImplementedError",
                "MergePullRequestByThreeWay is not yet implemented in winterbaume-codecommit",
            ),
            "OverridePullRequestApprovalRules" => json_error_response(
                501,
                "NotImplementedError",
                "OverridePullRequestApprovalRules is not yet implemented in winterbaume-codecommit",
            ),
            "PostCommentForComparedCommit" => json_error_response(
                501,
                "NotImplementedError",
                "PostCommentForComparedCommit is not yet implemented in winterbaume-codecommit",
            ),
            "PostCommentForPullRequest" => json_error_response(
                501,
                "NotImplementedError",
                "PostCommentForPullRequest is not yet implemented in winterbaume-codecommit",
            ),
            "PostCommentReply" => json_error_response(
                501,
                "NotImplementedError",
                "PostCommentReply is not yet implemented in winterbaume-codecommit",
            ),
            "PutCommentReaction" => json_error_response(
                501,
                "NotImplementedError",
                "PutCommentReaction is not yet implemented in winterbaume-codecommit",
            ),
            "PutRepositoryTriggers" => json_error_response(
                501,
                "NotImplementedError",
                "PutRepositoryTriggers is not yet implemented in winterbaume-codecommit",
            ),
            "TestRepositoryTriggers" => json_error_response(
                501,
                "NotImplementedError",
                "TestRepositoryTriggers is not yet implemented in winterbaume-codecommit",
            ),
            "UpdateApprovalRuleTemplateContent" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateApprovalRuleTemplateContent is not yet implemented in winterbaume-codecommit",
            ),
            "UpdateApprovalRuleTemplateDescription" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateApprovalRuleTemplateDescription is not yet implemented in winterbaume-codecommit",
            ),
            "UpdateApprovalRuleTemplateName" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateApprovalRuleTemplateName is not yet implemented in winterbaume-codecommit",
            ),
            "UpdateComment" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateComment is not yet implemented in winterbaume-codecommit",
            ),
            "UpdatePullRequestApprovalRuleContent" => json_error_response(
                501,
                "NotImplementedError",
                "UpdatePullRequestApprovalRuleContent is not yet implemented in winterbaume-codecommit",
            ),
            "UpdatePullRequestApprovalState" => json_error_response(
                501,
                "NotImplementedError",
                "UpdatePullRequestApprovalState is not yet implemented in winterbaume-codecommit",
            ),
            "UpdatePullRequestDescription" => json_error_response(
                501,
                "NotImplementedError",
                "UpdatePullRequestDescription is not yet implemented in winterbaume-codecommit",
            ),
            "UpdatePullRequestTitle" => json_error_response(
                501,
                "NotImplementedError",
                "UpdatePullRequestTitle is not yet implemented in winterbaume-codecommit",
            ),
            "UpdateRepositoryEncryptionKey" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateRepositoryEncryptionKey is not yet implemented in winterbaume-codecommit",
            ),
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for CodeCommit"),
            ),
        }
    }

    async fn handle_create_repository(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_repository_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }
        let description = input.repository_description.as_deref().unwrap_or("");

        let mut state = state.write().await;
        match state.create_repository(&input.repository_name, description, account_id, region) {
            Ok(repo) => wire::serialize_create_repository_response(&wire::CreateRepositoryOutput {
                repository_metadata: Some(repo_to_wire(repo)),
            }),
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_get_repository(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_repository_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }

        let state = state.read().await;
        match state.get_repository(&input.repository_name) {
            Ok(repo) => wire::serialize_get_repository_response(&wire::GetRepositoryOutput {
                repository_metadata: Some(repo_to_wire(repo)),
            }),
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_delete_repository(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_repository_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }

        let mut state = state.write().await;
        let repo_id = state.delete_repository(&input.repository_name);
        let repo_id_opt = if repo_id.is_empty() {
            None
        } else {
            Some(repo_id)
        };
        wire::serialize_delete_repository_response(&wire::DeleteRepositoryOutput {
            repository_id: repo_id_opt,
        })
    }

    async fn handle_list_repositories(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_repositories_request(body) {
            return json_error_response(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let repos = state.list_repositories();
        wire::serialize_list_repositories_response(&wire::ListRepositoriesOutput {
            repositories: Some(
                repos
                    .iter()
                    .map(|r| wire::RepositoryNameIdPair {
                        repository_id: Some(r.repository_id.clone()),
                        repository_name: Some(r.repository_name.clone()),
                    })
                    .collect(),
            ),
            next_token: None,
        })
    }

    async fn handle_update_repository_description(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_repository_description_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }
        let description = input.repository_description.as_deref();

        let mut state = state.write().await;
        match state.update_repository_description(&input.repository_name, description) {
            Ok(()) => wire::serialize_update_repository_description_response(),
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_update_repository_name(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_repository_name_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.old_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Old repository name is required",
            );
        }
        if input.new_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "New repository name is required",
            );
        }

        let mut state = state.write().await;
        match state.update_repository_name(&input.old_name, &input.new_name, region, account_id) {
            Ok(()) => wire::serialize_update_repository_name_response(),
            Err(e) => codecommit_error_response(&e),
        }
    }

    // ---- Branches ----

    async fn handle_create_branch(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_branch_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }
        if input.branch_name.is_empty() {
            return json_error_response(
                400,
                "BranchNameRequiredException",
                "Branch name is required",
            );
        }
        if input.commit_id.is_empty() {
            return json_error_response(400, "CommitIdRequiredException", "Commit ID is required");
        }

        let mut state = state.write().await;
        match state.create_branch(&input.repository_name, &input.branch_name, &input.commit_id) {
            Ok(()) => wire::serialize_create_branch_response(),
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_get_branch(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_branch_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let repo_name = match input.repository_name.as_deref() {
            Some(n) if !n.is_empty() => n,
            _ => {
                return json_error_response(
                    400,
                    "RepositoryNameRequiredException",
                    "Repository name is required",
                );
            }
        };
        let branch_name = match input.branch_name.as_deref() {
            Some(n) if !n.is_empty() => n,
            _ => {
                return json_error_response(
                    400,
                    "BranchNameRequiredException",
                    "Branch name is required",
                );
            }
        };

        let state = state.read().await;
        match state.get_branch(repo_name, branch_name) {
            Ok(branch) => wire::serialize_get_branch_response(&wire::GetBranchOutput {
                branch: Some(wire::BranchInfo {
                    branch_name: Some(branch.branch_name.clone()),
                    commit_id: Some(branch.commit_id.clone()),
                }),
            }),
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_list_branches(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_branches_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }

        let state = state.read().await;
        match state.list_branches(&input.repository_name) {
            Ok(branches) => wire::serialize_list_branches_response(&wire::ListBranchesOutput {
                branches: Some(branches),
                next_token: None,
            }),
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_delete_branch(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_branch_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }
        if input.branch_name.is_empty() {
            return json_error_response(
                400,
                "BranchNameRequiredException",
                "Branch name is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_branch(&input.repository_name, &input.branch_name) {
            Ok(branch) => wire::serialize_delete_branch_response(&wire::DeleteBranchOutput {
                deleted_branch: Some(wire::BranchInfo {
                    branch_name: Some(branch.branch_name.clone()),
                    commit_id: Some(branch.commit_id.clone()),
                }),
            }),
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_update_default_branch(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_default_branch_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }
        if input.default_branch_name.is_empty() {
            return json_error_response(
                400,
                "BranchNameRequiredException",
                "Default branch name is required",
            );
        }

        let mut state = state.write().await;
        match state.update_default_branch(&input.repository_name, &input.default_branch_name) {
            Ok(()) => wire::serialize_update_default_branch_response(),
            Err(e) => codecommit_error_response(&e),
        }
    }

    // ---- Commits ----

    async fn handle_create_commit(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_commit_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }
        if input.branch_name.is_empty() {
            return json_error_response(
                400,
                "BranchNameRequiredException",
                "Branch name is required",
            );
        }
        let parent_commit_id = input.parent_commit_id.as_deref();
        let author_name = input.author_name.as_deref();
        let author_email = input.email.as_deref();
        let commit_message = input.commit_message.as_deref();

        // Collect put files: putFiles array of {filePath, fileContent (base64), fileMode}
        let put_files: Vec<(String, String)> = input
            .put_files
            .unwrap_or_default()
            .into_iter()
            .map(|item| {
                let mode = item.file_mode.unwrap_or_else(|| "NORMAL".to_string());
                (item.file_path, mode)
            })
            .collect();

        // Collect delete files: deleteFiles array of {filePath}
        let delete_files: Vec<String> = input
            .delete_files
            .unwrap_or_default()
            .into_iter()
            .map(|item| item.file_path)
            .collect();

        let mut state = state.write().await;
        match state.create_commit(
            &input.repository_name,
            &input.branch_name,
            parent_commit_id,
            author_name,
            author_email,
            commit_message,
            put_files,
            delete_files,
        ) {
            Ok(commit) => wire::serialize_create_commit_response(&wire::CreateCommitOutput {
                commit_id: Some(commit.commit_id.clone()),
                tree_id: Some(commit.tree_id.clone()),
                files_added: None,
                files_updated: None,
                files_deleted: None,
            }),
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_get_commit(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_commit_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }
        if input.commit_id.is_empty() {
            return json_error_response(400, "CommitIdRequiredException", "Commit ID is required");
        }

        let state = state.read().await;
        match state.get_commit(&input.repository_name, &input.commit_id) {
            Ok(commit) => wire::serialize_get_commit_response(&wire::GetCommitOutput {
                commit: Some(commit_to_wire(commit)),
            }),
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_get_differences(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_differences_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }
        if input.after_commit_specifier.is_empty() {
            return json_error_response(
                400,
                "CommitRequiredException",
                "afterCommitSpecifier is required",
            );
        }
        let before_commit = input.before_commit_specifier.as_deref();

        let state = state.read().await;
        match state.get_differences(
            &input.repository_name,
            &input.after_commit_specifier,
            before_commit,
        ) {
            Ok(diffs) => wire::serialize_get_differences_response(&wire::GetDifferencesOutput {
                differences: Some(
                    diffs
                        .iter()
                        .map(|(before, after, change_type)| wire::Difference {
                            before_blob: before.as_ref().map(|fe| wire::BlobMetadata {
                                blob_id: Some(fe.blob_id.clone()),
                                path: Some(fe.file_path.clone()),
                                mode: Some(fe.file_mode.clone()),
                            }),
                            after_blob: after.as_ref().map(|fe| wire::BlobMetadata {
                                blob_id: Some(fe.blob_id.clone()),
                                path: Some(fe.file_path.clone()),
                                mode: Some(fe.file_mode.clone()),
                            }),
                            change_type: Some(change_type.clone()),
                        })
                        .collect(),
                ),
                next_token: None,
            }),
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_get_file(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_file_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }
        if input.file_path.is_empty() {
            return json_error_response(400, "FilePathRequiredException", "File path is required");
        }
        let commit_specifier = input.commit_specifier.as_deref();

        let state = state.read().await;
        match state.get_file(&input.repository_name, commit_specifier, &input.file_path) {
            Ok((commit, file)) => wire::serialize_get_file_response(&wire::GetFileOutput {
                commit_id: Some(commit.commit_id.clone()),
                blob_id: Some(file.blob_id.clone()),
                file_path: Some(file.file_path.clone()),
                file_mode: Some(file.file_mode.clone()),
                file_size: Some(0i64),
                file_content: Some(String::new()),
            }),
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_get_folder(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_folder_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }
        let folder_path = if input.folder_path.is_empty() {
            "/"
        } else {
            input.folder_path.as_str()
        };
        let commit_specifier = input.commit_specifier.as_deref();

        let state = state.read().await;
        match state.get_folder(&input.repository_name, commit_specifier, folder_path) {
            Ok((commit_id, files, sub_folders)) => {
                wire::serialize_get_folder_response(&wire::GetFolderOutput {
                    commit_id: Some(commit_id),
                    folder_path: Some(folder_path.to_string()),
                    tree_id: None,
                    sub_folders: Some(
                        sub_folders
                            .iter()
                            .map(|sf| wire::Folder {
                                absolute_path: Some(format!("{folder_path}/{sf}")),
                                relative_path: Some(sf.clone()),
                                tree_id: None,
                            })
                            .collect(),
                    ),
                    files: Some(
                        files
                            .iter()
                            .map(|fe| wire::File {
                                absolute_path: Some(fe.file_path.clone()),
                                blob_id: Some(fe.blob_id.clone()),
                                file_mode: Some(fe.file_mode.clone()),
                                relative_path: Some(
                                    fe.file_path
                                        .strip_prefix(folder_path.trim_start_matches('/'))
                                        .unwrap_or(&fe.file_path)
                                        .to_string(),
                                ),
                            })
                            .collect(),
                    ),
                    sub_modules: None,
                    symbolic_links: None,
                })
            }
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_put_file(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_file_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }
        if input.branch_name.is_empty() {
            return json_error_response(
                400,
                "BranchNameRequiredException",
                "Branch name is required",
            );
        }
        if input.file_path.is_empty() {
            return json_error_response(400, "FilePathRequiredException", "File path is required");
        }
        if input.file_content.is_empty() {
            return json_error_response(
                400,
                "FileContentRequiredException",
                "File content is required",
            );
        }
        let parent_commit_id = match input.parent_commit_id.as_deref() {
            Some(c) if !c.is_empty() => c,
            _ => {
                return json_error_response(
                    400,
                    "ParentCommitIdRequiredException",
                    "Parent commit ID is required",
                );
            }
        };
        let file_mode = input.file_mode.as_deref();
        let author_name = input.name.as_deref();
        let author_email = input.email.as_deref();
        let commit_message = input.commit_message.as_deref();

        let mut state = state.write().await;
        match state.put_file(
            &input.repository_name,
            &input.branch_name,
            parent_commit_id,
            &input.file_path,
            file_mode,
            author_name,
            author_email,
            commit_message,
        ) {
            Ok(commit) => wire::serialize_put_file_response(&wire::PutFileOutput {
                commit_id: Some(commit.commit_id.clone()),
                tree_id: Some(commit.tree_id.clone()),
                blob_id: None,
            }),
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_delete_file(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_file_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }
        if input.branch_name.is_empty() {
            return json_error_response(
                400,
                "BranchNameRequiredException",
                "Branch name is required",
            );
        }
        if input.file_path.is_empty() {
            return json_error_response(400, "FilePathRequiredException", "File path is required");
        }
        if input.parent_commit_id.is_empty() {
            return json_error_response(
                400,
                "ParentCommitIdRequiredException",
                "Parent commit ID is required",
            );
        }
        let author_name = input.name.as_deref();
        let author_email = input.email.as_deref();
        let commit_message = input.commit_message.as_deref();

        let mut state = state.write().await;
        match state.delete_file(
            &input.repository_name,
            &input.branch_name,
            &input.parent_commit_id,
            &input.file_path,
            author_name,
            author_email,
            commit_message,
        ) {
            Ok(commit) => wire::serialize_delete_file_response(&wire::DeleteFileOutput {
                commit_id: Some(commit.commit_id.clone()),
                tree_id: Some(commit.tree_id.clone()),
                blob_id: None,
                file_path: Some(input.file_path.clone()),
            }),
            Err(e) => codecommit_error_response(&e),
        }
    }

    // ---- Pull Requests ----

    async fn handle_create_pull_request(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_pull_request_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.title.is_empty() {
            return json_error_response(
                400,
                "TitleRequiredException",
                "Pull request title is required",
            );
        }
        let description = input.description.as_deref().unwrap_or("");

        if input.targets.is_empty() {
            return json_error_response(
                400,
                "TargetsRequiredException",
                "At least one target is required",
            );
        }

        let target = &input.targets[0];
        if target.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required in target",
            );
        }
        if target.source_reference.is_empty() {
            return json_error_response(
                400,
                "SourceBranchRequiredException",
                "Source reference is required",
            );
        }
        let dest_ref = target
            .destination_reference
            .as_deref()
            .unwrap_or("refs/heads/main");

        let mut state = state.write().await;
        match state.create_pull_request(
            &input.title,
            description,
            &target.repository_name,
            &target.source_reference,
            dest_ref,
        ) {
            Ok(pr) => {
                wire::serialize_create_pull_request_response(&wire::CreatePullRequestOutput {
                    pull_request: Some(pr_to_wire(&pr)),
                })
            }
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_get_pull_request(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_pull_request_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pull_request_id.is_empty() {
            return json_error_response(
                400,
                "PullRequestIdRequiredException",
                "Pull request ID is required",
            );
        }

        let state = state.read().await;
        match state.get_pull_request(&input.pull_request_id) {
            Ok(pr) => wire::serialize_get_pull_request_response(&wire::GetPullRequestOutput {
                pull_request: Some(pr_to_wire(pr)),
            }),
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_list_pull_requests(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_pull_requests_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "RepositoryNameRequiredException",
                "Repository name is required",
            );
        }
        let status = input.pull_request_status.as_deref();

        let state = state.read().await;
        let pr_ids = state.list_pull_requests(&input.repository_name, status);
        wire::serialize_list_pull_requests_response(&wire::ListPullRequestsOutput {
            pull_request_ids: Some(pr_ids),
            next_token: None,
        })
    }

    async fn handle_update_pull_request_status(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_pull_request_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pull_request_id.is_empty() {
            return json_error_response(
                400,
                "PullRequestIdRequiredException",
                "Pull request ID is required",
            );
        }
        if input.pull_request_status.is_empty() {
            return json_error_response(
                400,
                "PullRequestStatusRequiredException",
                "Pull request status is required",
            );
        }

        let mut state = state.write().await;
        match state.update_pull_request_status(&input.pull_request_id, &input.pull_request_status) {
            Ok(pr) => wire::serialize_update_pull_request_status_response(
                &wire::UpdatePullRequestStatusOutput {
                    pull_request: Some(pr_to_wire(&pr)),
                },
            ),
            Err(e) => codecommit_error_response(&e),
        }
    }

    // ---- Tags ----

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "ResourceArnRequiredException",
                "Resource ARN is required",
            );
        }

        // Find repo name from ARN (last segment)
        let repo_name = input.resource_arn.split(':').next_back().unwrap_or("");

        let mut state = state.write().await;
        match state.tag_resource(repo_name, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(),
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "ResourceArnRequiredException",
                "Resource ARN is required",
            );
        }

        let repo_name = input.resource_arn.split(':').next_back().unwrap_or("");

        let mut state = state.write().await;
        match state.untag_resource(repo_name, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(),
            Err(e) => codecommit_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeCommitState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "ResourceArnRequiredException",
                "Resource ARN is required",
            );
        }

        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_arn) {
            Ok(tags) => {
                wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceOutput {
                    tags: Some(tags),
                    next_token: None,
                })
            }
            Err(e) => codecommit_error_response(&e),
        }
    }
}

fn repo_to_wire(repo: &crate::types::Repository) -> wire::RepositoryMetadata {
    wire::RepositoryMetadata {
        account_id: Some(repo.account_id.clone()),
        repository_id: Some(repo.repository_id.clone()),
        repository_name: Some(repo.repository_name.clone()),
        repository_description: Some(repo.description.clone()),
        arn: Some(repo.arn.clone()),
        clone_url_http: Some(repo.clone_url_http.clone()),
        clone_url_ssh: Some(repo.clone_url_ssh.clone()),
        creation_date: Some(repo.creation_date.timestamp() as f64),
        last_modified_date: Some(repo.last_modified_date.timestamp() as f64),
        default_branch: repo.default_branch.clone(),
        ..Default::default()
    }
}

fn commit_to_wire(commit: &crate::types::CommitRecord) -> wire::Commit {
    wire::Commit {
        commit_id: Some(commit.commit_id.clone()),
        tree_id: Some(commit.tree_id.clone()),
        parents: Some(commit.parent_ids.clone()),
        message: Some(commit.message.clone()),
        author: Some(wire::UserInfo {
            name: Some(commit.author_name.clone()),
            email: Some(commit.author_email.clone()),
            date: Some(commit.date.to_rfc3339()),
        }),
        committer: Some(wire::UserInfo {
            name: Some(commit.author_name.clone()),
            email: Some(commit.author_email.clone()),
            date: Some(commit.date.to_rfc3339()),
        }),
        additional_data: None,
    }
}

fn pr_to_wire(pr: &crate::types::PullRequestRecord) -> wire::PullRequest {
    wire::PullRequest {
        pull_request_id: Some(pr.pull_request_id.clone()),
        title: Some(pr.title.clone()),
        description: Some(pr.description.clone()),
        pull_request_status: Some(pr.status.clone()),
        creation_date: Some(pr.creation_date.timestamp() as f64),
        last_activity_date: Some(pr.last_activity_date.timestamp() as f64),
        author_arn: Some(pr.author_arn.clone()),
        revision_id: None,
        client_request_token: None,
        approval_rules: None,
        pull_request_targets: Some(vec![wire::PullRequestTarget {
            repository_name: Some(pr.repository_name.clone()),
            source_reference: Some(pr.source_reference.clone()),
            destination_reference: Some(pr.destination_reference.clone()),
            source_commit: Some(pr.source_commit.clone()),
            destination_commit: Some(pr.destination_commit.clone()),
            merge_base: None,
            merge_metadata: Some(wire::MergeMetadata {
                is_merged: Some(pr.status == "CLOSED"),
                merge_commit_id: None,
                merge_option: None,
                merged_by: None,
            }),
        }]),
    }
}

fn codecommit_error_response(err: &CodeCommitError) -> MockResponse {
    let (status, error_type) = match err {
        CodeCommitError::RepositoryAlreadyExists { .. } => {
            (400u16, "RepositoryNameExistsException")
        }
        CodeCommitError::RepositoryNameTaken { .. } => (400, "RepositoryNameExistsException"),
        CodeCommitError::RepositoryDoesNotExist { .. } => (400, "RepositoryDoesNotExistException"),
        CodeCommitError::RepositoryDoesNotExistByArn { .. } => {
            (400, "RepositoryDoesNotExistException")
        }
        CodeCommitError::BranchAlreadyExists { .. } => (400, "BranchNameExistsException"),
        CodeCommitError::BranchDoesNotExist { .. } => (400, "BranchDoesNotExistException"),
        CodeCommitError::BranchNotFound { .. } => (400, "BranchDoesNotExistException"),
        CodeCommitError::DefaultBranchCannotBeDeleted => {
            (400, "DefaultBranchCannotBeDeletedException")
        }
        CodeCommitError::CommitDoesNotExist { .. } => (400, "CommitDoesNotExistException"),
        CodeCommitError::SpecifierDoesNotResolve { .. } => (400, "CommitDoesNotExistException"),
        CodeCommitError::FileDoesNotExist { .. } => (400, "FileDoesNotExistException"),
        CodeCommitError::PullRequestDoesNotExist { .. } => {
            (400, "PullRequestDoesNotExistException")
        }
        CodeCommitError::RepositoryEmpty => (400, "RepositoryEmptyException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}
