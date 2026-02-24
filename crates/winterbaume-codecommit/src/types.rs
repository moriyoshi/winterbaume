use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Repository {
    pub repository_id: String,
    pub repository_name: String,
    pub arn: String,
    pub description: String,
    pub clone_url_http: String,
    pub clone_url_ssh: String,
    pub creation_date: DateTime<Utc>,
    pub last_modified_date: DateTime<Utc>,
    pub account_id: String,
    pub default_branch: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Branch {
    pub branch_name: String,
    pub commit_id: String,
}

#[derive(Debug, Clone)]
pub struct CommitRecord {
    pub commit_id: String,
    pub tree_id: String,
    pub parent_ids: Vec<String>,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub date: DateTime<Utc>,
}

/// A file entry stored in the repository under a given commit.
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub file_path: String,
    pub blob_id: String,
    pub file_mode: String,
}

#[derive(Debug, Clone)]
pub struct PullRequestRecord {
    pub pull_request_id: String,
    pub title: String,
    pub description: String,
    pub status: String, // "OPEN" | "CLOSED"
    pub repository_name: String,
    pub source_reference: String,
    pub destination_reference: String,
    pub source_commit: String,
    pub destination_commit: String,
    pub creation_date: DateTime<Utc>,
    pub last_activity_date: DateTime<Utc>,
    pub author_arn: String,
}
