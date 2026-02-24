//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-codecommit

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize void response for awsJson protocol.
pub fn serialize_associate_approval_rule_template_with_repository_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_associate_approval_rule_template_with_repositories_response(
    result: &BatchAssociateApprovalRuleTemplateWithRepositoriesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_describe_merge_conflicts_response(
    result: &BatchDescribeMergeConflictsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_disassociate_approval_rule_template_from_repositories_response(
    result: &BatchDisassociateApprovalRuleTemplateFromRepositoriesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_commits_response(result: &BatchGetCommitsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_repositories_response(
    result: &BatchGetRepositoriesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_approval_rule_template_response(
    result: &CreateApprovalRuleTemplateOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_create_branch_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_commit_response(result: &CreateCommitOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_pull_request_response(result: &CreatePullRequestOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_pull_request_approval_rule_response(
    result: &CreatePullRequestApprovalRuleOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_repository_response(result: &CreateRepositoryOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_unreferenced_merge_commit_response(
    result: &CreateUnreferencedMergeCommitOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_approval_rule_template_response(
    result: &DeleteApprovalRuleTemplateOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_branch_response(result: &DeleteBranchOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_comment_content_response(
    result: &DeleteCommentContentOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_file_response(result: &DeleteFileOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_pull_request_approval_rule_response(
    result: &DeletePullRequestApprovalRuleOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_repository_response(result: &DeleteRepositoryOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_merge_conflicts_response(
    result: &DescribeMergeConflictsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_pull_request_events_response(
    result: &DescribePullRequestEventsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_disassociate_approval_rule_template_from_repository_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_evaluate_pull_request_approval_rules_response(
    result: &EvaluatePullRequestApprovalRulesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_approval_rule_template_response(
    result: &GetApprovalRuleTemplateOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_blob_response(result: &GetBlobOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_branch_response(result: &GetBranchOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_comment_response(result: &GetCommentOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_comment_reactions_response(
    result: &GetCommentReactionsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_comments_for_compared_commit_response(
    result: &GetCommentsForComparedCommitOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_comments_for_pull_request_response(
    result: &GetCommentsForPullRequestOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_commit_response(result: &GetCommitOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_differences_response(result: &GetDifferencesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_file_response(result: &GetFileOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_folder_response(result: &GetFolderOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_merge_commit_response(result: &GetMergeCommitOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_merge_conflicts_response(result: &GetMergeConflictsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_merge_options_response(result: &GetMergeOptionsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_pull_request_response(result: &GetPullRequestOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_pull_request_approval_states_response(
    result: &GetPullRequestApprovalStatesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_pull_request_override_state_response(
    result: &GetPullRequestOverrideStateOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_repository_response(result: &GetRepositoryOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_repository_triggers_response(
    result: &GetRepositoryTriggersOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_approval_rule_templates_response(
    result: &ListApprovalRuleTemplatesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_associated_approval_rule_templates_for_repository_response(
    result: &ListAssociatedApprovalRuleTemplatesForRepositoryOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_branches_response(result: &ListBranchesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_file_commit_history_response(
    result: &ListFileCommitHistoryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_pull_requests_response(result: &ListPullRequestsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_repositories_response(result: &ListRepositoriesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_repositories_for_approval_rule_template_response(
    result: &ListRepositoriesForApprovalRuleTemplateOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_merge_branches_by_fast_forward_response(
    result: &MergeBranchesByFastForwardOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_merge_branches_by_squash_response(
    result: &MergeBranchesBySquashOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_merge_branches_by_three_way_response(
    result: &MergeBranchesByThreeWayOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_merge_pull_request_by_fast_forward_response(
    result: &MergePullRequestByFastForwardOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_merge_pull_request_by_squash_response(
    result: &MergePullRequestBySquashOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_merge_pull_request_by_three_way_response(
    result: &MergePullRequestByThreeWayOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_override_pull_request_approval_rules_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_post_comment_for_compared_commit_response(
    result: &PostCommentForComparedCommitOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_post_comment_for_pull_request_response(
    result: &PostCommentForPullRequestOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_post_comment_reply_response(result: &PostCommentReplyOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_comment_reaction_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_file_response(result: &PutFileOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_repository_triggers_response(
    result: &PutRepositoryTriggersOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_test_repository_triggers_response(
    result: &TestRepositoryTriggersOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_approval_rule_template_content_response(
    result: &UpdateApprovalRuleTemplateContentOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_approval_rule_template_description_response(
    result: &UpdateApprovalRuleTemplateDescriptionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_approval_rule_template_name_response(
    result: &UpdateApprovalRuleTemplateNameOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_comment_response(result: &UpdateCommentOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_default_branch_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_pull_request_approval_rule_content_response(
    result: &UpdatePullRequestApprovalRuleContentOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_pull_request_approval_state_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_pull_request_description_response(
    result: &UpdatePullRequestDescriptionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_pull_request_status_response(
    result: &UpdatePullRequestStatusOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_pull_request_title_response(
    result: &UpdatePullRequestTitleOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_repository_description_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_repository_encryption_key_response(
    result: &UpdateRepositoryEncryptionKeyOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_repository_name_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_approval_rule_template_with_repository_request(
    body: &[u8],
) -> Result<AssociateApprovalRuleTemplateWithRepositoryInput, String> {
    if body.is_empty() {
        return Ok(AssociateApprovalRuleTemplateWithRepositoryInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize AssociateApprovalRuleTemplateWithRepository request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_associate_approval_rule_template_with_repositories_request(
    body: &[u8],
) -> Result<BatchAssociateApprovalRuleTemplateWithRepositoriesInput, String> {
    if body.is_empty() {
        return Ok(BatchAssociateApprovalRuleTemplateWithRepositoriesInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!(
            "Failed to deserialize BatchAssociateApprovalRuleTemplateWithRepositories request: {e}"
        )
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_describe_merge_conflicts_request(
    body: &[u8],
) -> Result<BatchDescribeMergeConflictsInput, String> {
    if body.is_empty() {
        return Ok(BatchDescribeMergeConflictsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDescribeMergeConflicts request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_disassociate_approval_rule_template_from_repositories_request(
    body: &[u8],
) -> Result<BatchDisassociateApprovalRuleTemplateFromRepositoriesInput, String> {
    if body.is_empty() {
        return Ok(BatchDisassociateApprovalRuleTemplateFromRepositoriesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDisassociateApprovalRuleTemplateFromRepositories request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_commits_request(body: &[u8]) -> Result<BatchGetCommitsInput, String> {
    if body.is_empty() {
        return Ok(BatchGetCommitsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetCommits request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_repositories_request(
    body: &[u8],
) -> Result<BatchGetRepositoriesInput, String> {
    if body.is_empty() {
        return Ok(BatchGetRepositoriesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetRepositories request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_approval_rule_template_request(
    body: &[u8],
) -> Result<CreateApprovalRuleTemplateInput, String> {
    if body.is_empty() {
        return Ok(CreateApprovalRuleTemplateInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateApprovalRuleTemplate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_branch_request(body: &[u8]) -> Result<CreateBranchInput, String> {
    if body.is_empty() {
        return Ok(CreateBranchInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateBranch request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_commit_request(body: &[u8]) -> Result<CreateCommitInput, String> {
    if body.is_empty() {
        return Ok(CreateCommitInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCommit request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_pull_request_request(
    body: &[u8],
) -> Result<CreatePullRequestInput, String> {
    if body.is_empty() {
        return Ok(CreatePullRequestInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePullRequest request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_pull_request_approval_rule_request(
    body: &[u8],
) -> Result<CreatePullRequestApprovalRuleInput, String> {
    if body.is_empty() {
        return Ok(CreatePullRequestApprovalRuleInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePullRequestApprovalRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_repository_request(body: &[u8]) -> Result<CreateRepositoryInput, String> {
    if body.is_empty() {
        return Ok(CreateRepositoryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateRepository request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_unreferenced_merge_commit_request(
    body: &[u8],
) -> Result<CreateUnreferencedMergeCommitInput, String> {
    if body.is_empty() {
        return Ok(CreateUnreferencedMergeCommitInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateUnreferencedMergeCommit request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_approval_rule_template_request(
    body: &[u8],
) -> Result<DeleteApprovalRuleTemplateInput, String> {
    if body.is_empty() {
        return Ok(DeleteApprovalRuleTemplateInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteApprovalRuleTemplate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_branch_request(body: &[u8]) -> Result<DeleteBranchInput, String> {
    if body.is_empty() {
        return Ok(DeleteBranchInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteBranch request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_comment_content_request(
    body: &[u8],
) -> Result<DeleteCommentContentInput, String> {
    if body.is_empty() {
        return Ok(DeleteCommentContentInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCommentContent request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_file_request(body: &[u8]) -> Result<DeleteFileInput, String> {
    if body.is_empty() {
        return Ok(DeleteFileInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteFile request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_pull_request_approval_rule_request(
    body: &[u8],
) -> Result<DeletePullRequestApprovalRuleInput, String> {
    if body.is_empty() {
        return Ok(DeletePullRequestApprovalRuleInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePullRequestApprovalRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_repository_request(body: &[u8]) -> Result<DeleteRepositoryInput, String> {
    if body.is_empty() {
        return Ok(DeleteRepositoryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteRepository request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_merge_conflicts_request(
    body: &[u8],
) -> Result<DescribeMergeConflictsInput, String> {
    if body.is_empty() {
        return Ok(DescribeMergeConflictsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMergeConflicts request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_pull_request_events_request(
    body: &[u8],
) -> Result<DescribePullRequestEventsInput, String> {
    if body.is_empty() {
        return Ok(DescribePullRequestEventsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePullRequestEvents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_approval_rule_template_from_repository_request(
    body: &[u8],
) -> Result<DisassociateApprovalRuleTemplateFromRepositoryInput, String> {
    if body.is_empty() {
        return Ok(DisassociateApprovalRuleTemplateFromRepositoryInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DisassociateApprovalRuleTemplateFromRepository request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_evaluate_pull_request_approval_rules_request(
    body: &[u8],
) -> Result<EvaluatePullRequestApprovalRulesInput, String> {
    if body.is_empty() {
        return Ok(EvaluatePullRequestApprovalRulesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EvaluatePullRequestApprovalRules request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_approval_rule_template_request(
    body: &[u8],
) -> Result<GetApprovalRuleTemplateInput, String> {
    if body.is_empty() {
        return Ok(GetApprovalRuleTemplateInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetApprovalRuleTemplate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_blob_request(body: &[u8]) -> Result<GetBlobInput, String> {
    if body.is_empty() {
        return Ok(GetBlobInput::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetBlob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_branch_request(body: &[u8]) -> Result<GetBranchInput, String> {
    if body.is_empty() {
        return Ok(GetBranchInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetBranch request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_comment_request(body: &[u8]) -> Result<GetCommentInput, String> {
    if body.is_empty() {
        return Ok(GetCommentInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetComment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_comment_reactions_request(
    body: &[u8],
) -> Result<GetCommentReactionsInput, String> {
    if body.is_empty() {
        return Ok(GetCommentReactionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCommentReactions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_comments_for_compared_commit_request(
    body: &[u8],
) -> Result<GetCommentsForComparedCommitInput, String> {
    if body.is_empty() {
        return Ok(GetCommentsForComparedCommitInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCommentsForComparedCommit request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_comments_for_pull_request_request(
    body: &[u8],
) -> Result<GetCommentsForPullRequestInput, String> {
    if body.is_empty() {
        return Ok(GetCommentsForPullRequestInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCommentsForPullRequest request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_commit_request(body: &[u8]) -> Result<GetCommitInput, String> {
    if body.is_empty() {
        return Ok(GetCommitInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCommit request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_differences_request(body: &[u8]) -> Result<GetDifferencesInput, String> {
    if body.is_empty() {
        return Ok(GetDifferencesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDifferences request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_file_request(body: &[u8]) -> Result<GetFileInput, String> {
    if body.is_empty() {
        return Ok(GetFileInput::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetFile request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_folder_request(body: &[u8]) -> Result<GetFolderInput, String> {
    if body.is_empty() {
        return Ok(GetFolderInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetFolder request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_merge_commit_request(body: &[u8]) -> Result<GetMergeCommitInput, String> {
    if body.is_empty() {
        return Ok(GetMergeCommitInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMergeCommit request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_merge_conflicts_request(
    body: &[u8],
) -> Result<GetMergeConflictsInput, String> {
    if body.is_empty() {
        return Ok(GetMergeConflictsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMergeConflicts request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_merge_options_request(body: &[u8]) -> Result<GetMergeOptionsInput, String> {
    if body.is_empty() {
        return Ok(GetMergeOptionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMergeOptions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_pull_request_request(body: &[u8]) -> Result<GetPullRequestInput, String> {
    if body.is_empty() {
        return Ok(GetPullRequestInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPullRequest request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_pull_request_approval_states_request(
    body: &[u8],
) -> Result<GetPullRequestApprovalStatesInput, String> {
    if body.is_empty() {
        return Ok(GetPullRequestApprovalStatesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPullRequestApprovalStates request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_pull_request_override_state_request(
    body: &[u8],
) -> Result<GetPullRequestOverrideStateInput, String> {
    if body.is_empty() {
        return Ok(GetPullRequestOverrideStateInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPullRequestOverrideState request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_repository_request(body: &[u8]) -> Result<GetRepositoryInput, String> {
    if body.is_empty() {
        return Ok(GetRepositoryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetRepository request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_repository_triggers_request(
    body: &[u8],
) -> Result<GetRepositoryTriggersInput, String> {
    if body.is_empty() {
        return Ok(GetRepositoryTriggersInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetRepositoryTriggers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_approval_rule_templates_request(
    body: &[u8],
) -> Result<ListApprovalRuleTemplatesInput, String> {
    if body.is_empty() {
        return Ok(ListApprovalRuleTemplatesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListApprovalRuleTemplates request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_associated_approval_rule_templates_for_repository_request(
    body: &[u8],
) -> Result<ListAssociatedApprovalRuleTemplatesForRepositoryInput, String> {
    if body.is_empty() {
        return Ok(ListAssociatedApprovalRuleTemplatesForRepositoryInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!(
            "Failed to deserialize ListAssociatedApprovalRuleTemplatesForRepository request: {e}"
        )
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_branches_request(body: &[u8]) -> Result<ListBranchesInput, String> {
    if body.is_empty() {
        return Ok(ListBranchesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListBranches request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_file_commit_history_request(
    body: &[u8],
) -> Result<ListFileCommitHistoryRequest, String> {
    if body.is_empty() {
        return Ok(ListFileCommitHistoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFileCommitHistory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_pull_requests_request(
    body: &[u8],
) -> Result<ListPullRequestsInput, String> {
    if body.is_empty() {
        return Ok(ListPullRequestsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPullRequests request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_repositories_request(body: &[u8]) -> Result<ListRepositoriesInput, String> {
    if body.is_empty() {
        return Ok(ListRepositoriesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListRepositories request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_repositories_for_approval_rule_template_request(
    body: &[u8],
) -> Result<ListRepositoriesForApprovalRuleTemplateInput, String> {
    if body.is_empty() {
        return Ok(ListRepositoriesForApprovalRuleTemplateInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListRepositoriesForApprovalRuleTemplate request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceInput, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_merge_branches_by_fast_forward_request(
    body: &[u8],
) -> Result<MergeBranchesByFastForwardInput, String> {
    if body.is_empty() {
        return Ok(MergeBranchesByFastForwardInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize MergeBranchesByFastForward request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_merge_branches_by_squash_request(
    body: &[u8],
) -> Result<MergeBranchesBySquashInput, String> {
    if body.is_empty() {
        return Ok(MergeBranchesBySquashInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize MergeBranchesBySquash request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_merge_branches_by_three_way_request(
    body: &[u8],
) -> Result<MergeBranchesByThreeWayInput, String> {
    if body.is_empty() {
        return Ok(MergeBranchesByThreeWayInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize MergeBranchesByThreeWay request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_merge_pull_request_by_fast_forward_request(
    body: &[u8],
) -> Result<MergePullRequestByFastForwardInput, String> {
    if body.is_empty() {
        return Ok(MergePullRequestByFastForwardInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize MergePullRequestByFastForward request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_merge_pull_request_by_squash_request(
    body: &[u8],
) -> Result<MergePullRequestBySquashInput, String> {
    if body.is_empty() {
        return Ok(MergePullRequestBySquashInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize MergePullRequestBySquash request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_merge_pull_request_by_three_way_request(
    body: &[u8],
) -> Result<MergePullRequestByThreeWayInput, String> {
    if body.is_empty() {
        return Ok(MergePullRequestByThreeWayInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize MergePullRequestByThreeWay request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_override_pull_request_approval_rules_request(
    body: &[u8],
) -> Result<OverridePullRequestApprovalRulesInput, String> {
    if body.is_empty() {
        return Ok(OverridePullRequestApprovalRulesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize OverridePullRequestApprovalRules request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_post_comment_for_compared_commit_request(
    body: &[u8],
) -> Result<PostCommentForComparedCommitInput, String> {
    if body.is_empty() {
        return Ok(PostCommentForComparedCommitInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PostCommentForComparedCommit request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_post_comment_for_pull_request_request(
    body: &[u8],
) -> Result<PostCommentForPullRequestInput, String> {
    if body.is_empty() {
        return Ok(PostCommentForPullRequestInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PostCommentForPullRequest request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_post_comment_reply_request(
    body: &[u8],
) -> Result<PostCommentReplyInput, String> {
    if body.is_empty() {
        return Ok(PostCommentReplyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PostCommentReply request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_comment_reaction_request(
    body: &[u8],
) -> Result<PutCommentReactionInput, String> {
    if body.is_empty() {
        return Ok(PutCommentReactionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutCommentReaction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_file_request(body: &[u8]) -> Result<PutFileInput, String> {
    if body.is_empty() {
        return Ok(PutFileInput::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize PutFile request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_repository_triggers_request(
    body: &[u8],
) -> Result<PutRepositoryTriggersInput, String> {
    if body.is_empty() {
        return Ok(PutRepositoryTriggersInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutRepositoryTriggers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_resource_request(body: &[u8]) -> Result<TagResourceInput, String> {
    if body.is_empty() {
        return Ok(TagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_test_repository_triggers_request(
    body: &[u8],
) -> Result<TestRepositoryTriggersInput, String> {
    if body.is_empty() {
        return Ok(TestRepositoryTriggersInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TestRepositoryTriggers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceInput, String> {
    if body.is_empty() {
        return Ok(UntagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_approval_rule_template_content_request(
    body: &[u8],
) -> Result<UpdateApprovalRuleTemplateContentInput, String> {
    if body.is_empty() {
        return Ok(UpdateApprovalRuleTemplateContentInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateApprovalRuleTemplateContent request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_approval_rule_template_description_request(
    body: &[u8],
) -> Result<UpdateApprovalRuleTemplateDescriptionInput, String> {
    if body.is_empty() {
        return Ok(UpdateApprovalRuleTemplateDescriptionInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateApprovalRuleTemplateDescription request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_approval_rule_template_name_request(
    body: &[u8],
) -> Result<UpdateApprovalRuleTemplateNameInput, String> {
    if body.is_empty() {
        return Ok(UpdateApprovalRuleTemplateNameInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateApprovalRuleTemplateName request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_comment_request(body: &[u8]) -> Result<UpdateCommentInput, String> {
    if body.is_empty() {
        return Ok(UpdateCommentInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateComment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_default_branch_request(
    body: &[u8],
) -> Result<UpdateDefaultBranchInput, String> {
    if body.is_empty() {
        return Ok(UpdateDefaultBranchInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDefaultBranch request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_pull_request_approval_rule_content_request(
    body: &[u8],
) -> Result<UpdatePullRequestApprovalRuleContentInput, String> {
    if body.is_empty() {
        return Ok(UpdatePullRequestApprovalRuleContentInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdatePullRequestApprovalRuleContent request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_pull_request_approval_state_request(
    body: &[u8],
) -> Result<UpdatePullRequestApprovalStateInput, String> {
    if body.is_empty() {
        return Ok(UpdatePullRequestApprovalStateInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePullRequestApprovalState request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_pull_request_description_request(
    body: &[u8],
) -> Result<UpdatePullRequestDescriptionInput, String> {
    if body.is_empty() {
        return Ok(UpdatePullRequestDescriptionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePullRequestDescription request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_pull_request_status_request(
    body: &[u8],
) -> Result<UpdatePullRequestStatusInput, String> {
    if body.is_empty() {
        return Ok(UpdatePullRequestStatusInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePullRequestStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_pull_request_title_request(
    body: &[u8],
) -> Result<UpdatePullRequestTitleInput, String> {
    if body.is_empty() {
        return Ok(UpdatePullRequestTitleInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePullRequestTitle request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_repository_description_request(
    body: &[u8],
) -> Result<UpdateRepositoryDescriptionInput, String> {
    if body.is_empty() {
        return Ok(UpdateRepositoryDescriptionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateRepositoryDescription request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_repository_encryption_key_request(
    body: &[u8],
) -> Result<UpdateRepositoryEncryptionKeyInput, String> {
    if body.is_empty() {
        return Ok(UpdateRepositoryEncryptionKeyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateRepositoryEncryptionKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_repository_name_request(
    body: &[u8],
) -> Result<UpdateRepositoryNameInput, String> {
    if body.is_empty() {
        return Ok(UpdateRepositoryNameInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateRepositoryName request: {e}"))
}
