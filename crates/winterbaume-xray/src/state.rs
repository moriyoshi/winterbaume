use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct XRayState {
    pub groups: HashMap<String, Group>,
    /// Parsed trace segment records keyed by segment id.
    pub trace_segments: HashMap<String, TraceSegmentRecord>,
    /// Resource-based policies keyed by policy_name.
    pub resource_policies: HashMap<String, ResourcePolicy>,
    /// Sampling rules keyed by rule_name.
    pub sampling_rules: HashMap<String, SamplingRuleData>,
    /// Encryption configuration.
    pub encryption_config: EncryptionConfig,
    /// Per-resource tags keyed by resource ARN.
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// Trace segment destination configuration.
    pub trace_segment_destination: TraceSegmentDestination,
    /// Indexing rules keyed by rule name.
    pub indexing_rules: HashMap<String, IndexingRuleData>,
    /// Telemetry records submitted via PutTelemetryRecords.
    pub telemetry_records: Vec<TelemetryRecord>,
    /// Trace retrieval jobs keyed by retrieval token.
    pub trace_retrieval_jobs: HashMap<String, TraceRetrievalJob>,
    /// Sampling statistic summaries (accumulated).
    pub sampling_statistic_summaries: Vec<SamplingStatisticSummary>,
}

#[derive(Debug, Error)]
#[allow(clippy::upper_case_acronyms)]
pub enum XRayError {
    #[error("Group {name} already exists.")]
    GroupAlreadyExists { name: String },

    #[error("Invalid GroupARN format.")]
    InvalidGroupArn,

    #[error("Either GroupName or GroupARN is required.")]
    MissingGroupIdentifier,

    #[error("Group {name} not found.")]
    GroupNotFound { name: String },

    #[error("Policy revision ID mismatch for policy '{policy_name}'.")]
    InvalidPolicyRevisionId { policy_name: String },

    #[error("Resource policy '{policy_name}' not found.")]
    ResourcePolicyNotFound { policy_name: String },

    #[error("Sampling rule '{rule_name}' already exists.")]
    SamplingRuleAlreadyExists { rule_name: String },

    #[error("Sampling rule '{rule_name}' not found.")]
    SamplingRuleNotFound { rule_name: String },

    #[error("Trace retrieval '{token}' not found.")]
    TraceRetrievalNotFound { token: String },
}

impl XRayState {
    pub fn create_group(
        &mut self,
        group_name: &str,
        filter_expression: &str,
        insights_enabled: bool,
        notifications_enabled: bool,
        account_id: &str,
        region: &str,
    ) -> Result<&Group, XRayError> {
        if self.groups.contains_key(group_name) {
            return Err(XRayError::GroupAlreadyExists {
                name: group_name.to_string(),
            });
        }

        let group_arn = format!("arn:aws:xray:{region}:{account_id}:group/{group_name}");

        let group = Group {
            group_name: group_name.to_string(),
            group_arn,
            filter_expression: filter_expression.to_string(),
            insights_configuration: Some(InsightsConfiguration {
                insights_enabled,
                notifications_enabled,
            }),
        };

        self.groups.insert(group_name.to_string(), group);
        Ok(self.groups.get(group_name).unwrap())
    }

    /// Resolve a group name from either a GroupName or a GroupARN.
    fn resolve_group_name<'a>(
        &self,
        group_name: Option<&'a str>,
        group_arn: Option<&'a str>,
    ) -> Result<&'a str, XRayError> {
        if let Some(name) = group_name {
            return Ok(name);
        }
        if let Some(arn) = group_arn {
            // ARN format: arn:aws:xray:{region}:{account_id}:group/{group_name}
            if let Some(name) = arn.split('/').next_back() {
                if !name.is_empty() {
                    return Ok(name);
                }
            }
            return Err(XRayError::InvalidGroupArn);
        }
        Err(XRayError::MissingGroupIdentifier)
    }

    pub fn get_group_by_name_or_arn(
        &self,
        group_name: Option<&str>,
        group_arn: Option<&str>,
    ) -> Result<&Group, XRayError> {
        let name = self.resolve_group_name(group_name, group_arn)?;
        self.groups
            .get(name)
            .ok_or_else(|| XRayError::GroupNotFound {
                name: name.to_string(),
            })
    }

    pub fn get_group(&self, group_name: &str) -> Result<&Group, XRayError> {
        self.get_group_by_name_or_arn(Some(group_name), None)
    }

    pub fn delete_group_by_name_or_arn(
        &mut self,
        group_name: Option<&str>,
        group_arn: Option<&str>,
    ) -> Result<(), XRayError> {
        let name = self.resolve_group_name(group_name, group_arn)?.to_string();
        if self.groups.remove(&name).is_none() {
            return Err(XRayError::GroupNotFound {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    pub fn delete_group(&mut self, group_name: &str) -> Result<(), XRayError> {
        self.delete_group_by_name_or_arn(Some(group_name), None)
    }

    pub fn update_group(
        &mut self,
        group_name: Option<&str>,
        group_arn: Option<&str>,
        filter_expression: Option<&str>,
        insights_enabled: Option<bool>,
        notifications_enabled: Option<bool>,
    ) -> Result<&Group, XRayError> {
        let name = self.resolve_group_name(group_name, group_arn)?.to_string();
        let group = self
            .groups
            .get_mut(&name)
            .ok_or_else(|| XRayError::GroupNotFound {
                name: name.to_string(),
            })?;
        if let Some(expr) = filter_expression {
            group.filter_expression = expr.to_string();
        }
        if insights_enabled.is_some() || notifications_enabled.is_some() {
            let ic = group
                .insights_configuration
                .get_or_insert(InsightsConfiguration {
                    insights_enabled: false,
                    notifications_enabled: false,
                });
            if let Some(v) = insights_enabled {
                ic.insights_enabled = v;
            }
            if let Some(v) = notifications_enabled {
                ic.notifications_enabled = v;
            }
        }
        Ok(self.groups.get(&name).unwrap())
    }

    /// Parse and store trace segment documents.
    ///
    /// Returns a list of `(segment_id, error_code, message)` for documents that
    /// could not be parsed (bad JSON or missing required fields).
    pub fn put_trace_segments(&mut self, documents: Vec<String>) -> Vec<(String, String, String)> {
        let mut unprocessed = Vec::new();

        for doc in documents {
            match Self::parse_segment_document(&doc) {
                Ok(record) => {
                    self.trace_segments.insert(record.id.clone(), record);
                }
                Err((seg_id, code, msg)) => {
                    unprocessed.push((seg_id, code, msg));
                }
            }
        }

        unprocessed
    }

    fn parse_segment_document(doc: &str) -> Result<TraceSegmentRecord, (String, String, String)> {
        let data: serde_json::Value = serde_json::from_str(doc).map_err(|e| {
            (
                String::new(),
                "JSONFormatError".to_string(),
                format!("Bad JSON data: {e}"),
            )
        })?;

        let id = data
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let trace_id = data
            .get("trace_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let start_time = data
            .get("start_time")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let end_time = data.get("end_time").and_then(|v| v.as_f64());

        if trace_id.is_empty() {
            return Err((
                id,
                "JSONFormatError".to_string(),
                "Missing trace_id field".to_string(),
            ));
        }

        let has_error = data.get("error").and_then(|v| v.as_bool()).unwrap_or(false);
        let has_fault = data.get("fault").and_then(|v| v.as_bool()).unwrap_or(false);
        let has_throttle = data
            .get("throttle")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Ok(TraceSegmentRecord {
            document: doc.to_string(),
            id,
            trace_id,
            start_time,
            end_time,
            has_error,
            has_fault,
            has_throttle,
        })
    }

    /// Summarise traces whose start_time and end_time both fall within
    /// [start_time_epoch, end_time_epoch].  Only traces that have at least one
    /// segment with a non-None end_time are returned (same semantics as moto).
    pub fn get_trace_summaries(
        &self,
        start_time_epoch: f64,
        end_time_epoch: f64,
    ) -> Vec<TraceSummaryData> {
        // Group segments by trace_id
        let mut by_trace: HashMap<String, Vec<&TraceSegmentRecord>> = HashMap::new();
        for seg in self.trace_segments.values() {
            by_trace.entry(seg.trace_id.clone()).or_default().push(seg);
        }

        let mut summaries = Vec::new();
        for (trace_id, segs) in &by_trace {
            // Only include "finished" traces (at least one segment with end_time)
            let finished_segs: Vec<&&TraceSegmentRecord> =
                segs.iter().filter(|s| s.end_time.is_some()).collect();
            if finished_segs.is_empty() {
                continue;
            }

            let min_start = segs
                .iter()
                .map(|s| s.start_time)
                .fold(f64::INFINITY, f64::min);
            let max_end = finished_segs
                .iter()
                .filter_map(|s| s.end_time)
                .fold(0.0_f64, f64::max);

            if min_start < start_time_epoch || max_end > end_time_epoch {
                continue;
            }

            let has_error = segs.iter().any(|s| s.has_error);
            let has_fault = segs.iter().any(|s| s.has_fault);
            let has_throttle = segs.iter().any(|s| s.has_throttle);
            let duration = max_end - min_start;

            summaries.push(TraceSummaryData {
                id: trace_id.clone(),
                duration,
                has_error,
                has_fault,
                has_throttle,
                start_time: min_start,
            });
        }

        summaries
    }

    /// Retrieve full trace data for a list of trace IDs.
    ///
    /// Returns a tuple of `(found_traces, unprocessed_ids)`.
    pub fn batch_get_traces(&self, trace_ids: &[String]) -> (Vec<TraceData>, Vec<String>) {
        // Group segments by trace_id
        let mut by_trace: HashMap<&str, Vec<&TraceSegmentRecord>> = HashMap::new();
        for seg in self.trace_segments.values() {
            by_trace.entry(&seg.trace_id).or_default().push(seg);
        }

        let mut found = Vec::new();
        let mut unprocessed = Vec::new();

        for trace_id in trace_ids {
            if let Some(segs) = by_trace.get(trace_id.as_str()) {
                let start_time = segs
                    .iter()
                    .map(|s| s.start_time)
                    .fold(f64::INFINITY, f64::min);
                let end_time = segs
                    .iter()
                    .filter_map(|s| s.end_time)
                    .fold(0.0_f64, f64::max);
                let duration = if end_time > start_time {
                    end_time - start_time
                } else {
                    0.0
                };
                found.push(TraceData {
                    id: trace_id.clone(),
                    duration,
                    segments: segs
                        .iter()
                        .map(|s| SegmentData {
                            id: s.id.clone(),
                            document: s.document.clone(),
                        })
                        .collect(),
                });
            } else {
                unprocessed.push(trace_id.clone());
            }
        }

        (found, unprocessed)
    }

    // --- Resource Policy operations ---

    pub fn put_resource_policy(
        &mut self,
        policy_name: &str,
        policy_document: &str,
        policy_revision_id: Option<&str>,
    ) -> Result<&ResourcePolicy, XRayError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        // If policy_revision_id is provided, verify it matches the existing policy revision
        if let Some(rev_id) = policy_revision_id {
            if let Some(existing) = self.resource_policies.get(policy_name) {
                if existing.policy_revision_id != rev_id {
                    return Err(XRayError::InvalidPolicyRevisionId {
                        policy_name: policy_name.to_string(),
                    });
                }
            }
        }

        let new_revision = uuid::Uuid::new_v4().to_string();
        let policy = ResourcePolicy {
            policy_name: policy_name.to_string(),
            policy_document: policy_document.to_string(),
            policy_revision_id: new_revision,
            last_updated_time: now,
        };
        self.resource_policies
            .insert(policy_name.to_string(), policy);
        Ok(self.resource_policies.get(policy_name).unwrap())
    }

    pub fn delete_resource_policy(
        &mut self,
        policy_name: &str,
        policy_revision_id: &str,
    ) -> Result<(), XRayError> {
        let existing = self.resource_policies.get(policy_name).ok_or_else(|| {
            XRayError::ResourcePolicyNotFound {
                policy_name: policy_name.to_string(),
            }
        })?;
        if existing.policy_revision_id != policy_revision_id {
            return Err(XRayError::InvalidPolicyRevisionId {
                policy_name: policy_name.to_string(),
            });
        }
        self.resource_policies.remove(policy_name);
        Ok(())
    }

    pub fn list_resource_policies(&self) -> Vec<&ResourcePolicy> {
        self.resource_policies.values().collect()
    }

    // --- Sampling Rule operations ---

    pub fn create_sampling_rule(
        &mut self,
        rule_name: &str,
        resource_arn: &str,
        priority: i32,
        fixed_rate: f64,
        reservoir_size: i32,
        service_name: &str,
        service_type: &str,
        host: &str,
        http_method: &str,
        url_path: &str,
        version: i32,
        account_id: &str,
        region: &str,
    ) -> Result<&SamplingRuleData, XRayError> {
        if self.sampling_rules.contains_key(rule_name) {
            return Err(XRayError::SamplingRuleAlreadyExists {
                rule_name: rule_name.to_string(),
            });
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        let rule_arn = format!("arn:aws:xray:{region}:{account_id}:sampling-rule/{rule_name}");

        let rule = SamplingRuleData {
            rule_name: rule_name.to_string(),
            rule_arn,
            resource_arn: resource_arn.to_string(),
            priority,
            fixed_rate,
            reservoir_size,
            service_name: service_name.to_string(),
            service_type: service_type.to_string(),
            host: host.to_string(),
            http_method: http_method.to_string(),
            url_path: url_path.to_string(),
            version,
            created_at: now,
            modified_at: now,
        };

        self.sampling_rules.insert(rule_name.to_string(), rule);
        Ok(self.sampling_rules.get(rule_name).unwrap())
    }

    pub fn delete_sampling_rule(&mut self, rule_name: &str) -> Result<SamplingRuleData, XRayError> {
        self.sampling_rules
            .remove(rule_name)
            .ok_or_else(|| XRayError::SamplingRuleNotFound {
                rule_name: rule_name.to_string(),
            })
    }

    pub fn get_sampling_rules(&self) -> Vec<&SamplingRuleData> {
        self.sampling_rules.values().collect()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_sampling_rule(
        &mut self,
        rule_name: &str,
        resource_arn: Option<&str>,
        priority: Option<i32>,
        fixed_rate: Option<f64>,
        reservoir_size: Option<i32>,
        service_name: Option<&str>,
        service_type: Option<&str>,
        host: Option<&str>,
        http_method: Option<&str>,
        url_path: Option<&str>,
    ) -> Result<&SamplingRuleData, XRayError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        let rule = self.sampling_rules.get_mut(rule_name).ok_or_else(|| {
            XRayError::SamplingRuleNotFound {
                rule_name: rule_name.to_string(),
            }
        })?;

        if let Some(v) = resource_arn {
            rule.resource_arn = v.to_string();
        }
        if let Some(v) = priority {
            rule.priority = v;
        }
        if let Some(v) = fixed_rate {
            rule.fixed_rate = v;
        }
        if let Some(v) = reservoir_size {
            rule.reservoir_size = v;
        }
        if let Some(v) = service_name {
            rule.service_name = v.to_string();
        }
        if let Some(v) = service_type {
            rule.service_type = v.to_string();
        }
        if let Some(v) = host {
            rule.host = v.to_string();
        }
        if let Some(v) = http_method {
            rule.http_method = v.to_string();
        }
        if let Some(v) = url_path {
            rule.url_path = v.to_string();
        }
        rule.modified_at = now;

        Ok(self.sampling_rules.get(rule_name).unwrap())
    }

    // --- Encryption config ---

    pub fn get_encryption_config(&self) -> &EncryptionConfig {
        &self.encryption_config
    }

    pub fn put_encryption_config(&mut self, key_id: Option<String>, encryption_type: &str) {
        self.encryption_config = EncryptionConfig {
            key_id,
            status: "ACTIVE".to_string(),
            encryption_type: encryption_type.to_string(),
        };
    }

    // --- Resource tags ---

    pub fn tag_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        self.resource_tags
            .entry(arn.to_string())
            .or_default()
            .extend(tags);
    }

    pub fn untag_resource(&mut self, arn: &str, tag_keys: &[String]) {
        if let Some(resource_tags) = self.resource_tags.get_mut(arn) {
            for key in tag_keys {
                resource_tags.remove(key);
            }
        }
    }

    pub fn list_tags_for_resource(&self, arn: &str) -> Vec<(String, String)> {
        self.resource_tags
            .get(arn)
            .map(|tags| tags.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default()
    }

    // --- Trace segment destination ---

    pub fn get_trace_segment_destination(&self) -> &TraceSegmentDestination {
        &self.trace_segment_destination
    }

    pub fn update_trace_segment_destination(&mut self, destination: &str) {
        self.trace_segment_destination.destination = destination.to_string();
    }

    // --- Indexing rules ---

    pub fn get_indexing_rules(&self) -> Vec<&IndexingRuleData> {
        self.indexing_rules.values().collect()
    }

    pub fn update_indexing_rule(
        &mut self,
        name: &str,
        desired_sampling_percentage: Option<f64>,
    ) -> &IndexingRuleData {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        let rule = self
            .indexing_rules
            .entry(name.to_string())
            .or_insert_with(|| IndexingRuleData {
                name: name.to_string(),
                modified_at: now,
                desired_sampling_percentage: None,
            });
        rule.modified_at = now;
        if let Some(pct) = desired_sampling_percentage {
            rule.desired_sampling_percentage = Some(pct);
        }
        self.indexing_rules.get(name).unwrap()
    }

    // --- Telemetry records ---

    pub fn put_telemetry_records(&mut self, records: Vec<TelemetryRecord>) {
        self.telemetry_records.extend(records);
    }

    pub fn get_telemetry_records(&self) -> &[TelemetryRecord] {
        &self.telemetry_records
    }

    // --- Trace retrieval ---

    pub fn start_trace_retrieval(&mut self, trace_ids: Vec<String>) -> &TraceRetrievalJob {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        let token = uuid::Uuid::new_v4().to_string();
        let job = TraceRetrievalJob {
            retrieval_token: token.clone(),
            trace_ids,
            status: "RUNNING".to_string(),
            created_at: now,
        };
        self.trace_retrieval_jobs.insert(token.clone(), job);
        self.trace_retrieval_jobs.get(&token).unwrap()
    }

    pub fn cancel_trace_retrieval(&mut self, token: &str) -> Result<(), XRayError> {
        let job = self.trace_retrieval_jobs.get_mut(token).ok_or_else(|| {
            XRayError::TraceRetrievalNotFound {
                token: token.to_string(),
            }
        })?;
        job.status = "CANCELLED".to_string();
        Ok(())
    }

    pub fn get_trace_retrieval_job(&self, token: &str) -> Option<&TraceRetrievalJob> {
        self.trace_retrieval_jobs.get(token)
    }

    /// List retrieved traces for a given retrieval token, returning segments
    /// from matching trace IDs in state.
    pub fn list_retrieved_traces(
        &self,
        token: &str,
    ) -> Result<(Vec<&TraceSegmentRecord>, &str), XRayError> {
        let job = self.trace_retrieval_jobs.get(token).ok_or_else(|| {
            XRayError::TraceRetrievalNotFound {
                token: token.to_string(),
            }
        })?;
        let segments: Vec<&TraceSegmentRecord> = self
            .trace_segments
            .values()
            .filter(|s| job.trace_ids.contains(&s.trace_id))
            .collect();
        Ok((segments, &job.status))
    }

    // --- Sampling statistic summaries ---

    /// Record a sampling statistic summary entry.
    pub fn add_sampling_statistic_summary(&mut self, summary: SamplingStatisticSummary) {
        self.sampling_statistic_summaries.push(summary);
    }

    pub fn get_sampling_statistic_summaries(&self) -> &[SamplingStatisticSummary] {
        &self.sampling_statistic_summaries
    }
}
