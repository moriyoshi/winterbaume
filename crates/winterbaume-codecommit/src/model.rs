//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-codecommit

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateApprovalRuleTemplateWithRepositoryInput {
    #[serde(rename = "approvalRuleTemplateName")]
    #[serde(default)]
    pub approval_rule_template_name: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAssociateApprovalRuleTemplateWithRepositoriesInput {
    #[serde(rename = "approvalRuleTemplateName")]
    #[serde(default)]
    pub approval_rule_template_name: String,
    #[serde(rename = "repositoryNames")]
    #[serde(default)]
    pub repository_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAssociateApprovalRuleTemplateWithRepositoriesOutput {
    #[serde(rename = "associatedRepositoryNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_repository_names: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchAssociateApprovalRuleTemplateWithRepositoriesError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAssociateApprovalRuleTemplateWithRepositoriesError {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDescribeMergeConflictsInput {
    #[serde(rename = "conflictDetailLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    #[serde(rename = "destinationCommitSpecifier")]
    #[serde(default)]
    pub destination_commit_specifier: String,
    #[serde(rename = "filePaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_paths: Option<Vec<String>>,
    #[serde(rename = "maxConflictFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_conflict_files: Option<i32>,
    #[serde(rename = "maxMergeHunks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_merge_hunks: Option<i32>,
    #[serde(rename = "mergeOption")]
    #[serde(default)]
    pub merge_option: String,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "sourceCommitSpecifier")]
    #[serde(default)]
    pub source_commit_specifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDescribeMergeConflictsOutput {
    #[serde(rename = "baseCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_commit_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<Vec<Conflict>>,
    #[serde(rename = "destinationCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_commit_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchDescribeMergeConflictsError>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sourceCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Conflict {
    #[serde(rename = "conflictMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_metadata: Option<ConflictMetadata>,
    #[serde(rename = "mergeHunks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_hunks: Option<Vec<MergeHunk>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConflictMetadata {
    #[serde(rename = "contentConflict")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_conflict: Option<bool>,
    #[serde(rename = "fileModeConflict")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode_conflict: Option<bool>,
    #[serde(rename = "fileModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_modes: Option<FileModes>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "fileSizes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_sizes: Option<FileSizes>,
    #[serde(rename = "isBinaryFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_binary_file: Option<IsBinaryFile>,
    #[serde(rename = "mergeOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_operations: Option<MergeOperations>,
    #[serde(rename = "numberOfConflicts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_conflicts: Option<i32>,
    #[serde(rename = "objectTypeConflict")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type_conflict: Option<bool>,
    #[serde(rename = "objectTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_types: Option<ObjectTypes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileModes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileSizes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IsBinaryFile {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeOperations {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectTypes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeHunk {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<MergeHunkDetail>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<MergeHunkDetail>,
    #[serde(rename = "isConflict")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_conflict: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<MergeHunkDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeHunkDetail {
    #[serde(rename = "endLine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,
    #[serde(rename = "hunkContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hunk_content: Option<String>,
    #[serde(rename = "startLine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDescribeMergeConflictsError {
    #[serde(rename = "exceptionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_name: Option<String>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDisassociateApprovalRuleTemplateFromRepositoriesInput {
    #[serde(rename = "approvalRuleTemplateName")]
    #[serde(default)]
    pub approval_rule_template_name: String,
    #[serde(rename = "repositoryNames")]
    #[serde(default)]
    pub repository_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDisassociateApprovalRuleTemplateFromRepositoriesOutput {
    #[serde(rename = "disassociatedRepositoryNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociated_repository_names: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchDisassociateApprovalRuleTemplateFromRepositoriesError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDisassociateApprovalRuleTemplateFromRepositoriesError {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCommitsInput {
    #[serde(rename = "commitIds")]
    #[serde(default)]
    pub commit_ids: Vec<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCommitsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits: Option<Vec<Commit>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchGetCommitsError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Commit {
    #[serde(rename = "additionalData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<UserInfo>,
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committer: Option<UserInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<String>>,
    #[serde(rename = "treeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCommitsError {
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetRepositoriesInput {
    #[serde(rename = "repositoryNames")]
    #[serde(default)]
    pub repository_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetRepositoriesOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchGetRepositoriesError>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<RepositoryMetadata>>,
    #[serde(rename = "repositoriesNotFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories_not_found: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetRepositoriesError {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "repositoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryMetadata {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "cloneUrlHttp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_url_http: Option<String>,
    #[serde(rename = "cloneUrlSsh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_url_ssh: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "defaultBranch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "repositoryDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_description: Option<String>,
    #[serde(rename = "repositoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApprovalRuleTemplateInput {
    #[serde(rename = "approvalRuleTemplateContent")]
    #[serde(default)]
    pub approval_rule_template_content: String,
    #[serde(rename = "approvalRuleTemplateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_description: Option<String>,
    #[serde(rename = "approvalRuleTemplateName")]
    #[serde(default)]
    pub approval_rule_template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApprovalRuleTemplateOutput {
    #[serde(rename = "approvalRuleTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template: Option<ApprovalRuleTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApprovalRuleTemplate {
    #[serde(rename = "approvalRuleTemplateContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_content: Option<String>,
    #[serde(rename = "approvalRuleTemplateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_description: Option<String>,
    #[serde(rename = "approvalRuleTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_id: Option<String>,
    #[serde(rename = "approvalRuleTemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_name: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "lastModifiedUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
    #[serde(rename = "ruleContentSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_content_sha256: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBranchInput {
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(rename = "commitId")]
    #[serde(default)]
    pub commit_id: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCommitInput {
    #[serde(rename = "authorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(rename = "commitMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    #[serde(rename = "deleteFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_files: Option<Vec<DeleteFileEntry>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "keepEmptyFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
    #[serde(rename = "parentCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_commit_id: Option<String>,
    #[serde(rename = "putFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_files: Option<Vec<PutFileEntry>>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "setFileModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_file_modes: Option<Vec<SetFileModeEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileEntry {
    #[serde(rename = "filePath")]
    #[serde(default)]
    pub file_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFileEntry {
    #[serde(rename = "fileContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_content: Option<String>,
    #[serde(rename = "fileMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    pub file_path: String,
    #[serde(rename = "sourceFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file: Option<SourceFileSpecifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceFileSpecifier {
    #[serde(rename = "filePath")]
    #[serde(default)]
    pub file_path: String,
    #[serde(rename = "isMove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_move: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetFileModeEntry {
    #[serde(rename = "fileMode")]
    #[serde(default)]
    pub file_mode: String,
    #[serde(rename = "filePath")]
    #[serde(default)]
    pub file_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCommitOutput {
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "filesAdded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_added: Option<Vec<FileMetadata>>,
    #[serde(rename = "filesDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_deleted: Option<Vec<FileMetadata>>,
    #[serde(rename = "filesUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_updated: Option<Vec<FileMetadata>>,
    #[serde(rename = "treeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileMetadata {
    #[serde(rename = "absolutePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_path: Option<String>,
    #[serde(rename = "blobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_id: Option<String>,
    #[serde(rename = "fileMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePullRequestApprovalRuleInput {
    #[serde(rename = "approvalRuleContent")]
    #[serde(default)]
    pub approval_rule_content: String,
    #[serde(rename = "approvalRuleName")]
    #[serde(default)]
    pub approval_rule_name: String,
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePullRequestApprovalRuleOutput {
    #[serde(rename = "approvalRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule: Option<ApprovalRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApprovalRule {
    #[serde(rename = "approvalRuleContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_content: Option<String>,
    #[serde(rename = "approvalRuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_id: Option<String>,
    #[serde(rename = "approvalRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_name: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "lastModifiedUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
    #[serde(rename = "originApprovalRuleTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_approval_rule_template: Option<OriginApprovalRuleTemplate>,
    #[serde(rename = "ruleContentSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_content_sha256: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OriginApprovalRuleTemplate {
    #[serde(rename = "approvalRuleTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_id: Option<String>,
    #[serde(rename = "approvalRuleTemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePullRequestInput {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub targets: Vec<Target>,
    #[serde(default)]
    pub title: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Target {
    #[serde(rename = "destinationReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_reference: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "sourceReference")]
    #[serde(default)]
    pub source_reference: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePullRequestOutput {
    #[serde(rename = "pullRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PullRequest {
    #[serde(rename = "approvalRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<Vec<ApprovalRule>>,
    #[serde(rename = "authorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_arn: Option<String>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastActivityDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activity_date: Option<f64>,
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<String>,
    #[serde(rename = "pullRequestStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_status: Option<String>,
    #[serde(rename = "pullRequestTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_targets: Option<Vec<PullRequestTarget>>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PullRequestTarget {
    #[serde(rename = "destinationCommit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_commit: Option<String>,
    #[serde(rename = "destinationReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_reference: Option<String>,
    #[serde(rename = "mergeBase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_base: Option<String>,
    #[serde(rename = "mergeMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_metadata: Option<MergeMetadata>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "sourceCommit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit: Option<String>,
    #[serde(rename = "sourceReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_reference: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeMetadata {
    #[serde(rename = "isMerged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_merged: Option<bool>,
    #[serde(rename = "mergeCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_commit_id: Option<String>,
    #[serde(rename = "mergeOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_option: Option<String>,
    #[serde(rename = "mergedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRepositoryInput {
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "repositoryDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_description: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRepositoryOutput {
    #[serde(rename = "repositoryMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_metadata: Option<RepositoryMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUnreferencedMergeCommitInput {
    #[serde(rename = "authorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(rename = "commitMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    #[serde(rename = "conflictDetailLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    #[serde(rename = "conflictResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution: Option<ConflictResolution>,
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    #[serde(rename = "destinationCommitSpecifier")]
    #[serde(default)]
    pub destination_commit_specifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "keepEmptyFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
    #[serde(rename = "mergeOption")]
    #[serde(default)]
    pub merge_option: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "sourceCommitSpecifier")]
    #[serde(default)]
    pub source_commit_specifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConflictResolution {
    #[serde(rename = "deleteFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_files: Option<Vec<DeleteFileEntry>>,
    #[serde(rename = "replaceContents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_contents: Option<Vec<ReplaceContentEntry>>,
    #[serde(rename = "setFileModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_file_modes: Option<Vec<SetFileModeEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplaceContentEntry {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "fileMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    pub file_path: String,
    #[serde(rename = "replacementType")]
    #[serde(default)]
    pub replacement_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUnreferencedMergeCommitOutput {
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "treeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApprovalRuleTemplateInput {
    #[serde(rename = "approvalRuleTemplateName")]
    #[serde(default)]
    pub approval_rule_template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApprovalRuleTemplateOutput {
    #[serde(rename = "approvalRuleTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBranchInput {
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBranchOutput {
    #[serde(rename = "deletedBranch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_branch: Option<BranchInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BranchInfo {
    #[serde(rename = "branchName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCommentContentInput {
    #[serde(rename = "commentId")]
    #[serde(default)]
    pub comment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCommentContentOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Comment {
    #[serde(rename = "authorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_arn: Option<String>,
    #[serde(rename = "callerReactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_reactions: Option<Vec<String>>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "commentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "inReplyTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "reactionCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction_counts: Option<std::collections::HashMap<String, i32>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileInput {
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(rename = "commitMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    pub file_path: String,
    #[serde(rename = "keepEmptyFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parentCommitId")]
    #[serde(default)]
    pub parent_commit_id: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileOutput {
    #[serde(rename = "blobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_id: Option<String>,
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "treeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePullRequestApprovalRuleInput {
    #[serde(rename = "approvalRuleName")]
    #[serde(default)]
    pub approval_rule_name: String,
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePullRequestApprovalRuleOutput {
    #[serde(rename = "approvalRuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRepositoryInput {
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRepositoryOutput {
    #[serde(rename = "repositoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMergeConflictsInput {
    #[serde(rename = "conflictDetailLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    #[serde(rename = "destinationCommitSpecifier")]
    #[serde(default)]
    pub destination_commit_specifier: String,
    #[serde(rename = "filePath")]
    #[serde(default)]
    pub file_path: String,
    #[serde(rename = "maxMergeHunks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_merge_hunks: Option<i32>,
    #[serde(rename = "mergeOption")]
    #[serde(default)]
    pub merge_option: String,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "sourceCommitSpecifier")]
    #[serde(default)]
    pub source_commit_specifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMergeConflictsOutput {
    #[serde(rename = "baseCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_commit_id: Option<String>,
    #[serde(rename = "conflictMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_metadata: Option<ConflictMetadata>,
    #[serde(rename = "destinationCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_commit_id: Option<String>,
    #[serde(rename = "mergeHunks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_hunks: Option<Vec<MergeHunk>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sourceCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePullRequestEventsInput {
    #[serde(rename = "actorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_arn: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "pullRequestEventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_event_type: Option<String>,
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePullRequestEventsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "pullRequestEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_events: Option<Vec<PullRequestEvent>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PullRequestEvent {
    #[serde(rename = "actorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_arn: Option<String>,
    #[serde(rename = "approvalRuleEventMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_event_metadata: Option<ApprovalRuleEventMetadata>,
    #[serde(rename = "approvalRuleOverriddenEventMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_overridden_event_metadata: Option<ApprovalRuleOverriddenEventMetadata>,
    #[serde(rename = "approvalStateChangedEventMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_state_changed_event_metadata: Option<ApprovalStateChangedEventMetadata>,
    #[serde(rename = "eventDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_date: Option<f64>,
    #[serde(rename = "pullRequestCreatedEventMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_created_event_metadata: Option<PullRequestCreatedEventMetadata>,
    #[serde(rename = "pullRequestEventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_event_type: Option<String>,
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<String>,
    #[serde(rename = "pullRequestMergedStateChangedEventMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_merged_state_changed_event_metadata:
        Option<PullRequestMergedStateChangedEventMetadata>,
    #[serde(rename = "pullRequestSourceReferenceUpdatedEventMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_source_reference_updated_event_metadata:
        Option<PullRequestSourceReferenceUpdatedEventMetadata>,
    #[serde(rename = "pullRequestStatusChangedEventMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_status_changed_event_metadata: Option<PullRequestStatusChangedEventMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApprovalRuleEventMetadata {
    #[serde(rename = "approvalRuleContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_content: Option<String>,
    #[serde(rename = "approvalRuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_id: Option<String>,
    #[serde(rename = "approvalRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApprovalRuleOverriddenEventMetadata {
    #[serde(rename = "overrideStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_status: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApprovalStateChangedEventMetadata {
    #[serde(rename = "approvalStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_status: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PullRequestCreatedEventMetadata {
    #[serde(rename = "destinationCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_commit_id: Option<String>,
    #[serde(rename = "mergeBase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_base: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "sourceCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PullRequestMergedStateChangedEventMetadata {
    #[serde(rename = "destinationReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_reference: Option<String>,
    #[serde(rename = "mergeMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_metadata: Option<MergeMetadata>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PullRequestSourceReferenceUpdatedEventMetadata {
    #[serde(rename = "afterCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    #[serde(rename = "beforeCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    #[serde(rename = "mergeBase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_base: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PullRequestStatusChangedEventMetadata {
    #[serde(rename = "pullRequestStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateApprovalRuleTemplateFromRepositoryInput {
    #[serde(rename = "approvalRuleTemplateName")]
    #[serde(default)]
    pub approval_rule_template_name: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluatePullRequestApprovalRulesInput {
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    pub revision_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluatePullRequestApprovalRulesOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation: Option<Evaluation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Evaluation {
    #[serde(rename = "approvalRulesNotSatisfied")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules_not_satisfied: Option<Vec<String>>,
    #[serde(rename = "approvalRulesSatisfied")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules_satisfied: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overridden: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApprovalRuleTemplateInput {
    #[serde(rename = "approvalRuleTemplateName")]
    #[serde(default)]
    pub approval_rule_template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApprovalRuleTemplateOutput {
    #[serde(rename = "approvalRuleTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template: Option<ApprovalRuleTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBlobInput {
    #[serde(rename = "blobId")]
    #[serde(default)]
    pub blob_id: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBlobOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBranchInput {
    #[serde(rename = "branchName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBranchOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<BranchInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommentInput {
    #[serde(rename = "commentId")]
    #[serde(default)]
    pub comment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommentOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommentReactionsInput {
    #[serde(rename = "commentId")]
    #[serde(default)]
    pub comment_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "reactionUserArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction_user_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommentReactionsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "reactionsForComment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions_for_comment: Option<Vec<ReactionForComment>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReactionForComment {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<ReactionValueFormats>,
    #[serde(rename = "reactionUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction_users: Option<Vec<String>>,
    #[serde(rename = "reactionsFromDeletedUsersCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions_from_deleted_users_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReactionValueFormats {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(rename = "shortCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommentsForComparedCommitInput {
    #[serde(rename = "afterCommitId")]
    #[serde(default)]
    pub after_commit_id: String,
    #[serde(rename = "beforeCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommentsForComparedCommitOutput {
    #[serde(rename = "commentsForComparedCommitData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_for_compared_commit_data: Option<Vec<CommentsForComparedCommit>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommentsForComparedCommit {
    #[serde(rename = "afterBlobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_blob_id: Option<String>,
    #[serde(rename = "afterCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    #[serde(rename = "beforeBlobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_blob_id: Option<String>,
    #[serde(rename = "beforeCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<Comment>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Location {
    #[serde(rename = "filePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "filePosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_position: Option<i64>,
    #[serde(rename = "relativeFileVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_file_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommentsForPullRequestInput {
    #[serde(rename = "afterCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    #[serde(rename = "beforeCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommentsForPullRequestOutput {
    #[serde(rename = "commentsForPullRequestData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_for_pull_request_data: Option<Vec<CommentsForPullRequest>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommentsForPullRequest {
    #[serde(rename = "afterBlobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_blob_id: Option<String>,
    #[serde(rename = "afterCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    #[serde(rename = "beforeBlobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_blob_id: Option<String>,
    #[serde(rename = "beforeCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<Comment>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommitInput {
    #[serde(rename = "commitId")]
    #[serde(default)]
    pub commit_id: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommitOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<Commit>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDifferencesInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "afterCommitSpecifier")]
    #[serde(default)]
    pub after_commit_specifier: String,
    #[serde(rename = "afterPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_path: Option<String>,
    #[serde(rename = "beforeCommitSpecifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_specifier: Option<String>,
    #[serde(rename = "beforePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_path: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDifferencesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differences: Option<Vec<Difference>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Difference {
    #[serde(rename = "afterBlob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_blob: Option<BlobMetadata>,
    #[serde(rename = "beforeBlob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_blob: Option<BlobMetadata>,
    #[serde(rename = "changeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlobMetadata {
    #[serde(rename = "blobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFileInput {
    #[serde(rename = "commitSpecifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_specifier: Option<String>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    pub file_path: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFileOutput {
    #[serde(rename = "blobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_id: Option<String>,
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "fileContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_content: Option<String>,
    #[serde(rename = "fileMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "fileSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFolderInput {
    #[serde(rename = "commitSpecifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_specifier: Option<String>,
    #[serde(rename = "folderPath")]
    #[serde(default)]
    pub folder_path: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFolderOutput {
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<File>>,
    #[serde(rename = "folderPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
    #[serde(rename = "subFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_folders: Option<Vec<Folder>>,
    #[serde(rename = "subModules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_modules: Option<Vec<SubModule>>,
    #[serde(rename = "symbolicLinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbolic_links: Option<Vec<SymbolicLink>>,
    #[serde(rename = "treeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct File {
    #[serde(rename = "absolutePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_path: Option<String>,
    #[serde(rename = "blobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_id: Option<String>,
    #[serde(rename = "fileMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    #[serde(rename = "relativePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Folder {
    #[serde(rename = "absolutePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_path: Option<String>,
    #[serde(rename = "relativePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
    #[serde(rename = "treeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubModule {
    #[serde(rename = "absolutePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_path: Option<String>,
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "relativePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SymbolicLink {
    #[serde(rename = "absolutePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_path: Option<String>,
    #[serde(rename = "blobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_id: Option<String>,
    #[serde(rename = "fileMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    #[serde(rename = "relativePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMergeCommitInput {
    #[serde(rename = "conflictDetailLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    #[serde(rename = "destinationCommitSpecifier")]
    #[serde(default)]
    pub destination_commit_specifier: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "sourceCommitSpecifier")]
    #[serde(default)]
    pub source_commit_specifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMergeCommitOutput {
    #[serde(rename = "baseCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_commit_id: Option<String>,
    #[serde(rename = "destinationCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_commit_id: Option<String>,
    #[serde(rename = "mergedCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_commit_id: Option<String>,
    #[serde(rename = "sourceCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMergeConflictsInput {
    #[serde(rename = "conflictDetailLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    #[serde(rename = "destinationCommitSpecifier")]
    #[serde(default)]
    pub destination_commit_specifier: String,
    #[serde(rename = "maxConflictFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_conflict_files: Option<i32>,
    #[serde(rename = "mergeOption")]
    #[serde(default)]
    pub merge_option: String,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "sourceCommitSpecifier")]
    #[serde(default)]
    pub source_commit_specifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMergeConflictsOutput {
    #[serde(rename = "baseCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_commit_id: Option<String>,
    #[serde(rename = "conflictMetadataList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_metadata_list: Option<Vec<ConflictMetadata>>,
    #[serde(rename = "destinationCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_commit_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sourceCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMergeOptionsInput {
    #[serde(rename = "conflictDetailLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    #[serde(rename = "destinationCommitSpecifier")]
    #[serde(default)]
    pub destination_commit_specifier: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "sourceCommitSpecifier")]
    #[serde(default)]
    pub source_commit_specifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMergeOptionsOutput {
    #[serde(rename = "baseCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_commit_id: Option<String>,
    #[serde(rename = "destinationCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_commit_id: Option<String>,
    #[serde(rename = "mergeOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_options: Option<Vec<String>>,
    #[serde(rename = "sourceCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPullRequestApprovalStatesInput {
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    pub revision_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPullRequestApprovalStatesOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approvals: Option<Vec<Approval>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Approval {
    #[serde(rename = "approvalState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_state: Option<String>,
    #[serde(rename = "userArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPullRequestInput {
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPullRequestOutput {
    #[serde(rename = "pullRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPullRequestOverrideStateInput {
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    pub revision_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPullRequestOverrideStateOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overridden: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrider: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRepositoryInput {
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRepositoryOutput {
    #[serde(rename = "repositoryMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_metadata: Option<RepositoryMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRepositoryTriggersInput {
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRepositoryTriggersOutput {
    #[serde(rename = "configurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<RepositoryTrigger>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryTrigger {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<String>>,
    #[serde(rename = "customData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<String>,
    #[serde(rename = "destinationArn")]
    #[serde(default)]
    pub destination_arn: String,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApprovalRuleTemplatesInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApprovalRuleTemplatesOutput {
    #[serde(rename = "approvalRuleTemplateNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_names: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssociatedApprovalRuleTemplatesForRepositoryInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssociatedApprovalRuleTemplatesForRepositoryOutput {
    #[serde(rename = "approvalRuleTemplateNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_names: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBranchesInput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBranchesOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFileCommitHistoryRequest {
    #[serde(rename = "commitSpecifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_specifier: Option<String>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    pub file_path: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFileCommitHistoryResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "revisionDag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_dag: Option<Vec<FileVersion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileVersion {
    #[serde(rename = "blobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<Commit>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "revisionChildren")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_children: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPullRequestsInput {
    #[serde(rename = "authorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_arn: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "pullRequestStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_status: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPullRequestsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "pullRequestIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRepositoriesForApprovalRuleTemplateInput {
    #[serde(rename = "approvalRuleTemplateName")]
    #[serde(default)]
    pub approval_rule_template_name: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRepositoriesForApprovalRuleTemplateOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "repositoryNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRepositoriesInput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRepositoriesOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<RepositoryNameIdPair>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryNameIdPair {
    #[serde(rename = "repositoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {
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
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeBranchesByFastForwardInput {
    #[serde(rename = "destinationCommitSpecifier")]
    #[serde(default)]
    pub destination_commit_specifier: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "sourceCommitSpecifier")]
    #[serde(default)]
    pub source_commit_specifier: String,
    #[serde(rename = "targetBranch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_branch: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeBranchesByFastForwardOutput {
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "treeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeBranchesBySquashInput {
    #[serde(rename = "authorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(rename = "commitMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    #[serde(rename = "conflictDetailLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    #[serde(rename = "conflictResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution: Option<ConflictResolution>,
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    #[serde(rename = "destinationCommitSpecifier")]
    #[serde(default)]
    pub destination_commit_specifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "keepEmptyFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "sourceCommitSpecifier")]
    #[serde(default)]
    pub source_commit_specifier: String,
    #[serde(rename = "targetBranch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_branch: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeBranchesBySquashOutput {
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "treeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeBranchesByThreeWayInput {
    #[serde(rename = "authorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(rename = "commitMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    #[serde(rename = "conflictDetailLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    #[serde(rename = "conflictResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution: Option<ConflictResolution>,
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    #[serde(rename = "destinationCommitSpecifier")]
    #[serde(default)]
    pub destination_commit_specifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "keepEmptyFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "sourceCommitSpecifier")]
    #[serde(default)]
    pub source_commit_specifier: String,
    #[serde(rename = "targetBranch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_branch: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeBranchesByThreeWayOutput {
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "treeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergePullRequestByFastForwardInput {
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "sourceCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergePullRequestByFastForwardOutput {
    #[serde(rename = "pullRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergePullRequestBySquashInput {
    #[serde(rename = "authorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(rename = "commitMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    #[serde(rename = "conflictDetailLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    #[serde(rename = "conflictResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution: Option<ConflictResolution>,
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "keepEmptyFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "sourceCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergePullRequestBySquashOutput {
    #[serde(rename = "pullRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergePullRequestByThreeWayInput {
    #[serde(rename = "authorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(rename = "commitMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    #[serde(rename = "conflictDetailLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    #[serde(rename = "conflictResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution: Option<ConflictResolution>,
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "keepEmptyFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "sourceCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergePullRequestByThreeWayOutput {
    #[serde(rename = "pullRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OverridePullRequestApprovalRulesInput {
    #[serde(rename = "overrideStatus")]
    #[serde(default)]
    pub override_status: String,
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    pub revision_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PostCommentForComparedCommitInput {
    #[serde(rename = "afterCommitId")]
    #[serde(default)]
    pub after_commit_id: String,
    #[serde(rename = "beforeCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PostCommentForComparedCommitOutput {
    #[serde(rename = "afterBlobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_blob_id: Option<String>,
    #[serde(rename = "afterCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    #[serde(rename = "beforeBlobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_blob_id: Option<String>,
    #[serde(rename = "beforeCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PostCommentForPullRequestInput {
    #[serde(rename = "afterCommitId")]
    #[serde(default)]
    pub after_commit_id: String,
    #[serde(rename = "beforeCommitId")]
    #[serde(default)]
    pub before_commit_id: String,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PostCommentForPullRequestOutput {
    #[serde(rename = "afterBlobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_blob_id: Option<String>,
    #[serde(rename = "afterCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    #[serde(rename = "beforeBlobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_blob_id: Option<String>,
    #[serde(rename = "beforeCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PostCommentReplyInput {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(default)]
    pub content: String,
    #[serde(rename = "inReplyTo")]
    #[serde(default)]
    pub in_reply_to: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PostCommentReplyOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutCommentReactionInput {
    #[serde(rename = "commentId")]
    #[serde(default)]
    pub comment_id: String,
    #[serde(rename = "reactionValue")]
    #[serde(default)]
    pub reaction_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFileInput {
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(rename = "commitMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "fileContent")]
    #[serde(default)]
    pub file_content: String,
    #[serde(rename = "fileMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    pub file_path: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parentCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_commit_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFileOutput {
    #[serde(rename = "blobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_id: Option<String>,
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "treeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRepositoryTriggersInput {
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(default)]
    pub triggers: Vec<RepositoryTrigger>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRepositoryTriggersOutput {
    #[serde(rename = "configurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestRepositoryTriggersInput {
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(default)]
    pub triggers: Vec<RepositoryTrigger>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestRepositoryTriggersOutput {
    #[serde(rename = "failedExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_executions: Option<Vec<RepositoryTriggerExecutionFailure>>,
    #[serde(rename = "successfulExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_executions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryTriggerExecutionFailure {
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
}

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
pub struct UpdateApprovalRuleTemplateContentInput {
    #[serde(rename = "approvalRuleTemplateName")]
    #[serde(default)]
    pub approval_rule_template_name: String,
    #[serde(rename = "existingRuleContentSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_rule_content_sha256: Option<String>,
    #[serde(rename = "newRuleContent")]
    #[serde(default)]
    pub new_rule_content: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApprovalRuleTemplateContentOutput {
    #[serde(rename = "approvalRuleTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template: Option<ApprovalRuleTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApprovalRuleTemplateDescriptionInput {
    #[serde(rename = "approvalRuleTemplateDescription")]
    #[serde(default)]
    pub approval_rule_template_description: String,
    #[serde(rename = "approvalRuleTemplateName")]
    #[serde(default)]
    pub approval_rule_template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApprovalRuleTemplateDescriptionOutput {
    #[serde(rename = "approvalRuleTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template: Option<ApprovalRuleTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApprovalRuleTemplateNameInput {
    #[serde(rename = "newApprovalRuleTemplateName")]
    #[serde(default)]
    pub new_approval_rule_template_name: String,
    #[serde(rename = "oldApprovalRuleTemplateName")]
    #[serde(default)]
    pub old_approval_rule_template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApprovalRuleTemplateNameOutput {
    #[serde(rename = "approvalRuleTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template: Option<ApprovalRuleTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCommentInput {
    #[serde(rename = "commentId")]
    #[serde(default)]
    pub comment_id: String,
    #[serde(default)]
    pub content: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCommentOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDefaultBranchInput {
    #[serde(rename = "defaultBranchName")]
    #[serde(default)]
    pub default_branch_name: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePullRequestApprovalRuleContentInput {
    #[serde(rename = "approvalRuleName")]
    #[serde(default)]
    pub approval_rule_name: String,
    #[serde(rename = "existingRuleContentSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_rule_content_sha256: Option<String>,
    #[serde(rename = "newRuleContent")]
    #[serde(default)]
    pub new_rule_content: String,
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePullRequestApprovalRuleContentOutput {
    #[serde(rename = "approvalRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule: Option<ApprovalRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePullRequestApprovalStateInput {
    #[serde(rename = "approvalState")]
    #[serde(default)]
    pub approval_state: String,
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    pub revision_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePullRequestDescriptionInput {
    #[serde(default)]
    pub description: String,
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePullRequestDescriptionOutput {
    #[serde(rename = "pullRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePullRequestStatusInput {
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
    #[serde(rename = "pullRequestStatus")]
    #[serde(default)]
    pub pull_request_status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePullRequestStatusOutput {
    #[serde(rename = "pullRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePullRequestTitleInput {
    #[serde(rename = "pullRequestId")]
    #[serde(default)]
    pub pull_request_id: String,
    #[serde(default)]
    pub title: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePullRequestTitleOutput {
    #[serde(rename = "pullRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRepositoryDescriptionInput {
    #[serde(rename = "repositoryDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_description: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRepositoryEncryptionKeyInput {
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    pub kms_key_id: String,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRepositoryEncryptionKeyOutput {
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "originalKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_kms_key_id: Option<String>,
    #[serde(rename = "repositoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRepositoryNameInput {
    #[serde(rename = "newName")]
    #[serde(default)]
    pub new_name: String,
    #[serde(rename = "oldName")]
    #[serde(default)]
    pub old_name: String,
}
