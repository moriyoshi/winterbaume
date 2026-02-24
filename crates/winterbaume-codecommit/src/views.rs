//! Serde-compatible view types for CodeCommit state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CodeCommitService;
use crate::state::CodeCommitState;

/// Serializable view of the entire CodeCommit state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeCommitStateView {
    /// Repositories keyed by repository name.
    #[serde(default)]
    pub repositories: HashMap<String, RepositoryView>,
    #[serde(default)]
    pub branches: HashMap<String, HashMap<String, BranchView>>,
    #[serde(default)]
    pub commits: HashMap<String, HashMap<String, CommitView>>,
    #[serde(default)]
    pub files: HashMap<String, HashMap<String, HashMap<String, FileEntryView>>>,
    #[serde(default)]
    pub pull_requests: HashMap<String, PullRequestView>,
    #[serde(default)]
    pub pull_request_counter: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryView {
    pub repository_id: String,
    pub repository_name: String,
    pub arn: String,
    pub description: String,
    pub clone_url_http: String,
    pub clone_url_ssh: String,
    pub creation_date: String,
    pub last_modified_date: String,
    pub account_id: String,
    #[serde(default)]
    pub default_branch: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchView {
    pub branch_name: String,
    pub commit_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitView {
    pub commit_id: String,
    pub tree_id: String,
    pub parent_ids: Vec<String>,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub date: String,
}

/// File content is excluded from snapshots; restored as empty bytes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntryView {
    pub file_path: String,
    pub blob_id: String,
    pub file_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestView {
    pub pull_request_id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub repository_name: String,
    pub source_reference: String,
    pub destination_reference: String,
    pub source_commit: String,
    pub destination_commit: String,
    pub creation_date: String,
    pub last_activity_date: String,
    pub author_arn: String,
}

fn parse_datetime(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}

// --- From internal types to view types ---

impl From<&CodeCommitState> for CodeCommitStateView {
    fn from(state: &CodeCommitState) -> Self {
        let repositories = state
            .repositories
            .iter()
            .map(|(k, r)| {
                (
                    k.clone(),
                    RepositoryView {
                        repository_id: r.repository_id.clone(),
                        repository_name: r.repository_name.clone(),
                        arn: r.arn.clone(),
                        description: r.description.clone(),
                        clone_url_http: r.clone_url_http.clone(),
                        clone_url_ssh: r.clone_url_ssh.clone(),
                        creation_date: r.creation_date.to_rfc3339(),
                        last_modified_date: r.last_modified_date.to_rfc3339(),
                        account_id: r.account_id.clone(),
                        default_branch: r.default_branch.clone(),
                        tags: r.tags.clone(),
                    },
                )
            })
            .collect();

        let branches = state
            .branches
            .iter()
            .map(|(repo, bmap)| {
                (
                    repo.clone(),
                    bmap.iter()
                        .map(|(bn, b)| {
                            (
                                bn.clone(),
                                BranchView {
                                    branch_name: b.branch_name.clone(),
                                    commit_id: b.commit_id.clone(),
                                },
                            )
                        })
                        .collect(),
                )
            })
            .collect();

        let commits = state
            .commits
            .iter()
            .map(|(repo, cmap)| {
                (
                    repo.clone(),
                    cmap.iter()
                        .map(|(cid, c)| {
                            (
                                cid.clone(),
                                CommitView {
                                    commit_id: c.commit_id.clone(),
                                    tree_id: c.tree_id.clone(),
                                    parent_ids: c.parent_ids.clone(),
                                    message: c.message.clone(),
                                    author_name: c.author_name.clone(),
                                    author_email: c.author_email.clone(),
                                    date: c.date.to_rfc3339(),
                                },
                            )
                        })
                        .collect(),
                )
            })
            .collect();

        let files = state
            .files
            .iter()
            .map(|(repo, commit_map)| {
                (
                    repo.clone(),
                    commit_map
                        .iter()
                        .map(|(cid, fmap)| {
                            (
                                cid.clone(),
                                fmap.iter()
                                    .map(|(fp, fe)| {
                                        (
                                            fp.clone(),
                                            FileEntryView {
                                                file_path: fe.file_path.clone(),
                                                blob_id: fe.blob_id.clone(),
                                                file_mode: fe.file_mode.clone(),
                                            },
                                        )
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                )
            })
            .collect();

        let pull_requests = state
            .pull_requests
            .iter()
            .map(|(id, pr)| {
                (
                    id.clone(),
                    PullRequestView {
                        pull_request_id: pr.pull_request_id.clone(),
                        title: pr.title.clone(),
                        description: pr.description.clone(),
                        status: pr.status.clone(),
                        repository_name: pr.repository_name.clone(),
                        source_reference: pr.source_reference.clone(),
                        destination_reference: pr.destination_reference.clone(),
                        source_commit: pr.source_commit.clone(),
                        destination_commit: pr.destination_commit.clone(),
                        creation_date: pr.creation_date.to_rfc3339(),
                        last_activity_date: pr.last_activity_date.to_rfc3339(),
                        author_arn: pr.author_arn.clone(),
                    },
                )
            })
            .collect();

        CodeCommitStateView {
            repositories,
            branches,
            commits,
            files,
            pull_requests,
            pull_request_counter: state.pull_request_counter,
        }
    }
}

// --- From view types to internal types ---

impl From<CodeCommitStateView> for CodeCommitState {
    fn from(view: CodeCommitStateView) -> Self {
        let repositories = view
            .repositories
            .into_iter()
            .map(|(k, r)| {
                (
                    k,
                    crate::types::Repository {
                        repository_id: r.repository_id,
                        repository_name: r.repository_name,
                        arn: r.arn,
                        description: r.description,
                        clone_url_http: r.clone_url_http,
                        clone_url_ssh: r.clone_url_ssh,
                        creation_date: parse_datetime(&r.creation_date),
                        last_modified_date: parse_datetime(&r.last_modified_date),
                        account_id: r.account_id,
                        default_branch: r.default_branch,
                        tags: r.tags,
                    },
                )
            })
            .collect();

        let branches = view
            .branches
            .into_iter()
            .map(|(repo, bmap)| {
                (
                    repo,
                    bmap.into_iter()
                        .map(|(bn, b)| {
                            (
                                bn,
                                crate::types::Branch {
                                    branch_name: b.branch_name,
                                    commit_id: b.commit_id,
                                },
                            )
                        })
                        .collect(),
                )
            })
            .collect();

        let commits = view
            .commits
            .into_iter()
            .map(|(repo, cmap)| {
                (
                    repo,
                    cmap.into_iter()
                        .map(|(cid, c)| {
                            (
                                cid,
                                crate::types::CommitRecord {
                                    commit_id: c.commit_id,
                                    tree_id: c.tree_id,
                                    parent_ids: c.parent_ids,
                                    message: c.message,
                                    author_name: c.author_name,
                                    author_email: c.author_email,
                                    date: parse_datetime(&c.date),
                                },
                            )
                        })
                        .collect(),
                )
            })
            .collect();

        let files = view
            .files
            .into_iter()
            .map(|(repo, commit_map)| {
                (
                    repo,
                    commit_map
                        .into_iter()
                        .map(|(cid, fmap)| {
                            (
                                cid,
                                fmap.into_iter()
                                    .map(|(fp, fe)| {
                                        (
                                            fp,
                                            crate::types::FileEntry {
                                                file_path: fe.file_path,
                                                blob_id: fe.blob_id,
                                                file_mode: fe.file_mode,
                                            },
                                        )
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                )
            })
            .collect();

        let pull_requests = view
            .pull_requests
            .into_iter()
            .map(|(id, pr)| {
                (
                    id,
                    crate::types::PullRequestRecord {
                        pull_request_id: pr.pull_request_id,
                        title: pr.title,
                        description: pr.description,
                        status: pr.status,
                        repository_name: pr.repository_name,
                        source_reference: pr.source_reference,
                        destination_reference: pr.destination_reference,
                        source_commit: pr.source_commit,
                        destination_commit: pr.destination_commit,
                        creation_date: parse_datetime(&pr.creation_date),
                        last_activity_date: parse_datetime(&pr.last_activity_date),
                        author_arn: pr.author_arn,
                    },
                )
            })
            .collect();

        CodeCommitState {
            repositories,
            branches,
            commits,
            files,
            pull_requests,
            pull_request_counter: view.pull_request_counter,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for CodeCommitService {
    type StateView = CodeCommitStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CodeCommitStateView::from(&*guard)
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
            *guard = CodeCommitState::from(view);
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
            let merged = CodeCommitState::from(view);
            for (k, v) in merged.repositories {
                guard.repositories.insert(k, v);
            }
            for (k, v) in merged.branches {
                guard.branches.insert(k, v);
            }
            for (k, v) in merged.commits {
                guard.commits.insert(k, v);
            }
            for (k, v) in merged.files {
                guard.files.insert(k, v);
            }
            for (k, v) in merged.pull_requests {
                guard.pull_requests.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
