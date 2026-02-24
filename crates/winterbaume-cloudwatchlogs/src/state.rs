use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct LogsState {
    pub log_groups: HashMap<String, LogGroup>,
    pub metric_filters: HashMap<String, MetricFilter>,
    pub subscription_filters: HashMap<String, SubscriptionFilter>,
    pub resource_policies: HashMap<String, ResourcePolicy>,
    pub destinations: HashMap<String, Destination>,
    pub export_tasks: HashMap<String, ExportTask>,
    pub queries: HashMap<String, Query>,
    pub delivery_sources: HashMap<String, DeliverySource>,
    pub delivery_destinations: HashMap<String, DeliveryDestination>,
    pub delivery_destination_policies: HashMap<String, DeliveryDestinationPolicy>,
    pub deliveries: HashMap<String, Delivery>,
    pub account_policies: HashMap<String, AccountPolicy>,
    pub query_definitions: HashMap<String, QueryDefinition>,
    pub anomaly_detectors: HashMap<String, LogAnomalyDetector>,
    pub anomalies: HashMap<String, Anomaly>,
    pub index_policies: HashMap<String, IndexPolicy>,
    pub transformers: HashMap<String, Transformer>,
    pub integrations: HashMap<String, Integration>,
    pub import_tasks: HashMap<String, ImportTask>,
    pub scheduled_queries: HashMap<String, ScheduledQuery>,
}

#[derive(Debug, thiserror::Error)]
pub enum LogsError {
    #[error("{0}")]
    ResourceNotFound(String),
    #[error("{0}")]
    AlreadyExists(String),
    #[error("{0}")]
    InvalidParameter(String),
}

impl LogsError {
    pub fn resource_not_found(msg: &str) -> Self {
        LogsError::ResourceNotFound(msg.to_string())
    }

    pub fn already_exists(msg: &str) -> Self {
        LogsError::AlreadyExists(msg.to_string())
    }

    pub fn invalid_parameter(msg: &str) -> Self {
        LogsError::InvalidParameter(msg.to_string())
    }
}

/// Used as a return type for FilterLogEvents
pub struct FilteredLogEvent {
    pub log_stream_name: String,
    pub timestamp: i64,
    pub message: String,
    pub ingestion_time: i64,
    pub event_id: String,
}

impl LogsState {
    // ========== Log Groups ==========

    pub fn create_log_group(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&LogGroup, LogsError> {
        if self.log_groups.contains_key(name) {
            return Err(LogsError::already_exists(
                "The specified log group already exists",
            ));
        }

        let arn = format!("arn:aws:logs:{region}:{account_id}:log-group:{name}:*");
        let now = Utc::now().timestamp_millis();

        let group = LogGroup {
            name: name.to_string(),
            arn,
            creation_time: now,
            retention_in_days: None,
            streams: HashMap::new(),
            tags,
            kms_key_id: None,
            data_protection_policy: None,
            deletion_protection_enabled: false,
        };

        self.log_groups.insert(name.to_string(), group);
        Ok(self.log_groups.get(name).unwrap())
    }

    pub fn delete_log_group(&mut self, name: &str) -> Result<(), LogsError> {
        if self.log_groups.remove(name).is_none() {
            return Err(LogsError::resource_not_found(
                "The specified log group does not exist.",
            ));
        }
        Ok(())
    }

    pub fn describe_log_groups(&self, prefix: Option<&str>) -> Vec<&LogGroup> {
        self.log_groups
            .values()
            .filter(|g| {
                if let Some(p) = prefix {
                    g.name.starts_with(p)
                } else {
                    true
                }
            })
            .collect()
    }

    // ========== Log Streams ==========

    pub fn create_log_stream(
        &mut self,
        group_name: &str,
        stream_name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<(), LogsError> {
        let group = self.log_groups.get_mut(group_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log group does not exist.")
        })?;

        if group.streams.contains_key(stream_name) {
            return Err(LogsError::already_exists(
                "The specified log stream already exists",
            ));
        }

        let arn = format!(
            "arn:aws:logs:{region}:{account_id}:log-group:{group_name}:log-stream:{stream_name}"
        );
        let now = Utc::now().timestamp_millis();

        let stream = LogStream {
            name: stream_name.to_string(),
            arn,
            creation_time: now,
            events: Vec::new(),
            first_event_timestamp: None,
            last_event_timestamp: None,
            upload_sequence_token: uuid::Uuid::new_v4().to_string(),
        };

        group.streams.insert(stream_name.to_string(), stream);
        Ok(())
    }

    pub fn delete_log_stream(
        &mut self,
        group_name: &str,
        stream_name: &str,
    ) -> Result<(), LogsError> {
        let group = self.log_groups.get_mut(group_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log group does not exist.")
        })?;

        if group.streams.remove(stream_name).is_none() {
            return Err(LogsError::resource_not_found(
                "The specified log stream does not exist.",
            ));
        }
        Ok(())
    }

    pub fn describe_log_streams(
        &self,
        group_name: &str,
        prefix: Option<&str>,
    ) -> Result<Vec<&LogStream>, LogsError> {
        let group = self.log_groups.get(group_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log group does not exist.")
        })?;

        let streams: Vec<&LogStream> = group
            .streams
            .values()
            .filter(|s| {
                if let Some(p) = prefix {
                    s.name.starts_with(p)
                } else {
                    true
                }
            })
            .collect();

        Ok(streams)
    }

    // ========== Log Events ==========

    pub fn put_log_events(
        &mut self,
        group_name: &str,
        stream_name: &str,
        events: Vec<(i64, String)>,
    ) -> Result<String, LogsError> {
        let group = self.log_groups.get_mut(group_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log group does not exist.")
        })?;

        let stream = group.streams.get_mut(stream_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log stream does not exist.")
        })?;

        let ingestion_time = Utc::now().timestamp_millis();

        for (timestamp, message) in &events {
            let event = LogEvent {
                timestamp: *timestamp,
                message: message.clone(),
                ingestion_time,
            };

            match stream.first_event_timestamp {
                None => stream.first_event_timestamp = Some(*timestamp),
                Some(first) if *timestamp < first => {
                    stream.first_event_timestamp = Some(*timestamp)
                }
                _ => {}
            }
            match stream.last_event_timestamp {
                None => stream.last_event_timestamp = Some(*timestamp),
                Some(last) if *timestamp > last => stream.last_event_timestamp = Some(*timestamp),
                _ => {}
            }

            stream.events.push(event);
        }

        stream.upload_sequence_token = uuid::Uuid::new_v4().to_string();
        Ok(stream.upload_sequence_token.clone())
    }

    pub fn get_log_events(
        &self,
        group_name: &str,
        stream_name: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<usize>,
    ) -> Result<Vec<&LogEvent>, LogsError> {
        let group = self.log_groups.get(group_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log group does not exist.")
        })?;

        let stream = group.streams.get(stream_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log stream does not exist.")
        })?;

        let mut events: Vec<&LogEvent> = stream
            .events
            .iter()
            .filter(|e| {
                if let Some(start) = start_time {
                    if e.timestamp < start {
                        return false;
                    }
                }
                if let Some(end) = end_time {
                    if e.timestamp >= end {
                        return false;
                    }
                }
                true
            })
            .collect();

        events.sort_by_key(|e| e.timestamp);

        if let Some(lim) = limit {
            events.truncate(lim);
        }

        Ok(events)
    }

    // ========== Retention Policy ==========

    pub fn put_retention_policy(
        &mut self,
        log_group_name: &str,
        retention_in_days: i32,
    ) -> Result<(), LogsError> {
        let group = self.log_groups.get_mut(log_group_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log group does not exist.")
        })?;
        group.retention_in_days = Some(retention_in_days);
        Ok(())
    }

    pub fn delete_retention_policy(&mut self, log_group_name: &str) -> Result<(), LogsError> {
        let group = self.log_groups.get_mut(log_group_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log group does not exist.")
        })?;
        group.retention_in_days = None;
        Ok(())
    }

    // ========== Metric Filters ==========

    pub fn put_metric_filter(&mut self, filter: MetricFilter) -> Result<(), LogsError> {
        if !self.log_groups.contains_key(&filter.log_group_name) {
            return Err(LogsError::resource_not_found(
                "The specified log group does not exist.",
            ));
        }
        let key = format!("{}:{}", filter.log_group_name, filter.filter_name);
        self.metric_filters.insert(key, filter);
        Ok(())
    }

    pub fn delete_metric_filter(
        &mut self,
        log_group_name: &str,
        filter_name: &str,
    ) -> Result<(), LogsError> {
        let key = format!("{log_group_name}:{filter_name}");
        if self.metric_filters.remove(&key).is_none() {
            return Err(LogsError::resource_not_found(
                "The specified metric filter does not exist.",
            ));
        }
        Ok(())
    }

    pub fn describe_metric_filters(
        &self,
        log_group_name: Option<&str>,
        filter_name_prefix: Option<&str>,
        metric_name: Option<&str>,
        metric_namespace: Option<&str>,
    ) -> Vec<&MetricFilter> {
        self.metric_filters
            .values()
            .filter(|f| {
                if let Some(lgn) = log_group_name {
                    if f.log_group_name != lgn {
                        return false;
                    }
                }
                if let Some(prefix) = filter_name_prefix {
                    if !f.filter_name.starts_with(prefix) {
                        return false;
                    }
                }
                if let Some(mn) = metric_name {
                    if !f.metric_transformations.iter().any(|t| t.metric_name == mn) {
                        return false;
                    }
                }
                if let Some(mns) = metric_namespace {
                    if !f
                        .metric_transformations
                        .iter()
                        .any(|t| t.metric_namespace == mns)
                    {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    // ========== Subscription Filters ==========

    pub fn put_subscription_filter(&mut self, filter: SubscriptionFilter) -> Result<(), LogsError> {
        if !self.log_groups.contains_key(&filter.log_group_name) {
            return Err(LogsError::resource_not_found(
                "The specified log group does not exist.",
            ));
        }
        let key = format!("{}:{}", filter.log_group_name, filter.filter_name);
        self.subscription_filters.insert(key, filter);
        Ok(())
    }

    pub fn delete_subscription_filter(
        &mut self,
        log_group_name: &str,
        filter_name: &str,
    ) -> Result<(), LogsError> {
        let key = format!("{log_group_name}:{filter_name}");
        if self.subscription_filters.remove(&key).is_none() {
            return Err(LogsError::resource_not_found(
                "The specified subscription filter does not exist.",
            ));
        }
        Ok(())
    }

    pub fn describe_subscription_filters(
        &self,
        log_group_name: &str,
        filter_name_prefix: Option<&str>,
    ) -> Result<Vec<&SubscriptionFilter>, LogsError> {
        if !self.log_groups.contains_key(log_group_name) {
            return Err(LogsError::resource_not_found(
                "The specified log group does not exist.",
            ));
        }
        Ok(self
            .subscription_filters
            .values()
            .filter(|f| {
                if f.log_group_name != log_group_name {
                    return false;
                }
                if let Some(prefix) = filter_name_prefix {
                    if !f.filter_name.starts_with(prefix) {
                        return false;
                    }
                }
                true
            })
            .collect())
    }

    // ========== Resource Policies ==========

    pub fn put_resource_policy(
        &mut self,
        policy_name: &str,
        policy_document: &str,
    ) -> &ResourcePolicy {
        let now = Utc::now().timestamp_millis();
        let policy = ResourcePolicy {
            policy_name: policy_name.to_string(),
            policy_document: policy_document.to_string(),
            last_updated_time: now,
        };
        self.resource_policies
            .insert(policy_name.to_string(), policy);
        self.resource_policies.get(policy_name).unwrap()
    }

    pub fn delete_resource_policy(&mut self, policy_name: &str) -> Result<(), LogsError> {
        if self.resource_policies.remove(policy_name).is_none() {
            return Err(LogsError::resource_not_found(
                "The specified resource policy does not exist.",
            ));
        }
        Ok(())
    }

    pub fn describe_resource_policies(&self) -> Vec<&ResourcePolicy> {
        self.resource_policies.values().collect()
    }

    // ========== Destinations ==========

    pub fn put_destination(
        &mut self,
        destination_name: &str,
        target_arn: &str,
        role_arn: &str,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> &Destination {
        let arn = format!("arn:aws:logs:{region}:{account_id}:destination:{destination_name}");
        let now = Utc::now().timestamp_millis();
        // Preserve existing tags if updating, merge with new ones
        let existing_tags = self
            .destinations
            .get(destination_name)
            .map(|d| d.tags.clone())
            .unwrap_or_default();
        let mut merged_tags = existing_tags;
        merged_tags.extend(tags);
        let dest = Destination {
            destination_name: destination_name.to_string(),
            target_arn: target_arn.to_string(),
            role_arn: role_arn.to_string(),
            access_policy: None,
            arn,
            creation_time: now,
            tags: merged_tags,
        };
        self.destinations.insert(destination_name.to_string(), dest);
        self.destinations.get(destination_name).unwrap()
    }

    pub fn put_destination_policy(
        &mut self,
        destination_name: &str,
        access_policy: &str,
    ) -> Result<(), LogsError> {
        let dest = self.destinations.get_mut(destination_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified destination does not exist.")
        })?;
        dest.access_policy = Some(access_policy.to_string());
        Ok(())
    }

    pub fn delete_destination(&mut self, destination_name: &str) -> Result<(), LogsError> {
        if self.destinations.remove(destination_name).is_none() {
            return Err(LogsError::resource_not_found(
                "The specified destination does not exist.",
            ));
        }
        Ok(())
    }

    pub fn describe_destinations(
        &self,
        destination_name_prefix: Option<&str>,
    ) -> Vec<&Destination> {
        self.destinations
            .values()
            .filter(|d| {
                if let Some(prefix) = destination_name_prefix {
                    d.destination_name.starts_with(prefix)
                } else {
                    true
                }
            })
            .collect()
    }

    // ========== Export Tasks ==========

    pub fn create_export_task(
        &mut self,
        task_name: Option<&str>,
        log_group_name: &str,
        destination: &str,
        from_time: i64,
        to_time: i64,
    ) -> Result<String, LogsError> {
        if !self.log_groups.contains_key(log_group_name) {
            return Err(LogsError::resource_not_found(
                "The specified log group does not exist.",
            ));
        }
        let task_id = uuid::Uuid::new_v4().to_string();
        let task = ExportTask {
            task_id: task_id.clone(),
            task_name: task_name.map(|s| s.to_string()),
            log_group_name: log_group_name.to_string(),
            destination: destination.to_string(),
            from_time,
            to_time,
            status: "PENDING".to_string(),
        };
        self.export_tasks.insert(task_id.clone(), task);
        Ok(task_id)
    }

    pub fn cancel_export_task(&mut self, task_id: &str) -> Result<(), LogsError> {
        let task = self.export_tasks.get_mut(task_id).ok_or_else(|| {
            LogsError::resource_not_found("The specified export task does not exist.")
        })?;
        if task.status == "COMPLETED" || task.status == "CANCELLED" || task.status == "FAILED" {
            return Err(LogsError::invalid_parameter(
                "The export task is already in a terminal state.",
            ));
        }
        task.status = "CANCELLED".to_string();
        Ok(())
    }

    pub fn describe_export_tasks(
        &self,
        task_id: Option<&str>,
    ) -> Result<Vec<&ExportTask>, LogsError> {
        if let Some(id) = task_id {
            if !self.export_tasks.contains_key(id) {
                return Err(LogsError::resource_not_found(
                    "The specified export task does not exist.",
                ));
            }
        }
        Ok(self
            .export_tasks
            .values()
            .filter(|t| {
                if let Some(id) = task_id {
                    t.task_id == id
                } else {
                    true
                }
            })
            .collect())
    }

    // ========== Queries ==========

    pub fn start_query(
        &mut self,
        log_group_name: &str,
        query_string: &str,
        _start_time: i64,
        _end_time: i64,
    ) -> Result<String, LogsError> {
        if !self.log_groups.contains_key(log_group_name) {
            return Err(LogsError::resource_not_found(
                "The specified log group does not exist.",
            ));
        }
        let query_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().timestamp_millis();
        let query = Query {
            query_id: query_id.clone(),
            log_group_name: log_group_name.to_string(),
            query_string: query_string.to_string(),
            status: "Complete".to_string(),
            create_time: now,
        };
        self.queries.insert(query_id.clone(), query);
        Ok(query_id)
    }

    pub fn stop_query(&mut self, query_id: &str) -> Result<bool, LogsError> {
        let query = self
            .queries
            .get_mut(query_id)
            .ok_or_else(|| LogsError::resource_not_found("The specified query does not exist."))?;
        if query.status == "Complete" || query.status == "Cancelled" {
            return Ok(false);
        }
        query.status = "Cancelled".to_string();
        Ok(true)
    }

    pub fn describe_queries(&self, log_group_name: Option<&str>) -> Vec<&Query> {
        self.queries
            .values()
            .filter(|q| {
                if let Some(lgn) = log_group_name {
                    q.log_group_name == lgn
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn get_query_results(&self, query_id: &str) -> Result<&Query, LogsError> {
        self.queries
            .get(query_id)
            .ok_or_else(|| LogsError::resource_not_found("The specified query does not exist."))
    }

    // ========== Filter Log Events ==========

    pub fn filter_log_events(
        &self,
        log_group_name: &str,
        log_stream_names: Option<&[String]>,
        filter_pattern: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<usize>,
    ) -> Result<Vec<FilteredLogEvent>, LogsError> {
        let group = self.log_groups.get(log_group_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log group does not exist.")
        })?;

        let mut results = Vec::new();

        for (stream_name, stream) in &group.streams {
            if let Some(names) = log_stream_names {
                if !names.iter().any(|n| n == stream_name) {
                    continue;
                }
            }

            for event in &stream.events {
                if let Some(start) = start_time {
                    if event.timestamp < start {
                        continue;
                    }
                }
                if let Some(end) = end_time {
                    if event.timestamp >= end {
                        continue;
                    }
                }
                if let Some(pattern) = filter_pattern {
                    if !pattern.is_empty() {
                        // Multi-word pattern: all words must appear in the message
                        let words: Vec<&str> = pattern.split_whitespace().collect();
                        let all_match = words.iter().all(|w| event.message.contains(w));
                        if !all_match {
                            continue;
                        }
                    }
                }
                results.push(FilteredLogEvent {
                    log_stream_name: stream_name.clone(),
                    timestamp: event.timestamp,
                    message: event.message.clone(),
                    ingestion_time: event.ingestion_time,
                    event_id: uuid::Uuid::new_v4().to_string(),
                });
            }
        }

        results.sort_by_key(|e| e.timestamp);

        if let Some(lim) = limit {
            results.truncate(lim);
        }

        Ok(results)
    }

    // ========== Tags (legacy log group APIs) ==========

    pub fn tag_log_group(
        &mut self,
        log_group_name: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), LogsError> {
        let group = self.log_groups.get_mut(log_group_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log group does not exist.")
        })?;
        group.tags.extend(tags);
        Ok(())
    }

    pub fn untag_log_group(
        &mut self,
        log_group_name: &str,
        tag_keys: &[String],
    ) -> Result<(), LogsError> {
        let group = self.log_groups.get_mut(log_group_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log group does not exist.")
        })?;
        for key in tag_keys {
            group.tags.remove(key);
        }
        Ok(())
    }

    pub fn list_tags_log_group(
        &self,
        log_group_name: &str,
    ) -> Result<&HashMap<String, String>, LogsError> {
        let group = self.log_groups.get(log_group_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log group does not exist.")
        })?;
        Ok(&group.tags)
    }

    // ========== Tags (new ARN-based APIs) ==========

    // FIX(terraform-e2e): ARN normalization in tag_resource, untag_resource, and
    // list_tags_for_resource. Winterbaume stores log group ARNs with a trailing `:*`
    // suffix (matching AWS API docs), but the terraform provider sends ARNs without
    // the suffix. Stripping `:*` before comparison fixes the mismatch.
    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), LogsError> {
        let normalized = resource_arn.strip_suffix(":*").unwrap_or(resource_arn);
        for group in self.log_groups.values_mut() {
            let group_arn = group.arn.strip_suffix(":*").unwrap_or(&group.arn);
            if group_arn == normalized {
                group.tags.extend(tags);
                return Ok(());
            }
        }
        for dest in self.destinations.values_mut() {
            let dest_arn = dest.arn.strip_suffix(":*").unwrap_or(&dest.arn);
            if dest_arn == normalized {
                dest.tags.extend(tags);
                return Ok(());
            }
        }
        Err(LogsError::resource_not_found(
            "The specified resource does not exist.",
        ))
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), LogsError> {
        let normalized = resource_arn.strip_suffix(":*").unwrap_or(resource_arn);
        for group in self.log_groups.values_mut() {
            let group_arn = group.arn.strip_suffix(":*").unwrap_or(&group.arn);
            if group_arn == normalized {
                for key in tag_keys {
                    group.tags.remove(key);
                }
                return Ok(());
            }
        }
        for dest in self.destinations.values_mut() {
            let dest_arn = dest.arn.strip_suffix(":*").unwrap_or(&dest.arn);
            if dest_arn == normalized {
                for key in tag_keys {
                    dest.tags.remove(key);
                }
                return Ok(());
            }
        }
        Err(LogsError::resource_not_found(
            "The specified resource does not exist.",
        ))
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<&HashMap<String, String>, LogsError> {
        // CloudWatch Logs ARNs may or may not have a trailing `:*` suffix.
        // Normalize for comparison.
        let normalized = resource_arn.strip_suffix(":*").unwrap_or(resource_arn);
        for group in self.log_groups.values() {
            let group_arn = group.arn.strip_suffix(":*").unwrap_or(&group.arn);
            if group_arn == normalized {
                return Ok(&group.tags);
            }
        }
        for dest in self.destinations.values() {
            let dest_arn = dest.arn.strip_suffix(":*").unwrap_or(&dest.arn);
            if dest_arn == normalized {
                return Ok(&dest.tags);
            }
        }
        Err(LogsError::resource_not_found(
            "The specified resource does not exist.",
        ))
    }

    // ========== Delivery Sources ==========

    pub fn put_delivery_source(&mut self, source: DeliverySource) {
        self.delivery_sources.insert(source.name.clone(), source);
    }

    pub fn get_delivery_source(&self, name: &str) -> Result<&DeliverySource, LogsError> {
        self.delivery_sources.get(name).ok_or_else(|| {
            LogsError::resource_not_found("The specified delivery source does not exist.")
        })
    }

    pub fn delete_delivery_source(&mut self, name: &str) -> Result<(), LogsError> {
        if self.delivery_sources.remove(name).is_none() {
            return Err(LogsError::resource_not_found(
                "The specified delivery source does not exist.",
            ));
        }
        Ok(())
    }

    pub fn describe_delivery_sources(&self) -> Vec<&DeliverySource> {
        self.delivery_sources.values().collect()
    }

    // ========== Delivery Destinations ==========

    pub fn put_delivery_destination(&mut self, dest: DeliveryDestination) {
        self.delivery_destinations.insert(dest.name.clone(), dest);
    }

    pub fn get_delivery_destination(&self, name: &str) -> Result<&DeliveryDestination, LogsError> {
        self.delivery_destinations.get(name).ok_or_else(|| {
            LogsError::resource_not_found("The specified delivery destination does not exist.")
        })
    }

    pub fn delete_delivery_destination(&mut self, name: &str) -> Result<(), LogsError> {
        if self.delivery_destinations.remove(name).is_none() {
            return Err(LogsError::resource_not_found(
                "The specified delivery destination does not exist.",
            ));
        }
        Ok(())
    }

    pub fn describe_delivery_destinations(&self) -> Vec<&DeliveryDestination> {
        self.delivery_destinations.values().collect()
    }

    // ========== Delivery Destination Policy ==========

    pub fn put_delivery_destination_policy(
        &mut self,
        delivery_destination_name: &str,
        policy: &str,
    ) -> Result<(), LogsError> {
        if !self
            .delivery_destinations
            .contains_key(delivery_destination_name)
        {
            return Err(LogsError::resource_not_found(
                "The specified delivery destination does not exist.",
            ));
        }
        let p = DeliveryDestinationPolicy {
            delivery_destination_name: delivery_destination_name.to_string(),
            policy: policy.to_string(),
        };
        self.delivery_destination_policies
            .insert(delivery_destination_name.to_string(), p);
        Ok(())
    }

    pub fn get_delivery_destination_policy(
        &self,
        delivery_destination_name: &str,
    ) -> Result<&DeliveryDestinationPolicy, LogsError> {
        self.delivery_destination_policies
            .get(delivery_destination_name)
            .ok_or_else(|| {
                LogsError::resource_not_found(
                    "The specified delivery destination policy does not exist.",
                )
            })
    }

    pub fn delete_delivery_destination_policy(
        &mut self,
        delivery_destination_name: &str,
    ) -> Result<(), LogsError> {
        if self
            .delivery_destination_policies
            .remove(delivery_destination_name)
            .is_none()
        {
            return Err(LogsError::resource_not_found(
                "The specified delivery destination policy does not exist.",
            ));
        }
        Ok(())
    }

    // ========== Deliveries ==========

    pub fn create_delivery(
        &mut self,
        source: &str,
        destination: &str,
        tags: HashMap<String, String>,
    ) -> Result<&Delivery, LogsError> {
        if !self.delivery_sources.contains_key(source) {
            return Err(LogsError::resource_not_found(
                "The specified delivery source does not exist.",
            ));
        }
        if !self.delivery_destinations.contains_key(destination) {
            return Err(LogsError::resource_not_found(
                "The specified delivery destination does not exist.",
            ));
        }
        let id = uuid::Uuid::new_v4().to_string();
        let delivery = Delivery {
            id: id.clone(),
            source: source.to_string(),
            destination: destination.to_string(),
            tags,
        };
        self.deliveries.insert(id.clone(), delivery);
        Ok(self.deliveries.get(&id).unwrap())
    }

    pub fn get_delivery(&self, id: &str) -> Result<&Delivery, LogsError> {
        self.deliveries
            .get(id)
            .ok_or_else(|| LogsError::resource_not_found("The specified delivery does not exist."))
    }

    pub fn delete_delivery(&mut self, id: &str) -> Result<(), LogsError> {
        if self.deliveries.remove(id).is_none() {
            return Err(LogsError::resource_not_found(
                "The specified delivery does not exist.",
            ));
        }
        Ok(())
    }

    pub fn describe_deliveries(&self) -> Vec<&Delivery> {
        self.deliveries.values().collect()
    }

    // ========== KMS Key Association ==========

    pub fn associate_kms_key(
        &mut self,
        log_group_name: &str,
        kms_key_id: &str,
    ) -> Result<(), LogsError> {
        let group = self.log_groups.get_mut(log_group_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log group does not exist.")
        })?;
        group.kms_key_id = Some(kms_key_id.to_string());
        Ok(())
    }

    pub fn disassociate_kms_key(&mut self, log_group_name: &str) -> Result<(), LogsError> {
        let group = self.log_groups.get_mut(log_group_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log group does not exist.")
        })?;
        group.kms_key_id = None;
        Ok(())
    }

    // ========== Account Policies ==========

    pub fn put_account_policy(&mut self, policy: AccountPolicy) -> &AccountPolicy {
        let key = format!("{}:{}", policy.policy_type, policy.policy_name);
        self.account_policies.insert(key.clone(), policy);
        self.account_policies.get(&key).unwrap()
    }

    pub fn delete_account_policy(
        &mut self,
        policy_name: &str,
        policy_type: &str,
    ) -> Result<(), LogsError> {
        let key = format!("{policy_type}:{policy_name}");
        if self.account_policies.remove(&key).is_none() {
            return Err(LogsError::resource_not_found(
                "The specified account policy does not exist.",
            ));
        }
        Ok(())
    }

    pub fn describe_account_policies(
        &self,
        policy_type: &str,
        policy_name: Option<&str>,
    ) -> Vec<&AccountPolicy> {
        self.account_policies
            .values()
            .filter(|p| {
                if p.policy_type != policy_type {
                    return false;
                }
                if let Some(name) = policy_name {
                    if p.policy_name != name {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    // ========== Data Protection Policy ==========

    pub fn put_data_protection_policy(
        &mut self,
        log_group_identifier: &str,
        policy_document: &str,
    ) -> Result<(), LogsError> {
        let group = self
            .log_groups
            .get_mut(log_group_identifier)
            .ok_or_else(|| {
                LogsError::resource_not_found("The specified log group does not exist.")
            })?;
        group.data_protection_policy = Some(policy_document.to_string());
        Ok(())
    }

    pub fn get_data_protection_policy(
        &self,
        log_group_identifier: &str,
    ) -> Result<Option<&str>, LogsError> {
        let group = self.log_groups.get(log_group_identifier).ok_or_else(|| {
            LogsError::resource_not_found("The specified log group does not exist.")
        })?;
        Ok(group.data_protection_policy.as_deref())
    }

    pub fn delete_data_protection_policy(
        &mut self,
        log_group_identifier: &str,
    ) -> Result<(), LogsError> {
        let group = self
            .log_groups
            .get_mut(log_group_identifier)
            .ok_or_else(|| {
                LogsError::resource_not_found("The specified log group does not exist.")
            })?;
        group.data_protection_policy = None;
        Ok(())
    }

    // ========== Query Definitions ==========

    pub fn put_query_definition(&mut self, qd: QueryDefinition) -> String {
        let id = qd.query_definition_id.clone();
        self.query_definitions.insert(id.clone(), qd);
        id
    }

    pub fn delete_query_definition(
        &mut self,
        query_definition_id: &str,
    ) -> Result<bool, LogsError> {
        if self.query_definitions.remove(query_definition_id).is_some() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn describe_query_definitions(
        &self,
        query_definition_name_prefix: Option<&str>,
    ) -> Vec<&QueryDefinition> {
        self.query_definitions
            .values()
            .filter(|qd| {
                if let Some(prefix) = query_definition_name_prefix {
                    qd.name.starts_with(prefix)
                } else {
                    true
                }
            })
            .collect()
    }

    // ========== Log Anomaly Detectors ==========

    pub fn create_log_anomaly_detector(&mut self, detector: LogAnomalyDetector) -> String {
        let arn = detector.anomaly_detector_arn.clone();
        self.anomaly_detectors.insert(arn.clone(), detector);
        arn
    }

    pub fn get_log_anomaly_detector(
        &self,
        anomaly_detector_arn: &str,
    ) -> Result<&LogAnomalyDetector, LogsError> {
        self.anomaly_detectors
            .get(anomaly_detector_arn)
            .ok_or_else(|| {
                LogsError::resource_not_found("The specified anomaly detector does not exist.")
            })
    }

    pub fn update_log_anomaly_detector(
        &mut self,
        anomaly_detector_arn: &str,
        evaluation_frequency: Option<&str>,
        filter_pattern: Option<&str>,
        anomaly_visibility_time: Option<i64>,
        enabled: Option<bool>,
    ) -> Result<(), LogsError> {
        let detector = self
            .anomaly_detectors
            .get_mut(anomaly_detector_arn)
            .ok_or_else(|| {
                LogsError::resource_not_found("The specified anomaly detector does not exist.")
            })?;
        if let Some(ef) = evaluation_frequency {
            detector.evaluation_frequency = Some(ef.to_string());
        }
        if let Some(fp) = filter_pattern {
            detector.filter_pattern = Some(fp.to_string());
        }
        if let Some(avt) = anomaly_visibility_time {
            detector.anomaly_visibility_time = Some(avt);
        }
        if let Some(en) = enabled {
            detector.status = if en {
                "ENABLED".to_string()
            } else {
                "DISABLED".to_string()
            };
        }
        detector.last_modified_time = Utc::now().timestamp_millis();
        Ok(())
    }

    pub fn delete_log_anomaly_detector(
        &mut self,
        anomaly_detector_arn: &str,
    ) -> Result<(), LogsError> {
        if self
            .anomaly_detectors
            .remove(anomaly_detector_arn)
            .is_none()
        {
            return Err(LogsError::resource_not_found(
                "The specified anomaly detector does not exist.",
            ));
        }
        Ok(())
    }

    pub fn list_log_anomaly_detectors(
        &self,
        filter_log_group_arn: Option<&str>,
    ) -> Vec<&LogAnomalyDetector> {
        self.anomaly_detectors
            .values()
            .filter(|d| {
                if let Some(arn) = filter_log_group_arn {
                    d.log_group_arn_list.iter().any(|a| a == arn)
                } else {
                    true
                }
            })
            .collect()
    }

    // ========== Anomalies ==========

    pub fn list_anomalies(&self, anomaly_detector_arn: Option<&str>) -> Vec<&Anomaly> {
        self.anomalies
            .values()
            .filter(|a| {
                if let Some(arn) = anomaly_detector_arn {
                    a.anomaly_detector_arn == arn
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn update_anomaly(
        &mut self,
        anomaly_id: &str,
        anomaly_detector_arn: &str,
        suppression_type: Option<&str>,
        suppression_period_value: Option<i64>,
    ) -> Result<(), LogsError> {
        // If anomaly_id is provided, update that specific anomaly
        if !anomaly_id.is_empty() {
            let anomaly = self.anomalies.get_mut(anomaly_id).ok_or_else(|| {
                LogsError::resource_not_found("The specified anomaly does not exist.")
            })?;
            if anomaly.anomaly_detector_arn != anomaly_detector_arn {
                return Err(LogsError::invalid_parameter(
                    "The anomaly does not belong to the specified detector.",
                ));
            }
            if suppression_type == Some("SUPPRESSED") {
                anomaly.suppressed = true;
                anomaly.suppressed_date = Some(Utc::now().timestamp_millis());
                if let Some(val) = suppression_period_value {
                    anomaly.suppressed_until = Some(Utc::now().timestamp_millis() + val * 1000);
                }
            } else {
                anomaly.suppressed = false;
                anomaly.suppressed_date = None;
                anomaly.suppressed_until = None;
            }
        }
        Ok(())
    }

    // ========== Index Policies ==========

    pub fn put_index_policy(
        &mut self,
        log_group_identifier: &str,
        policy_document: &str,
    ) -> Result<&IndexPolicy, LogsError> {
        if !self.log_groups.contains_key(log_group_identifier) {
            // Check by ARN
            let found = self
                .log_groups
                .values()
                .any(|g| g.arn == log_group_identifier);
            if !found {
                return Err(LogsError::resource_not_found(
                    "The specified log group does not exist.",
                ));
            }
        }
        let now = Utc::now().timestamp_millis();
        let policy = IndexPolicy {
            policy_name: format!("{log_group_identifier}-index-policy"),
            log_group_identifier: log_group_identifier.to_string(),
            policy_document: policy_document.to_string(),
            last_update_time: now,
        };
        self.index_policies
            .insert(log_group_identifier.to_string(), policy);
        Ok(self.index_policies.get(log_group_identifier).unwrap())
    }

    pub fn delete_index_policy(&mut self, log_group_identifier: &str) -> Result<(), LogsError> {
        if self.index_policies.remove(log_group_identifier).is_none() {
            return Err(LogsError::resource_not_found(
                "The specified index policy does not exist.",
            ));
        }
        Ok(())
    }

    pub fn describe_index_policies(&self, log_group_identifiers: &[String]) -> Vec<&IndexPolicy> {
        self.index_policies
            .values()
            .filter(|p| {
                if log_group_identifiers.is_empty() {
                    true
                } else {
                    log_group_identifiers.contains(&p.log_group_identifier)
                }
            })
            .collect()
    }

    // ========== Transformers ==========

    pub fn put_transformer(
        &mut self,
        log_group_identifier: &str,
        transformers: Vec<serde_json::Value>,
    ) -> Result<(), LogsError> {
        if !self.log_groups.contains_key(log_group_identifier) {
            let found = self
                .log_groups
                .values()
                .any(|g| g.arn == log_group_identifier);
            if !found {
                return Err(LogsError::resource_not_found(
                    "The specified log group does not exist.",
                ));
            }
        }
        let now = Utc::now().timestamp_millis();
        let transformer = Transformer {
            log_group_identifier: log_group_identifier.to_string(),
            transformers,
            creation_time: now,
            last_modified_time: now,
        };
        self.transformers
            .insert(log_group_identifier.to_string(), transformer);
        Ok(())
    }

    pub fn get_transformer(&self, log_group_identifier: &str) -> Result<&Transformer, LogsError> {
        self.transformers.get(log_group_identifier).ok_or_else(|| {
            LogsError::resource_not_found("The specified transformer does not exist.")
        })
    }

    pub fn delete_transformer(&mut self, log_group_identifier: &str) -> Result<(), LogsError> {
        if self.transformers.remove(log_group_identifier).is_none() {
            return Err(LogsError::resource_not_found(
                "The specified transformer does not exist.",
            ));
        }
        Ok(())
    }

    // ========== Integrations ==========

    pub fn put_integration(&mut self, integration: Integration) -> &Integration {
        let name = integration.integration_name.clone();
        self.integrations.insert(name.clone(), integration);
        self.integrations.get(&name).unwrap()
    }

    pub fn get_integration(&self, integration_name: &str) -> Result<&Integration, LogsError> {
        self.integrations.get(integration_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified integration does not exist.")
        })
    }

    pub fn delete_integration(&mut self, integration_name: &str) -> Result<(), LogsError> {
        if self.integrations.remove(integration_name).is_none() {
            return Err(LogsError::resource_not_found(
                "The specified integration does not exist.",
            ));
        }
        Ok(())
    }

    pub fn list_integrations(&self) -> Vec<&Integration> {
        self.integrations.values().collect()
    }

    // ========== Import Tasks ==========

    pub fn create_import_task(&mut self, task: ImportTask) -> String {
        let id = task.task_id.clone();
        self.import_tasks.insert(id.clone(), task);
        id
    }

    pub fn cancel_import_task(&mut self, task_id: &str) -> Result<(), LogsError> {
        let task = self.import_tasks.get_mut(task_id).ok_or_else(|| {
            LogsError::resource_not_found("The specified import task does not exist.")
        })?;
        task.status = "CANCELLED".to_string();
        Ok(())
    }

    pub fn describe_import_tasks(&self, task_id: Option<&str>) -> Vec<&ImportTask> {
        self.import_tasks
            .values()
            .filter(|t| {
                if let Some(id) = task_id {
                    t.task_id == id
                } else {
                    true
                }
            })
            .collect()
    }

    // ========== Scheduled Queries ==========

    pub fn create_scheduled_query(&mut self, sq: ScheduledQuery) -> String {
        let id = sq.scheduled_query_id.clone();
        self.scheduled_queries.insert(id.clone(), sq);
        id
    }

    pub fn get_scheduled_query(&self, id: &str) -> Result<&ScheduledQuery, LogsError> {
        self.scheduled_queries.get(id).ok_or_else(|| {
            LogsError::resource_not_found("The specified scheduled query does not exist.")
        })
    }

    pub fn update_scheduled_query(
        &mut self,
        id: &str,
        enabled: Option<bool>,
    ) -> Result<(), LogsError> {
        let sq = self.scheduled_queries.get_mut(id).ok_or_else(|| {
            LogsError::resource_not_found("The specified scheduled query does not exist.")
        })?;
        if let Some(en) = enabled {
            sq.status = if en {
                "ENABLED".to_string()
            } else {
                "DISABLED".to_string()
            };
        }
        Ok(())
    }

    pub fn delete_scheduled_query(&mut self, id: &str) -> Result<(), LogsError> {
        if self.scheduled_queries.remove(id).is_none() {
            return Err(LogsError::resource_not_found(
                "The specified scheduled query does not exist.",
            ));
        }
        Ok(())
    }

    pub fn list_scheduled_queries(&self) -> Vec<&ScheduledQuery> {
        self.scheduled_queries.values().collect()
    }

    // ========== Log Group Deletion Protection ==========

    pub fn put_log_group_deletion_protection(
        &mut self,
        log_group_name: &str,
        deletion_protection_enabled: bool,
    ) -> Result<(), LogsError> {
        let group = self.log_groups.get_mut(log_group_name).ok_or_else(|| {
            LogsError::resource_not_found("The specified log group does not exist.")
        })?;
        group.deletion_protection_enabled = deletion_protection_enabled;
        Ok(())
    }
}
