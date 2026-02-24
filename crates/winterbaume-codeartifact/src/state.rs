use std::collections::HashMap;

use crate::types::{Domain, Repository};

#[derive(Debug, Default)]
pub struct CodeArtifactState {
    pub domains: HashMap<String, Domain>,
    /// Key: "domain/repository"
    pub repositories: HashMap<String, Repository>,
}

#[derive(Debug, thiserror::Error)]
pub enum CodeArtifactError {
    #[error("Domain '{0}' already exists")]
    DomainAlreadyExists(String),
    #[error("Domain '{0}' not found")]
    DomainNotFound(String),
    #[error("Repository '{repo}' already exists in domain '{domain}'")]
    RepositoryAlreadyExists { repo: String, domain: String },
    #[error("Repository '{repo}' not found in domain '{domain}'")]
    RepositoryNotFound { repo: String, domain: String },
}

impl CodeArtifactState {
    pub fn create_domain(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
        encryption_key: Option<String>,
        tags: HashMap<String, String>,
        created_time: f64,
    ) -> Result<Domain, CodeArtifactError> {
        if self.domains.contains_key(name) {
            return Err(CodeArtifactError::DomainAlreadyExists(name.to_string()));
        }
        let arn = format!("arn:aws:codeartifact:{region}:{account_id}:domain/{name}");
        let domain = Domain {
            name: name.to_string(),
            owner: account_id.to_string(),
            arn,
            encryption_key,
            status: "Active".to_string(),
            created_time,
            tags,
        };
        self.domains.insert(name.to_string(), domain.clone());
        Ok(domain)
    }

    pub fn describe_domain(&self, name: &str) -> Result<&Domain, CodeArtifactError> {
        self.domains
            .get(name)
            .ok_or_else(|| CodeArtifactError::DomainNotFound(name.to_string()))
    }

    pub fn delete_domain(&mut self, name: &str) -> Result<Domain, CodeArtifactError> {
        self.domains
            .remove(name)
            .ok_or_else(|| CodeArtifactError::DomainNotFound(name.to_string()))
    }

    pub fn list_domains(&self) -> Vec<&Domain> {
        self.domains.values().collect()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_repository(
        &mut self,
        domain_name: &str,
        repo_name: &str,
        account_id: &str,
        region: &str,
        description: Option<String>,
        tags: HashMap<String, String>,
        created_time: f64,
    ) -> Result<Repository, CodeArtifactError> {
        // Verify domain exists
        if !self.domains.contains_key(domain_name) {
            return Err(CodeArtifactError::DomainNotFound(domain_name.to_string()));
        }
        let key = format!("{domain_name}/{repo_name}");
        if self.repositories.contains_key(&key) {
            return Err(CodeArtifactError::RepositoryAlreadyExists {
                repo: repo_name.to_string(),
                domain: domain_name.to_string(),
            });
        }
        let arn = format!(
            "arn:aws:codeartifact:{region}:{account_id}:repository/{domain_name}/{repo_name}"
        );
        let repo = Repository {
            name: repo_name.to_string(),
            domain_name: domain_name.to_string(),
            domain_owner: account_id.to_string(),
            arn,
            description,
            created_time,
            tags,
        };
        self.repositories.insert(key, repo.clone());
        Ok(repo)
    }

    pub fn describe_repository(
        &self,
        domain_name: &str,
        repo_name: &str,
    ) -> Result<&Repository, CodeArtifactError> {
        let key = format!("{domain_name}/{repo_name}");
        self.repositories
            .get(&key)
            .ok_or_else(|| CodeArtifactError::RepositoryNotFound {
                repo: repo_name.to_string(),
                domain: domain_name.to_string(),
            })
    }

    pub fn delete_repository(
        &mut self,
        domain_name: &str,
        repo_name: &str,
    ) -> Result<Repository, CodeArtifactError> {
        let key = format!("{domain_name}/{repo_name}");
        self.repositories
            .remove(&key)
            .ok_or_else(|| CodeArtifactError::RepositoryNotFound {
                repo: repo_name.to_string(),
                domain: domain_name.to_string(),
            })
    }

    pub fn list_repositories(&self) -> Vec<&Repository> {
        self.repositories.values().collect()
    }

    pub fn list_repositories_in_domain(&self, domain_name: &str) -> Vec<&Repository> {
        self.repositories
            .values()
            .filter(|r| r.domain_name == domain_name)
            .collect()
    }
}
