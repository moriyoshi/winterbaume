//! Serde-compatible view types for DataBrew state snapshots.

use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::DataBrewService;
use crate::state::DataBrewState;
use crate::types::{Dataset, Job, Recipe, RecipeVersion, Ruleset, Schedule};

/// Serializable view of the entire DataBrew state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataBrewStateView {
    #[serde(default)]
    pub datasets: HashMap<String, DatasetView>,
    #[serde(default)]
    pub recipes: HashMap<String, RecipeView>,
    #[serde(default)]
    pub rulesets: HashMap<String, RulesetView>,
    #[serde(default)]
    pub schedules: HashMap<String, ScheduleView>,
    #[serde(default)]
    pub jobs: HashMap<String, JobView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetView {
    pub name: String,
    pub account_id: String,
    pub created_by: String,
    pub create_date: DateTime<Utc>,
    pub last_modified_by: String,
    pub last_modified_date: DateTime<Utc>,
    pub source: String,
    pub format: Option<String>,
    pub format_options: Option<Value>,
    pub input: Value,
    #[serde(default)]
    pub tags: Option<HashMap<String, String>>,
    pub resource_arn: String,
    pub path_options: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeVersionView {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(default)]
    pub steps: Vec<Value>,
    pub project_name: Option<String>,
    pub tags: Option<HashMap<String, String>>,
    pub account_id: String,
    pub created_by: String,
    pub create_date: DateTime<Utc>,
    pub last_modified_by: String,
    pub last_modified_date: DateTime<Utc>,
    pub resource_arn: String,
    pub published_by: Option<String>,
    pub published_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeView {
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub steps: Vec<Value>,
    pub project_name: Option<String>,
    pub tags: Option<HashMap<String, String>>,
    pub account_id: String,
    pub created_by: String,
    pub create_date: DateTime<Utc>,
    pub last_modified_by: String,
    pub last_modified_date: DateTime<Utc>,
    pub resource_arn: String,
    #[serde(default)]
    pub versions: BTreeMap<String, RecipeVersionView>,
    pub latest_working: i64,
    pub latest_published: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetView {
    pub name: String,
    pub description: Option<String>,
    pub target_arn: String,
    #[serde(default)]
    pub rules: Vec<Value>,
    pub tags: Option<HashMap<String, String>>,
    pub account_id: String,
    pub created_by: String,
    pub create_date: DateTime<Utc>,
    pub last_modified_by: String,
    pub last_modified_date: DateTime<Utc>,
    pub resource_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleView {
    pub name: String,
    pub cron_expression: Option<String>,
    pub job_names: Option<Vec<String>>,
    pub tags: Option<HashMap<String, String>>,
    pub account_id: String,
    pub created_by: String,
    pub create_date: DateTime<Utc>,
    pub last_modified_by: String,
    pub last_modified_date: DateTime<Utc>,
    pub resource_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobView {
    pub name: String,
    pub job_type: String,
    pub dataset_name: Option<String>,
    pub project_name: Option<String>,
    pub role_arn: String,
    pub encryption_mode: Option<String>,
    pub encryption_key_arn: Option<String>,
    pub log_subscription: Option<String>,
    pub output_location: Option<Value>,
    pub outputs: Option<Value>,
    pub tags: Option<HashMap<String, String>>,
    pub account_id: String,
    pub created_by: String,
    pub create_date: DateTime<Utc>,
    pub last_modified_by: String,
    pub last_modified_date: DateTime<Utc>,
    pub resource_arn: String,
    pub max_capacity: Option<i32>,
    pub max_retries: Option<i32>,
    pub timeout: Option<i32>,
}

// --- From internal types to view types ---

impl From<&DataBrewState> for DataBrewStateView {
    fn from(state: &DataBrewState) -> Self {
        DataBrewStateView {
            datasets: state
                .datasets
                .iter()
                .map(|(k, v)| (k.clone(), DatasetView::from(v)))
                .collect(),
            recipes: state
                .recipes
                .iter()
                .map(|(k, v)| (k.clone(), RecipeView::from(v)))
                .collect(),
            rulesets: state
                .rulesets
                .iter()
                .map(|(k, v)| (k.clone(), RulesetView::from(v)))
                .collect(),
            schedules: state
                .schedules
                .iter()
                .map(|(k, v)| (k.clone(), ScheduleView::from(v)))
                .collect(),
            jobs: state
                .jobs
                .iter()
                .map(|(k, v)| (k.clone(), JobView::from(v)))
                .collect(),
        }
    }
}

impl From<&Dataset> for DatasetView {
    fn from(d: &Dataset) -> Self {
        DatasetView {
            name: d.name.clone(),
            account_id: d.account_id.clone(),
            created_by: d.created_by.clone(),
            create_date: d.create_date,
            last_modified_by: d.last_modified_by.clone(),
            last_modified_date: d.last_modified_date,
            source: d.source.clone(),
            format: d.format.clone(),
            format_options: d.format_options.clone(),
            input: d.input.clone(),
            tags: d.tags.clone(),
            resource_arn: d.resource_arn.clone(),
            path_options: d.path_options.clone(),
        }
    }
}

impl From<&RecipeVersion> for RecipeVersionView {
    fn from(rv: &RecipeVersion) -> Self {
        RecipeVersionView {
            name: rv.name.clone(),
            version: rv.version.clone(),
            description: rv.description.clone(),
            steps: rv.steps.clone(),
            project_name: rv.project_name.clone(),
            tags: rv.tags.clone(),
            account_id: rv.account_id.clone(),
            created_by: rv.created_by.clone(),
            create_date: rv.create_date,
            last_modified_by: rv.last_modified_by.clone(),
            last_modified_date: rv.last_modified_date,
            resource_arn: rv.resource_arn.clone(),
            published_by: rv.published_by.clone(),
            published_date: rv.published_date,
        }
    }
}

impl From<&Recipe> for RecipeView {
    fn from(r: &Recipe) -> Self {
        RecipeView {
            name: r.name.clone(),
            description: r.description.clone(),
            steps: r.steps.clone(),
            project_name: r.project_name.clone(),
            tags: r.tags.clone(),
            account_id: r.account_id.clone(),
            created_by: r.created_by.clone(),
            create_date: r.create_date,
            last_modified_by: r.last_modified_by.clone(),
            last_modified_date: r.last_modified_date,
            resource_arn: r.resource_arn.clone(),
            versions: r
                .versions
                .iter()
                .map(|(k, v)| (k.clone(), RecipeVersionView::from(v)))
                .collect(),
            latest_working: r.latest_working,
            latest_published: r.latest_published,
        }
    }
}

impl From<&Ruleset> for RulesetView {
    fn from(rs: &Ruleset) -> Self {
        RulesetView {
            name: rs.name.clone(),
            description: rs.description.clone(),
            target_arn: rs.target_arn.clone(),
            rules: rs.rules.clone(),
            tags: rs.tags.clone(),
            account_id: rs.account_id.clone(),
            created_by: rs.created_by.clone(),
            create_date: rs.create_date,
            last_modified_by: rs.last_modified_by.clone(),
            last_modified_date: rs.last_modified_date,
            resource_arn: rs.resource_arn.clone(),
        }
    }
}

impl From<&Schedule> for ScheduleView {
    fn from(s: &Schedule) -> Self {
        ScheduleView {
            name: s.name.clone(),
            cron_expression: s.cron_expression.clone(),
            job_names: s.job_names.clone(),
            tags: s.tags.clone(),
            account_id: s.account_id.clone(),
            created_by: s.created_by.clone(),
            create_date: s.create_date,
            last_modified_by: s.last_modified_by.clone(),
            last_modified_date: s.last_modified_date,
            resource_arn: s.resource_arn.clone(),
        }
    }
}

impl From<&Job> for JobView {
    fn from(j: &Job) -> Self {
        JobView {
            name: j.name.clone(),
            job_type: j.job_type.clone(),
            dataset_name: j.dataset_name.clone(),
            project_name: j.project_name.clone(),
            role_arn: j.role_arn.clone(),
            encryption_mode: j.encryption_mode.clone(),
            encryption_key_arn: j.encryption_key_arn.clone(),
            log_subscription: j.log_subscription.clone(),
            output_location: j.output_location.clone(),
            outputs: j.outputs.clone(),
            tags: j.tags.clone(),
            account_id: j.account_id.clone(),
            created_by: j.created_by.clone(),
            create_date: j.create_date,
            last_modified_by: j.last_modified_by.clone(),
            last_modified_date: j.last_modified_date,
            resource_arn: j.resource_arn.clone(),
            max_capacity: j.max_capacity,
            max_retries: j.max_retries,
            timeout: j.timeout,
        }
    }
}

// --- From view types to internal types ---

impl From<DataBrewStateView> for DataBrewState {
    fn from(view: DataBrewStateView) -> Self {
        DataBrewState {
            datasets: view
                .datasets
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Dataset {
                            name: v.name,
                            account_id: v.account_id,
                            created_by: v.created_by,
                            create_date: v.create_date,
                            last_modified_by: v.last_modified_by,
                            last_modified_date: v.last_modified_date,
                            source: v.source,
                            format: v.format,
                            format_options: v.format_options,
                            input: v.input,
                            tags: v.tags,
                            resource_arn: v.resource_arn,
                            path_options: v.path_options,
                        },
                    )
                })
                .collect(),
            recipes: view
                .recipes
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Recipe {
                            name: v.name,
                            description: v.description,
                            steps: v.steps,
                            project_name: v.project_name,
                            tags: v.tags,
                            account_id: v.account_id,
                            created_by: v.created_by,
                            create_date: v.create_date,
                            last_modified_by: v.last_modified_by,
                            last_modified_date: v.last_modified_date,
                            resource_arn: v.resource_arn,
                            versions: v
                                .versions
                                .into_iter()
                                .map(|(vk, vv)| {
                                    (
                                        vk,
                                        RecipeVersion {
                                            name: vv.name,
                                            version: vv.version,
                                            description: vv.description,
                                            steps: vv.steps,
                                            project_name: vv.project_name,
                                            tags: vv.tags,
                                            account_id: vv.account_id,
                                            created_by: vv.created_by,
                                            create_date: vv.create_date,
                                            last_modified_by: vv.last_modified_by,
                                            last_modified_date: vv.last_modified_date,
                                            resource_arn: vv.resource_arn,
                                            published_by: vv.published_by,
                                            published_date: vv.published_date,
                                        },
                                    )
                                })
                                .collect(),
                            latest_working: v.latest_working,
                            latest_published: v.latest_published,
                        },
                    )
                })
                .collect(),
            rulesets: view
                .rulesets
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Ruleset {
                            name: v.name,
                            description: v.description,
                            target_arn: v.target_arn,
                            rules: v.rules,
                            tags: v.tags,
                            account_id: v.account_id,
                            created_by: v.created_by,
                            create_date: v.create_date,
                            last_modified_by: v.last_modified_by,
                            last_modified_date: v.last_modified_date,
                            resource_arn: v.resource_arn,
                        },
                    )
                })
                .collect(),
            schedules: view
                .schedules
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Schedule {
                            name: v.name,
                            cron_expression: v.cron_expression,
                            job_names: v.job_names,
                            tags: v.tags,
                            account_id: v.account_id,
                            created_by: v.created_by,
                            create_date: v.create_date,
                            last_modified_by: v.last_modified_by,
                            last_modified_date: v.last_modified_date,
                            resource_arn: v.resource_arn,
                        },
                    )
                })
                .collect(),
            jobs: view
                .jobs
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Job {
                            name: v.name,
                            job_type: v.job_type,
                            dataset_name: v.dataset_name,
                            project_name: v.project_name,
                            role_arn: v.role_arn,
                            encryption_mode: v.encryption_mode,
                            encryption_key_arn: v.encryption_key_arn,
                            log_subscription: v.log_subscription,
                            output_location: v.output_location,
                            outputs: v.outputs,
                            tags: v.tags,
                            account_id: v.account_id,
                            created_by: v.created_by,
                            create_date: v.create_date,
                            last_modified_by: v.last_modified_by,
                            last_modified_date: v.last_modified_date,
                            resource_arn: v.resource_arn,
                            max_capacity: v.max_capacity,
                            max_retries: v.max_retries,
                            timeout: v.timeout,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for DataBrewService {
    type StateView = DataBrewStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        DataBrewStateView::from(&*guard)
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
            *guard = DataBrewState::from(view);
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
            let incoming = DataBrewState::from(view);
            for (k, v) in incoming.datasets {
                guard.datasets.insert(k, v);
            }
            for (k, v) in incoming.recipes {
                guard.recipes.insert(k, v);
            }
            for (k, v) in incoming.rulesets {
                guard.rulesets.insert(k, v);
            }
            for (k, v) in incoming.schedules {
                guard.schedules.insert(k, v);
            }
            for (k, v) in incoming.jobs {
                guard.jobs.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
