use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct CodeCommitState {
    pub repositories: HashMap<String, Repository>,
    /// branches keyed by repo_name -> branch_name -> Branch
    pub branches: HashMap<String, HashMap<String, Branch>>,
    /// commits keyed by repo_name -> commit_id -> CommitRecord
    pub commits: HashMap<String, HashMap<String, CommitRecord>>,
    /// files keyed by repo_name -> commit_id -> file_path -> FileEntry
    pub files: HashMap<String, HashMap<String, HashMap<String, FileEntry>>>,
    /// pull requests keyed by pull_request_id
    pub pull_requests: HashMap<String, PullRequestRecord>,
    /// next pull request number per repo
    pub pull_request_counter: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum CodeCommitError {
    #[error("Repository named {name} already exists in this account.")]
    RepositoryAlreadyExists { name: String },
    #[error("Repository named {name} already exists")]
    RepositoryNameTaken { name: String },
    #[error("{name} does not exist")]
    RepositoryDoesNotExist { name: String },
    #[error("Repository with ARN {arn} does not exist")]
    RepositoryDoesNotExistByArn { arn: String },
    #[error("Branch {branch_name} already exists in {repo_name}")]
    BranchAlreadyExists {
        branch_name: String,
        repo_name: String,
    },
    #[error("Branch {branch_name} does not exist in {repo_name}")]
    BranchDoesNotExist {
        branch_name: String,
        repo_name: String,
    },
    #[error("Branch {branch_name} not found")]
    BranchNotFound { branch_name: String },
    #[error("The default branch cannot be deleted")]
    DefaultBranchCannotBeDeleted,
    #[error("Commit {commit_id} does not exist in {repo_name}")]
    CommitDoesNotExist {
        commit_id: String,
        repo_name: String,
    },
    #[error("Specifier {spec} does not resolve to a commit")]
    SpecifierDoesNotResolve { spec: String },
    #[error("File {file_path} does not exist in commit {commit_id}")]
    FileDoesNotExist {
        file_path: String,
        commit_id: String,
    },
    #[error("Pull request {pr_id} does not exist")]
    PullRequestDoesNotExist { pr_id: String },
    #[error("Repository has no default branch")]
    RepositoryEmpty,
}

impl CodeCommitState {
    pub fn create_repository(
        &mut self,
        name: &str,
        description: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&Repository, CodeCommitError> {
        if self.repositories.contains_key(name) {
            return Err(CodeCommitError::RepositoryAlreadyExists {
                name: name.to_string(),
            });
        }

        let repo_id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:codecommit:{region}:{account_id}:{name}");
        let now = Utc::now();

        let repo = Repository {
            repository_id: repo_id,
            repository_name: name.to_string(),
            arn,
            description: description.to_string(),
            clone_url_http: format!(
                "https://git-codecommit.{region}.amazonaws.com/v1/repos/{name}"
            ),
            clone_url_ssh: format!("ssh://git-codecommit.{region}.amazonaws.com/v1/repos/{name}"),
            creation_date: now,
            last_modified_date: now,
            account_id: account_id.to_string(),
            default_branch: None,
            tags: HashMap::new(),
        };

        self.repositories.insert(name.to_string(), repo);
        Ok(self.repositories.get(name).unwrap())
    }

    pub fn get_repository(&self, name: &str) -> Result<&Repository, CodeCommitError> {
        self.repositories
            .get(name)
            .ok_or_else(|| CodeCommitError::RepositoryDoesNotExist {
                name: name.to_string(),
            })
    }

    pub fn delete_repository(&mut self, name: &str) -> String {
        // AWS DeleteRepository is idempotent: if the repository does not exist,
        // a null repository ID is returned rather than an error.
        self.branches.remove(name);
        self.commits.remove(name);
        self.files.remove(name);
        match self.repositories.remove(name) {
            Some(repo) => repo.repository_id,
            None => String::new(),
        }
    }

    pub fn list_repositories(&self) -> Vec<&Repository> {
        let mut repos: Vec<&Repository> = self.repositories.values().collect();
        repos.sort_by(|a, b| a.repository_name.cmp(&b.repository_name));
        repos
    }

    pub fn update_repository_description(
        &mut self,
        name: &str,
        description: Option<&str>,
    ) -> Result<(), CodeCommitError> {
        let repo = self.repositories.get_mut(name).ok_or_else(|| {
            CodeCommitError::RepositoryDoesNotExist {
                name: name.to_string(),
            }
        })?;
        repo.description = description.unwrap_or("").to_string();
        repo.last_modified_date = Utc::now();
        Ok(())
    }

    pub fn update_repository_name(
        &mut self,
        old_name: &str,
        new_name: &str,
        region: &str,
        account_id: &str,
    ) -> Result<(), CodeCommitError> {
        if !self.repositories.contains_key(old_name) {
            return Err(CodeCommitError::RepositoryDoesNotExist {
                name: old_name.to_string(),
            });
        }
        if self.repositories.contains_key(new_name) {
            return Err(CodeCommitError::RepositoryNameTaken {
                name: new_name.to_string(),
            });
        }
        let mut repo = self.repositories.remove(old_name).unwrap();
        repo.repository_name = new_name.to_string();
        repo.arn = format!("arn:aws:codecommit:{region}:{account_id}:{new_name}");
        repo.clone_url_http =
            format!("https://git-codecommit.{region}.amazonaws.com/v1/repos/{new_name}");
        repo.clone_url_ssh =
            format!("ssh://git-codecommit.{region}.amazonaws.com/v1/repos/{new_name}");
        repo.last_modified_date = Utc::now();
        self.repositories.insert(new_name.to_string(), repo);
        // Migrate associated branch/commit/file state
        if let Some(branches) = self.branches.remove(old_name) {
            self.branches.insert(new_name.to_string(), branches);
        }
        if let Some(commits) = self.commits.remove(old_name) {
            self.commits.insert(new_name.to_string(), commits);
        }
        if let Some(files) = self.files.remove(old_name) {
            self.files.insert(new_name.to_string(), files);
        }
        Ok(())
    }

    // ---- Branches ----

    pub fn create_branch(
        &mut self,
        repo_name: &str,
        branch_name: &str,
        commit_id: &str,
    ) -> Result<(), CodeCommitError> {
        if !self.repositories.contains_key(repo_name) {
            return Err(CodeCommitError::RepositoryDoesNotExist {
                name: repo_name.to_string(),
            });
        }
        let repo_branches = self.branches.entry(repo_name.to_string()).or_default();
        if repo_branches.contains_key(branch_name) {
            return Err(CodeCommitError::BranchAlreadyExists {
                branch_name: branch_name.to_string(),
                repo_name: repo_name.to_string(),
            });
        }
        repo_branches.insert(
            branch_name.to_string(),
            Branch {
                branch_name: branch_name.to_string(),
                commit_id: commit_id.to_string(),
            },
        );
        Ok(())
    }

    pub fn get_branch(
        &self,
        repo_name: &str,
        branch_name: &str,
    ) -> Result<&Branch, CodeCommitError> {
        if !self.repositories.contains_key(repo_name) {
            return Err(CodeCommitError::RepositoryDoesNotExist {
                name: repo_name.to_string(),
            });
        }
        self.branches
            .get(repo_name)
            .and_then(|m| m.get(branch_name))
            .ok_or_else(|| CodeCommitError::BranchDoesNotExist {
                branch_name: branch_name.to_string(),
                repo_name: repo_name.to_string(),
            })
    }

    pub fn list_branches(&self, repo_name: &str) -> Result<Vec<String>, CodeCommitError> {
        if !self.repositories.contains_key(repo_name) {
            return Err(CodeCommitError::RepositoryDoesNotExist {
                name: repo_name.to_string(),
            });
        }
        let mut names: Vec<String> = self
            .branches
            .get(repo_name)
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default();
        names.sort();
        Ok(names)
    }

    pub fn delete_branch(
        &mut self,
        repo_name: &str,
        branch_name: &str,
    ) -> Result<Branch, CodeCommitError> {
        if !self.repositories.contains_key(repo_name) {
            return Err(CodeCommitError::RepositoryDoesNotExist {
                name: repo_name.to_string(),
            });
        }
        // Prevent deleting default branch
        if let Some(repo) = self.repositories.get(repo_name) {
            if repo.default_branch.as_deref() == Some(branch_name) {
                return Err(CodeCommitError::DefaultBranchCannotBeDeleted);
            }
        }
        self.branches
            .get_mut(repo_name)
            .and_then(|m| m.remove(branch_name))
            .ok_or_else(|| CodeCommitError::BranchDoesNotExist {
                branch_name: branch_name.to_string(),
                repo_name: repo_name.to_string(),
            })
    }

    pub fn update_default_branch(
        &mut self,
        repo_name: &str,
        branch_name: &str,
    ) -> Result<(), CodeCommitError> {
        if !self.repositories.contains_key(repo_name) {
            return Err(CodeCommitError::RepositoryDoesNotExist {
                name: repo_name.to_string(),
            });
        }
        // Branch must exist
        let branch_exists = self
            .branches
            .get(repo_name)
            .map(|m| m.contains_key(branch_name))
            .unwrap_or(false);
        if !branch_exists {
            return Err(CodeCommitError::BranchDoesNotExist {
                branch_name: branch_name.to_string(),
                repo_name: repo_name.to_string(),
            });
        }
        let repo = self.repositories.get_mut(repo_name).unwrap();
        repo.default_branch = Some(branch_name.to_string());
        Ok(())
    }

    // ---- Commits ----

    pub fn create_commit(
        &mut self,
        repo_name: &str,
        branch_name: &str,
        parent_commit_id: Option<&str>,
        author_name: Option<&str>,
        author_email: Option<&str>,
        commit_message: Option<&str>,
        put_files: Vec<(String, String)>, // (path, mode)
        delete_files: Vec<String>,
    ) -> Result<CommitRecord, CodeCommitError> {
        if !self.repositories.contains_key(repo_name) {
            return Err(CodeCommitError::RepositoryDoesNotExist {
                name: repo_name.to_string(),
            });
        }

        let now = Utc::now();
        let commit_id = format!("{:x}", uuid::Uuid::new_v4().as_u128());
        let tree_id = format!("{:x}", uuid::Uuid::new_v4().as_u128());

        // Determine parent files to inherit
        let parent_files: HashMap<String, FileEntry> = if let Some(pid) = parent_commit_id {
            self.files
                .get(repo_name)
                .and_then(|r| r.get(pid))
                .cloned()
                .unwrap_or_default()
        } else {
            HashMap::new()
        };

        // Build new file set
        let mut new_files = parent_files;
        for path in delete_files {
            new_files.remove(&path);
        }
        for (path, mode) in put_files {
            let blob_id = format!("{:x}", uuid::Uuid::new_v4().as_u128());
            new_files.insert(
                path.clone(),
                FileEntry {
                    file_path: path,
                    blob_id,
                    file_mode: mode,
                },
            );
        }

        let record = CommitRecord {
            commit_id: commit_id.clone(),
            tree_id,
            parent_ids: parent_commit_id
                .map(|p| vec![p.to_string()])
                .unwrap_or_default(),
            message: commit_message.unwrap_or("").to_string(),
            author_name: author_name.unwrap_or("").to_string(),
            author_email: author_email.unwrap_or("").to_string(),
            date: now,
        };

        self.commits
            .entry(repo_name.to_string())
            .or_default()
            .insert(commit_id.clone(), record.clone());

        self.files
            .entry(repo_name.to_string())
            .or_default()
            .insert(commit_id.clone(), new_files);

        // Advance the branch
        let repo_branches = self.branches.entry(repo_name.to_string()).or_default();
        if let Some(branch) = repo_branches.get_mut(branch_name) {
            branch.commit_id = commit_id.clone();
        } else {
            // Auto-create branch if it doesn't exist
            repo_branches.insert(
                branch_name.to_string(),
                Branch {
                    branch_name: branch_name.to_string(),
                    commit_id: commit_id.clone(),
                },
            );
            // Set default branch if none
            let repo = self.repositories.get_mut(repo_name).unwrap();
            if repo.default_branch.is_none() {
                repo.default_branch = Some(branch_name.to_string());
            }
        }

        Ok(record)
    }

    pub fn get_commit(
        &self,
        repo_name: &str,
        commit_id: &str,
    ) -> Result<&CommitRecord, CodeCommitError> {
        if !self.repositories.contains_key(repo_name) {
            return Err(CodeCommitError::RepositoryDoesNotExist {
                name: repo_name.to_string(),
            });
        }
        self.commits
            .get(repo_name)
            .and_then(|m| m.get(commit_id))
            .ok_or_else(|| CodeCommitError::CommitDoesNotExist {
                commit_id: commit_id.to_string(),
                repo_name: repo_name.to_string(),
            })
    }

    // ---- Files ----

    pub fn get_file(
        &self,
        repo_name: &str,
        commit_specifier: Option<&str>,
        file_path: &str,
    ) -> Result<(&CommitRecord, &FileEntry), CodeCommitError> {
        if !self.repositories.contains_key(repo_name) {
            return Err(CodeCommitError::RepositoryDoesNotExist {
                name: repo_name.to_string(),
            });
        }

        // Resolve specifier (branch name or commit id) to a commit id
        let commit_id = self.resolve_specifier(repo_name, commit_specifier)?;
        let commit = self.get_commit(repo_name, &commit_id)?;

        let file = self
            .files
            .get(repo_name)
            .and_then(|r| r.get(&commit_id))
            .and_then(|f| f.get(file_path))
            .ok_or_else(|| CodeCommitError::FileDoesNotExist {
                file_path: file_path.to_string(),
                commit_id: commit_id.clone(),
            })?;
        Ok((commit, file))
    }

    pub fn get_folder(
        &self,
        repo_name: &str,
        commit_specifier: Option<&str>,
        folder_path: &str,
    ) -> Result<(String, Vec<FileEntry>, Vec<String>), CodeCommitError> {
        if !self.repositories.contains_key(repo_name) {
            return Err(CodeCommitError::RepositoryDoesNotExist {
                name: repo_name.to_string(),
            });
        }
        let commit_id = self.resolve_specifier(repo_name, commit_specifier)?;
        let prefix = if folder_path == "/" || folder_path.is_empty() {
            "".to_string()
        } else {
            let p = folder_path.trim_start_matches('/');
            format!("{p}/")
        };

        let all_files: Vec<FileEntry> = self
            .files
            .get(repo_name)
            .and_then(|r| r.get(&commit_id))
            .map(|f| {
                f.values()
                    .filter(|fe| fe.file_path.starts_with(&prefix))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();

        // Collect direct children that are files (no additional slash after prefix)
        let mut direct_files: Vec<FileEntry> = Vec::new();
        let mut sub_folders: std::collections::HashSet<String> = std::collections::HashSet::new();
        for fe in &all_files {
            let rest = &fe.file_path[prefix.len()..];
            if rest.contains('/') {
                let sub = rest.split('/').next().unwrap_or("");
                sub_folders.insert(sub.to_string());
            } else {
                direct_files.push(fe.clone());
            }
        }
        let sub_folder_list: Vec<String> = sub_folders.into_iter().collect();
        Ok((commit_id, direct_files, sub_folder_list))
    }

    pub fn put_file(
        &mut self,
        repo_name: &str,
        branch_name: &str,
        parent_commit_id: &str,
        file_path: &str,
        file_mode: Option<&str>,
        author_name: Option<&str>,
        author_email: Option<&str>,
        commit_message: Option<&str>,
    ) -> Result<CommitRecord, CodeCommitError> {
        let mode = file_mode.unwrap_or("NORMAL").to_string();
        self.create_commit(
            repo_name,
            branch_name,
            Some(parent_commit_id),
            author_name,
            author_email,
            commit_message,
            vec![(file_path.to_string(), mode)],
            vec![],
        )
    }

    pub fn delete_file(
        &mut self,
        repo_name: &str,
        branch_name: &str,
        parent_commit_id: &str,
        file_path: &str,
        author_name: Option<&str>,
        author_email: Option<&str>,
        commit_message: Option<&str>,
    ) -> Result<CommitRecord, CodeCommitError> {
        self.create_commit(
            repo_name,
            branch_name,
            Some(parent_commit_id),
            author_name,
            author_email,
            commit_message,
            vec![],
            vec![file_path.to_string()],
        )
    }

    pub fn get_differences(
        &self,
        repo_name: &str,
        after_commit_specifier: &str,
        before_commit_specifier: Option<&str>,
    ) -> Result<Vec<(Option<FileEntry>, Option<FileEntry>, String)>, CodeCommitError> {
        if !self.repositories.contains_key(repo_name) {
            return Err(CodeCommitError::RepositoryDoesNotExist {
                name: repo_name.to_string(),
            });
        }
        let after_id = self.resolve_specifier(repo_name, Some(after_commit_specifier))?;
        let after_files: HashMap<String, FileEntry> = self
            .files
            .get(repo_name)
            .and_then(|r| r.get(&after_id))
            .cloned()
            .unwrap_or_default();

        let before_files: HashMap<String, FileEntry> = if let Some(spec) = before_commit_specifier {
            let before_id = self.resolve_specifier(repo_name, Some(spec))?;
            self.files
                .get(repo_name)
                .and_then(|r| r.get(&before_id))
                .cloned()
                .unwrap_or_default()
        } else {
            HashMap::new()
        };

        let mut diffs = Vec::new();

        // Added or modified
        for (path, after_fe) in &after_files {
            if let Some(before_fe) = before_files.get(path) {
                if before_fe.blob_id != after_fe.blob_id {
                    diffs.push((
                        Some(before_fe.clone()),
                        Some(after_fe.clone()),
                        "M".to_string(),
                    ));
                }
            } else {
                diffs.push((None, Some(after_fe.clone()), "A".to_string()));
            }
        }
        // Deleted
        for (path, before_fe) in &before_files {
            if !after_files.contains_key(path) {
                diffs.push((Some(before_fe.clone()), None, "D".to_string()));
            }
        }

        Ok(diffs)
    }

    // ---- Pull Requests ----

    pub fn create_pull_request(
        &mut self,
        title: &str,
        description: &str,
        repo_name: &str,
        source_reference: &str,
        destination_reference: &str,
    ) -> Result<PullRequestRecord, CodeCommitError> {
        if !self.repositories.contains_key(repo_name) {
            return Err(CodeCommitError::RepositoryDoesNotExist {
                name: repo_name.to_string(),
            });
        }

        // Resolve source/dest commit IDs from branch names
        let source_commit = self
            .branches
            .get(repo_name)
            .and_then(|m| m.get(source_reference.trim_start_matches("refs/heads/")))
            .map(|b| b.commit_id.clone())
            .unwrap_or_default();
        let destination_commit = self
            .branches
            .get(repo_name)
            .and_then(|m| m.get(destination_reference.trim_start_matches("refs/heads/")))
            .map(|b| b.commit_id.clone())
            .unwrap_or_default();

        self.pull_request_counter += 1;
        let pr_id = self.pull_request_counter.to_string();
        let now = Utc::now();
        let pr = PullRequestRecord {
            pull_request_id: pr_id.clone(),
            title: title.to_string(),
            description: description.to_string(),
            status: "OPEN".to_string(),
            repository_name: repo_name.to_string(),
            source_reference: source_reference.to_string(),
            destination_reference: destination_reference.to_string(),
            source_commit,
            destination_commit,
            creation_date: now,
            last_activity_date: now,
            author_arn: "arn:aws:iam::123456789012:root".to_string(),
        };
        self.pull_requests.insert(pr_id, pr.clone());
        Ok(pr)
    }

    pub fn get_pull_request(&self, pr_id: &str) -> Result<&PullRequestRecord, CodeCommitError> {
        self.pull_requests
            .get(pr_id)
            .ok_or_else(|| CodeCommitError::PullRequestDoesNotExist {
                pr_id: pr_id.to_string(),
            })
    }

    pub fn list_pull_requests(&self, repo_name: &str, status: Option<&str>) -> Vec<String> {
        self.pull_requests
            .values()
            .filter(|pr| {
                pr.repository_name == repo_name && status.map(|s| pr.status == s).unwrap_or(true)
            })
            .map(|pr| pr.pull_request_id.clone())
            .collect()
    }

    pub fn update_pull_request_status(
        &mut self,
        pr_id: &str,
        status: &str,
    ) -> Result<PullRequestRecord, CodeCommitError> {
        let pr = self.pull_requests.get_mut(pr_id).ok_or_else(|| {
            CodeCommitError::PullRequestDoesNotExist {
                pr_id: pr_id.to_string(),
            }
        })?;
        pr.status = status.to_string();
        pr.last_activity_date = Utc::now();
        Ok(pr.clone())
    }

    // ---- Tags ----

    pub fn tag_resource(
        &mut self,
        repo_name: &str,
        tags: std::collections::HashMap<String, String>,
    ) -> Result<(), CodeCommitError> {
        let repo = self.repositories.get_mut(repo_name).ok_or_else(|| {
            CodeCommitError::RepositoryDoesNotExist {
                name: repo_name.to_string(),
            }
        })?;
        repo.tags.extend(tags);
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        repo_name: &str,
        tag_keys: &[String],
    ) -> Result<(), CodeCommitError> {
        let repo = self.repositories.get_mut(repo_name).ok_or_else(|| {
            CodeCommitError::RepositoryDoesNotExist {
                name: repo_name.to_string(),
            }
        })?;
        for key in tag_keys {
            repo.tags.remove(key);
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        repo_arn: &str,
    ) -> Result<std::collections::HashMap<String, String>, CodeCommitError> {
        // Find repo by ARN
        let repo = self
            .repositories
            .values()
            .find(|r| r.arn == repo_arn)
            .ok_or_else(|| CodeCommitError::RepositoryDoesNotExistByArn {
                arn: repo_arn.to_string(),
            })?;
        Ok(repo.tags.clone())
    }

    // ---- Helper ----

    fn resolve_specifier(
        &self,
        repo_name: &str,
        specifier: Option<&str>,
    ) -> Result<String, CodeCommitError> {
        match specifier {
            None => {
                // Use default branch
                let repo = self.repositories.get(repo_name).ok_or_else(|| {
                    CodeCommitError::RepositoryDoesNotExist {
                        name: repo_name.to_string(),
                    }
                })?;
                let branch_name = repo
                    .default_branch
                    .as_deref()
                    .ok_or(CodeCommitError::RepositoryEmpty)?;
                self.branches
                    .get(repo_name)
                    .and_then(|m| m.get(branch_name))
                    .map(|b| b.commit_id.clone())
                    .ok_or_else(|| CodeCommitError::BranchNotFound {
                        branch_name: branch_name.to_string(),
                    })
            }
            Some(spec) => {
                // Try branch first, then treat as commit id
                let branch_name = spec.trim_start_matches("refs/heads/");
                if let Some(branch) = self
                    .branches
                    .get(repo_name)
                    .and_then(|m| m.get(branch_name))
                {
                    return Ok(branch.commit_id.clone());
                }
                // Treat as a commit ID directly
                if self
                    .commits
                    .get(repo_name)
                    .map(|m| m.contains_key(spec))
                    .unwrap_or(false)
                {
                    return Ok(spec.to_string());
                }
                Err(CodeCommitError::SpecifierDoesNotResolve {
                    spec: spec.to_string(),
                })
            }
        }
    }
}
