use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::{Analyzer, ArchiveRule, CriterionValue};

#[derive(Debug, Default)]
pub struct AccessAnalyzerState {
    /// Analyzers keyed by name.
    pub analyzers: HashMap<String, AnalyzerState>,
}

/// Internal state for a single analyzer, including its archive rules.
#[derive(Debug, Clone)]
pub struct AnalyzerState {
    pub analyzer: Analyzer,
    pub archive_rules: HashMap<String, ArchiveRule>,
}

/// Domain-specific error enum. Contains no HTTP status codes or AWS error type strings.
#[derive(Debug, Error)]
pub enum AccessAnalyzerError {
    #[error("Analyzer {name} already exists.")]
    AnalyzerAlreadyExists { name: String },

    #[error("Analyzer {name} not found.")]
    AnalyzerNotFound { name: String },

    #[error("Archive rule {rule_name} already exists for analyzer {analyzer_name}.")]
    ArchiveRuleAlreadyExists {
        analyzer_name: String,
        rule_name: String,
    },

    #[error("Archive rule {rule_name} not found for analyzer {analyzer_name}.")]
    ArchiveRuleNotFound {
        analyzer_name: String,
        rule_name: String,
    },

    #[error("Resource {arn} not found.")]
    ResourceNotFound { arn: String },

    #[error("{message}")]
    Validation { message: String },
}

impl AccessAnalyzerState {
    // -------------------------------------------------------------------------
    // Analyzers
    // -------------------------------------------------------------------------

    pub fn create_analyzer(
        &mut self,
        name: &str,
        analyzer_type: &str,
        tags: HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> Result<&Analyzer, AccessAnalyzerError> {
        if self.analyzers.contains_key(name) {
            return Err(AccessAnalyzerError::AnalyzerAlreadyExists {
                name: name.to_string(),
            });
        }

        let arn = format!("arn:aws:access-analyzer:{region}:{account_id}:analyzer/{name}");
        let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

        let analyzer = Analyzer {
            arn,
            name: name.to_string(),
            analyzer_type: analyzer_type.to_string(),
            status: "ACTIVE".to_string(),
            created_at: now,
            tags,
        };

        let state = AnalyzerState {
            analyzer,
            archive_rules: HashMap::new(),
        };

        self.analyzers.insert(name.to_string(), state);
        Ok(&self.analyzers.get(name).unwrap().analyzer)
    }

    pub fn get_analyzer(&self, name: &str) -> Result<&AnalyzerState, AccessAnalyzerError> {
        self.analyzers
            .get(name)
            .ok_or_else(|| AccessAnalyzerError::AnalyzerNotFound {
                name: name.to_string(),
            })
    }

    pub fn delete_analyzer(&mut self, name: &str) -> Result<(), AccessAnalyzerError> {
        if self.analyzers.remove(name).is_none() {
            return Err(AccessAnalyzerError::AnalyzerNotFound {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_analyzers(&self, type_filter: Option<&str>) -> Vec<&Analyzer> {
        self.analyzers
            .values()
            .filter(|s| {
                type_filter.is_none() || type_filter == Some(s.analyzer.analyzer_type.as_str())
            })
            .map(|s| &s.analyzer)
            .collect()
    }

    // -------------------------------------------------------------------------
    // Archive Rules
    // -------------------------------------------------------------------------

    pub fn create_archive_rule(
        &mut self,
        analyzer_name: &str,
        rule_name: &str,
        filter: HashMap<String, CriterionValue>,
    ) -> Result<(), AccessAnalyzerError> {
        let analyzer_state = self.analyzers.get_mut(analyzer_name).ok_or_else(|| {
            AccessAnalyzerError::AnalyzerNotFound {
                name: analyzer_name.to_string(),
            }
        })?;

        if analyzer_state.archive_rules.contains_key(rule_name) {
            return Err(AccessAnalyzerError::ArchiveRuleAlreadyExists {
                analyzer_name: analyzer_name.to_string(),
                rule_name: rule_name.to_string(),
            });
        }

        let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let rule = ArchiveRule {
            rule_name: rule_name.to_string(),
            filter,
            created_at: now.clone(),
            updated_at: now,
        };

        analyzer_state
            .archive_rules
            .insert(rule_name.to_string(), rule);
        Ok(())
    }

    pub fn get_archive_rule(
        &self,
        analyzer_name: &str,
        rule_name: &str,
    ) -> Result<&ArchiveRule, AccessAnalyzerError> {
        let analyzer_state = self.analyzers.get(analyzer_name).ok_or_else(|| {
            AccessAnalyzerError::AnalyzerNotFound {
                name: analyzer_name.to_string(),
            }
        })?;

        analyzer_state.archive_rules.get(rule_name).ok_or_else(|| {
            AccessAnalyzerError::ArchiveRuleNotFound {
                analyzer_name: analyzer_name.to_string(),
                rule_name: rule_name.to_string(),
            }
        })
    }

    pub fn delete_archive_rule(
        &mut self,
        analyzer_name: &str,
        rule_name: &str,
    ) -> Result<(), AccessAnalyzerError> {
        let analyzer_state = self.analyzers.get_mut(analyzer_name).ok_or_else(|| {
            AccessAnalyzerError::AnalyzerNotFound {
                name: analyzer_name.to_string(),
            }
        })?;

        if analyzer_state.archive_rules.remove(rule_name).is_none() {
            return Err(AccessAnalyzerError::ArchiveRuleNotFound {
                analyzer_name: analyzer_name.to_string(),
                rule_name: rule_name.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_archive_rules(
        &self,
        analyzer_name: &str,
    ) -> Result<Vec<&ArchiveRule>, AccessAnalyzerError> {
        let analyzer_state = self.analyzers.get(analyzer_name).ok_or_else(|| {
            AccessAnalyzerError::AnalyzerNotFound {
                name: analyzer_name.to_string(),
            }
        })?;

        Ok(analyzer_state.archive_rules.values().collect())
    }

    // -------------------------------------------------------------------------
    // Tags
    // -------------------------------------------------------------------------

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), AccessAnalyzerError> {
        // Find analyzer by ARN
        if let Some(state) = self
            .analyzers
            .values_mut()
            .find(|s| s.analyzer.arn == resource_arn)
        {
            state.analyzer.tags.extend(tags);
            return Ok(());
        }
        Err(AccessAnalyzerError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), AccessAnalyzerError> {
        if let Some(state) = self
            .analyzers
            .values_mut()
            .find(|s| s.analyzer.arn == resource_arn)
        {
            for key in tag_keys {
                state.analyzer.tags.remove(key);
            }
            return Ok(());
        }
        Err(AccessAnalyzerError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<&HashMap<String, String>, AccessAnalyzerError> {
        if let Some(state) = self
            .analyzers
            .values()
            .find(|s| s.analyzer.arn == resource_arn)
        {
            return Ok(&state.analyzer.tags);
        }
        Err(AccessAnalyzerError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }
}
