use std::collections::HashMap;

use chrono::Utc;
use md5::{Digest, Md5};

use crate::types::{Message, MessageAttribute, MessageMoveTask, Queue};

/// In-memory state for the SQS service.
#[derive(Debug, Default)]
pub struct SqsState {
    /// Queues keyed by queue name.
    pub queues: HashMap<String, QueueState>,
    /// Message move tasks.
    pub message_move_tasks: Vec<MessageMoveTask>,
    /// Counter for generating unique task handles.
    pub message_move_task_counter: u64,
}

/// State for a single queue (metadata + messages).
#[derive(Debug, Clone)]
pub struct QueueState {
    pub queue: Queue,
    pub messages: Vec<Message>,
    /// Receipt handles of deleted messages (for idempotent deletes).
    pub deleted_handles: std::collections::HashSet<String>,
    /// Tags associated with the queue.
    pub tags: HashMap<String, String>,
    /// Permissions keyed by label. Each label maps to (aws_account_ids, actions).
    pub permissions: HashMap<String, (Vec<String>, Vec<String>)>,
    /// The ARN of the dead-letter queue target (parsed from RedrivePolicy).
    pub dead_letter_target_arn: Option<String>,
    /// The raw RedrivePolicy JSON string (stored as-is).
    pub redrive_policy: Option<String>,
    /// The raw Policy JSON string (stored as-is).
    pub policy: Option<String>,
    /// FIFO deduplication index: dedup_id → (message_id, expires_at_ms).
    /// Entries past their expiry are evicted lazily on each send.
    #[allow(clippy::type_complexity)]
    pub dedup_index: HashMap<String, (String, i64)>,
}

/// FIFO deduplication window in milliseconds (5 minutes per AWS spec).
const DEDUP_WINDOW_MS: i64 = 5 * 60 * 1000;

/// Output of [`SqsState::send_message`].
#[derive(Debug, Clone)]
pub struct SendMessageOutcome {
    pub message_id: String,
    pub body_md5: String,
}

/// Error type for SQS operations.
#[derive(Debug, thiserror::Error)]
pub enum SqsError {
    #[error("Can only include alphanumeric characters, hyphens, or underscores. 1 to 80 in length")]
    InvalidQueueName,
    #[error(
        "The name of a FIFO queue can only include alphanumeric characters, hyphens, or underscores, must end with .fifo suffix and be 1 to 80 in length"
    )]
    InvalidFifoQueueName,
    #[error(
        "A queue already exists with the same name and a different value for attribute VisibilityTimeout"
    )]
    QueueAlreadyExists,
    #[error("The specified queue does not exist.")]
    NonExistentQueue,
    #[error("The input receipt handle \"{receipt_handle}\" is not a valid receipt handle.")]
    InvalidReceiptHandle { receipt_handle: String },
    #[error("The request must contain the parameter Actions.")]
    MissingActions,
    #[error("Value [] for parameter PrincipalId is invalid. Reason: Unable to verify.")]
    EmptyPrincipalId,
    #[error("{count} Actions were found, maximum allowed is 7.")]
    TooManyActions { count: usize },
    #[error(
        "Value SQS:{action} for parameter ActionName is invalid. Reason: Only the queue owner is allowed to invoke this action."
    )]
    DisallowedAction { action: String },
    #[error("Value {label} for parameter Label is invalid. Reason: Already exists.")]
    DuplicatePermissionLabel { label: String },
    #[error(
        "Value {label} for parameter Label is invalid. Reason: can't find label on existing policy."
    )]
    PermissionLabelNotFound { label: String },
    #[error(
        "The resource that you specified for the SourceArn parameter doesn't exist: {source_arn}"
    )]
    SourceArnNotFound { source_arn: String },
    #[error("A message move task is already running on the source queue.")]
    MessageMoveTaskAlreadyRunning,
    #[error("The task handle \"{task_handle}\" is not valid or does not exist.")]
    InvalidTaskHandle { task_handle: String },
    #[error("The message move task is not in a cancellable state.")]
    TaskNotCancellable,
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl SqsState {
    pub fn create_queue(
        &mut self,
        name: &str,
        region: &str,
        account_id: &str,
        attributes: &HashMap<String, String>,
        tags: Option<&HashMap<String, String>>,
    ) -> Result<&Queue, SqsError> {
        // Validate queue name
        if name.is_empty() || name.len() > 80 || !is_valid_queue_name(name) {
            return Err(SqsError::InvalidQueueName);
        }

        // Check if FifoQueue attribute is "true" but name doesn't end with .fifo
        let fifo_attr = attributes.get("FifoQueue").map(|v| v.as_str());
        if fifo_attr == Some("true") && !name.ends_with(".fifo") {
            return Err(SqsError::InvalidFifoQueueName);
        }

        if let Some(existing) = self.queues.get(name) {
            // Queue already exists - check if attributes differ
            let eq = &existing.queue;
            let vis = parse_attr(attributes, "VisibilityTimeout", eq.visibility_timeout);
            let delay = parse_attr(attributes, "DelaySeconds", eq.delay_seconds);
            let max_msg = parse_attr(attributes, "MaximumMessageSize", eq.maximum_message_size);
            let retention = parse_attr(
                attributes,
                "MessageRetentionPeriod",
                eq.message_retention_period,
            );
            let wait = parse_attr(
                attributes,
                "ReceiveMessageWaitTimeSeconds",
                eq.receive_wait_time_seconds,
            );

            // Compare redrive policy if specified
            let new_redrive = attributes.get("RedrivePolicy");
            let old_redrive = existing.redrive_policy.as_ref();
            let redrive_matches = match (new_redrive, old_redrive) {
                (None, _) => true,
                (Some(a), Some(b)) => a == b,
                (Some(_), None) => false,
            };

            if vis != eq.visibility_timeout
                || delay != eq.delay_seconds
                || max_msg != eq.maximum_message_size
                || retention != eq.message_retention_period
                || wait != eq.receive_wait_time_seconds
                || !redrive_matches
            {
                return Err(SqsError::QueueAlreadyExists);
            }

            return Ok(&self.queues.get(name).unwrap().queue);
        }

        let fifo = name.ends_with(".fifo");
        let now = Utc::now();
        let url = format!("https://sqs.{region}.amazonaws.com/{account_id}/{name}");
        let arn = format!("arn:aws:sqs:{region}:{account_id}:{name}");

        let redrive_policy_str = attributes.get("RedrivePolicy").cloned();
        let dead_letter_target_arn = extract_dead_letter_target_arn(attributes);
        let policy = attributes.get("Policy").cloned();

        let queue = Queue {
            name: name.to_string(),
            url,
            arn,
            region: region.to_string(),
            account_id: account_id.to_string(),
            created_timestamp: now,
            last_modified_timestamp: now,
            visibility_timeout: parse_attr(attributes, "VisibilityTimeout", 30),
            delay_seconds: parse_attr(attributes, "DelaySeconds", 0),
            maximum_message_size: parse_attr(attributes, "MaximumMessageSize", 262144),
            message_retention_period: parse_attr(attributes, "MessageRetentionPeriod", 345600),
            receive_wait_time_seconds: parse_attr(attributes, "ReceiveMessageWaitTimeSeconds", 0),
            fifo_queue: fifo,
            // FIX(terraform-e2e): Parse and store ContentBasedDeduplication for FIFO queues.
            // Without this, GetQueueAttributes always returned "false" and the terraform
            // provider kept polling for 25+ seconds waiting for the value to match.
            content_based_deduplication: attributes
                .get("ContentBasedDeduplication")
                .map(|v| v == "true")
                .unwrap_or(false),
        };

        let initial_tags = tags.cloned().unwrap_or_default();

        self.queues.insert(
            name.to_string(),
            QueueState {
                queue,
                messages: Vec::new(),
                deleted_handles: std::collections::HashSet::new(),
                tags: initial_tags,
                permissions: HashMap::new(),
                dead_letter_target_arn,
                redrive_policy: redrive_policy_str,
                policy,
                dedup_index: HashMap::new(),
            },
        );

        Ok(&self.queues.get(name).unwrap().queue)
    }

    pub fn delete_queue(&mut self, queue_name: &str) -> Result<(), SqsError> {
        if self.queues.remove(queue_name).is_none() {
            return Err(SqsError::NonExistentQueue);
        }
        Ok(())
    }

    pub fn get_queue_url(&self, queue_name: &str) -> Result<&str, SqsError> {
        self.queues
            .get(queue_name)
            .map(|qs| qs.queue.url.as_str())
            .ok_or(SqsError::NonExistentQueue)
    }

    pub fn list_queues(&self, prefix: Option<&str>) -> Vec<&Queue> {
        self.queues
            .values()
            .filter(|qs| {
                if let Some(p) = prefix {
                    qs.queue.name.starts_with(p)
                } else {
                    true
                }
            })
            .map(|qs| &qs.queue)
            .collect()
    }

    pub fn get_queue_attributes(
        &self,
        queue_name: &str,
    ) -> Result<HashMap<String, String>, SqsError> {
        let qs = self.get_queue_state(queue_name)?;
        let q = &qs.queue;
        let now_ms = Utc::now().timestamp_millis();
        let mut attrs = HashMap::new();
        attrs.insert("QueueArn".to_string(), q.arn.clone());
        attrs.insert(
            "ApproximateNumberOfMessages".to_string(),
            qs.messages
                .iter()
                .filter(|m| m.is_visible() && m.receipt_handle.is_none())
                .count()
                .to_string(),
        );
        // NotVisible = in-flight (received at least once but not yet expired)
        attrs.insert(
            "ApproximateNumberOfMessagesNotVisible".to_string(),
            qs.messages
                .iter()
                .filter(|m| now_ms < m.visible_at && m.approximate_receive_count > 0)
                .count()
                .to_string(),
        );
        // Delayed = never received yet, still hidden by send-time delay
        attrs.insert(
            "ApproximateNumberOfMessagesDelayed".to_string(),
            qs.messages
                .iter()
                .filter(|m| now_ms < m.visible_at && m.approximate_receive_count == 0)
                .count()
                .to_string(),
        );
        attrs.insert(
            "VisibilityTimeout".to_string(),
            q.visibility_timeout.to_string(),
        );
        attrs.insert("DelaySeconds".to_string(), q.delay_seconds.to_string());
        attrs.insert(
            "MaximumMessageSize".to_string(),
            q.maximum_message_size.to_string(),
        );
        attrs.insert(
            "MessageRetentionPeriod".to_string(),
            q.message_retention_period.to_string(),
        );
        attrs.insert(
            "ReceiveMessageWaitTimeSeconds".to_string(),
            q.receive_wait_time_seconds.to_string(),
        );
        attrs.insert(
            "CreatedTimestamp".to_string(),
            q.created_timestamp.timestamp().to_string(),
        );
        attrs.insert(
            "LastModifiedTimestamp".to_string(),
            q.last_modified_timestamp.timestamp().to_string(),
        );
        if q.fifo_queue {
            attrs.insert("FifoQueue".to_string(), "true".to_string());
            attrs.insert(
                "ContentBasedDeduplication".to_string(),
                q.content_based_deduplication.to_string(),
            );
            attrs.insert("DeduplicationScope".to_string(), "queue".to_string());
            attrs.insert("FifoThroughputLimit".to_string(), "perQueue".to_string());
        }
        if let Some(ref policy) = qs.redrive_policy {
            attrs.insert("RedrivePolicy".to_string(), policy.clone());
        }
        if let Some(ref policy) = qs.policy {
            attrs.insert("Policy".to_string(), policy.clone());
        }
        // FIX(terraform-e2e): SqsManagedSseEnabled must be present in GetQueueAttributes.
        // The terraform provider polls until it sees this attribute (since 2023).
        attrs.insert("SqsManagedSseEnabled".to_string(), "true".to_string());
        Ok(attrs)
    }

    pub fn send_message(
        &mut self,
        queue_name: &str,
        body: &str,
        delay_seconds: Option<u32>,
        message_attributes: HashMap<String, MessageAttribute>,
        message_group_id: Option<&str>,
        message_deduplication_id: Option<&str>,
    ) -> Result<SendMessageOutcome, SqsError> {
        let qs = self
            .queues
            .get(queue_name)
            .ok_or(SqsError::NonExistentQueue)?;

        let queue_delay = qs.queue.delay_seconds;
        let delay = delay_seconds.unwrap_or(queue_delay);
        let now_ms = Utc::now().timestamp_millis();
        let visible_at = now_ms + (delay as i64 * 1000);
        let body_md5 = compute_md5(body);

        // FIFO deduplication: compute the effective dedup id and short-circuit
        // duplicate sends within DEDUP_WINDOW_MS by returning the original
        // MessageId without enqueueing again.
        let dedup_id: Option<String> = if qs.queue.fifo_queue {
            if let Some(explicit) = message_deduplication_id {
                Some(explicit.to_string())
            } else if qs.queue.content_based_deduplication {
                Some(body_md5.clone())
            } else {
                None
            }
        } else {
            None
        };

        if let Some(ref did) = dedup_id {
            let qs_mut = self.queues.get_mut(queue_name).unwrap();
            // Evict expired entries.
            qs_mut
                .dedup_index
                .retain(|_, (_, expires)| *expires > now_ms);
            if let Some((existing_id, _)) = qs_mut.dedup_index.get(did) {
                return Ok(SendMessageOutcome {
                    message_id: existing_id.clone(),
                    body_md5,
                });
            }
        }

        let _ = message_group_id; // currently informational; FIFO ordering not yet enforced

        let message_id = uuid::Uuid::new_v4().to_string();

        let message = Message {
            message_id: message_id.clone(),
            body: body.to_string(),
            body_md5: body_md5.clone(),
            receipt_handle: None,
            all_receipt_handles: Vec::new(),
            sender_id: "AIDAIT2UOQQY3AUEKVGXU".to_string(),
            sent_timestamp: now_ms,
            approximate_first_receive_timestamp: None,
            approximate_receive_count: 0,
            visible_at,
            message_attributes,
        };

        let qs = self.queues.get_mut(queue_name).unwrap();
        qs.messages.push(message);
        if let Some(did) = dedup_id {
            qs.dedup_index
                .insert(did, (message_id.clone(), now_ms + DEDUP_WINDOW_MS));
        }
        Ok(SendMessageOutcome {
            message_id,
            body_md5,
        })
    }

    pub fn receive_message(
        &mut self,
        queue_name: &str,
        max_messages: usize,
        visibility_timeout: Option<u32>,
    ) -> Result<Vec<Message>, SqsError> {
        let qs = self
            .queues
            .get_mut(queue_name)
            .ok_or(SqsError::NonExistentQueue)?;

        let vis_timeout = visibility_timeout.unwrap_or(qs.queue.visibility_timeout);
        let now_ms = Utc::now().timestamp_millis();
        let max_receive_count = redrive_max_receive_count(qs.redrive_policy.as_deref());
        let dlq_target_arn = qs.dead_letter_target_arn.clone();

        let mut received = Vec::new();
        let mut to_redrive: Vec<Message> = Vec::new();

        let mut idx = 0;
        while idx < qs.messages.len() {
            if received.len() >= max_messages {
                break;
            }
            let msg = &mut qs.messages[idx];

            if now_ms < msg.visible_at {
                idx += 1;
                continue;
            }

            // Increment the receive count first; if we've already exceeded
            // maxReceiveCount before this attempt, the message must be
            // redriven rather than handed back to the caller.
            let new_count = msg.approximate_receive_count + 1;
            if let Some(max) = max_receive_count {
                if new_count > max {
                    let to_move = qs.messages.remove(idx);
                    to_redrive.push(to_move);
                    continue;
                }
            }

            let receipt_handle = generate_receipt_handle();
            msg.receipt_handle = Some(receipt_handle.clone());
            msg.all_receipt_handles.push(receipt_handle.clone());
            msg.approximate_receive_count = new_count;
            if msg.approximate_first_receive_timestamp.is_none() {
                msg.approximate_first_receive_timestamp = Some(now_ms);
            }
            msg.visible_at = now_ms + (vis_timeout as i64 * 1000);

            let mut clone = msg.clone();
            clone.receipt_handle = Some(receipt_handle);
            received.push(clone);
            idx += 1;
        }

        // Forward redriven messages to the DLQ (if it exists). If the target
        // ARN does not resolve to a known queue, drop them silently — this
        // matches moto's behaviour for an unconfigured DLQ.
        if !to_redrive.is_empty() {
            if let Some(arn) = dlq_target_arn {
                if let Some(dlq_name) = self.find_queue_name_by_arn(&arn) {
                    if let Some(dlq) = self.queues.get_mut(&dlq_name) {
                        for mut m in to_redrive {
                            m.receipt_handle = None;
                            m.all_receipt_handles.clear();
                            m.approximate_receive_count = 0;
                            m.approximate_first_receive_timestamp = None;
                            m.visible_at = now_ms;
                            dlq.messages.push(m);
                        }
                    }
                }
            }
        }

        Ok(received)
    }

    fn find_queue_name_by_arn(&self, arn: &str) -> Option<String> {
        self.queues
            .iter()
            .find(|(_, qs)| qs.queue.arn == arn)
            .map(|(name, _)| name.clone())
    }

    pub fn delete_message(
        &mut self,
        queue_name: &str,
        receipt_handle: &str,
    ) -> Result<(), SqsError> {
        let qs = self
            .queues
            .get_mut(queue_name)
            .ok_or(SqsError::NonExistentQueue)?;

        // Idempotent: already-deleted handles succeed silently
        if qs.deleted_handles.contains(receipt_handle) {
            return Ok(());
        }

        // Find by current receipt handle or any historical receipt handle
        let pos = qs.messages.iter().position(|m| {
            m.receipt_handle.as_deref() == Some(receipt_handle)
                || m.all_receipt_handles.iter().any(|h| h == receipt_handle)
        });

        match pos {
            Some(idx) => {
                // Mark all receipt handles for this message as deleted (for idempotent deletes)
                let msg = &qs.messages[idx];
                for h in &msg.all_receipt_handles {
                    qs.deleted_handles.insert(h.clone());
                }
                qs.deleted_handles.insert(receipt_handle.to_string());
                qs.messages.remove(idx);
                Ok(())
            }
            None => Err(SqsError::InvalidReceiptHandle {
                receipt_handle: receipt_handle.to_string(),
            }),
        }
    }

    pub fn purge_queue(&mut self, queue_name: &str) -> Result<(), SqsError> {
        let qs = self
            .queues
            .get_mut(queue_name)
            .ok_or(SqsError::NonExistentQueue)?;
        qs.messages.clear();
        Ok(())
    }

    pub fn set_queue_attributes(
        &mut self,
        queue_name: &str,
        attributes: &HashMap<String, String>,
    ) -> Result<(), SqsError> {
        let qs = self
            .queues
            .get_mut(queue_name)
            .ok_or(SqsError::NonExistentQueue)?;

        if let Some(v) = attributes.get("VisibilityTimeout")
            && let Ok(val) = v.parse()
        {
            qs.queue.visibility_timeout = val;
        }
        if let Some(v) = attributes.get("DelaySeconds")
            && let Ok(val) = v.parse()
        {
            qs.queue.delay_seconds = val;
        }
        if let Some(v) = attributes.get("MaximumMessageSize")
            && let Ok(val) = v.parse()
        {
            qs.queue.maximum_message_size = val;
        }
        if let Some(v) = attributes.get("MessageRetentionPeriod")
            && let Ok(val) = v.parse()
        {
            qs.queue.message_retention_period = val;
        }
        if let Some(v) = attributes.get("ReceiveMessageWaitTimeSeconds")
            && let Ok(val) = v.parse()
        {
            qs.queue.receive_wait_time_seconds = val;
        }

        if let Some(redrive) = attributes.get("RedrivePolicy") {
            if redrive.is_empty() || redrive == "{}" {
                qs.dead_letter_target_arn = None;
                qs.redrive_policy = None;
            } else {
                qs.dead_letter_target_arn = extract_dead_letter_target_arn_from_policy(redrive);
                qs.redrive_policy = Some(redrive.clone());
            }
        }

        if let Some(policy) = attributes.get("Policy") {
            if policy.is_empty() {
                qs.policy = None;
            } else {
                qs.policy = Some(policy.clone());
            }
        }

        // FIX(terraform-e2e): Allow updating ContentBasedDeduplication via SetQueueAttributes.
        if let Some(v) = attributes.get("ContentBasedDeduplication") {
            qs.queue.content_based_deduplication = v == "true";
        }

        qs.queue.last_modified_timestamp = Utc::now();
        Ok(())
    }

    pub fn list_queue_tags(&self, queue_name: &str) -> Result<HashMap<String, String>, SqsError> {
        let qs = self.get_queue_state(queue_name)?;
        Ok(qs.tags.clone())
    }

    pub fn tag_queue(
        &mut self,
        queue_name: &str,
        tags: &HashMap<String, String>,
    ) -> Result<(), SqsError> {
        let qs = self
            .queues
            .get_mut(queue_name)
            .ok_or(SqsError::NonExistentQueue)?;
        for (k, v) in tags {
            qs.tags.insert(k.clone(), v.clone());
        }
        Ok(())
    }

    pub fn untag_queue(&mut self, queue_name: &str, tag_keys: &[String]) -> Result<(), SqsError> {
        let qs = self
            .queues
            .get_mut(queue_name)
            .ok_or(SqsError::NonExistentQueue)?;
        for key in tag_keys {
            qs.tags.remove(key);
        }
        Ok(())
    }

    pub fn add_permission(
        &mut self,
        queue_name: &str,
        label: &str,
        aws_account_ids: Vec<String>,
        actions: Vec<String>,
    ) -> Result<(), SqsError> {
        let qs = self
            .queues
            .get_mut(queue_name)
            .ok_or(SqsError::NonExistentQueue)?;

        // Validate: empty actions
        if actions.is_empty() {
            return Err(SqsError::MissingActions);
        }

        // Validate: empty account ids
        if aws_account_ids.is_empty() {
            return Err(SqsError::EmptyPrincipalId);
        }

        // Validate: max 7 actions
        if actions.len() > 7 {
            return Err(SqsError::TooManyActions {
                count: actions.len(),
            });
        }

        // Validate: disallowed actions
        let disallowed = [
            "AddPermission",
            "RemovePermission",
            "TagQueue",
            "UntagQueue",
        ];
        for action in &actions {
            if disallowed.contains(&action.as_str()) {
                return Err(SqsError::DisallowedAction {
                    action: action.clone(),
                });
            }
        }

        // Validate: duplicate label
        if qs.permissions.contains_key(label) {
            return Err(SqsError::DuplicatePermissionLabel {
                label: label.to_string(),
            });
        }

        qs.permissions
            .insert(label.to_string(), (aws_account_ids, actions));
        self.rebuild_policy(queue_name);
        Ok(())
    }

    pub fn remove_permission(&mut self, queue_name: &str, label: &str) -> Result<(), SqsError> {
        let qs = self
            .queues
            .get_mut(queue_name)
            .ok_or(SqsError::NonExistentQueue)?;
        if !qs.permissions.contains_key(label) {
            return Err(SqsError::PermissionLabelNotFound {
                label: label.to_string(),
            });
        }
        qs.permissions.remove(label);
        self.rebuild_policy(queue_name);
        Ok(())
    }

    pub fn list_dead_letter_source_queues(
        &self,
        queue_name: &str,
    ) -> Result<Vec<String>, SqsError> {
        let target_qs = self.get_queue_state(queue_name)?;
        let target_arn = &target_qs.queue.arn;

        let urls: Vec<String> = self
            .queues
            .values()
            .filter(|qs| qs.dead_letter_target_arn.as_deref() == Some(target_arn.as_str()))
            .map(|qs| qs.queue.url.clone())
            .collect();
        Ok(urls)
    }

    pub fn change_message_visibility_batch(
        &mut self,
        queue_name: &str,
        entries: &[(String, String, u32)], // (id, receipt_handle, visibility_timeout)
    ) -> Result<Vec<String>, SqsError> {
        // Verify queue exists first
        if !self.queues.contains_key(queue_name) {
            return Err(SqsError::NonExistentQueue);
        }

        let mut successful_ids = Vec::new();
        for (id, receipt_handle, visibility_timeout) in entries {
            self.change_message_visibility(queue_name, receipt_handle, *visibility_timeout)?;
            successful_ids.push(id.clone());
        }
        Ok(successful_ids)
    }

    pub fn change_message_visibility(
        &mut self,
        queue_name: &str,
        receipt_handle: &str,
        visibility_timeout: u32,
    ) -> Result<(), SqsError> {
        let qs = self
            .queues
            .get_mut(queue_name)
            .ok_or(SqsError::NonExistentQueue)?;

        let now_ms = Utc::now().timestamp_millis();
        let msg = qs
            .messages
            .iter_mut()
            .find(|m| m.receipt_handle.as_deref() == Some(receipt_handle))
            .ok_or_else(|| SqsError::InvalidReceiptHandle {
                receipt_handle: receipt_handle.to_string(),
            })?;

        msg.visible_at = now_ms + (visibility_timeout as i64 * 1000);
        Ok(())
    }

    pub fn start_message_move_task(
        &mut self,
        source_arn: &str,
        destination_arn: Option<&str>,
        max_number_of_messages_per_second: Option<i32>,
    ) -> Result<String, SqsError> {
        // Validate source ARN refers to an existing queue
        let source_exists = self.queues.values().any(|qs| qs.queue.arn == source_arn);
        if !source_exists {
            return Err(SqsError::SourceArnNotFound {
                source_arn: source_arn.to_string(),
            });
        }

        // Check if there is already a running task for this source
        let already_running = self
            .message_move_tasks
            .iter()
            .any(|t| t.source_arn == source_arn && t.status == "RUNNING");
        if already_running {
            return Err(SqsError::MessageMoveTaskAlreadyRunning);
        }

        self.message_move_task_counter += 1;
        let task_handle = uuid::Uuid::new_v4().to_string();
        let now_ms = Utc::now().timestamp_millis();

        let task = MessageMoveTask {
            task_handle: task_handle.clone(),
            source_arn: source_arn.to_string(),
            destination_arn: destination_arn.map(|s| s.to_string()),
            status: "RUNNING".to_string(),
            max_number_of_messages_per_second,
            started_timestamp: now_ms,
            approximate_number_of_messages_moved: 0,
            approximate_number_of_messages_to_move: Some(0),
        };

        self.message_move_tasks.push(task);
        Ok(task_handle)
    }

    pub fn cancel_message_move_task(&mut self, task_handle: &str) -> Result<i64, SqsError> {
        let task = self
            .message_move_tasks
            .iter_mut()
            .find(|t| t.task_handle == task_handle)
            .ok_or_else(|| SqsError::InvalidTaskHandle {
                task_handle: task_handle.to_string(),
            })?;

        if task.status != "RUNNING" {
            return Err(SqsError::TaskNotCancellable);
        }

        task.status = "CANCELLING".to_string();
        Ok(task.approximate_number_of_messages_moved)
    }

    pub fn list_message_move_tasks(&self, source_arn: &str) -> Vec<&MessageMoveTask> {
        self.message_move_tasks
            .iter()
            .filter(|t| t.source_arn == source_arn)
            .collect()
    }

    fn get_queue_state(&self, queue_name: &str) -> Result<&QueueState, SqsError> {
        self.queues
            .get(queue_name)
            .ok_or(SqsError::NonExistentQueue)
    }

    /// Rebuild the Policy JSON from the permissions map.
    fn rebuild_policy(&mut self, queue_name: &str) {
        let qs = match self.queues.get_mut(queue_name) {
            Some(qs) => qs,
            None => return,
        };
        if qs.permissions.is_empty() {
            qs.policy = None;
            return;
        }
        let queue_arn = &qs.queue.arn;
        let statements: Vec<serde_json::Value> = qs
            .permissions
            .iter()
            .map(|(label, (account_ids, actions))| {
                let principal = if account_ids.len() == 1 {
                    serde_json::json!({"AWS": format!("arn:aws:iam::{}:root", account_ids[0])})
                } else {
                    let arns: Vec<String> = account_ids
                        .iter()
                        .map(|id| format!("arn:aws:iam::{}:root", id))
                        .collect();
                    serde_json::json!({"AWS": arns})
                };
                let action = if actions.len() == 1 {
                    serde_json::json!(format!("SQS:{}", actions[0]))
                } else {
                    let acts: Vec<String> = actions.iter().map(|a| format!("SQS:{a}")).collect();
                    serde_json::json!(acts)
                };
                serde_json::json!({
                    "Sid": label,
                    "Effect": "Allow",
                    "Principal": principal,
                    "Action": action,
                    "Resource": queue_arn,
                })
            })
            .collect();

        let policy = serde_json::json!({
            "Version": "2012-10-17",
            "Id": format!("{}/SQSDefaultPolicy", queue_arn),
            "Statement": statements,
        });
        qs.policy = Some(serde_json::to_string(&policy).unwrap());
    }
}

fn parse_attr(attrs: &HashMap<String, String>, key: &str, default: u32) -> u32 {
    attrs
        .get(key)
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn compute_md5(data: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn extract_dead_letter_target_arn(attributes: &HashMap<String, String>) -> Option<String> {
    attributes
        .get("RedrivePolicy")
        .and_then(|policy| extract_dead_letter_target_arn_from_policy(policy))
}

fn extract_dead_letter_target_arn_from_policy(policy: &str) -> Option<String> {
    serde_json::from_str::<serde_json::Value>(policy)
        .ok()
        .and_then(|v| {
            v.get("deadLetterTargetArn")
                .and_then(|a| a.as_str())
                .map(|s| s.to_string())
        })
}

/// Parse `maxReceiveCount` from a RedrivePolicy JSON string. AWS accepts the
/// value as either a JSON number or a quoted string.
fn redrive_max_receive_count(policy: Option<&str>) -> Option<u32> {
    let v: serde_json::Value = serde_json::from_str(policy?).ok()?;
    let m = v.get("maxReceiveCount")?;
    if let Some(n) = m.as_u64() {
        return Some(n as u32);
    }
    if let Some(s) = m.as_str() {
        return s.parse().ok();
    }
    None
}

fn is_valid_queue_name(name: &str) -> bool {
    // Queue names can contain alphanumeric, hyphens, underscores, and dots (for .fifo)
    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
}

fn generate_receipt_handle() -> String {
    // Generate a receipt handle similar to AWS format
    let parts: Vec<String> = (0..4).map(|_| uuid::Uuid::new_v4().to_string()).collect();
    parts.join("")
}
