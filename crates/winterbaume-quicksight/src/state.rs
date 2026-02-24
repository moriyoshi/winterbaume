use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct QuickSightState {
    pub data_sets: HashMap<String, DataSet>,
    pub data_sources: HashMap<String, QuickSightDataSource>,
    pub dashboards: HashMap<String, Dashboard>,
    pub groups: HashMap<String, HashMap<String, QuickSightGroup>>, // namespace -> group_name -> group
    pub users: HashMap<String, HashMap<String, QuickSightUser>>,   // namespace -> user_name -> user
    pub group_memberships: HashMap<String, HashMap<String, Vec<String>>>, // namespace -> group_name -> [user_name]
    pub ingestions: HashMap<String, Ingestion>, // "dataSetId/ingestionId" -> ingestion
    pub account_settings: AccountSettings,
    pub tags: HashMap<String, HashMap<String, String>>, // arn -> tag_key -> tag_value
    pub public_sharing_enabled: bool,
    pub analyses: HashMap<String, QuickSightAnalysis>,
    pub folders: HashMap<String, QuickSightFolder>,
    pub folder_members: HashMap<String, Vec<FolderMemberEntry>>, // folder_id -> members
    pub namespaces: HashMap<String, QuickSightNamespace>,
    pub templates: HashMap<String, QuickSightTemplate>,
    pub themes: HashMap<String, QuickSightTheme>,
}

#[derive(Debug, thiserror::Error)]
pub enum QuickSightError {
    // ResourceExistsException (409)
    #[error("A resource with the ID '{id}' already exists.")]
    ResourceExists { id: String },
    #[error("Group '{group_name}' already exists.")]
    GroupExists { group_name: String },
    #[error("User '{user_name}' already exists.")]
    UserExists { user_name: String },
    #[error("Namespace '{namespace}' already exists.")]
    NamespaceExists { namespace: String },

    // ResourceNotFoundException (404)
    #[error(
        "An error occurred (ResourceNotFoundException) when calling \
         the DescribeDataSet operation: DataSet '{data_set_id}' is not found."
    )]
    DescribeDataSetNotFound { data_set_id: String },
    #[error(
        "An error occurred (ResourceNotFoundException) when calling \
         the UpdateDataSet operation: DataSet '{data_set_id}' is not found."
    )]
    UpdateDataSetNotFound { data_set_id: String },
    #[error(
        "An error occurred (ResourceNotFoundException) when calling \
         the DeleteDataSet operation: DataSet '{data_set_id}' is not found."
    )]
    DeleteDataSetNotFound { data_set_id: String },
    #[error("DataSource '{data_source_id}' is not found.")]
    DataSourceNotFound { data_source_id: String },
    #[error("Dashboard '{dashboard_id}' is not found.")]
    DashboardNotFound { dashboard_id: String },
    #[error("Group '{group_name}' is not found.")]
    GroupNotFound { group_name: String },
    #[error("Member '{member_name}' is not in group '{group_name}'.")]
    MemberNotInGroup {
        member_name: String,
        group_name: String,
    },
    #[error("User '{user_name}' is not found.")]
    UserNotFound { user_name: String },
    #[error("DataSet '{data_set_id}' is not found.")]
    DataSetNotFound { data_set_id: String },
    #[error("Analysis '{analysis_id}' is not found.")]
    AnalysisNotFound { analysis_id: String },
    #[error("Folder '{folder_id}' is not found.")]
    FolderNotFound { folder_id: String },
    #[error("Member '{member_id}' is not in folder '{folder_id}'.")]
    MemberNotInFolder {
        member_id: String,
        folder_id: String,
    },
    #[error("Namespace '{namespace}' is not found.")]
    NamespaceNotFound { namespace: String },
    #[error("Template '{template_id}' is not found.")]
    TemplateNotFound { template_id: String },
    #[error("Theme '{theme_id}' is not found.")]
    ThemeNotFound { theme_id: String },
    #[error("Ingestion '{ingestion_id}' for DataSet '{data_set_id}' is not found.")]
    IngestionNotFound {
        ingestion_id: String,
        data_set_id: String,
    },
}

impl QuickSightState {
    // ---- DataSet ----

    pub fn create_data_set(
        &mut self,
        data_set_id: &str,
        name: &str,
        import_mode: &str,
        physical_table_map: HashMap<String, serde_json::Value>,
        account_id: &str,
        region: &str,
    ) -> Result<&DataSet, QuickSightError> {
        if self.data_sets.contains_key(data_set_id) {
            return Err(QuickSightError::ResourceExists {
                id: data_set_id.to_string(),
            });
        }

        let arn = format!("arn:aws:quicksight:{region}:{account_id}:dataset/{data_set_id}");
        let now = Utc::now();

        let data_set = DataSet {
            data_set_id: data_set_id.to_string(),
            name: name.to_string(),
            arn,
            import_mode: import_mode.to_string(),
            physical_table_map,
            created_time: now,
            last_updated_time: now,
        };

        self.data_sets.insert(data_set_id.to_string(), data_set);
        Ok(self.data_sets.get(data_set_id).unwrap())
    }

    pub fn describe_data_set(&self, data_set_id: &str) -> Result<&DataSet, QuickSightError> {
        self.data_sets
            .get(data_set_id)
            .ok_or_else(|| QuickSightError::DescribeDataSetNotFound {
                data_set_id: data_set_id.to_string(),
            })
    }

    pub fn update_data_set(
        &mut self,
        data_set_id: &str,
        name: Option<&str>,
        import_mode: Option<&str>,
        physical_table_map: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<&DataSet, QuickSightError> {
        let ds = self.data_sets.get_mut(data_set_id).ok_or_else(|| {
            QuickSightError::UpdateDataSetNotFound {
                data_set_id: data_set_id.to_string(),
            }
        })?;

        if let Some(n) = name {
            ds.name = n.to_string();
        }
        if let Some(m) = import_mode {
            ds.import_mode = m.to_string();
        }
        if let Some(ptm) = physical_table_map {
            ds.physical_table_map = ptm;
        }
        ds.last_updated_time = Utc::now();

        Ok(self.data_sets.get(data_set_id).unwrap())
    }

    pub fn delete_data_set(&mut self, data_set_id: &str) -> Result<String, QuickSightError> {
        match self.data_sets.remove(data_set_id) {
            Some(ds) => Ok(ds.arn),
            None => Err(QuickSightError::DeleteDataSetNotFound {
                data_set_id: data_set_id.to_string(),
            }),
        }
    }

    pub fn list_data_sets(&self) -> Vec<&DataSet> {
        self.data_sets.values().collect()
    }

    // ---- DataSource ----

    pub fn create_data_source(
        &mut self,
        data_source_id: &str,
        name: &str,
        ds_type: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&QuickSightDataSource, QuickSightError> {
        if self.data_sources.contains_key(data_source_id) {
            return Err(QuickSightError::ResourceExists {
                id: data_source_id.to_string(),
            });
        }

        let arn = format!("arn:aws:quicksight:{region}:{account_id}:datasource/{data_source_id}");
        let now = Utc::now();

        let ds = QuickSightDataSource {
            data_source_id: data_source_id.to_string(),
            name: name.to_string(),
            arn,
            r#type: ds_type.to_string(),
            status: "CREATION_SUCCESSFUL".to_string(),
            created_time: now,
            last_updated_time: now,
            permissions: Vec::new(),
        };

        self.data_sources.insert(data_source_id.to_string(), ds);
        Ok(self.data_sources.get(data_source_id).unwrap())
    }

    pub fn describe_data_source_permissions(
        &self,
        data_source_id: &str,
    ) -> Result<(String, Vec<DataSourceResourcePermission>), QuickSightError> {
        let ds = self.data_sources.get(data_source_id).ok_or_else(|| {
            QuickSightError::DataSourceNotFound {
                data_source_id: data_source_id.to_string(),
            }
        })?;
        Ok((ds.arn.clone(), ds.permissions.clone()))
    }

    pub fn update_data_source_permissions(
        &mut self,
        data_source_id: &str,
        grants: Vec<DataSourceResourcePermission>,
        revokes: Vec<DataSourceResourcePermission>,
    ) -> Result<(String, Vec<DataSourceResourcePermission>), QuickSightError> {
        let ds = self.data_sources.get_mut(data_source_id).ok_or_else(|| {
            QuickSightError::DataSourceNotFound {
                data_source_id: data_source_id.to_string(),
            }
        })?;

        // Apply revokes first: remove specific actions for matching principals.
        for revoke in &revokes {
            if let Some(existing) = ds
                .permissions
                .iter_mut()
                .find(|p| p.principal == revoke.principal)
            {
                existing
                    .actions
                    .retain(|a| !revoke.actions.iter().any(|ra| ra == a));
            }
        }
        // Drop principals with empty action lists after revoke.
        ds.permissions.retain(|p| !p.actions.is_empty());

        // Apply grants: merge actions per principal, deduplicating.
        for grant in grants {
            if let Some(existing) = ds
                .permissions
                .iter_mut()
                .find(|p| p.principal == grant.principal)
            {
                for action in grant.actions {
                    if !existing.actions.contains(&action) {
                        existing.actions.push(action);
                    }
                }
            } else {
                ds.permissions.push(grant);
            }
        }

        Ok((ds.arn.clone(), ds.permissions.clone()))
    }

    pub fn describe_data_source(
        &self,
        data_source_id: &str,
    ) -> Result<&QuickSightDataSource, QuickSightError> {
        self.data_sources
            .get(data_source_id)
            .ok_or_else(|| QuickSightError::DataSourceNotFound {
                data_source_id: data_source_id.to_string(),
            })
    }

    pub fn update_data_source(
        &mut self,
        data_source_id: &str,
        name: Option<&str>,
    ) -> Result<&QuickSightDataSource, QuickSightError> {
        let ds = self.data_sources.get_mut(data_source_id).ok_or_else(|| {
            QuickSightError::DataSourceNotFound {
                data_source_id: data_source_id.to_string(),
            }
        })?;

        if let Some(n) = name {
            ds.name = n.to_string();
        }
        ds.status = "UPDATE_SUCCESSFUL".to_string();
        ds.last_updated_time = Utc::now();

        Ok(self.data_sources.get(data_source_id).unwrap())
    }

    pub fn delete_data_source(&mut self, data_source_id: &str) -> Result<String, QuickSightError> {
        match self.data_sources.remove(data_source_id) {
            Some(ds) => Ok(ds.arn),
            None => Err(QuickSightError::DataSourceNotFound {
                data_source_id: data_source_id.to_string(),
            }),
        }
    }

    pub fn list_data_sources(&self) -> Vec<&QuickSightDataSource> {
        self.data_sources.values().collect()
    }

    // ---- Dashboard ----

    pub fn create_dashboard(
        &mut self,
        dashboard_id: &str,
        name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&Dashboard, QuickSightError> {
        if self.dashboards.contains_key(dashboard_id) {
            return Err(QuickSightError::ResourceExists {
                id: dashboard_id.to_string(),
            });
        }

        let arn = format!("arn:aws:quicksight:{region}:{account_id}:dashboard/{dashboard_id}");
        let version_arn = format!("{arn}/version/1");
        let now = Utc::now();

        let db = Dashboard {
            dashboard_id: dashboard_id.to_string(),
            name: name.to_string(),
            arn,
            version_arn,
            created_time: now,
            last_updated_time: now,
        };

        self.dashboards.insert(dashboard_id.to_string(), db);
        Ok(self.dashboards.get(dashboard_id).unwrap())
    }

    pub fn describe_dashboard(&self, dashboard_id: &str) -> Result<&Dashboard, QuickSightError> {
        self.dashboards
            .get(dashboard_id)
            .ok_or_else(|| QuickSightError::DashboardNotFound {
                dashboard_id: dashboard_id.to_string(),
            })
    }

    pub fn list_dashboards(&self) -> Vec<&Dashboard> {
        self.dashboards.values().collect()
    }

    // ---- Group ----

    pub fn create_group(
        &mut self,
        namespace: &str,
        group_name: &str,
        description: &str,
        account_id: &str,
        region: &str,
    ) -> Result<QuickSightGroup, QuickSightError> {
        let ns = self.groups.entry(namespace.to_string()).or_default();
        if ns.contains_key(group_name) {
            return Err(QuickSightError::GroupExists {
                group_name: group_name.to_string(),
            });
        }

        let arn =
            format!("arn:aws:quicksight:{region}:{account_id}:group/{namespace}/{group_name}");
        let group = QuickSightGroup {
            group_name: group_name.to_string(),
            arn,
            description: description.to_string(),
            principal_id: account_id.to_string(),
        };

        ns.insert(group_name.to_string(), group.clone());
        // init membership list
        self.group_memberships
            .entry(namespace.to_string())
            .or_default()
            .entry(group_name.to_string())
            .or_default();

        Ok(group)
    }

    pub fn describe_group(
        &self,
        namespace: &str,
        group_name: &str,
    ) -> Result<&QuickSightGroup, QuickSightError> {
        self.groups
            .get(namespace)
            .and_then(|ns| ns.get(group_name))
            .ok_or_else(|| QuickSightError::GroupNotFound {
                group_name: group_name.to_string(),
            })
    }

    pub fn update_group(
        &mut self,
        namespace: &str,
        group_name: &str,
        description: Option<&str>,
    ) -> Result<QuickSightGroup, QuickSightError> {
        let group = self
            .groups
            .get_mut(namespace)
            .and_then(|ns| ns.get_mut(group_name))
            .ok_or_else(|| QuickSightError::GroupNotFound {
                group_name: group_name.to_string(),
            })?;

        if let Some(d) = description {
            group.description = d.to_string();
        }

        Ok(group.clone())
    }

    pub fn delete_group(
        &mut self,
        namespace: &str,
        group_name: &str,
    ) -> Result<(), QuickSightError> {
        let ns = self
            .groups
            .get_mut(namespace)
            .ok_or_else(|| QuickSightError::GroupNotFound {
                group_name: group_name.to_string(),
            })?;

        if ns.remove(group_name).is_none() {
            return Err(QuickSightError::GroupNotFound {
                group_name: group_name.to_string(),
            });
        }

        // Clean up memberships
        if let Some(ns_members) = self.group_memberships.get_mut(namespace) {
            ns_members.remove(group_name);
        }

        Ok(())
    }

    pub fn list_groups(&self, namespace: &str) -> Vec<&QuickSightGroup> {
        self.groups
            .get(namespace)
            .map(|ns| ns.values().collect())
            .unwrap_or_default()
    }

    pub fn search_groups(
        &self,
        namespace: &str,
        filters: &[(String, String, String)], // (name, operator, value)
    ) -> Vec<&QuickSightGroup> {
        let all = self.list_groups(namespace);
        all.into_iter()
            .filter(|g| {
                filters.iter().all(|(name, op, value)| match name.as_str() {
                    "GROUP_NAME" => match op.as_str() {
                        "StartsWith" => g.group_name.starts_with(value.as_str()),
                        "StringEquals" => g.group_name == value.as_str(),
                        _ => g.group_name.contains(value.as_str()),
                    },
                    _ => true,
                })
            })
            .collect()
    }

    // ---- Group Membership ----

    pub fn create_group_membership(
        &mut self,
        namespace: &str,
        group_name: &str,
        member_name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<(String, String), QuickSightError> {
        // Verify group exists
        if !self
            .groups
            .get(namespace)
            .is_some_and(|ns| ns.contains_key(group_name))
        {
            return Err(QuickSightError::GroupNotFound {
                group_name: group_name.to_string(),
            });
        }

        let members = self
            .group_memberships
            .entry(namespace.to_string())
            .or_default()
            .entry(group_name.to_string())
            .or_default();

        if !members.contains(&member_name.to_string()) {
            members.push(member_name.to_string());
        }

        let arn = format!(
            "arn:aws:quicksight:{region}:{account_id}:group/{namespace}/{group_name}/{member_name}"
        );

        Ok((arn, member_name.to_string()))
    }

    pub fn describe_group_membership(
        &self,
        namespace: &str,
        group_name: &str,
        member_name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<(String, String), QuickSightError> {
        let members = self
            .group_memberships
            .get(namespace)
            .and_then(|ns| ns.get(group_name))
            .ok_or_else(|| QuickSightError::GroupNotFound {
                group_name: group_name.to_string(),
            })?;

        if !members.contains(&member_name.to_string()) {
            return Err(QuickSightError::MemberNotInGroup {
                member_name: member_name.to_string(),
                group_name: group_name.to_string(),
            });
        }

        let arn = format!(
            "arn:aws:quicksight:{region}:{account_id}:group/{namespace}/{group_name}/{member_name}"
        );

        Ok((arn, member_name.to_string()))
    }

    pub fn list_group_memberships(
        &self,
        namespace: &str,
        group_name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<Vec<(String, String)>, QuickSightError> {
        let members = self
            .group_memberships
            .get(namespace)
            .and_then(|ns| ns.get(group_name))
            .ok_or_else(|| QuickSightError::GroupNotFound {
                group_name: group_name.to_string(),
            })?;

        Ok(members
            .iter()
            .map(|m| {
                let arn = format!(
                    "arn:aws:quicksight:{region}:{account_id}:group/{namespace}/{group_name}/{m}"
                );
                (arn, m.clone())
            })
            .collect())
    }

    // ---- User ----

    pub fn register_user(
        &mut self,
        namespace: &str,
        email: &str,
        identity_type: &str,
        user_role: &str,
        user_name: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<QuickSightUser, QuickSightError> {
        let uname = user_name.unwrap_or_else(|| email.split('@').next().unwrap_or("user"));
        let ns = self.users.entry(namespace.to_string()).or_default();

        if ns.contains_key(uname) {
            return Err(QuickSightError::UserExists {
                user_name: uname.to_string(),
            });
        }

        let arn = format!("arn:aws:quicksight:{region}:{account_id}:user/{namespace}/{uname}");
        let user = QuickSightUser {
            user_name: uname.to_string(),
            arn,
            email: email.to_string(),
            role: user_role.to_string(),
            identity_type: identity_type.to_string(),
            active: false,
            principal_id: account_id.to_string(),
        };

        ns.insert(uname.to_string(), user.clone());
        Ok(user)
    }

    pub fn describe_user(
        &self,
        namespace: &str,
        user_name: &str,
    ) -> Result<&QuickSightUser, QuickSightError> {
        self.users
            .get(namespace)
            .and_then(|ns| ns.get(user_name))
            .ok_or_else(|| QuickSightError::UserNotFound {
                user_name: user_name.to_string(),
            })
    }

    pub fn update_user(
        &mut self,
        namespace: &str,
        user_name: &str,
        email: Option<&str>,
        role: Option<&str>,
    ) -> Result<QuickSightUser, QuickSightError> {
        let user = self
            .users
            .get_mut(namespace)
            .and_then(|ns| ns.get_mut(user_name))
            .ok_or_else(|| QuickSightError::UserNotFound {
                user_name: user_name.to_string(),
            })?;

        if let Some(e) = email {
            user.email = e.to_string();
        }
        if let Some(r) = role {
            user.role = r.to_string();
        }

        Ok(user.clone())
    }

    pub fn delete_user(&mut self, namespace: &str, user_name: &str) -> Result<(), QuickSightError> {
        let ns = self
            .users
            .get_mut(namespace)
            .ok_or_else(|| QuickSightError::UserNotFound {
                user_name: user_name.to_string(),
            })?;

        if ns.remove(user_name).is_none() {
            return Err(QuickSightError::UserNotFound {
                user_name: user_name.to_string(),
            });
        }

        Ok(())
    }

    pub fn list_users(&self, namespace: &str) -> Vec<&QuickSightUser> {
        self.users
            .get(namespace)
            .map(|ns| ns.values().collect())
            .unwrap_or_default()
    }

    pub fn list_user_groups(
        &self,
        namespace: &str,
        user_name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<Vec<QuickSightGroup>, QuickSightError> {
        // Verify user exists
        if !self
            .users
            .get(namespace)
            .is_some_and(|ns| ns.contains_key(user_name))
        {
            return Err(QuickSightError::UserNotFound {
                user_name: user_name.to_string(),
            });
        }

        let mut groups = Vec::new();
        if let Some(ns_memberships) = self.group_memberships.get(namespace) {
            for (group_name, members) in ns_memberships {
                if members.contains(&user_name.to_string()) {
                    if let Some(group) =
                        self.groups.get(namespace).and_then(|ns| ns.get(group_name))
                    {
                        groups.push(group.clone());
                    }
                }
            }
        }

        let _ = (account_id, region); // used for context but not needed in response
        Ok(groups)
    }

    // ---- Ingestion ----

    pub fn create_ingestion(
        &mut self,
        data_set_id: &str,
        ingestion_id: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&Ingestion, QuickSightError> {
        // Verify data set exists
        if !self.data_sets.contains_key(data_set_id) {
            return Err(QuickSightError::DataSetNotFound {
                data_set_id: data_set_id.to_string(),
            });
        }

        let key = format!("{data_set_id}/{ingestion_id}");
        let arn = format!(
            "arn:aws:quicksight:{region}:{account_id}:dataset/{data_set_id}/ingestion/{ingestion_id}"
        );

        let ingestion = Ingestion {
            ingestion_id: ingestion_id.to_string(),
            arn,
            ingestion_status: "INITIALIZED".to_string(),
            data_set_id: data_set_id.to_string(),
        };

        self.ingestions.insert(key.clone(), ingestion);
        Ok(self.ingestions.get(&key).unwrap())
    }

    // ---- Account Settings ----

    pub fn describe_account_settings(&self) -> &AccountSettings {
        &self.account_settings
    }

    pub fn update_account_settings(
        &mut self,
        default_namespace: Option<&str>,
        notification_email: Option<&str>,
        termination_protection_enabled: Option<bool>,
    ) {
        if let Some(ns) = default_namespace {
            self.account_settings.default_namespace = ns.to_string();
        }
        if let Some(email) = notification_email {
            self.account_settings.notification_email = email.to_string();
        }
        if let Some(tp) = termination_protection_enabled {
            self.account_settings.termination_protection_enabled = tp;
        }
    }

    pub fn update_public_sharing_settings(&mut self, public_sharing_enabled: Option<bool>) {
        if let Some(enabled) = public_sharing_enabled {
            self.public_sharing_enabled = enabled;
            self.account_settings.public_sharing_enabled = enabled;
        }
    }

    // ---- Tags ----

    pub fn tag_resource(&mut self, resource_arn: &str, tags: &[(String, String)]) {
        let entry = self.tags.entry(resource_arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k.clone(), v.clone());
        }
    }

    pub fn untag_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(entry) = self.tags.get_mut(resource_arn) {
            for k in tag_keys {
                entry.remove(k);
            }
        }
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> Vec<(String, String)> {
        self.tags
            .get(resource_arn)
            .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default()
    }

    // ---- Analysis ----

    pub fn create_analysis(
        &mut self,
        analysis_id: &str,
        name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&QuickSightAnalysis, QuickSightError> {
        if self.analyses.contains_key(analysis_id) {
            return Err(QuickSightError::ResourceExists {
                id: analysis_id.to_string(),
            });
        }

        let arn = format!("arn:aws:quicksight:{region}:{account_id}:analysis/{analysis_id}");
        let now = Utc::now();

        let analysis = QuickSightAnalysis {
            analysis_id: analysis_id.to_string(),
            name: name.to_string(),
            arn,
            status: "CREATION_SUCCESSFUL".to_string(),
            created_time: now,
            last_updated_time: now,
        };

        self.analyses.insert(analysis_id.to_string(), analysis);
        Ok(self.analyses.get(analysis_id).unwrap())
    }

    pub fn describe_analysis(
        &self,
        analysis_id: &str,
    ) -> Result<&QuickSightAnalysis, QuickSightError> {
        self.analyses
            .get(analysis_id)
            .ok_or_else(|| QuickSightError::AnalysisNotFound {
                analysis_id: analysis_id.to_string(),
            })
    }

    pub fn update_analysis(
        &mut self,
        analysis_id: &str,
        name: Option<&str>,
    ) -> Result<&QuickSightAnalysis, QuickSightError> {
        let analysis = self.analyses.get_mut(analysis_id).ok_or_else(|| {
            QuickSightError::AnalysisNotFound {
                analysis_id: analysis_id.to_string(),
            }
        })?;

        if let Some(n) = name {
            analysis.name = n.to_string();
        }
        analysis.last_updated_time = Utc::now();

        Ok(self.analyses.get(analysis_id).unwrap())
    }

    pub fn delete_analysis(
        &mut self,
        analysis_id: &str,
    ) -> Result<(String, String), QuickSightError> {
        match self.analyses.remove(analysis_id) {
            Some(a) => Ok((a.arn, a.analysis_id)),
            None => Err(QuickSightError::AnalysisNotFound {
                analysis_id: analysis_id.to_string(),
            }),
        }
    }

    pub fn list_analyses(&self) -> Vec<&QuickSightAnalysis> {
        self.analyses.values().collect()
    }

    // ---- Dashboard (delete, update) ----

    pub fn delete_dashboard(
        &mut self,
        dashboard_id: &str,
    ) -> Result<(String, String), QuickSightError> {
        match self.dashboards.remove(dashboard_id) {
            Some(db) => Ok((db.arn, db.dashboard_id)),
            None => Err(QuickSightError::DashboardNotFound {
                dashboard_id: dashboard_id.to_string(),
            }),
        }
    }

    pub fn update_dashboard(
        &mut self,
        dashboard_id: &str,
        name: Option<&str>,
    ) -> Result<&Dashboard, QuickSightError> {
        let db = self.dashboards.get_mut(dashboard_id).ok_or_else(|| {
            QuickSightError::DashboardNotFound {
                dashboard_id: dashboard_id.to_string(),
            }
        })?;

        if let Some(n) = name {
            db.name = n.to_string();
        }
        db.last_updated_time = Utc::now();

        Ok(self.dashboards.get(dashboard_id).unwrap())
    }

    // ---- Folder ----

    pub fn create_folder(
        &mut self,
        folder_id: &str,
        name: &str,
        folder_type: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&QuickSightFolder, QuickSightError> {
        if self.folders.contains_key(folder_id) {
            return Err(QuickSightError::ResourceExists {
                id: folder_id.to_string(),
            });
        }

        let arn = format!("arn:aws:quicksight:{region}:{account_id}:folder/{folder_id}");
        let now = Utc::now();

        let folder = QuickSightFolder {
            folder_id: folder_id.to_string(),
            name: name.to_string(),
            arn,
            folder_type: folder_type.to_string(),
            created_time: now,
            last_updated_time: now,
            member_ids: Vec::new(),
        };

        self.folders.insert(folder_id.to_string(), folder);
        Ok(self.folders.get(folder_id).unwrap())
    }

    pub fn describe_folder(&self, folder_id: &str) -> Result<&QuickSightFolder, QuickSightError> {
        self.folders
            .get(folder_id)
            .ok_or_else(|| QuickSightError::FolderNotFound {
                folder_id: folder_id.to_string(),
            })
    }

    pub fn delete_folder(&mut self, folder_id: &str) -> Result<(String, String), QuickSightError> {
        match self.folders.remove(folder_id) {
            Some(f) => {
                self.folder_members.remove(folder_id);
                Ok((f.arn, f.folder_id))
            }
            None => Err(QuickSightError::FolderNotFound {
                folder_id: folder_id.to_string(),
            }),
        }
    }

    pub fn list_folders(&self) -> Vec<&QuickSightFolder> {
        self.folders.values().collect()
    }

    pub fn create_folder_membership(
        &mut self,
        folder_id: &str,
        member_id: &str,
        member_type: &str,
    ) -> Result<FolderMemberEntry, QuickSightError> {
        if !self.folders.contains_key(folder_id) {
            return Err(QuickSightError::FolderNotFound {
                folder_id: folder_id.to_string(),
            });
        }

        let entry = FolderMemberEntry {
            member_id: member_id.to_string(),
            member_type: member_type.to_string(),
        };

        let members = self
            .folder_members
            .entry(folder_id.to_string())
            .or_default();
        if !members.iter().any(|m| m.member_id == member_id) {
            members.push(entry.clone());
        }

        Ok(entry)
    }

    pub fn delete_folder_membership(
        &mut self,
        folder_id: &str,
        member_id: &str,
        member_type: &str,
    ) -> Result<(), QuickSightError> {
        if !self.folders.contains_key(folder_id) {
            return Err(QuickSightError::FolderNotFound {
                folder_id: folder_id.to_string(),
            });
        }

        if let Some(members) = self.folder_members.get_mut(folder_id) {
            let before = members.len();
            members.retain(|m| !(m.member_id == member_id && m.member_type == member_type));
            if members.len() == before {
                return Err(QuickSightError::MemberNotInFolder {
                    member_id: member_id.to_string(),
                    folder_id: folder_id.to_string(),
                });
            }
        } else {
            return Err(QuickSightError::MemberNotInFolder {
                member_id: member_id.to_string(),
                folder_id: folder_id.to_string(),
            });
        }

        Ok(())
    }

    pub fn list_folder_members(
        &self,
        folder_id: &str,
    ) -> Result<Vec<&FolderMemberEntry>, QuickSightError> {
        if !self.folders.contains_key(folder_id) {
            return Err(QuickSightError::FolderNotFound {
                folder_id: folder_id.to_string(),
            });
        }

        Ok(self
            .folder_members
            .get(folder_id)
            .map(|members| members.iter().collect())
            .unwrap_or_default())
    }

    // ---- Namespace ----

    pub fn create_namespace(
        &mut self,
        namespace: &str,
        identity_store: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&QuickSightNamespace, QuickSightError> {
        if self.namespaces.contains_key(namespace) {
            return Err(QuickSightError::NamespaceExists {
                namespace: namespace.to_string(),
            });
        }

        let arn = format!("arn:aws:quicksight:{region}:{account_id}:namespace/{namespace}");

        let ns = QuickSightNamespace {
            name: namespace.to_string(),
            arn,
            capacity_region: region.to_string(),
            creation_status: "CREATED".to_string(),
            identity_store: identity_store.to_string(),
        };

        self.namespaces.insert(namespace.to_string(), ns);
        Ok(self.namespaces.get(namespace).unwrap())
    }

    pub fn describe_namespace(
        &self,
        namespace: &str,
    ) -> Result<&QuickSightNamespace, QuickSightError> {
        self.namespaces
            .get(namespace)
            .ok_or_else(|| QuickSightError::NamespaceNotFound {
                namespace: namespace.to_string(),
            })
    }

    pub fn delete_namespace(&mut self, namespace: &str) -> Result<(), QuickSightError> {
        if self.namespaces.remove(namespace).is_none() {
            return Err(QuickSightError::NamespaceNotFound {
                namespace: namespace.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_namespaces(&self) -> Vec<&QuickSightNamespace> {
        self.namespaces.values().collect()
    }

    // ---- Template ----

    pub fn create_template(
        &mut self,
        template_id: &str,
        name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&QuickSightTemplate, QuickSightError> {
        if self.templates.contains_key(template_id) {
            return Err(QuickSightError::ResourceExists {
                id: template_id.to_string(),
            });
        }

        let arn = format!("arn:aws:quicksight:{region}:{account_id}:template/{template_id}");
        let version_arn = format!("{arn}/version/1");
        let now = Utc::now();

        let template = QuickSightTemplate {
            template_id: template_id.to_string(),
            name: name.to_string(),
            arn,
            version_arn,
            created_time: now,
            last_updated_time: now,
            version_number: 1,
        };

        self.templates.insert(template_id.to_string(), template);
        Ok(self.templates.get(template_id).unwrap())
    }

    pub fn describe_template(
        &self,
        template_id: &str,
    ) -> Result<&QuickSightTemplate, QuickSightError> {
        self.templates
            .get(template_id)
            .ok_or_else(|| QuickSightError::TemplateNotFound {
                template_id: template_id.to_string(),
            })
    }

    pub fn update_template(
        &mut self,
        template_id: &str,
        name: Option<&str>,
    ) -> Result<&QuickSightTemplate, QuickSightError> {
        let template = self.templates.get_mut(template_id).ok_or_else(|| {
            QuickSightError::TemplateNotFound {
                template_id: template_id.to_string(),
            }
        })?;

        if let Some(n) = name {
            template.name = n.to_string();
        }
        template.version_number += 1;
        template.version_arn = format!("{}/version/{}", template.arn, template.version_number);
        template.last_updated_time = Utc::now();

        Ok(self.templates.get(template_id).unwrap())
    }

    pub fn delete_template(
        &mut self,
        template_id: &str,
    ) -> Result<(String, String), QuickSightError> {
        match self.templates.remove(template_id) {
            Some(t) => Ok((t.arn, t.template_id)),
            None => Err(QuickSightError::TemplateNotFound {
                template_id: template_id.to_string(),
            }),
        }
    }

    pub fn list_templates(&self) -> Vec<&QuickSightTemplate> {
        self.templates.values().collect()
    }

    // ---- Theme ----

    pub fn create_theme(
        &mut self,
        theme_id: &str,
        name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&QuickSightTheme, QuickSightError> {
        if self.themes.contains_key(theme_id) {
            return Err(QuickSightError::ResourceExists {
                id: theme_id.to_string(),
            });
        }

        let arn = format!("arn:aws:quicksight:{region}:{account_id}:theme/{theme_id}");
        let version_arn = format!("{arn}/version/1");
        let now = Utc::now();

        let theme = QuickSightTheme {
            theme_id: theme_id.to_string(),
            name: name.to_string(),
            arn,
            version_arn,
            created_time: now,
            last_updated_time: now,
            version_number: 1,
        };

        self.themes.insert(theme_id.to_string(), theme);
        Ok(self.themes.get(theme_id).unwrap())
    }

    pub fn describe_theme(&self, theme_id: &str) -> Result<&QuickSightTheme, QuickSightError> {
        self.themes
            .get(theme_id)
            .ok_or_else(|| QuickSightError::ThemeNotFound {
                theme_id: theme_id.to_string(),
            })
    }

    pub fn update_theme(
        &mut self,
        theme_id: &str,
        name: Option<&str>,
    ) -> Result<&QuickSightTheme, QuickSightError> {
        let theme =
            self.themes
                .get_mut(theme_id)
                .ok_or_else(|| QuickSightError::ThemeNotFound {
                    theme_id: theme_id.to_string(),
                })?;

        if let Some(n) = name {
            theme.name = n.to_string();
        }
        theme.version_number += 1;
        theme.version_arn = format!("{}/version/{}", theme.arn, theme.version_number);
        theme.last_updated_time = Utc::now();

        Ok(self.themes.get(theme_id).unwrap())
    }

    pub fn delete_theme(&mut self, theme_id: &str) -> Result<(String, String), QuickSightError> {
        match self.themes.remove(theme_id) {
            Some(t) => Ok((t.arn, t.theme_id)),
            None => Err(QuickSightError::ThemeNotFound {
                theme_id: theme_id.to_string(),
            }),
        }
    }

    pub fn list_themes(&self) -> Vec<&QuickSightTheme> {
        self.themes.values().collect()
    }

    // ---- Ingestion (describe, list) ----

    pub fn describe_ingestion(
        &self,
        data_set_id: &str,
        ingestion_id: &str,
    ) -> Result<&Ingestion, QuickSightError> {
        let key = format!("{data_set_id}/{ingestion_id}");
        self.ingestions
            .get(&key)
            .ok_or_else(|| QuickSightError::IngestionNotFound {
                ingestion_id: ingestion_id.to_string(),
                data_set_id: data_set_id.to_string(),
            })
    }

    pub fn list_ingestions(&self, data_set_id: &str) -> Vec<&Ingestion> {
        let prefix = format!("{data_set_id}/");
        self.ingestions
            .iter()
            .filter_map(|(k, v)| {
                if k.starts_with(&prefix) {
                    Some(v)
                } else {
                    None
                }
            })
            .collect()
    }

    // ---- Group Membership (delete) ----

    pub fn delete_group_membership(
        &mut self,
        namespace: &str,
        group_name: &str,
        member_name: &str,
    ) -> Result<(), QuickSightError> {
        // Verify group exists
        if !self
            .groups
            .get(namespace)
            .is_some_and(|ns| ns.contains_key(group_name))
        {
            return Err(QuickSightError::GroupNotFound {
                group_name: group_name.to_string(),
            });
        }

        let members = self
            .group_memberships
            .get_mut(namespace)
            .and_then(|ns| ns.get_mut(group_name));

        match members {
            Some(list) => {
                let before = list.len();
                list.retain(|m| m != member_name);
                if list.len() == before {
                    return Err(QuickSightError::MemberNotInGroup {
                        member_name: member_name.to_string(),
                        group_name: group_name.to_string(),
                    });
                }
                Ok(())
            }
            None => Err(QuickSightError::MemberNotInGroup {
                member_name: member_name.to_string(),
                group_name: group_name.to_string(),
            }),
        }
    }
}
