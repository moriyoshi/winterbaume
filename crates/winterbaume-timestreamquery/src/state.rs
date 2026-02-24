use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

/// Account-level settings for Timestream Query.
#[derive(Debug, Clone)]
pub struct AccountSettings {
    pub max_query_tcu: i32,
    pub query_pricing_model: String,
}

impl Default for AccountSettings {
    fn default() -> Self {
        Self {
            max_query_tcu: 1000,
            query_pricing_model: "BYTES_SCANNED".to_string(),
        }
    }
}

/// In-memory state for the Timestream Query service.
#[derive(Debug, Default)]
pub struct TimestreamQueryState {
    /// Scheduled queries keyed by ARN.
    pub scheduled_queries: HashMap<String, ScheduledQuery>,
    /// Account-level settings.
    pub account_settings: AccountSettings,
    /// Tags keyed by resource ARN, each value is a map of tag key -> tag value.
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

/// Error type for Timestream Query operations.
#[derive(Debug, thiserror::Error)]
pub enum TimestreamQueryError {
    #[error("Query string must not be empty")]
    EmptyQueryString,

    #[error("A scheduled query with name '{name}' already exists")]
    ScheduledQueryAlreadyExists { name: String },

    #[error("Scheduled query with ARN '{arn}' not found")]
    ScheduledQueryNotFound { arn: String },
}

impl TimestreamQueryState {
    /// Execute a query. Returns mock column info and rows.
    pub fn query(
        &self,
        query_string: &str,
    ) -> Result<(Vec<ColumnInfo>, Vec<Row>, String), TimestreamQueryError> {
        if query_string.trim().is_empty() {
            return Err(TimestreamQueryError::EmptyQueryString);
        }

        // Return a mock result with a single column and no rows.
        // Real queries would need a full SQL parser; for mock purposes
        // we return an empty result set with a query ID.
        let query_id = uuid::Uuid::new_v4().to_string();
        let columns = vec![ColumnInfo {
            name: "measure_value::double".to_string(),
            column_type: "DOUBLE".to_string(),
        }];
        let rows: Vec<Row> = Vec::new();

        Ok((columns, rows, query_id))
    }

    /// Create a scheduled query.
    pub fn create_scheduled_query(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        query_string: &str,
        schedule_expression: &str,
        notification_topic_arn: &str,
        target_database: &str,
        target_table: &str,
        role_arn: &str,
        s3_error_report_bucket: Option<&str>,
    ) -> Result<&ScheduledQuery, TimestreamQueryError> {
        // Check for duplicate name
        for sq in self.scheduled_queries.values() {
            if sq.name == name {
                return Err(TimestreamQueryError::ScheduledQueryAlreadyExists {
                    name: name.to_string(),
                });
            }
        }

        let arn = format!("arn:aws:timestream:{region}:{account_id}:scheduled-query/{name}");

        let sq = ScheduledQuery {
            arn: arn.clone(),
            name: name.to_string(),
            query_string: query_string.to_string(),
            state: ScheduledQueryState::Enabled,
            schedule_expression: schedule_expression.to_string(),
            target_database: target_database.to_string(),
            target_table: target_table.to_string(),
            s3_error_report_bucket: s3_error_report_bucket.map(|s| s.to_string()),
            creation_time: Utc::now(),
            last_run_status: None,
            notification_topic_arn: notification_topic_arn.to_string(),
            role_arn: role_arn.to_string(),
        };

        self.scheduled_queries.insert(arn.clone(), sq);
        Ok(self.scheduled_queries.get(&arn).unwrap())
    }

    /// List all scheduled queries.
    pub fn list_scheduled_queries(&self) -> Vec<&ScheduledQuery> {
        self.scheduled_queries.values().collect()
    }

    /// Describe a scheduled query by ARN.
    pub fn describe_scheduled_query(
        &self,
        arn: &str,
    ) -> Result<&ScheduledQuery, TimestreamQueryError> {
        self.scheduled_queries.get(arn).ok_or_else(|| {
            TimestreamQueryError::ScheduledQueryNotFound {
                arn: arn.to_string(),
            }
        })
    }

    /// Delete a scheduled query by ARN.
    pub fn delete_scheduled_query(&mut self, arn: &str) -> Result<(), TimestreamQueryError> {
        if self.scheduled_queries.remove(arn).is_none() {
            return Err(TimestreamQueryError::ScheduledQueryNotFound {
                arn: arn.to_string(),
            });
        }
        Ok(())
    }

    /// Update the state of a scheduled query.
    pub fn update_scheduled_query(
        &mut self,
        arn: &str,
        state: ScheduledQueryState,
    ) -> Result<(), TimestreamQueryError> {
        match self.scheduled_queries.get_mut(arn) {
            Some(sq) => {
                sq.state = state;
                Ok(())
            }
            None => Err(TimestreamQueryError::ScheduledQueryNotFound {
                arn: arn.to_string(),
            }),
        }
    }

    /// Cancel a query by ID. Returns a cancellation message.
    pub fn cancel_query(&self, _query_id: &str) -> Result<String, TimestreamQueryError> {
        // In a real implementation we'd look up and cancel the running query.
        // For mock purposes we just return success.
        Ok("Query cancelled successfully".to_string())
    }

    /// Describe current account settings.
    pub fn describe_account_settings(&self) -> &AccountSettings {
        &self.account_settings
    }

    /// Update account settings. Returns the updated settings.
    pub fn update_account_settings(
        &mut self,
        max_query_tcu: Option<i32>,
        query_pricing_model: Option<&str>,
    ) -> &AccountSettings {
        if let Some(v) = max_query_tcu {
            self.account_settings.max_query_tcu = v;
        }
        if let Some(v) = query_pricing_model {
            self.account_settings.query_pricing_model = v.to_string();
        }
        &self.account_settings
    }

    /// Execute a scheduled query (mock: just verify it exists and return success).
    pub fn execute_scheduled_query(&self, arn: &str) -> Result<(), TimestreamQueryError> {
        if !self.scheduled_queries.contains_key(arn) {
            return Err(TimestreamQueryError::ScheduledQueryNotFound {
                arn: arn.to_string(),
            });
        }
        Ok(())
    }

    /// List tags for a resource.
    pub fn list_tags_for_resource(&self, resource_arn: &str) -> HashMap<String, String> {
        self.resource_tags
            .get(resource_arn)
            .cloned()
            .unwrap_or_default()
    }

    /// Add or overwrite tags on a resource.
    pub fn tag_resource(&mut self, resource_arn: &str, tags: HashMap<String, String>) {
        let entry = self
            .resource_tags
            .entry(resource_arn.to_string())
            .or_default();
        for (k, v) in tags {
            entry.insert(k, v);
        }
    }

    /// Remove specific tag keys from a resource.
    pub fn untag_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(entry) = self.resource_tags.get_mut(resource_arn) {
            for k in tag_keys {
                entry.remove(k);
            }
        }
    }

    /// Return mock endpoints.
    pub fn describe_endpoints(&self, region: &str) -> Vec<(String, i64)> {
        vec![(
            format!("query.timestream.{region}.amazonaws.com"),
            // Cache period in minutes
            1440,
        )]
    }
}
