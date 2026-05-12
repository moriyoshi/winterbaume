use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::backend::{InMemorySqsBackend, SqsBackend};
use crate::state::SqsError;
use crate::types::MessageAttribute;
use crate::views::SqsStateView;
use crate::wire;

/// SQS service handler that processes awsJson1.0 protocol requests.
pub struct SqsService {
    /// Pluggable storage backend for all queue, message, and state operations.
    pub(crate) backend: Arc<dyn SqsBackend>,
    pub(crate) notifier: StateChangeNotifier<SqsStateView>,
}

impl SqsService {
    pub fn new() -> Self {
        Self {
            backend: Arc::new(InMemorySqsBackend::new()),
            notifier: StateChangeNotifier::new(),
        }
    }

    /// Create an SQS service backed by a custom storage backend.
    pub fn with_backend(backend: Arc<dyn SqsBackend>) -> Self {
        Self {
            backend,
            notifier: StateChangeNotifier::new(),
        }
    }

    /// Returns all `(account_id, region)` pairs that have state.
    pub fn scopes_with_state(&self) -> Vec<(String, String)> {
        self.backend.scopes_with_state()
    }
}

impl Default for SqsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SqsService {
    fn service_name(&self) -> &str {
        "sqs"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://sqs\..*\.amazonaws\.com",
            r"https?://sqs\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SqsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id().to_string();

        // Extract action from X-Amz-Target header (awsJson1.0 protocol)
        // Format: "AmazonSQS.CreateQueue"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        // Validate JSON body shape; typed deserialisers re-parse per operation.
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "InvalidInput", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        match action.as_str() {
            "CreateQueue" => {
                self.handle_create_queue(body_bytes, account_id, region)
                    .await
            }
            "DeleteQueue" => {
                self.handle_delete_queue(body_bytes, account_id, region)
                    .await
            }
            "GetQueueUrl" => {
                self.handle_get_queue_url(body_bytes, account_id, region)
                    .await
            }
            "GetQueueAttributes" => {
                self.handle_get_queue_attributes(body_bytes, account_id, region)
                    .await
            }
            "ListQueues" => {
                self.handle_list_queues(body_bytes, account_id, region)
                    .await
            }
            "SendMessage" => {
                self.handle_send_message(body_bytes, account_id, region)
                    .await
            }
            "ReceiveMessage" => {
                self.handle_receive_message(body_bytes, account_id, region)
                    .await
            }
            "DeleteMessage" => {
                self.handle_delete_message(body_bytes, account_id, region)
                    .await
            }
            "PurgeQueue" => {
                self.handle_purge_queue(body_bytes, account_id, region)
                    .await
            }
            "SetQueueAttributes" => {
                self.handle_set_queue_attributes(body_bytes, account_id, region)
                    .await
            }
            "SendMessageBatch" => {
                self.handle_send_message_batch(body_bytes, account_id, region)
                    .await
            }
            "DeleteMessageBatch" => {
                self.handle_delete_message_batch(body_bytes, account_id, region)
                    .await
            }
            "ChangeMessageVisibility" => {
                self.handle_change_message_visibility(body_bytes, account_id, region)
                    .await
            }
            "ChangeMessageVisibilityBatch" => {
                self.handle_change_message_visibility_batch(body_bytes, account_id, region)
                    .await
            }
            "ListQueueTags" => {
                self.handle_list_queue_tags(body_bytes, account_id, region)
                    .await
            }
            "TagQueue" => self.handle_tag_queue(body_bytes, account_id, region).await,
            "UntagQueue" => {
                self.handle_untag_queue(body_bytes, account_id, region)
                    .await
            }
            "AddPermission" => {
                self.handle_add_permission(body_bytes, account_id, region)
                    .await
            }
            "RemovePermission" => {
                self.handle_remove_permission(body_bytes, account_id, region)
                    .await
            }
            "ListDeadLetterSourceQueues" => {
                self.handle_list_dead_letter_source_queues(body_bytes, account_id, region)
                    .await
            }
            "StartMessageMoveTask" => {
                self.handle_start_message_move_task(body_bytes, account_id, region)
                    .await
            }
            "CancelMessageMoveTask" => {
                self.handle_cancel_message_move_task(body_bytes, account_id, region)
                    .await
            }
            "ListMessageMoveTasks" => {
                self.handle_list_message_move_tasks(body_bytes, account_id, region)
                    .await
            }
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for SQS"),
            ),
        }
    }

    async fn handle_create_queue(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_create_queue_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        let queue_name = input.queue_name;

        let attributes = input.attributes.unwrap_or_default();
        let tags = input.tags.filter(|t| !t.is_empty());

        match self
            .backend
            .create_queue(account_id, region, queue_name, attributes, tags)
            .await
        {
            Ok(queue) => wire::serialize_create_queue_response(&wire::CreateQueueResult {
                queue_url: Some(queue.url),
            }),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_delete_queue(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_queue_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        let queue_name = extract_queue_name_from_url(&input.queue_url);
        match self
            .backend
            .delete_queue(account_id, region, queue_name)
            .await
        {
            Ok(()) => wire::serialize_delete_queue_response(),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_get_queue_url(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_get_queue_url_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        match self
            .backend
            .get_queue_url(account_id, region, input.queue_name)
            .await
        {
            Ok(url) => wire::serialize_get_queue_url_response(&wire::GetQueueUrlResult {
                queue_url: Some(url),
            }),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_get_queue_attributes(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_get_queue_attributes_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        let queue_name = extract_queue_name_from_url(&input.queue_url);
        match self
            .backend
            .get_queue_attributes(account_id, region, queue_name)
            .await
        {
            Ok(attrs) => {
                wire::serialize_get_queue_attributes_response(&wire::GetQueueAttributesResult {
                    attributes: Some(attrs),
                })
            }
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_list_queues(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_list_queues_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        let prefix = input.queue_name_prefix;
        match self.backend.list_queues(account_id, region, prefix).await {
            Ok(queues) => {
                let urls: Vec<String> = queues.into_iter().map(|q| q.url).collect();
                wire::serialize_list_queues_response(&wire::ListQueuesResult {
                    next_token: None,
                    queue_urls: Some(urls),
                })
            }
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_send_message(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_send_message_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        if input.message_body.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'MessageBody'");
        }
        // Treat DelaySeconds=0 as "not specified" so the queue's default
        // DelaySeconds applies. Some clients send DelaySeconds=0 even when
        // the user did not pass --delay-seconds.
        let delay_seconds = input.delay_seconds.filter(|v| *v > 0).map(|v| v as u32);
        let message_attributes = wire_message_attributes_to_state(input.message_attributes);
        let message_group_id = input.message_group_id;
        let message_deduplication_id = input.message_deduplication_id;
        let queue_name = extract_queue_name_from_url(&input.queue_url);

        match self
            .backend
            .send_message(
                account_id,
                region,
                queue_name,
                input.message_body,
                delay_seconds,
                message_attributes,
                message_group_id,
                message_deduplication_id,
            )
            .await
        {
            Ok(out) => wire::serialize_send_message_response(&wire::SendMessageResult {
                message_id: Some(out.message_id),
                m_d5_of_message_body: Some(out.body_md5),
                ..Default::default()
            }),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_receive_message(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_receive_message_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        let max_messages = input.max_number_of_messages.unwrap_or(1).max(0) as usize;
        let max_messages = if max_messages == 0 { 1 } else { max_messages };
        // Treat VisibilityTimeout=0 as "not specified" so the queue default
        // applies. Some clients (notably the aws-cli) send VisibilityTimeout=0
        // when the user did not pass --visibility-timeout, which would
        // otherwise bypass the queue's default and immediately re-expose
        // received messages.
        let visibility_timeout = input
            .visibility_timeout
            .filter(|v| *v > 0)
            .map(|v| v as u32);
        let wait_time_seconds = input.wait_time_seconds.map(|v| v.max(0) as u32);
        let queue_name = extract_queue_name_from_url(&input.queue_url);

        // Resolve the effective long-poll duration, honouring the queue's
        // ReceiveMessageWaitTimeSeconds when the request omits an explicit
        // value. SQS caps long-poll at 20 s.
        let effective_wait_seconds = match wait_time_seconds {
            Some(v) => v,
            None => self
                .backend
                .get_queue_attributes(account_id.clone(), region.clone(), queue_name.clone())
                .await
                .ok()
                .and_then(|attrs| {
                    attrs
                        .get("ReceiveMessageWaitTimeSeconds")
                        .and_then(|v| v.parse().ok())
                })
                .unwrap_or(0),
        }
        .min(20);

        let deadline = if effective_wait_seconds > 0 {
            Some(
                tokio::time::Instant::now()
                    + std::time::Duration::from_secs(effective_wait_seconds as u64),
            )
        } else {
            None
        };

        let messages = loop {
            match self
                .backend
                .receive_messages(
                    account_id.clone(),
                    region.clone(),
                    queue_name.clone(),
                    max_messages,
                    visibility_timeout,
                )
                .await
            {
                Ok(msgs) => {
                    if !msgs.is_empty() {
                        break Ok(msgs);
                    }
                    match deadline {
                        Some(d) if tokio::time::Instant::now() < d => {
                            let remaining =
                                d.saturating_duration_since(tokio::time::Instant::now());
                            let sleep =
                                std::cmp::min(remaining, std::time::Duration::from_millis(100));
                            tokio::time::sleep(sleep).await;
                        }
                        _ => break Ok(msgs),
                    }
                }
                Err(e) => break Err(e),
            }
        };

        match messages {
            Ok(messages) => {
                let msgs: Vec<wire::Message> = messages
                    .iter()
                    .map(|m| {
                        let mut attrs = HashMap::new();
                        attrs.insert("SenderId".to_string(), m.sender_id.clone());
                        attrs.insert("SentTimestamp".to_string(), m.sent_timestamp.to_string());
                        attrs.insert(
                            "ApproximateReceiveCount".to_string(),
                            m.approximate_receive_count.to_string(),
                        );
                        attrs.insert(
                            "ApproximateFirstReceiveTimestamp".to_string(),
                            m.approximate_first_receive_timestamp
                                .map(|t| t.to_string())
                                .unwrap_or_default(),
                        );

                        let message_attributes = if m.message_attributes.is_empty() {
                            None
                        } else {
                            let mut wire_attrs = HashMap::new();
                            for (k, v) in &m.message_attributes {
                                wire_attrs.insert(
                                    k.clone(),
                                    wire::MessageAttributeValue {
                                        data_type: v.data_type.clone(),
                                        string_value: v.string_value.clone(),
                                        ..Default::default()
                                    },
                                );
                            }
                            Some(wire_attrs)
                        };

                        wire::Message {
                            message_id: Some(m.message_id.clone()),
                            receipt_handle: m.receipt_handle.clone(),
                            m_d5_of_body: Some(m.body_md5.clone()),
                            body: Some(m.body.clone()),
                            attributes: Some(attrs),
                            message_attributes,
                            ..Default::default()
                        }
                    })
                    .collect();

                wire::serialize_receive_message_response(&wire::ReceiveMessageResult {
                    messages: if msgs.is_empty() { None } else { Some(msgs) },
                })
            }
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_delete_message(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_message_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        if input.receipt_handle.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'ReceiptHandle'");
        }
        let queue_name = extract_queue_name_from_url(&input.queue_url);
        match self
            .backend
            .delete_message(account_id, region, queue_name, input.receipt_handle)
            .await
        {
            Ok(()) => wire::serialize_delete_message_response(),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_purge_queue(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_purge_queue_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        let queue_name = extract_queue_name_from_url(&input.queue_url);
        match self
            .backend
            .purge_queue(account_id, region, queue_name)
            .await
        {
            Ok(()) => wire::serialize_purge_queue_response(),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_set_queue_attributes(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_set_queue_attributes_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        let attributes = input.attributes;
        let queue_name = extract_queue_name_from_url(&input.queue_url);
        match self
            .backend
            .set_queue_attributes(account_id, region, queue_name, attributes)
            .await
        {
            Ok(()) => wire::serialize_set_queue_attributes_response(),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_send_message_batch(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_send_message_batch_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        let entries = input.entries;

        let queue_name = extract_queue_name_from_url(&input.queue_url);
        let mut successful = Vec::new();

        for entry in entries {
            let id = entry.id;
            let message_body = entry.message_body;
            if message_body.is_empty() {
                continue;
            }
            let delay_seconds = entry.delay_seconds.filter(|v| *v > 0).map(|v| v as u32);
            let message_attributes = wire_message_attributes_to_state(entry.message_attributes);
            let message_group_id = entry.message_group_id;
            let message_deduplication_id = entry.message_deduplication_id;

            match self
                .backend
                .send_message(
                    account_id.clone(),
                    region.clone(),
                    queue_name.clone(),
                    message_body,
                    delay_seconds,
                    message_attributes,
                    message_group_id,
                    message_deduplication_id,
                )
                .await
            {
                Ok(out) => {
                    successful.push(wire::SendMessageBatchResultEntry {
                        id: Some(id),
                        message_id: Some(out.message_id),
                        m_d5_of_message_body: Some(out.body_md5),
                        ..Default::default()
                    });
                }
                Err(e) => return sqs_error_response(&e),
            }
        }

        wire::serialize_send_message_batch_response(&wire::SendMessageBatchResult {
            successful: Some(successful),
            failed: Some(vec![]),
        })
    }

    async fn handle_delete_message_batch(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_message_batch_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        let entries = input.entries;

        let queue_name = extract_queue_name_from_url(&input.queue_url);
        let mut successful = Vec::new();

        for entry in entries {
            let id = entry.id;
            let receipt_handle = entry.receipt_handle;
            if receipt_handle.is_empty() {
                continue;
            }
            match self
                .backend
                .delete_message(
                    account_id.clone(),
                    region.clone(),
                    queue_name.clone(),
                    receipt_handle,
                )
                .await
            {
                Ok(()) => {
                    successful.push(wire::DeleteMessageBatchResultEntry { id: Some(id) });
                }
                Err(e) => return sqs_error_response(&e),
            }
        }

        wire::serialize_delete_message_batch_response(&wire::DeleteMessageBatchResult {
            successful: Some(successful),
            failed: Some(vec![]),
        })
    }

    async fn handle_change_message_visibility(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_change_message_visibility_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        if input.receipt_handle.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'ReceiptHandle'");
        }
        let visibility_timeout = input.visibility_timeout.max(0) as u32;
        let queue_name = extract_queue_name_from_url(&input.queue_url);
        match self
            .backend
            .change_message_visibility(
                account_id,
                region,
                queue_name,
                input.receipt_handle,
                visibility_timeout,
            )
            .await
        {
            Ok(()) => wire::serialize_change_message_visibility_response(),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_change_message_visibility_batch(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_change_message_visibility_batch_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        let entries = input.entries;
        let queue_name = extract_queue_name_from_url(&input.queue_url);
        let batch_entries: Vec<(String, String, u32)> = entries
            .into_iter()
            .filter_map(|entry| {
                if entry.id.is_empty() || entry.receipt_handle.is_empty() {
                    return None;
                }
                let visibility_timeout = entry.visibility_timeout.unwrap_or(0).max(0) as u32;
                Some((entry.id, entry.receipt_handle, visibility_timeout))
            })
            .collect();

        match self
            .backend
            .change_message_visibility_batch(account_id, region, queue_name, batch_entries)
            .await
        {
            Ok(successful_ids) => {
                let successful: Vec<wire::ChangeMessageVisibilityBatchResultEntry> = successful_ids
                    .into_iter()
                    .map(|id| wire::ChangeMessageVisibilityBatchResultEntry { id: Some(id) })
                    .collect();
                wire::serialize_change_message_visibility_batch_response(
                    &wire::ChangeMessageVisibilityBatchResult {
                        successful: Some(successful),
                        failed: Some(vec![]),
                    },
                )
            }
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_list_queue_tags(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_list_queue_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        let queue_name = extract_queue_name_from_url(&input.queue_url);
        match self
            .backend
            .list_queue_tags(account_id, region, queue_name)
            .await
        {
            Ok(tags) => wire::serialize_list_queue_tags_response(&wire::ListQueueTagsResult {
                tags: if tags.is_empty() { None } else { Some(tags) },
            }),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_tag_queue(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_queue_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        let tags = input.tags;
        let queue_name = extract_queue_name_from_url(&input.queue_url);
        match self
            .backend
            .tag_queue(account_id, region, queue_name, tags)
            .await
        {
            Ok(()) => wire::serialize_tag_queue_response(),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_untag_queue(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_queue_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        let tag_keys = input.tag_keys;
        let queue_name = extract_queue_name_from_url(&input.queue_url);
        match self
            .backend
            .untag_queue(account_id, region, queue_name, tag_keys)
            .await
        {
            Ok(()) => wire::serialize_untag_queue_response(),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_add_permission(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_add_permission_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        if input.label.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'Label'");
        }
        let aws_account_ids = input.a_w_s_account_ids;
        let actions = input.actions;
        let queue_name = extract_queue_name_from_url(&input.queue_url);
        match self
            .backend
            .add_permission(
                account_id,
                region,
                queue_name,
                input.label,
                aws_account_ids,
                actions,
            )
            .await
        {
            Ok(()) => wire::serialize_add_permission_response(),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_remove_permission(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_permission_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        if input.label.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'Label'");
        }
        let queue_name = extract_queue_name_from_url(&input.queue_url);
        match self
            .backend
            .remove_permission(account_id, region, queue_name, input.label)
            .await
        {
            Ok(()) => wire::serialize_remove_permission_response(),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_list_dead_letter_source_queues(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_list_dead_letter_source_queues_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.queue_url.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'QueueUrl'");
        }
        let queue_name = extract_queue_name_from_url(&input.queue_url);
        match self
            .backend
            .list_dead_letter_source_queues(account_id, region, queue_name)
            .await
        {
            Ok(urls) => wire::serialize_list_dead_letter_source_queues_response(
                &wire::ListDeadLetterSourceQueuesResult {
                    next_token: None,
                    queue_urls: Some(urls),
                },
            ),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_start_message_move_task(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_start_message_move_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.source_arn.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'SourceArn'");
        }
        let destination_arn = input.destination_arn;
        let max_number_of_messages_per_second = input.max_number_of_messages_per_second;

        match self
            .backend
            .start_message_move_task(
                account_id,
                region,
                input.source_arn,
                destination_arn,
                max_number_of_messages_per_second,
            )
            .await
        {
            Ok(task_handle) => wire::serialize_start_message_move_task_response(
                &wire::StartMessageMoveTaskResult {
                    task_handle: Some(task_handle),
                },
            ),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_cancel_message_move_task(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_message_move_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.task_handle.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'TaskHandle'");
        }
        match self
            .backend
            .cancel_message_move_task(account_id, region, input.task_handle)
            .await
        {
            Ok(approx_moved) => wire::serialize_cancel_message_move_task_response(
                &wire::CancelMessageMoveTaskResult {
                    approximate_number_of_messages_moved: Some(approx_moved),
                },
            ),
            Err(e) => sqs_error_response(&e),
        }
    }

    async fn handle_list_message_move_tasks(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_list_message_move_tasks_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInput", &e),
        };
        if input.source_arn.is_empty() {
            return json_error_response(400, "MissingParameter", "Missing 'SourceArn'");
        }
        match self
            .backend
            .list_message_move_tasks(account_id, region, input.source_arn)
            .await
        {
            Ok(tasks) => {
                let results: Vec<wire::ListMessageMoveTasksResultEntry> = tasks
                    .iter()
                    .map(|t| wire::ListMessageMoveTasksResultEntry {
                        task_handle: Some(t.task_handle.clone()),
                        source_arn: Some(t.source_arn.clone()),
                        destination_arn: t.destination_arn.clone(),
                        status: Some(t.status.clone()),
                        max_number_of_messages_per_second: t.max_number_of_messages_per_second,
                        started_timestamp: Some(t.started_timestamp),
                        approximate_number_of_messages_moved: Some(
                            t.approximate_number_of_messages_moved,
                        ),
                        approximate_number_of_messages_to_move: t
                            .approximate_number_of_messages_to_move,
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_message_move_tasks_response(
                    &wire::ListMessageMoveTasksResult {
                        results: if results.is_empty() {
                            None
                        } else {
                            Some(results)
                        },
                    },
                )
            }
            Err(e) => sqs_error_response(&e),
        }
    }
}

// --- Utility functions ---

fn extract_queue_name_from_url(url: &str) -> String {
    // Queue URL format: https://sqs.region.amazonaws.com/account_id/queue_name
    url.rsplit('/').next().unwrap_or("").to_string()
}

fn wire_message_attributes_to_state(
    attrs: Option<HashMap<String, wire::MessageAttributeValue>>,
) -> HashMap<String, MessageAttribute> {
    let mut out = HashMap::new();
    let Some(attrs) = attrs else { return out };
    for (name, value) in attrs {
        if value.data_type.is_empty() {
            continue;
        }
        out.insert(
            name,
            MessageAttribute {
                data_type: value.data_type,
                string_value: value.string_value,
                binary_value: None,
            },
        );
    }
    out
}

fn sqs_error_response(err: &SqsError) -> MockResponse {
    use crate::state::SqsError::*;
    let (status, error_type) = match err {
        InvalidQueueName => (400, "InvalidParameterValue"),
        InvalidFifoQueueName => (400, "InvalidParameterValue"),
        QueueAlreadyExists => (400, "QueueAlreadyExists"),
        NonExistentQueue => (400, "AWS.SimpleQueueService.NonExistentQueue"),
        InvalidReceiptHandle { .. } => (400, "ReceiptHandleIsInvalid"),
        MissingActions => (400, "MissingParameter"),
        EmptyPrincipalId => (400, "InvalidParameterValue"),
        TooManyActions { .. } => (403, "OverLimit"),
        DisallowedAction { .. } => (400, "InvalidParameterValue"),
        DuplicatePermissionLabel { .. } => (400, "InvalidParameterValue"),
        PermissionLabelNotFound { .. } => (400, "InvalidParameterValue"),
        SourceArnNotFound { .. } => (400, "ResourceNotFoundException"),
        MessageMoveTaskAlreadyRunning => (400, "ResourceNotFoundException"),
        InvalidTaskHandle { .. } => (400, "ResourceNotFoundException"),
        TaskNotCancellable => (400, "ResourceNotFoundException"),
        InternalError(_) => (500, "InternalError"),
    };
    json_error_response(status, error_type, &err.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
