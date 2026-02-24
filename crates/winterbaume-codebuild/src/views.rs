//! Serde-compatible view types for CodeBuild state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CodeBuildService;
use crate::state::CodeBuildState;
use crate::types::{Build, BuildPhase, Project, ReportGroup, SourceCredential, Tag, Webhook};

/// Serializable view of the entire CodeBuild state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeBuildStateView {
    /// Projects keyed by project name.
    #[serde(default)]
    pub projects: HashMap<String, ProjectView>,
    /// Builds keyed by build ID.
    #[serde(default)]
    pub builds: HashMap<String, BuildView>,
    /// Ordered list of build IDs.
    #[serde(default)]
    pub build_ids: Vec<String>,
    /// Next build number per project.
    #[serde(default)]
    pub build_counters: HashMap<String, i64>,
    /// Webhooks keyed by project name.
    #[serde(default)]
    pub webhooks: HashMap<String, WebhookView>,
    /// Source credentials keyed by ARN.
    #[serde(default)]
    pub source_credentials: HashMap<String, SourceCredentialView>,
    /// Resource policies keyed by resource ARN.
    #[serde(default)]
    pub resource_policies: HashMap<String, String>,
    /// Report groups keyed by ARN.
    #[serde(default)]
    pub report_groups: HashMap<String, ReportGroupView>,
    /// Ordered list of report group ARNs.
    #[serde(default)]
    pub report_group_arns: Vec<String>,
}

/// Serializable view of a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

/// Serializable view of a CodeBuild project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectView {
    pub name: String,
    pub arn: String,
    pub description: String,
    pub source_type: String,
    pub source_location: String,
    pub artifact_type: String,
    pub artifact_location: Option<String>,
    pub environment_type: String,
    pub environment_image: String,
    pub environment_compute_type: String,
    pub service_role: String,
    #[serde(default)]
    pub tags: Vec<TagView>,
    pub created: String,
    pub last_modified: String,
    /// `build_batch_config` nested block.
    #[serde(default)]
    pub build_batch_config: Option<serde_json::Value>,
    /// `cache` nested block.
    #[serde(default)]
    pub cache: Option<serde_json::Value>,
    /// `file_system_locations` nested blocks.
    #[serde(default)]
    pub file_system_locations: Vec<serde_json::Value>,
    /// `logs_config` nested block.
    #[serde(default)]
    pub logs_config: Option<serde_json::Value>,
    /// `secondary_artifacts` nested blocks.
    #[serde(default)]
    pub secondary_artifacts: Vec<serde_json::Value>,
    /// `secondary_sources` nested blocks.
    #[serde(default)]
    pub secondary_sources: Vec<serde_json::Value>,
    /// `vpc_config` nested block.
    #[serde(default)]
    pub vpc_config: Option<serde_json::Value>,
}

/// Serializable view of a build phase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildPhaseView {
    pub phase_type: String,
    pub phase_status: Option<String>,
    pub start_time: f64,
    pub end_time: Option<f64>,
    pub duration_in_seconds: Option<i64>,
}

/// Serializable view of a CodeBuild build.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildView {
    pub id: String,
    pub arn: String,
    pub project_name: String,
    pub build_status: String,
    pub current_phase: String,
    pub source_type: String,
    pub source_location: String,
    pub source_version: String,
    pub artifact_type: String,
    pub artifact_location: Option<String>,
    pub environment_type: String,
    pub environment_image: String,
    pub environment_compute_type: String,
    pub service_role: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub build_number: i64,
    #[serde(default)]
    pub phases: Vec<BuildPhaseView>,
}

/// Serializable view of a CodeBuild webhook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookView {
    pub project_name: String,
    pub url: String,
    pub branch_filter: Option<String>,
    pub build_type: Option<String>,
    pub secret: Option<String>,
}

/// Serializable view of a source credential.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceCredentialView {
    pub arn: String,
    pub server_type: String,
    pub auth_type: String,
    pub resource: Option<String>,
}

/// Serializable view of a report group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportGroupView {
    pub arn: String,
    pub name: String,
    pub r#type: String,
    pub export_config_type: Option<String>,
    #[serde(default)]
    pub tags: Vec<TagView>,
    pub created: String,
    pub last_modified: String,
    pub status: String,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&Tag> for TagView {
    fn from(t: &Tag) -> Self {
        TagView {
            key: t.key.clone(),
            value: t.value.clone(),
        }
    }
}

impl From<&Project> for ProjectView {
    fn from(p: &Project) -> Self {
        ProjectView {
            name: p.name.clone(),
            arn: p.arn.clone(),
            description: p.description.clone(),
            source_type: p.source_type.clone(),
            source_location: p.source_location.clone(),
            artifact_type: p.artifact_type.clone(),
            artifact_location: p.artifact_location.clone(),
            environment_type: p.environment_type.clone(),
            environment_image: p.environment_image.clone(),
            environment_compute_type: p.environment_compute_type.clone(),
            service_role: p.service_role.clone(),
            tags: p.tags.iter().map(TagView::from).collect(),
            created: p.created.to_rfc3339(),
            last_modified: p.last_modified.to_rfc3339(),
            build_batch_config: None,
            cache: None,
            file_system_locations: vec![],
            logs_config: None,
            secondary_artifacts: vec![],
            secondary_sources: vec![],
            vpc_config: None,
        }
    }
}

impl From<&BuildPhase> for BuildPhaseView {
    fn from(ph: &BuildPhase) -> Self {
        BuildPhaseView {
            phase_type: ph.phase_type.clone(),
            phase_status: ph.phase_status.clone(),
            start_time: ph.start_time,
            end_time: ph.end_time,
            duration_in_seconds: ph.duration_in_seconds,
        }
    }
}

impl From<&Build> for BuildView {
    fn from(b: &Build) -> Self {
        BuildView {
            id: b.id.clone(),
            arn: b.arn.clone(),
            project_name: b.project_name.clone(),
            build_status: b.build_status.clone(),
            current_phase: b.current_phase.clone(),
            source_type: b.source_type.clone(),
            source_location: b.source_location.clone(),
            source_version: b.source_version.clone(),
            artifact_type: b.artifact_type.clone(),
            artifact_location: b.artifact_location.clone(),
            environment_type: b.environment_type.clone(),
            environment_image: b.environment_image.clone(),
            environment_compute_type: b.environment_compute_type.clone(),
            service_role: b.service_role.clone(),
            start_time: b.start_time.to_rfc3339(),
            end_time: b.end_time.as_ref().map(|d| d.to_rfc3339()),
            build_number: b.build_number,
            phases: b.phases.iter().map(BuildPhaseView::from).collect(),
        }
    }
}

impl From<&Webhook> for WebhookView {
    fn from(w: &Webhook) -> Self {
        WebhookView {
            project_name: w.project_name.clone(),
            url: w.url.clone(),
            branch_filter: w.branch_filter.clone(),
            build_type: w.build_type.clone(),
            secret: w.secret.clone(),
        }
    }
}

impl From<&SourceCredential> for SourceCredentialView {
    fn from(c: &SourceCredential) -> Self {
        SourceCredentialView {
            arn: c.arn.clone(),
            server_type: c.server_type.clone(),
            auth_type: c.auth_type.clone(),
            resource: c.resource.clone(),
        }
    }
}

impl From<&ReportGroup> for ReportGroupView {
    fn from(rg: &ReportGroup) -> Self {
        ReportGroupView {
            arn: rg.arn.clone(),
            name: rg.name.clone(),
            r#type: rg.r#type.clone(),
            export_config_type: rg.export_config_type.clone(),
            tags: rg.tags.iter().map(TagView::from).collect(),
            created: rg.created.to_rfc3339(),
            last_modified: rg.last_modified.to_rfc3339(),
            status: rg.status.clone(),
        }
    }
}

impl From<&CodeBuildState> for CodeBuildStateView {
    fn from(s: &CodeBuildState) -> Self {
        let projects = s
            .projects
            .iter()
            .map(|(k, v)| (k.clone(), ProjectView::from(v)))
            .collect();
        let builds = s
            .builds
            .iter()
            .map(|(k, v)| (k.clone(), BuildView::from(v)))
            .collect();
        let webhooks = s
            .webhooks
            .iter()
            .map(|(k, v)| (k.clone(), WebhookView::from(v)))
            .collect();
        let source_credentials = s
            .source_credentials
            .iter()
            .map(|(k, v)| (k.clone(), SourceCredentialView::from(v)))
            .collect();
        let report_groups = s
            .report_groups
            .iter()
            .map(|(k, v)| (k.clone(), ReportGroupView::from(v)))
            .collect();
        CodeBuildStateView {
            projects,
            builds,
            build_ids: s.build_ids.clone(),
            build_counters: s.build_counters.clone(),
            webhooks,
            source_credentials,
            resource_policies: s.resource_policies.clone(),
            report_groups,
            report_group_arns: s.report_group_arns.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for CodeBuildService {
    type StateView = CodeBuildStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CodeBuildStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let mut new_state = CodeBuildState::default();

        for (name, pv) in view.projects {
            let created = DateTime::parse_from_rfc3339(&pv.created)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let last_modified = DateTime::parse_from_rfc3339(&pv.last_modified)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            new_state.projects.insert(
                name,
                Project {
                    name: pv.name,
                    arn: pv.arn,
                    description: pv.description,
                    source_type: pv.source_type,
                    source_location: pv.source_location,
                    artifact_type: pv.artifact_type,
                    artifact_location: pv.artifact_location,
                    environment_type: pv.environment_type,
                    environment_image: pv.environment_image,
                    environment_compute_type: pv.environment_compute_type,
                    service_role: pv.service_role,
                    tags: pv
                        .tags
                        .into_iter()
                        .map(|t| Tag {
                            key: t.key,
                            value: t.value,
                        })
                        .collect(),
                    created,
                    last_modified,
                },
            );
        }

        for (id, bv) in view.builds {
            let start_time = DateTime::parse_from_rfc3339(&bv.start_time)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let end_time = bv
                .end_time
                .as_deref()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|d| d.with_timezone(&Utc));
            new_state.builds.insert(
                id,
                Build {
                    id: bv.id,
                    arn: bv.arn,
                    project_name: bv.project_name,
                    build_status: bv.build_status,
                    current_phase: bv.current_phase,
                    source_type: bv.source_type,
                    source_location: bv.source_location,
                    source_version: bv.source_version,
                    artifact_type: bv.artifact_type,
                    artifact_location: bv.artifact_location,
                    environment_type: bv.environment_type,
                    environment_image: bv.environment_image,
                    environment_compute_type: bv.environment_compute_type,
                    service_role: bv.service_role,
                    start_time,
                    end_time,
                    build_number: bv.build_number,
                    phases: bv
                        .phases
                        .into_iter()
                        .map(|ph| BuildPhase {
                            phase_type: ph.phase_type,
                            phase_status: ph.phase_status,
                            start_time: ph.start_time,
                            end_time: ph.end_time,
                            duration_in_seconds: ph.duration_in_seconds,
                        })
                        .collect(),
                },
            );
        }

        new_state.build_ids = view.build_ids;
        new_state.build_counters = view.build_counters;

        for (name, wv) in view.webhooks {
            new_state.webhooks.insert(
                name,
                Webhook {
                    project_name: wv.project_name,
                    url: wv.url,
                    branch_filter: wv.branch_filter,
                    build_type: wv.build_type,
                    secret: wv.secret,
                },
            );
        }

        for (arn, cv) in view.source_credentials {
            new_state.source_credentials.insert(
                arn,
                SourceCredential {
                    arn: cv.arn,
                    server_type: cv.server_type,
                    auth_type: cv.auth_type,
                    resource: cv.resource,
                },
            );
        }

        new_state.resource_policies = view.resource_policies;

        for (arn, rgv) in view.report_groups {
            let created = DateTime::parse_from_rfc3339(&rgv.created)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let last_modified = DateTime::parse_from_rfc3339(&rgv.last_modified)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            new_state.report_groups.insert(
                arn,
                ReportGroup {
                    arn: rgv.arn,
                    name: rgv.name,
                    r#type: rgv.r#type,
                    export_config_type: rgv.export_config_type,
                    tags: rgv
                        .tags
                        .into_iter()
                        .map(|t| Tag {
                            key: t.key,
                            value: t.value,
                        })
                        .collect(),
                    created,
                    last_modified,
                    status: rgv.status,
                },
            );
        }

        new_state.report_group_arns = view.report_group_arns;

        {
            let state = self.state.get(account_id, region);
            *state.write().await = new_state;
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

            for (name, pv) in view.projects {
                let created = DateTime::parse_from_rfc3339(&pv.created)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());
                let last_modified = DateTime::parse_from_rfc3339(&pv.last_modified)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());
                guard.projects.insert(
                    name,
                    Project {
                        name: pv.name,
                        arn: pv.arn,
                        description: pv.description,
                        source_type: pv.source_type,
                        source_location: pv.source_location,
                        artifact_type: pv.artifact_type,
                        artifact_location: pv.artifact_location,
                        environment_type: pv.environment_type,
                        environment_image: pv.environment_image,
                        environment_compute_type: pv.environment_compute_type,
                        service_role: pv.service_role,
                        tags: pv
                            .tags
                            .into_iter()
                            .map(|t| Tag {
                                key: t.key,
                                value: t.value,
                            })
                            .collect(),
                        created,
                        last_modified,
                    },
                );
            }

            for (id, bv) in view.builds {
                let start_time = DateTime::parse_from_rfc3339(&bv.start_time)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());
                let end_time = bv
                    .end_time
                    .as_deref()
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|d| d.with_timezone(&Utc));
                if !guard.build_ids.contains(&bv.id) {
                    guard.build_ids.push(bv.id.clone());
                }
                guard.builds.insert(
                    id,
                    Build {
                        id: bv.id,
                        arn: bv.arn,
                        project_name: bv.project_name,
                        build_status: bv.build_status,
                        current_phase: bv.current_phase,
                        source_type: bv.source_type,
                        source_location: bv.source_location,
                        source_version: bv.source_version,
                        artifact_type: bv.artifact_type,
                        artifact_location: bv.artifact_location,
                        environment_type: bv.environment_type,
                        environment_image: bv.environment_image,
                        environment_compute_type: bv.environment_compute_type,
                        service_role: bv.service_role,
                        start_time,
                        end_time,
                        build_number: bv.build_number,
                        phases: bv
                            .phases
                            .into_iter()
                            .map(|ph| BuildPhase {
                                phase_type: ph.phase_type,
                                phase_status: ph.phase_status,
                                start_time: ph.start_time,
                                end_time: ph.end_time,
                                duration_in_seconds: ph.duration_in_seconds,
                            })
                            .collect(),
                    },
                );
            }

            for (name, counter) in view.build_counters {
                let entry = guard.build_counters.entry(name).or_insert(0);
                if counter > *entry {
                    *entry = counter;
                }
            }

            for (name, wv) in view.webhooks {
                guard.webhooks.insert(
                    name,
                    Webhook {
                        project_name: wv.project_name,
                        url: wv.url,
                        branch_filter: wv.branch_filter,
                        build_type: wv.build_type,
                        secret: wv.secret,
                    },
                );
            }

            for (arn, cv) in view.source_credentials {
                guard.source_credentials.insert(
                    arn,
                    SourceCredential {
                        arn: cv.arn,
                        server_type: cv.server_type,
                        auth_type: cv.auth_type,
                        resource: cv.resource,
                    },
                );
            }

            for (arn, policy) in view.resource_policies {
                guard.resource_policies.insert(arn, policy);
            }

            for (arn, rgv) in view.report_groups {
                let created = DateTime::parse_from_rfc3339(&rgv.created)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());
                let last_modified = DateTime::parse_from_rfc3339(&rgv.last_modified)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());
                if !guard.report_group_arns.contains(&rgv.arn) {
                    guard.report_group_arns.push(rgv.arn.clone());
                }
                guard.report_groups.insert(
                    arn,
                    ReportGroup {
                        arn: rgv.arn,
                        name: rgv.name,
                        r#type: rgv.r#type,
                        export_config_type: rgv.export_config_type,
                        tags: rgv
                            .tags
                            .into_iter()
                            .map(|t| Tag {
                                key: t.key,
                                value: t.value,
                            })
                            .collect(),
                        created,
                        last_modified,
                        status: rgv.status,
                    },
                );
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
