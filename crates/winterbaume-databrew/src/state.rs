use std::collections::{BTreeMap, HashMap};

use chrono::Utc;
use serde_json::Value;

use crate::types::*;

#[derive(Debug, Default)]
pub struct DataBrewState {
    pub datasets: HashMap<String, Dataset>,
    pub recipes: HashMap<String, Recipe>,
    pub rulesets: HashMap<String, Ruleset>,
    pub schedules: HashMap<String, Schedule>,
    pub jobs: HashMap<String, Job>,
}

#[derive(Debug, thiserror::Error)]
pub enum DataBrewError {
    #[error("{name} already exists.")]
    DatasetAlreadyExists { name: String },
    #[error("One or more resources can't be found.")]
    DatasetNotFound,
    #[error("The recipe {name} already exists")]
    RecipeAlreadyExists { name: String },
    #[error("The recipe {name} for version {version} wasn't found.")]
    RecipeNotFoundForVersion { name: String, version: String },
    #[error("The recipe {name} for version LATEST_PUBLISHED wasn't found.")]
    RecipeLatestPublishedNotFound { name: String },
    #[error("The recipe {name} for version LATEST_WORKING wasn't found.")]
    RecipeLatestWorkingNotFound { name: String },
    #[error("The recipe {name} version {version} wasn't found.")]
    RecipeVersionNotFound { name: String, version: String },
    #[error("The recipe {name} wasn't found")]
    RecipeNotFound { name: String },
    #[error("Recipe {name} wasn't found")]
    PublishRecipeNotFound { name: String },
    #[error("Recipe version LATEST_WORKING is not allowed to be deleted")]
    RecipeLatestWorkingCannotDelete,
    #[error("Recipe version {version} is not allowed to be deleted")]
    RecipeVersionCannotDelete { version: String },
    #[error("Ruleset already exists.")]
    RulesetAlreadyExists,
    #[error("Ruleset {name} not found.")]
    RulesetNotFound { name: String },
    #[error("Schedule already exists: {name}")]
    ScheduleAlreadyExists { name: String },
    #[error("Schedule not found: {name}")]
    ScheduleNotFound { name: String },
    #[error("The job {name} {job_type} job already exists.")]
    JobAlreadyExists { name: String, job_type: String },
    #[error(
        "1 validation error detected: Value '{value}' at 'encryptionMode' failed to satisfy constraint: Member must satisfy enum value set: [SSE-S3, SSE-KMS]"
    )]
    InvalidEncryptionMode { value: String },
    #[error(
        "1 validation error detected: Value '{value}' at 'logSubscription' failed to satisfy constraint: Member must satisfy enum value set: [ENABLE, DISABLE]"
    )]
    InvalidLogSubscription { value: String },
    #[error("Job {name} wasn't found.")]
    JobDescribeNotFound { name: String },
    #[error("The job {name} wasn't found")]
    JobUpdateNotFound { name: String },
    #[error("The job {name} wasn't found.")]
    JobDeleteNotFound { name: String },
    #[error("Resource not found: {resource_arn}")]
    TagResourceNotFound { resource_arn: String },
}

impl DataBrewState {
    // ── Dataset operations ──────────────────────────────────────────

    pub fn create_dataset(
        &mut self,
        name: &str,
        input: Value,
        format: Option<&str>,
        format_options: Option<Value>,
        tags: Option<HashMap<String, String>>,
        path_options: Option<Value>,
        account_id: &str,
        region: &str,
    ) -> Result<&Dataset, DataBrewError> {
        if self.datasets.contains_key(name) {
            return Err(DataBrewError::DatasetAlreadyExists {
                name: name.to_string(),
            });
        }

        let now = Utc::now();
        let resource_arn = format!("arn:aws:databrew:{region}:{account_id}:dataset/{name}");

        // Determine source from input
        let source = if input.get("S3InputDefinition").is_some() {
            "S3"
        } else if input.get("DatabaseInputDefinition").is_some() {
            "DATABASE"
        } else if input.get("DataCatalogInputDefinition").is_some() {
            "DATA-CATALOG"
        } else {
            "S3"
        };

        let dataset = Dataset {
            name: name.to_string(),
            account_id: account_id.to_string(),
            created_by: format!("arn:aws:iam::{account_id}:root"),
            create_date: now,
            last_modified_by: format!("arn:aws:iam::{account_id}:root"),
            last_modified_date: now,
            source: source.to_string(),
            format: format.map(|f| f.to_string()),
            format_options,
            input,
            tags,
            resource_arn,
            path_options,
        };

        self.datasets.insert(name.to_string(), dataset);
        Ok(self.datasets.get(name).unwrap())
    }

    pub fn describe_dataset(&self, name: &str) -> Result<&Dataset, DataBrewError> {
        self.datasets
            .get(name)
            .ok_or(DataBrewError::DatasetNotFound)
    }

    pub fn delete_dataset(&mut self, name: &str) -> Result<String, DataBrewError> {
        self.datasets
            .remove(name)
            .map(|d| d.name)
            .ok_or(DataBrewError::DatasetNotFound)
    }

    pub fn update_dataset(
        &mut self,
        name: &str,
        format: Option<&str>,
        format_options: Option<Value>,
        input: Option<Value>,
        path_options: Option<Value>,
        account_id: &str,
    ) -> Result<String, DataBrewError> {
        let dataset = self
            .datasets
            .get_mut(name)
            .ok_or(DataBrewError::DatasetNotFound)?;

        let now = Utc::now();
        dataset.last_modified_by = format!("arn:aws:iam::{account_id}:root");
        dataset.last_modified_date = now;

        if let Some(f) = format {
            dataset.format = Some(f.to_string());
        }
        if let Some(fo) = format_options {
            dataset.format_options = Some(fo);
        }
        if let Some(inp) = input {
            // Update source based on input
            let source = if inp.get("S3InputDefinition").is_some() {
                "S3"
            } else if inp.get("DatabaseInputDefinition").is_some() {
                "DATABASE"
            } else if inp.get("DataCatalogInputDefinition").is_some() {
                "DATA-CATALOG"
            } else {
                "S3"
            };
            dataset.source = source.to_string();
            dataset.input = inp;
        }
        if let Some(po) = path_options {
            dataset.path_options = Some(po);
        }

        Ok(name.to_string())
    }

    pub fn list_datasets(&self) -> Vec<&Dataset> {
        self.datasets.values().collect()
    }

    // ── Recipe operations ───────────────────────────────────────────

    pub fn create_recipe(
        &mut self,
        name: &str,
        description: Option<&str>,
        steps: Vec<Value>,
        tags: Option<HashMap<String, String>>,
        account_id: &str,
        region: &str,
    ) -> Result<&Recipe, DataBrewError> {
        if self.recipes.contains_key(name) {
            return Err(DataBrewError::RecipeAlreadyExists {
                name: name.to_string(),
            });
        }

        let now = Utc::now();
        let resource_arn = format!("arn:aws:databrew:{region}:{account_id}:recipe/{name}");

        let version_str = "0.1".to_string();
        let version = RecipeVersion {
            name: name.to_string(),
            version: version_str.clone(),
            description: description.map(|s| s.to_string()),
            steps: steps.clone(),
            project_name: None,
            tags: tags.clone(),
            account_id: account_id.to_string(),
            created_by: format!("arn:aws:iam::{account_id}:root"),
            create_date: now,
            last_modified_by: format!("arn:aws:iam::{account_id}:root"),
            last_modified_date: now,
            resource_arn: resource_arn.clone(),
            published_by: None,
            published_date: None,
        };

        let mut versions = BTreeMap::new();
        versions.insert(version_str, version);

        let recipe = Recipe {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            steps,
            project_name: None,
            tags,
            account_id: account_id.to_string(),
            created_by: format!("arn:aws:iam::{account_id}:root"),
            create_date: now,
            last_modified_by: format!("arn:aws:iam::{account_id}:root"),
            last_modified_date: now,
            resource_arn,
            versions,
            latest_working: 0,
            latest_published: None,
        };

        self.recipes.insert(name.to_string(), recipe);
        Ok(self.recipes.get(name).unwrap())
    }

    pub fn describe_recipe(
        &self,
        name: &str,
        recipe_version: Option<&str>,
    ) -> Result<&RecipeVersion, DataBrewError> {
        let recipe = self.recipes.get(name).ok_or_else(|| {
            let ver = recipe_version.unwrap_or("LATEST_PUBLISHED");
            DataBrewError::RecipeNotFoundForVersion {
                name: name.to_string(),
                version: ver.to_string(),
            }
        })?;

        match recipe_version {
            Some("LATEST_PUBLISHED") | None => {
                // Default behavior: return latest published version
                match recipe.latest_published {
                    Some(v) => {
                        let version_key = format!("{v}.0");
                        recipe.versions.get(&version_key).ok_or_else(|| {
                            DataBrewError::RecipeLatestPublishedNotFound {
                                name: name.to_string(),
                            }
                        })
                    }
                    None => Err(DataBrewError::RecipeLatestPublishedNotFound {
                        name: name.to_string(),
                    }),
                }
            }
            Some("LATEST_WORKING") => {
                // Return the latest working version (last entry with 0.x key)
                let working_versions: Vec<_> = recipe
                    .versions
                    .iter()
                    .filter(|(k, _)| k.starts_with("0."))
                    .collect();
                if let Some((_, ver)) = working_versions.last() {
                    Ok(ver)
                } else if let Some((_, ver)) = recipe.versions.iter().last() {
                    Ok(ver)
                } else {
                    Err(DataBrewError::RecipeLatestWorkingNotFound {
                        name: name.to_string(),
                    })
                }
            }
            Some(v) => recipe
                .versions
                .get(v)
                .ok_or_else(|| DataBrewError::RecipeVersionNotFound {
                    name: name.to_string(),
                    version: v.to_string(),
                }),
        }
    }

    pub fn update_recipe(
        &mut self,
        name: &str,
        description: Option<&str>,
        steps: Option<Vec<Value>>,
        account_id: &str,
    ) -> Result<String, DataBrewError> {
        let recipe = self
            .recipes
            .get_mut(name)
            .ok_or_else(|| DataBrewError::RecipeNotFound {
                name: name.to_string(),
            })?;

        let now = Utc::now();
        recipe.last_modified_by = format!("arn:aws:iam::{account_id}:root");
        recipe.last_modified_date = now;

        if let Some(desc) = description {
            recipe.description = Some(desc.to_string());
        }
        if let Some(new_steps) = &steps {
            recipe.steps = new_steps.clone();
        }

        // Create a new working version
        let new_version_str = format!("0.{}", recipe.versions.len() + 1);
        let version = RecipeVersion {
            name: name.to_string(),
            version: new_version_str.clone(),
            description: recipe.description.clone(),
            steps: recipe.steps.clone(),
            project_name: recipe.project_name.clone(),
            tags: recipe.tags.clone(),
            account_id: recipe.account_id.clone(),
            created_by: recipe.created_by.clone(),
            create_date: recipe.create_date,
            last_modified_by: recipe.last_modified_by.clone(),
            last_modified_date: now,
            resource_arn: recipe.resource_arn.clone(),
            published_by: None,
            published_date: None,
        };
        recipe.versions.insert(new_version_str, version);

        Ok(name.to_string())
    }

    pub fn publish_recipe(
        &mut self,
        name: &str,
        description: Option<&str>,
        account_id: &str,
    ) -> Result<String, DataBrewError> {
        let recipe =
            self.recipes
                .get_mut(name)
                .ok_or_else(|| DataBrewError::PublishRecipeNotFound {
                    name: name.to_string(),
                })?;

        let now = Utc::now();
        let pub_version = recipe.latest_published.unwrap_or(0) + 1;
        recipe.latest_published = Some(pub_version);
        recipe.last_modified_by = format!("arn:aws:iam::{account_id}:root");
        recipe.last_modified_date = now;

        if let Some(desc) = description {
            recipe.description = Some(desc.to_string());
        }

        let version_str = format!("{pub_version}.0");
        let version = RecipeVersion {
            name: name.to_string(),
            version: version_str.clone(),
            description: recipe.description.clone(),
            steps: recipe.steps.clone(),
            project_name: recipe.project_name.clone(),
            tags: recipe.tags.clone(),
            account_id: recipe.account_id.clone(),
            created_by: recipe.created_by.clone(),
            create_date: recipe.create_date,
            last_modified_by: recipe.last_modified_by.clone(),
            last_modified_date: now,
            resource_arn: recipe.resource_arn.clone(),
            published_by: Some(format!("arn:aws:iam::{account_id}:root")),
            published_date: Some(now),
        };
        recipe.versions.insert(version_str, version);

        // Create a new working version (N.1) with create_date = publish time
        let working_version_str = format!("{pub_version}.1");
        let working_version = RecipeVersion {
            name: name.to_string(),
            version: working_version_str.clone(),
            description: recipe.description.clone(),
            steps: recipe.steps.clone(),
            project_name: recipe.project_name.clone(),
            tags: recipe.tags.clone(),
            account_id: recipe.account_id.clone(),
            created_by: recipe.created_by.clone(),
            create_date: now,
            last_modified_by: recipe.last_modified_by.clone(),
            last_modified_date: now,
            resource_arn: recipe.resource_arn.clone(),
            published_by: None,
            published_date: None,
        };
        recipe.versions.insert(working_version_str, working_version);

        Ok(name.to_string())
    }

    pub fn delete_recipe_version(
        &mut self,
        name: &str,
        recipe_version: &str,
    ) -> Result<(String, String), DataBrewError> {
        let recipe = self
            .recipes
            .get_mut(name)
            .ok_or_else(|| DataBrewError::RecipeNotFound {
                name: name.to_string(),
            })?;

        // Resolve LATEST_WORKING to a version key
        let resolved_version = if recipe_version == "LATEST_WORKING" {
            // Find the latest working version (0.x key)
            let working: Vec<String> = recipe
                .versions
                .keys()
                .filter(|k| k.starts_with("0."))
                .cloned()
                .collect();
            if working.is_empty() {
                return Err(DataBrewError::RecipeVersionNotFound {
                    name: name.to_string(),
                    version: "LATEST_WORKING".to_string(),
                });
            }
            // After publish, deleting LATEST_WORKING when there's a published version + working version
            // is not allowed
            if let Some(pub_ver) = recipe.latest_published {
                // Find the latest working version number
                let latest_working_key = working.last().unwrap().clone();
                let expected_working = format!("{pub_ver}.1");
                if latest_working_key == expected_working || !working.is_empty() {
                    return Err(DataBrewError::RecipeLatestWorkingCannotDelete);
                }
            }
            working.last().unwrap().clone()
        } else {
            recipe_version.to_string()
        };

        // Check if version is the latest working after publish (e.g., 1.1)
        if let Some(pub_ver) = recipe.latest_published {
            let latest_working_key = format!("{pub_ver}.1");
            if resolved_version == latest_working_key {
                return Err(DataBrewError::RecipeVersionCannotDelete {
                    version: recipe_version.to_string(),
                });
            }
        }

        // Find and remove the version
        let removed = recipe.versions.remove(&resolved_version);
        if removed.is_none() {
            return Err(DataBrewError::RecipeVersionNotFound {
                name: name.to_string(),
                version: recipe_version.to_string(),
            });
        }

        // If no versions remain, remove the recipe entirely
        if recipe.versions.is_empty() {
            self.recipes.remove(name);
        }

        Ok((name.to_string(), recipe_version.to_string()))
    }

    pub fn list_recipes(&self) -> Vec<&Recipe> {
        self.recipes.values().collect()
    }

    pub fn list_recipe_versions(&self, name: &str) -> Vec<&RecipeVersion> {
        let recipe = match self.recipes.get(name) {
            Some(r) => r,
            None => return Vec::new(),
        };

        // Only return published versions (those with N.0 where N >= 1)
        recipe
            .versions
            .values()
            .filter(|v| v.published_by.is_some())
            .collect()
    }

    // ── Ruleset operations ──────────────────────────────────────────

    pub fn create_ruleset(
        &mut self,
        name: &str,
        description: Option<&str>,
        target_arn: &str,
        rules: Vec<Value>,
        tags: Option<HashMap<String, String>>,
        account_id: &str,
        region: &str,
    ) -> Result<&Ruleset, DataBrewError> {
        if self.rulesets.contains_key(name) {
            return Err(DataBrewError::RulesetAlreadyExists);
        }

        let now = Utc::now();
        let resource_arn = format!("arn:aws:databrew:{region}:{account_id}:ruleset/{name}");

        let ruleset = Ruleset {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            target_arn: target_arn.to_string(),
            rules,
            tags,
            account_id: account_id.to_string(),
            created_by: format!("arn:aws:iam::{account_id}:root"),
            create_date: now,
            last_modified_by: format!("arn:aws:iam::{account_id}:root"),
            last_modified_date: now,
            resource_arn,
        };

        self.rulesets.insert(name.to_string(), ruleset);
        Ok(self.rulesets.get(name).unwrap())
    }

    pub fn describe_ruleset(&self, name: &str) -> Result<&Ruleset, DataBrewError> {
        self.rulesets
            .get(name)
            .ok_or_else(|| DataBrewError::RulesetNotFound {
                name: name.to_string(),
            })
    }

    pub fn delete_ruleset(&mut self, name: &str) -> Result<String, DataBrewError> {
        self.rulesets
            .remove(name)
            .map(|r| r.name)
            .ok_or_else(|| DataBrewError::RulesetNotFound {
                name: name.to_string(),
            })
    }

    pub fn list_rulesets(&self) -> Vec<&Ruleset> {
        self.rulesets.values().collect()
    }

    pub fn update_ruleset(
        &mut self,
        name: &str,
        description: Option<&str>,
        rules: Option<Vec<Value>>,
        account_id: &str,
    ) -> Result<String, DataBrewError> {
        let ruleset =
            self.rulesets
                .get_mut(name)
                .ok_or_else(|| DataBrewError::RulesetNotFound {
                    name: name.to_string(),
                })?;

        let now = Utc::now();
        ruleset.last_modified_by = format!("arn:aws:iam::{account_id}:root");
        ruleset.last_modified_date = now;

        if let Some(desc) = description {
            ruleset.description = Some(desc.to_string());
        }
        if let Some(new_rules) = rules {
            ruleset.rules = new_rules;
        }

        Ok(name.to_string())
    }

    // ── Schedule operations ─────────────────────────────────────────

    pub fn create_schedule(
        &mut self,
        name: &str,
        cron_expression: Option<&str>,
        job_names: Option<Vec<String>>,
        tags: Option<HashMap<String, String>>,
        account_id: &str,
        region: &str,
    ) -> Result<&Schedule, DataBrewError> {
        if self.schedules.contains_key(name) {
            return Err(DataBrewError::ScheduleAlreadyExists {
                name: name.to_string(),
            });
        }

        let now = Utc::now();
        let resource_arn = format!("arn:aws:databrew:{region}:{account_id}:schedule/{name}");

        let schedule = Schedule {
            name: name.to_string(),
            cron_expression: cron_expression.map(|s| s.to_string()),
            job_names,
            tags,
            account_id: account_id.to_string(),
            created_by: format!("arn:aws:iam::{account_id}:root"),
            create_date: now,
            last_modified_by: format!("arn:aws:iam::{account_id}:root"),
            last_modified_date: now,
            resource_arn,
        };

        self.schedules.insert(name.to_string(), schedule);
        Ok(self.schedules.get(name).unwrap())
    }

    pub fn describe_schedule(&self, name: &str) -> Result<&Schedule, DataBrewError> {
        self.schedules
            .get(name)
            .ok_or_else(|| DataBrewError::ScheduleNotFound {
                name: name.to_string(),
            })
    }

    pub fn update_schedule(
        &mut self,
        name: &str,
        cron_expression: Option<&str>,
        job_names: Option<Vec<String>>,
        account_id: &str,
    ) -> Result<String, DataBrewError> {
        let schedule =
            self.schedules
                .get_mut(name)
                .ok_or_else(|| DataBrewError::ScheduleNotFound {
                    name: name.to_string(),
                })?;

        let now = Utc::now();
        schedule.last_modified_by = format!("arn:aws:iam::{account_id}:root");
        schedule.last_modified_date = now;

        if let Some(cron) = cron_expression {
            schedule.cron_expression = Some(cron.to_string());
        }
        if let Some(jobs) = job_names {
            schedule.job_names = Some(jobs);
        }

        Ok(name.to_string())
    }

    pub fn delete_schedule(&mut self, name: &str) -> Result<String, DataBrewError> {
        self.schedules
            .remove(name)
            .map(|s| s.name)
            .ok_or_else(|| DataBrewError::ScheduleNotFound {
                name: name.to_string(),
            })
    }

    pub fn list_schedules(&self) -> Vec<&Schedule> {
        self.schedules.values().collect()
    }

    // ── Job operations ───────────────────────────────────────────────

    pub fn create_profile_job(
        &mut self,
        name: &str,
        dataset_name: &str,
        role_arn: &str,
        output_location: Value,
        tags: Option<HashMap<String, String>>,
        account_id: &str,
        region: &str,
    ) -> Result<&Job, DataBrewError> {
        if self.jobs.contains_key(name) {
            let existing = self.jobs.get(name).unwrap();
            return Err(DataBrewError::JobAlreadyExists {
                name: name.to_string(),
                job_type: existing.job_type.to_lowercase(),
            });
        }

        let now = Utc::now();
        let resource_arn = format!("arn:aws:databrew:{region}:{account_id}:job/{name}");

        let job = Job {
            name: name.to_string(),
            job_type: "PROFILE".to_string(),
            dataset_name: Some(dataset_name.to_string()),
            project_name: None,
            role_arn: role_arn.to_string(),
            encryption_mode: None,
            encryption_key_arn: None,
            log_subscription: None,
            output_location: Some(output_location),
            outputs: None,
            tags,
            account_id: account_id.to_string(),
            created_by: format!("arn:aws:iam::{account_id}:root"),
            create_date: now,
            last_modified_by: format!("arn:aws:iam::{account_id}:root"),
            last_modified_date: now,
            resource_arn,
            max_capacity: None,
            max_retries: None,
            timeout: None,
        };

        self.jobs.insert(name.to_string(), job);
        Ok(self.jobs.get(name).unwrap())
    }

    pub fn create_recipe_job(
        &mut self,
        name: &str,
        role_arn: &str,
        dataset_name: Option<&str>,
        project_name: Option<&str>,
        encryption_mode: Option<&str>,
        log_subscription: Option<&str>,
        tags: Option<HashMap<String, String>>,
        account_id: &str,
        region: &str,
    ) -> Result<&Job, DataBrewError> {
        if self.jobs.contains_key(name) {
            let existing = self.jobs.get(name).unwrap();
            return Err(DataBrewError::JobAlreadyExists {
                name: name.to_string(),
                job_type: existing.job_type.to_lowercase(),
            });
        }

        // Validate encryption_mode
        if let Some(em) = encryption_mode
            && em != "SSE-S3"
            && em != "SSE-KMS"
        {
            return Err(DataBrewError::InvalidEncryptionMode {
                value: em.to_string(),
            });
        }

        // Validate log_subscription
        if let Some(ls) = log_subscription
            && ls != "ENABLE"
            && ls != "DISABLE"
        {
            return Err(DataBrewError::InvalidLogSubscription {
                value: ls.to_string(),
            });
        }

        let now = Utc::now();
        let resource_arn = format!("arn:aws:databrew:{region}:{account_id}:job/{name}");

        let job = Job {
            name: name.to_string(),
            job_type: "RECIPE".to_string(),
            dataset_name: dataset_name.map(|s| s.to_string()),
            project_name: project_name.map(|s| s.to_string()),
            role_arn: role_arn.to_string(),
            encryption_mode: encryption_mode.map(|s| s.to_string()),
            encryption_key_arn: None,
            log_subscription: log_subscription.map(|s| s.to_string()),
            output_location: None,
            outputs: None,
            tags,
            account_id: account_id.to_string(),
            created_by: format!("arn:aws:iam::{account_id}:root"),
            create_date: now,
            last_modified_by: format!("arn:aws:iam::{account_id}:root"),
            last_modified_date: now,
            resource_arn,
            max_capacity: None,
            max_retries: None,
            timeout: None,
        };

        self.jobs.insert(name.to_string(), job);
        Ok(self.jobs.get(name).unwrap())
    }

    pub fn describe_job(&self, name: &str) -> Result<&Job, DataBrewError> {
        self.jobs
            .get(name)
            .ok_or_else(|| DataBrewError::JobDescribeNotFound {
                name: name.to_string(),
            })
    }

    pub fn update_profile_job(
        &mut self,
        name: &str,
        role_arn: Option<&str>,
        output_location: Option<Value>,
        account_id: &str,
    ) -> Result<String, DataBrewError> {
        let job = self
            .jobs
            .get_mut(name)
            .ok_or_else(|| DataBrewError::JobUpdateNotFound {
                name: name.to_string(),
            })?;

        let now = Utc::now();
        job.last_modified_by = format!("arn:aws:iam::{account_id}:root");
        job.last_modified_date = now;

        if let Some(arn) = role_arn {
            job.role_arn = arn.to_string();
        }
        if let Some(loc) = output_location {
            job.output_location = Some(loc);
        }

        Ok(name.to_string())
    }

    pub fn update_recipe_job(
        &mut self,
        name: &str,
        role_arn: Option<&str>,
        account_id: &str,
    ) -> Result<String, DataBrewError> {
        let job = self
            .jobs
            .get_mut(name)
            .ok_or_else(|| DataBrewError::JobUpdateNotFound {
                name: name.to_string(),
            })?;

        let now = Utc::now();
        job.last_modified_by = format!("arn:aws:iam::{account_id}:root");
        job.last_modified_date = now;

        if let Some(arn) = role_arn {
            job.role_arn = arn.to_string();
        }

        Ok(name.to_string())
    }

    pub fn delete_job(&mut self, name: &str) -> Result<String, DataBrewError> {
        self.jobs
            .remove(name)
            .map(|j| j.name)
            .ok_or_else(|| DataBrewError::JobDeleteNotFound {
                name: name.to_string(),
            })
    }

    pub fn list_jobs(&self, dataset_name: Option<&str>, project_name: Option<&str>) -> Vec<&Job> {
        self.jobs
            .values()
            .filter(|j| {
                if let Some(dn) = dataset_name
                    && j.dataset_name.as_deref() != Some(dn)
                {
                    return false;
                }
                if let Some(pn) = project_name
                    && j.project_name.as_deref() != Some(pn)
                {
                    return false;
                }
                true
            })
            .collect()
    }

    // ── Tag operations ──────────────────────────────────────────────

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), DataBrewError> {
        // Find the resource by ARN and add tags
        if let Some(dataset) = self
            .datasets
            .values_mut()
            .find(|d| d.resource_arn == resource_arn)
        {
            let existing = dataset.tags.get_or_insert_with(HashMap::new);
            existing.extend(tags);
            return Ok(());
        }
        if let Some(recipe) = self
            .recipes
            .values_mut()
            .find(|r| r.resource_arn == resource_arn)
        {
            let existing = recipe.tags.get_or_insert_with(HashMap::new);
            existing.extend(tags);
            return Ok(());
        }
        if let Some(ruleset) = self
            .rulesets
            .values_mut()
            .find(|r| r.resource_arn == resource_arn)
        {
            let existing = ruleset.tags.get_or_insert_with(HashMap::new);
            existing.extend(tags);
            return Ok(());
        }
        if let Some(schedule) = self
            .schedules
            .values_mut()
            .find(|s| s.resource_arn == resource_arn)
        {
            let existing = schedule.tags.get_or_insert_with(HashMap::new);
            existing.extend(tags);
            return Ok(());
        }
        if let Some(job) = self
            .jobs
            .values_mut()
            .find(|j| j.resource_arn == resource_arn)
        {
            let existing = job.tags.get_or_insert_with(HashMap::new);
            existing.extend(tags);
            return Ok(());
        }

        Err(DataBrewError::TagResourceNotFound {
            resource_arn: resource_arn.to_string(),
        })
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), DataBrewError> {
        if let Some(dataset) = self
            .datasets
            .values_mut()
            .find(|d| d.resource_arn == resource_arn)
        {
            if let Some(tags) = &mut dataset.tags {
                for key in tag_keys {
                    tags.remove(key);
                }
            }
            return Ok(());
        }
        if let Some(recipe) = self
            .recipes
            .values_mut()
            .find(|r| r.resource_arn == resource_arn)
        {
            if let Some(tags) = &mut recipe.tags {
                for key in tag_keys {
                    tags.remove(key);
                }
            }
            return Ok(());
        }
        if let Some(ruleset) = self
            .rulesets
            .values_mut()
            .find(|r| r.resource_arn == resource_arn)
        {
            if let Some(tags) = &mut ruleset.tags {
                for key in tag_keys {
                    tags.remove(key);
                }
            }
            return Ok(());
        }
        if let Some(schedule) = self
            .schedules
            .values_mut()
            .find(|s| s.resource_arn == resource_arn)
        {
            if let Some(tags) = &mut schedule.tags {
                for key in tag_keys {
                    tags.remove(key);
                }
            }
            return Ok(());
        }
        if let Some(job) = self
            .jobs
            .values_mut()
            .find(|j| j.resource_arn == resource_arn)
        {
            if let Some(tags) = &mut job.tags {
                for key in tag_keys {
                    tags.remove(key);
                }
            }
            return Ok(());
        }

        Err(DataBrewError::TagResourceNotFound {
            resource_arn: resource_arn.to_string(),
        })
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<Option<HashMap<String, String>>, DataBrewError> {
        if let Some(dataset) = self
            .datasets
            .values()
            .find(|d| d.resource_arn == resource_arn)
        {
            return Ok(dataset.tags.clone());
        }
        if let Some(recipe) = self
            .recipes
            .values()
            .find(|r| r.resource_arn == resource_arn)
        {
            return Ok(recipe.tags.clone());
        }
        if let Some(ruleset) = self
            .rulesets
            .values()
            .find(|r| r.resource_arn == resource_arn)
        {
            return Ok(ruleset.tags.clone());
        }
        if let Some(schedule) = self
            .schedules
            .values()
            .find(|s| s.resource_arn == resource_arn)
        {
            return Ok(schedule.tags.clone());
        }
        if let Some(job) = self.jobs.values().find(|j| j.resource_arn == resource_arn) {
            return Ok(job.tags.clone());
        }

        Err(DataBrewError::TagResourceNotFound {
            resource_arn: resource_arn.to_string(),
        })
    }
}
