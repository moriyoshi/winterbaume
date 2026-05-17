//! Redis-backed [`SqsBackend`] implementation.
//!
//! Provided by the `winterbaume-sqs-redis` crate.
//!
//! # Key schema
//!
//! All keys are namespaced by `{account_id}` and `{region}`:
//!
//! | Key | Type | Contents |
//! |-----|------|----------|
//! | `sqs:{acct}:{rgn}:queues` | Set | Queue names |
//! | `sqs:{acct}:{rgn}:queue:{name}` | String | JSON `StoredQueue` |
//! | `sqs:{acct}:{rgn}:queue:{name}:msgs` | Hash | `msg_id` → JSON `StoredMsg` |
//! | `sqs:{acct}:{rgn}:queue:{name}:order` | List | `msg_id` in insertion order |
//! | `sqs:{acct}:{rgn}:queue:{name}:deleted` | Set | Deleted receipt handles |
//! | `sqs:{acct}:{rgn}:move_tasks` | List | Task handles |
//! | `sqs:{acct}:{rgn}:move_task:{handle}` | String | JSON `StoredTask` |
//!
//! `receive_messages` is performed atomically via an embedded Lua script so
//! that concurrent callers cannot pick up the same message.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use chrono::{TimeZone, Utc};
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use winterbaume_sqs::SqsStateView;
use winterbaume_sqs::StateViewError;
use winterbaume_sqs::backend::{SendMessageOutput, SqsBackend};
use winterbaume_sqs::state::SqsError;
use winterbaume_sqs::types::{Message, MessageAttribute, MessageMoveTask, Queue};
use winterbaume_sqs::views::QueueStateView;

// ---------------------------------------------------------------------------
// Error helpers
// ---------------------------------------------------------------------------

fn redis_err(e: redis::RedisError) -> SqsError {
    SqsError::InternalError(e.to_string())
}

fn json_err(e: serde_json::Error) -> SqsError {
    SqsError::InternalError(e.to_string())
}

fn queue_not_found() -> SqsError {
    SqsError::NonExistentQueue
}

// ---------------------------------------------------------------------------
// Redis key helpers
// ---------------------------------------------------------------------------

fn k_queues(acct: &str, rgn: &str) -> String {
    format!("sqs:{acct}:{rgn}:queues")
}
fn k_queue(acct: &str, rgn: &str, name: &str) -> String {
    format!("sqs:{acct}:{rgn}:queue:{name}")
}
fn k_msgs(acct: &str, rgn: &str, name: &str) -> String {
    format!("sqs:{acct}:{rgn}:queue:{name}:msgs")
}
fn k_order(acct: &str, rgn: &str, name: &str) -> String {
    format!("sqs:{acct}:{rgn}:queue:{name}:order")
}
fn k_deleted(acct: &str, rgn: &str, name: &str) -> String {
    format!("sqs:{acct}:{rgn}:queue:{name}:deleted")
}
fn k_tasks(acct: &str, rgn: &str) -> String {
    format!("sqs:{acct}:{rgn}:move_tasks")
}
fn k_task(acct: &str, rgn: &str, handle: &str) -> String {
    format!("sqs:{acct}:{rgn}:move_task:{handle}")
}

// ---------------------------------------------------------------------------
// Stored types (serde-serializable counterparts to the domain types)
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone)]
struct StoredQueue {
    name: String,
    url: String,
    arn: String,
    region: String,
    account_id: String,
    /// Unix timestamp (seconds).
    created_ts: i64,
    /// Unix timestamp (seconds).
    modified_ts: i64,
    visibility_timeout: u32,
    delay_seconds: u32,
    maximum_message_size: u32,
    message_retention_period: u32,
    receive_wait_time_seconds: u32,
    fifo_queue: bool,
    content_based_deduplication: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    redrive_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    dead_letter_target_arn: Option<String>,
    #[serde(default)]
    tags: HashMap<String, String>,
    /// label → (aws_account_ids, actions)
    #[serde(default)]
    permissions: HashMap<String, StoredPermEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
struct StoredPermEntry {
    aws_account_ids: Vec<String>,
    actions: Vec<String>,
}

impl StoredQueue {
    fn to_queue(&self) -> Queue {
        Queue {
            name: self.name.clone(),
            url: self.url.clone(),
            arn: self.arn.clone(),
            region: self.region.clone(),
            account_id: self.account_id.clone(),
            created_timestamp: Utc
                .timestamp_opt(self.created_ts, 0)
                .single()
                .unwrap_or_else(Utc::now),
            last_modified_timestamp: Utc
                .timestamp_opt(self.modified_ts, 0)
                .single()
                .unwrap_or_else(Utc::now),
            visibility_timeout: self.visibility_timeout,
            delay_seconds: self.delay_seconds,
            maximum_message_size: self.maximum_message_size,
            message_retention_period: self.message_retention_period,
            receive_wait_time_seconds: self.receive_wait_time_seconds,
            fifo_queue: self.fifo_queue,
            content_based_deduplication: self.content_based_deduplication,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct StoredMsg {
    message_id: String,
    body: String,
    body_md5: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    receipt_handle: Option<String>,
    #[serde(default)]
    all_receipt_handles: Vec<String>,
    sender_id: String,
    sent_timestamp: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    approximate_first_receive_timestamp: Option<i64>,
    approximate_receive_count: u32,
    /// Epoch-millisecond timestamp at which this message becomes visible.
    visible_at: i64,
    #[serde(default)]
    message_attributes: HashMap<String, StoredMsgAttr>,
}

#[derive(Serialize, Deserialize, Clone)]
struct StoredMsgAttr {
    data_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    string_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    binary_value: Option<Vec<u8>>,
}

impl StoredMsg {
    fn to_message(&self) -> Message {
        Message {
            message_id: self.message_id.clone(),
            body: self.body.clone(),
            body_md5: self.body_md5.clone(),
            receipt_handle: self.receipt_handle.clone(),
            all_receipt_handles: self.all_receipt_handles.clone(),
            sender_id: self.sender_id.clone(),
            sent_timestamp: self.sent_timestamp,
            approximate_first_receive_timestamp: self.approximate_first_receive_timestamp,
            approximate_receive_count: self.approximate_receive_count,
            visible_at: self.visible_at,
            message_attributes: self
                .message_attributes
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        MessageAttribute {
                            data_type: v.data_type.clone(),
                            string_value: v.string_value.clone(),
                            binary_value: v.binary_value.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct StoredTask {
    task_handle: String,
    source_arn: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    destination_arn: Option<String>,
    status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max_number_of_messages_per_second: Option<i32>,
    started_timestamp: i64,
    approximate_number_of_messages_moved: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    approximate_number_of_messages_to_move: Option<i64>,
}

impl StoredTask {
    fn to_task(&self) -> MessageMoveTask {
        MessageMoveTask {
            task_handle: self.task_handle.clone(),
            source_arn: self.source_arn.clone(),
            destination_arn: self.destination_arn.clone(),
            status: self.status.clone(),
            max_number_of_messages_per_second: self.max_number_of_messages_per_second,
            started_timestamp: self.started_timestamp,
            approximate_number_of_messages_moved: self.approximate_number_of_messages_moved,
            approximate_number_of_messages_to_move: self.approximate_number_of_messages_to_move,
        }
    }
}

// ---------------------------------------------------------------------------
// Lua script for atomic message receive
// ---------------------------------------------------------------------------
//
// KEYS[1] = msgs hash key
// KEYS[2] = order list key
// ARGV[1] = now_ms
// ARGV[2] = visibility_timeout_ms
// ARGV[3] = max_count
// ARGV[4..] = pre-generated receipt handles (one per potential visible message)
//
// Returns an array of JSON-encoded messages that were received.

// KEYS[1] = msgs hash key
// KEYS[2] = order list key
// ARGV[1] = now_ms
// ARGV[2] = visibility_timeout_ms
// ARGV[3] = max_count
// ARGV[4] = max_receive_count (0 means unlimited)
// ARGV[5..] = pre-generated receipt handles (one per potential visible message)
//
// Returns an array of strings, each prefixed with "R:" for received messages
// or "D:" for messages to be redriven to a DLQ. Redriven messages are also
// removed from the main queue's hash and order list.
// `recv_count` tracks only `R:` entries; `D:` entries are bookkeeping for the
// Rust caller's DLQ-push step and must not consume the caller's `max_n` receive
// budget. Receipt-handle indexing also uses `recv_count` so dead-lettered
// iterations don't burn a handle slot in `ARGV`.
const RECEIVE_SCRIPT: &str = r#"
local msgs_key  = KEYS[1]
local order_key = KEYS[2]
local now_ms    = tonumber(ARGV[1])
local vis_ms    = tonumber(ARGV[2])
local max_n     = tonumber(ARGV[3])
local max_recv  = tonumber(ARGV[4])
local results   = {}
local recv_count = 0

local ids = redis.call('LRANGE', order_key, 0, -1)
for _, id in ipairs(ids) do
    if recv_count >= max_n then break end
    local raw = redis.call('HGET', msgs_key, id)
    if raw and raw ~= false then
        local msg = cjson.decode(raw)
        local vis_at = tonumber(tostring(msg['visible_at']))
        if vis_at and now_ms >= vis_at then
            local cnt = msg['approximate_receive_count']
            if type(cnt) ~= 'number' then cnt = 0 end
            local new_cnt = cnt + 1
            if max_recv > 0 and new_cnt > max_recv then
                -- Hand off to Rust caller to push into the DLQ. Does not
                -- count against `recv_count` because the caller expected
                -- visible messages, not redrive bookkeeping.
                redis.call('HDEL', msgs_key, id)
                redis.call('LREM', order_key, 0, id)
                table.insert(results, 'D:' .. cjson.encode(msg))
            else
                local handle = ARGV[4 + recv_count + 1]
                if handle and handle ~= false then
                    msg['receipt_handle'] = handle
                    local rhs = msg['all_receipt_handles']
                    if type(rhs) ~= 'table' then rhs = {} end
                    table.insert(rhs, handle)
                    msg['all_receipt_handles'] = rhs
                    msg['approximate_receive_count'] = new_cnt
                    local afrt = msg['approximate_first_receive_timestamp']
                    if afrt == nil or afrt == cjson.null then
                        msg['approximate_first_receive_timestamp'] = now_ms
                    end
                    msg['visible_at'] = now_ms + vis_ms
                    redis.call('HSET', msgs_key, id, cjson.encode(msg))
                    table.insert(results, 'R:' .. cjson.encode(msg))
                    recv_count = recv_count + 1
                end
            end
        end
    end
end
return results
"#;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_attr(attrs: &HashMap<String, String>, key: &str, default: u32) -> u32 {
    attrs
        .get(key)
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn extract_dlq_arn(attrs: &HashMap<String, String>) -> Option<String> {
    attrs
        .get("RedrivePolicy")
        .and_then(|p| serde_json::from_str::<serde_json::Value>(p).ok())
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

fn gen_receipt_handle() -> String {
    (0..4)
        .map(|_| Uuid::new_v4().to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn is_valid_queue_name(name: &str) -> bool {
    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
}

/// Rebuild the policy JSON from the permissions map and store it on the queue.
fn rebuild_policy(sq: &mut StoredQueue) {
    if sq.permissions.is_empty() {
        sq.policy = None;
        return;
    }
    let statements: Vec<serde_json::Value> = sq
        .permissions
        .iter()
        .map(|(label, entry)| {
            let principal = if entry.aws_account_ids.len() == 1 {
                serde_json::json!({"AWS": format!("arn:aws:iam::{}:root", entry.aws_account_ids[0])})
            } else {
                let arns: Vec<String> = entry
                    .aws_account_ids
                    .iter()
                    .map(|id| format!("arn:aws:iam::{}:root", id))
                    .collect();
                serde_json::json!({"AWS": arns})
            };
            let action = if entry.actions.len() == 1 {
                serde_json::json!(format!("SQS:{}", entry.actions[0]))
            } else {
                let acts: Vec<String> = entry.actions.iter().map(|a| format!("SQS:{a}")).collect();
                serde_json::json!(acts)
            };
            serde_json::json!({
                "Sid": label, "Effect": "Allow",
                "Principal": principal, "Action": action,
                "Resource": sq.arn,
            })
        })
        .collect();
    let policy = serde_json::json!({
        "Version": "2012-10-17",
        "Id": format!("{}/SQSDefaultPolicy", sq.arn),
        "Statement": statements,
    });
    sq.policy = Some(serde_json::to_string(&policy).unwrap());
}

/// Load a `StoredQueue` from Redis, returning `None` if it does not exist.
async fn load_queue(
    cm: &mut ConnectionManager,
    acct: &str,
    rgn: &str,
    name: &str,
) -> Result<Option<StoredQueue>, SqsError> {
    let raw: Option<String> = cm.get(k_queue(acct, rgn, name)).await.map_err(redis_err)?;
    match raw {
        None => Ok(None),
        Some(json) => serde_json::from_str(&json).map(Some).map_err(json_err),
    }
}

/// Persist a `StoredQueue` to Redis and update the queues index.
async fn save_queue(
    cm: &mut ConnectionManager,
    acct: &str,
    rgn: &str,
    sq: &StoredQueue,
) -> Result<(), SqsError> {
    let json = serde_json::to_string(sq).map_err(json_err)?;
    let _: () = cm
        .set(k_queue(acct, rgn, &sq.name), json)
        .await
        .map_err(redis_err)?;
    let _: () = cm
        .sadd(k_queues(acct, rgn), &sq.name)
        .await
        .map_err(redis_err)?;
    Ok(())
}

/// Compute SQS queue attributes from a stored queue and its messages.
fn build_queue_attributes(sq: &StoredQueue, msgs: &[StoredMsg]) -> HashMap<String, String> {
    let now_ms = Utc::now().timestamp_millis();
    let mut attrs = HashMap::new();
    attrs.insert("QueueArn".to_string(), sq.arn.clone());
    attrs.insert(
        "ApproximateNumberOfMessages".to_string(),
        msgs.iter()
            .filter(|m| now_ms >= m.visible_at && m.receipt_handle.is_none())
            .count()
            .to_string(),
    );
    // NotVisible = in-flight (received at least once but not yet expired)
    attrs.insert(
        "ApproximateNumberOfMessagesNotVisible".to_string(),
        msgs.iter()
            .filter(|m| now_ms < m.visible_at && m.approximate_receive_count > 0)
            .count()
            .to_string(),
    );
    // Delayed = never received yet, still hidden by send-time delay
    attrs.insert(
        "ApproximateNumberOfMessagesDelayed".to_string(),
        msgs.iter()
            .filter(|m| now_ms < m.visible_at && m.approximate_receive_count == 0)
            .count()
            .to_string(),
    );
    attrs.insert(
        "VisibilityTimeout".to_string(),
        sq.visibility_timeout.to_string(),
    );
    attrs.insert("DelaySeconds".to_string(), sq.delay_seconds.to_string());
    attrs.insert(
        "MaximumMessageSize".to_string(),
        sq.maximum_message_size.to_string(),
    );
    attrs.insert(
        "MessageRetentionPeriod".to_string(),
        sq.message_retention_period.to_string(),
    );
    attrs.insert(
        "ReceiveMessageWaitTimeSeconds".to_string(),
        sq.receive_wait_time_seconds.to_string(),
    );
    attrs.insert("CreatedTimestamp".to_string(), sq.created_ts.to_string());
    attrs.insert(
        "LastModifiedTimestamp".to_string(),
        sq.modified_ts.to_string(),
    );
    if sq.fifo_queue {
        attrs.insert("FifoQueue".to_string(), "true".to_string());
        attrs.insert(
            "ContentBasedDeduplication".to_string(),
            sq.content_based_deduplication.to_string(),
        );
        attrs.insert("DeduplicationScope".to_string(), "queue".to_string());
        attrs.insert("FifoThroughputLimit".to_string(), "perQueue".to_string());
    }
    if let Some(ref rp) = sq.redrive_policy {
        attrs.insert("RedrivePolicy".to_string(), rp.clone());
    }
    if let Some(ref p) = sq.policy {
        attrs.insert("Policy".to_string(), p.clone());
    }
    attrs.insert("SqsManagedSseEnabled".to_string(), "true".to_string());
    attrs
}

/// Fetch all messages for a queue from the msgs hash (unordered).
async fn load_all_msgs(
    cm: &mut ConnectionManager,
    acct: &str,
    rgn: &str,
    name: &str,
) -> Result<Vec<StoredMsg>, SqsError> {
    let vals: Vec<String> = cm.hvals(k_msgs(acct, rgn, name)).await.map_err(redis_err)?;
    vals.iter()
        .map(|json| serde_json::from_str::<StoredMsg>(json).map_err(json_err))
        .collect()
}

// ---------------------------------------------------------------------------
// RedisSqsBackend
// ---------------------------------------------------------------------------

/// [`SqsBackend`] implementation backed by Redis.
///
/// Create with [`RedisSqsBackend::new`] (async) and pass to
/// [`crate::handlers::SqsService::with_backend`].
pub struct RedisSqsBackend {
    manager: ConnectionManager,
}

impl RedisSqsBackend {
    pub fn new(manager: ConnectionManager) -> Self {
        Self { manager }
    }

    /// Connect to Redis at `url` (e.g. `"redis://127.0.0.1/"`).
    pub async fn from_connection_info(
        info: impl redis::IntoConnectionInfo,
    ) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(info)?;
        let manager = ConnectionManager::new(client).await?;
        Ok(Self { manager })
    }
}

impl SqsBackend for RedisSqsBackend {
    // ---- Queue management ----

    fn create_queue(
        &self,
        account_id: String,
        region: String,
        name: String,
        attributes: HashMap<String, String>,
        tags: Option<HashMap<String, String>>,
    ) -> Pin<Box<dyn Future<Output = Result<Queue, SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            // Validate queue name.
            if name.is_empty() || name.len() > 80 || !is_valid_queue_name(&name) {
                return Err(SqsError::InvalidQueueName);
            }
            let fifo_attr = attributes.get("FifoQueue").map(|v| v.as_str());
            if fifo_attr == Some("true") && !name.ends_with(".fifo") {
                return Err(SqsError::InvalidFifoQueueName);
            }

            // Check for existing queue.
            if let Some(existing) = load_queue(&mut cm, &account_id, &region, &name).await? {
                // Verify attribute compatibility.
                let vis = parse_attr(
                    &attributes,
                    "VisibilityTimeout",
                    existing.visibility_timeout,
                );
                let delay = parse_attr(&attributes, "DelaySeconds", existing.delay_seconds);
                let maxmsg = parse_attr(
                    &attributes,
                    "MaximumMessageSize",
                    existing.maximum_message_size,
                );
                let ret = parse_attr(
                    &attributes,
                    "MessageRetentionPeriod",
                    existing.message_retention_period,
                );
                let wait = parse_attr(
                    &attributes,
                    "ReceiveMessageWaitTimeSeconds",
                    existing.receive_wait_time_seconds,
                );
                let new_rp = attributes.get("RedrivePolicy");
                let rp_ok = match (new_rp, existing.redrive_policy.as_ref()) {
                    (None, _) => true,
                    (Some(a), Some(b)) => a == b,
                    (Some(_), None) => false,
                };
                if vis != existing.visibility_timeout
                    || delay != existing.delay_seconds
                    || maxmsg != existing.maximum_message_size
                    || ret != existing.message_retention_period
                    || wait != existing.receive_wait_time_seconds
                    || !rp_ok
                {
                    return Err(SqsError::QueueAlreadyExists);
                }
                return Ok(existing.to_queue());
            }

            let now = Utc::now();
            let fifo = name.ends_with(".fifo");
            let url = format!("https://sqs.{region}.amazonaws.com/{account_id}/{name}");
            let arn = format!("arn:aws:sqs:{region}:{account_id}:{name}");
            let sq = StoredQueue {
                name: name.clone(),
                url,
                arn,
                region,
                account_id,
                created_ts: now.timestamp(),
                modified_ts: now.timestamp(),
                visibility_timeout: parse_attr(&attributes, "VisibilityTimeout", 30),
                delay_seconds: parse_attr(&attributes, "DelaySeconds", 0),
                maximum_message_size: parse_attr(&attributes, "MaximumMessageSize", 262144),
                message_retention_period: parse_attr(&attributes, "MessageRetentionPeriod", 345600),
                receive_wait_time_seconds: parse_attr(
                    &attributes,
                    "ReceiveMessageWaitTimeSeconds",
                    0,
                ),
                fifo_queue: fifo,
                content_based_deduplication: attributes
                    .get("ContentBasedDeduplication")
                    .map(|v| v == "true")
                    .unwrap_or(false),
                redrive_policy: attributes.get("RedrivePolicy").cloned(),
                policy: attributes.get("Policy").cloned(),
                dead_letter_target_arn: extract_dlq_arn(&attributes),
                tags: tags.unwrap_or_default(),
                permissions: HashMap::new(),
            };
            let queue = sq.to_queue();
            save_queue(&mut cm, &sq.account_id, &sq.region, &sq).await?;
            Ok(queue)
        })
    }

    fn delete_queue(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            if load_queue(&mut cm, &account_id, &region, &queue_name)
                .await?
                .is_none()
            {
                return Err(queue_not_found());
            }
            let dedup_key = format!("sqs:{account_id}:{region}:queue:{queue_name}:dedup");
            let _: () = cm
                .del(vec![
                    k_queue(&account_id, &region, &queue_name),
                    k_msgs(&account_id, &region, &queue_name),
                    k_order(&account_id, &region, &queue_name),
                    k_deleted(&account_id, &region, &queue_name),
                    dedup_key,
                ])
                .await
                .map_err(redis_err)?;
            let _: () = cm
                .srem(k_queues(&account_id, &region), &queue_name)
                .await
                .map_err(redis_err)?;
            Ok(())
        })
    }

    fn get_queue_url(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            match load_queue(&mut cm, &account_id, &region, &queue_name).await? {
                Some(sq) => Ok(sq.url),
                None => Err(queue_not_found()),
            }
        })
    }

    fn get_queue_attributes(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, String>, SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            let sq = load_queue(&mut cm, &account_id, &region, &queue_name)
                .await?
                .ok_or_else(queue_not_found)?;
            let msgs = load_all_msgs(&mut cm, &account_id, &region, &queue_name).await?;
            Ok(build_queue_attributes(&sq, &msgs))
        })
    }

    fn set_queue_attributes(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        attributes: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            let mut sq = load_queue(&mut cm, &account_id, &region, &queue_name)
                .await?
                .ok_or_else(queue_not_found)?;

            if let Some(v) = attributes
                .get("VisibilityTimeout")
                .and_then(|v| v.parse().ok())
            {
                sq.visibility_timeout = v;
            }
            if let Some(v) = attributes.get("DelaySeconds").and_then(|v| v.parse().ok()) {
                sq.delay_seconds = v;
            }
            if let Some(v) = attributes
                .get("MaximumMessageSize")
                .and_then(|v| v.parse().ok())
            {
                sq.maximum_message_size = v;
            }
            if let Some(v) = attributes
                .get("MessageRetentionPeriod")
                .and_then(|v| v.parse().ok())
            {
                sq.message_retention_period = v;
            }
            if let Some(v) = attributes
                .get("ReceiveMessageWaitTimeSeconds")
                .and_then(|v| v.parse().ok())
            {
                sq.receive_wait_time_seconds = v;
            }
            if let Some(redrive) = attributes.get("RedrivePolicy") {
                if redrive.is_empty() || redrive == "{}" {
                    sq.dead_letter_target_arn = None;
                    sq.redrive_policy = None;
                } else {
                    sq.dead_letter_target_arn = serde_json::from_str::<serde_json::Value>(redrive)
                        .ok()
                        .and_then(|v| {
                            v.get("deadLetterTargetArn")
                                .and_then(|a| a.as_str())
                                .map(|s| s.to_string())
                        });
                    sq.redrive_policy = Some(redrive.clone());
                }
            }
            if let Some(policy) = attributes.get("Policy") {
                sq.policy = if policy.is_empty() {
                    None
                } else {
                    Some(policy.clone())
                };
            }
            if let Some(v) = attributes.get("ContentBasedDeduplication") {
                sq.content_based_deduplication = v == "true";
            }
            sq.modified_ts = Utc::now().timestamp();

            save_queue(&mut cm, &account_id, &region, &sq).await
        })
    }

    fn list_queues(
        &self,
        account_id: String,
        region: String,
        prefix: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Queue>, SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            let names: Vec<String> = cm
                .smembers(k_queues(&account_id, &region))
                .await
                .map_err(redis_err)?;
            let mut queues = Vec::new();
            for name in names {
                if let Some(ref p) = prefix {
                    if !name.starts_with(p.as_str()) {
                        continue;
                    }
                }
                if let Some(sq) = load_queue(&mut cm, &account_id, &region, &name).await? {
                    queues.push(sq.to_queue());
                }
            }
            Ok(queues)
        })
    }

    // ---- Message operations ----

    fn send_message(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        body: String,
        delay_seconds: Option<u32>,
        message_attributes: HashMap<String, MessageAttribute>,
        message_group_id: Option<String>,
        message_deduplication_id: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<SendMessageOutput, SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            let _ = message_group_id; // FIFO ordering not enforced at this layer
            let sq = load_queue(&mut cm, &account_id, &region, &queue_name)
                .await?
                .ok_or_else(queue_not_found)?;

            let delay = delay_seconds.unwrap_or(sq.delay_seconds);
            let now_ms = Utc::now().timestamp_millis();
            let visible_at = now_ms + (delay as i64 * 1000);
            let body_md5 = {
                use md5::{Digest, Md5};
                let mut h = Md5::new();
                h.update(body.as_bytes());
                format!("{:x}", h.finalize())
            };

            // FIFO deduplication.  Mirrors the in-memory backend's semantics:
            // explicit MessageDeduplicationId wins over content-based hashing,
            // both are scoped to a 5-minute window stored in a Redis hash
            // keyed `sqs:{acct}:{rgn}:queue:{name}:dedup` (dedup_id ->
            // "{message_id}|{expires_ms}").
            let dedup_id: Option<String> = if sq.fifo_queue {
                if let Some(ref explicit) = message_deduplication_id {
                    Some(explicit.clone())
                } else if sq.content_based_deduplication {
                    Some(body_md5.clone())
                } else {
                    None
                }
            } else {
                None
            };
            let dedup_key = format!("sqs:{account_id}:{region}:queue:{queue_name}:dedup");
            const DEDUP_WINDOW_MS: i64 = 5 * 60 * 1000;

            if let Some(ref did) = dedup_id {
                // Lazy eviction of expired entries.
                let entries: Vec<(String, String)> =
                    cm.hgetall(&dedup_key).await.map_err(redis_err)?;
                for (k, v) in &entries {
                    if let Some((_, exp)) = v.split_once('|') {
                        if let Ok(exp_ms) = exp.parse::<i64>() {
                            if exp_ms <= now_ms {
                                let _: () = cm.hdel(&dedup_key, k).await.map_err(redis_err)?;
                            }
                        }
                    }
                }
                if let Some(existing) = entries
                    .iter()
                    .find(|(k, _)| k == did)
                    .and_then(|(_, v)| v.split_once('|').map(|(id, _)| id.to_string()))
                {
                    return Ok(SendMessageOutput {
                        message_id: existing,
                        body_md5,
                    });
                }
            }

            let message_id = Uuid::new_v4().to_string();

            let stored_attrs: HashMap<String, StoredMsgAttr> = message_attributes
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        StoredMsgAttr {
                            data_type: v.data_type,
                            string_value: v.string_value,
                            binary_value: v.binary_value,
                        },
                    )
                })
                .collect();

            let msg = StoredMsg {
                message_id: message_id.clone(),
                body,
                body_md5: body_md5.clone(),
                receipt_handle: None,
                all_receipt_handles: Vec::new(),
                sender_id: "AIDAIT2UOQQY3AUEKVGXU".to_string(),
                sent_timestamp: now_ms,
                approximate_first_receive_timestamp: None,
                approximate_receive_count: 0,
                visible_at,
                message_attributes: stored_attrs,
            };

            let json = serde_json::to_string(&msg).map_err(json_err)?;
            let _: () = cm
                .hset(
                    k_msgs(&account_id, &region, &queue_name),
                    &message_id,
                    &json,
                )
                .await
                .map_err(redis_err)?;
            let _: () = cm
                .rpush(k_order(&account_id, &region, &queue_name), &message_id)
                .await
                .map_err(redis_err)?;
            if let Some(did) = dedup_id {
                let entry = format!("{message_id}|{}", now_ms + DEDUP_WINDOW_MS);
                let _: () = cm.hset(&dedup_key, &did, &entry).await.map_err(redis_err)?;
            }

            Ok(SendMessageOutput {
                message_id,
                body_md5,
            })
        })
    }

    fn receive_messages(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        max: usize,
        visibility_timeout: Option<u32>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Message>, SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            let sq = load_queue(&mut cm, &account_id, &region, &queue_name)
                .await?
                .ok_or_else(queue_not_found)?;

            let vis_timeout = visibility_timeout.unwrap_or(sq.visibility_timeout);
            let now_ms = Utc::now().timestamp_millis();
            let vis_ms = vis_timeout as i64 * 1000;
            let max_recv = redrive_max_receive_count(sq.redrive_policy.as_deref()).unwrap_or(0);

            // Pre-generate receipt handles — the Lua script picks them up one-by-one.
            let handles: Vec<String> = (0..max).map(|_| gen_receipt_handle()).collect();

            let script = redis::Script::new(RECEIVE_SCRIPT);
            let mut inv = script.prepare_invoke();
            inv.key(k_msgs(&account_id, &region, &queue_name));
            inv.key(k_order(&account_id, &region, &queue_name));
            inv.arg(now_ms);
            inv.arg(vis_ms);
            inv.arg(max as i64);
            inv.arg(max_recv as i64);
            for h in &handles {
                inv.arg(h.as_str());
            }

            let raw_results: Vec<String> = inv.invoke_async(&mut cm).await.map_err(redis_err)?;

            // The Lua script emits each entry with a leading "R:" (received,
            // returned to caller) or "D:" (dead-lettered, must be moved to
            // the DLQ by this Rust caller).
            let mut received: Vec<Message> = Vec::new();
            let mut to_redrive: Vec<StoredMsg> = Vec::new();
            for raw in raw_results {
                let (tag, json) = raw.split_at(2);
                match tag {
                    "R:" => {
                        let m: StoredMsg = serde_json::from_str(json).map_err(json_err)?;
                        received.push(m.to_message());
                    }
                    "D:" => {
                        let m: StoredMsg = serde_json::from_str(json).map_err(json_err)?;
                        to_redrive.push(m);
                    }
                    _ => {
                        return Err(SqsError::InternalError(format!(
                            "unexpected Lua receive prefix: {raw}"
                        )));
                    }
                }
            }

            if !to_redrive.is_empty() {
                if let Some(target_arn) = sq.dead_letter_target_arn.as_deref() {
                    // Locate the DLQ by ARN.
                    let names: Vec<String> = cm
                        .smembers(k_queues(&account_id, &region))
                        .await
                        .map_err(redis_err)?;
                    let mut dlq_name: Option<String> = None;
                    for name in &names {
                        if let Some(other) = load_queue(&mut cm, &account_id, &region, name).await?
                        {
                            if other.arn == target_arn {
                                dlq_name = Some(name.clone());
                                break;
                            }
                        }
                    }
                    if let Some(dlq_name) = dlq_name {
                        for mut m in to_redrive {
                            m.receipt_handle = None;
                            m.all_receipt_handles.clear();
                            m.approximate_receive_count = 0;
                            m.approximate_first_receive_timestamp = None;
                            m.visible_at = now_ms;
                            let json = serde_json::to_string(&m).map_err(json_err)?;
                            let _: () = cm
                                .hset(
                                    k_msgs(&account_id, &region, &dlq_name),
                                    &m.message_id,
                                    &json,
                                )
                                .await
                                .map_err(redis_err)?;
                            let _: () = cm
                                .rpush(k_order(&account_id, &region, &dlq_name), &m.message_id)
                                .await
                                .map_err(redis_err)?;
                        }
                    }
                    // If the DLQ is missing, drop the messages silently
                    // (matches moto's behaviour for a stale RedrivePolicy).
                }
            }

            Ok(received)
        })
    }

    fn delete_message(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        receipt_handle: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            // Verify queue exists.
            if load_queue(&mut cm, &account_id, &region, &queue_name)
                .await?
                .is_none()
            {
                return Err(queue_not_found());
            }

            let deleted_key = k_deleted(&account_id, &region, &queue_name);

            // Idempotent: already-deleted handle succeeds silently.
            let already_deleted: bool = cm
                .sismember(&deleted_key, &receipt_handle)
                .await
                .map_err(redis_err)?;
            if already_deleted {
                return Ok(());
            }

            // Scan messages for a matching receipt handle.
            let msgs_key = k_msgs(&account_id, &region, &queue_name);
            let order_key = k_order(&account_id, &region, &queue_name);
            let msg_ids: Vec<String> = cm.lrange(&order_key, 0, -1).await.map_err(redis_err)?;

            for msg_id in msg_ids {
                let raw: Option<String> = cm.hget(&msgs_key, &msg_id).await.map_err(redis_err)?;
                if let Some(json) = raw {
                    let msg: StoredMsg = serde_json::from_str(&json).map_err(json_err)?;
                    let matches = msg.receipt_handle.as_deref() == Some(&receipt_handle)
                        || msg.all_receipt_handles.iter().any(|h| h == &receipt_handle);
                    if matches {
                        // Mark all handles as deleted for idempotent future calls.
                        for h in &msg.all_receipt_handles {
                            let _: () = cm.sadd(&deleted_key, h).await.map_err(redis_err)?;
                        }
                        let _: () = cm
                            .sadd(&deleted_key, &receipt_handle)
                            .await
                            .map_err(redis_err)?;
                        let _: () = cm.hdel(&msgs_key, &msg_id).await.map_err(redis_err)?;
                        let _: () = cm
                            .lrem(&order_key, 0isize, &msg_id)
                            .await
                            .map_err(redis_err)?;
                        return Ok(());
                    }
                }
            }

            Err(SqsError::InvalidReceiptHandle { receipt_handle })
        })
    }

    fn purge_queue(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            if load_queue(&mut cm, &account_id, &region, &queue_name)
                .await?
                .is_none()
            {
                return Err(queue_not_found());
            }
            let dedup_key = format!("sqs:{account_id}:{region}:queue:{queue_name}:dedup");
            let _: () = cm
                .del(vec![
                    k_msgs(&account_id, &region, &queue_name),
                    k_order(&account_id, &region, &queue_name),
                    k_deleted(&account_id, &region, &queue_name),
                    dedup_key,
                ])
                .await
                .map_err(redis_err)?;
            Ok(())
        })
    }

    fn change_message_visibility(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        receipt_handle: String,
        visibility_timeout: u32,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            if load_queue(&mut cm, &account_id, &region, &queue_name)
                .await?
                .is_none()
            {
                return Err(queue_not_found());
            }
            let msgs_key = k_msgs(&account_id, &region, &queue_name);
            let order_key = k_order(&account_id, &region, &queue_name);
            let now_ms = Utc::now().timestamp_millis();
            let msg_ids: Vec<String> = cm.lrange(&order_key, 0, -1).await.map_err(redis_err)?;

            for msg_id in msg_ids {
                let raw: Option<String> = cm.hget(&msgs_key, &msg_id).await.map_err(redis_err)?;
                if let Some(json) = raw {
                    let mut msg: StoredMsg = serde_json::from_str(&json).map_err(json_err)?;
                    if msg.receipt_handle.as_deref() == Some(&receipt_handle) {
                        msg.visible_at = now_ms + (visibility_timeout as i64 * 1000);
                        let updated = serde_json::to_string(&msg).map_err(json_err)?;
                        let _: () = cm
                            .hset(&msgs_key, &msg_id, &updated)
                            .await
                            .map_err(redis_err)?;
                        return Ok(());
                    }
                }
            }

            Err(SqsError::InvalidReceiptHandle { receipt_handle })
        })
    }

    fn change_message_visibility_batch(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        entries: Vec<(String, String, u32)>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            if load_queue(&mut cm, &account_id, &region, &queue_name)
                .await?
                .is_none()
            {
                return Err(queue_not_found());
            }
            let msgs_key = k_msgs(&account_id, &region, &queue_name);
            let order_key = k_order(&account_id, &region, &queue_name);
            let now_ms = Utc::now().timestamp_millis();
            let msg_ids: Vec<String> = cm.lrange(&order_key, 0, -1).await.map_err(redis_err)?;

            // Build a map of receipt_handle → msg_id for efficient lookup.
            let mut handle_to_id: HashMap<String, String> = HashMap::new();
            for msg_id in &msg_ids {
                let raw: Option<String> = cm.hget(&msgs_key, msg_id).await.map_err(redis_err)?;
                if let Some(json) = raw {
                    let msg: StoredMsg = serde_json::from_str(&json).map_err(json_err)?;
                    if let Some(ref h) = msg.receipt_handle {
                        handle_to_id.insert(h.clone(), msg_id.clone());
                    }
                }
            }

            let mut successful_ids = Vec::new();
            for (id, receipt_handle, visibility_timeout) in &entries {
                if let Some(msg_id) = handle_to_id.get(receipt_handle.as_str()) {
                    let raw: Option<String> =
                        cm.hget(&msgs_key, msg_id).await.map_err(redis_err)?;
                    if let Some(json) = raw {
                        let mut msg: StoredMsg = serde_json::from_str(&json).map_err(json_err)?;
                        msg.visible_at = now_ms + (*visibility_timeout as i64 * 1000);
                        let updated = serde_json::to_string(&msg).map_err(json_err)?;
                        let _: () = cm
                            .hset(&msgs_key, msg_id, &updated)
                            .await
                            .map_err(redis_err)?;
                        successful_ids.push(id.clone());
                    }
                } else {
                    return Err(SqsError::InvalidReceiptHandle {
                        receipt_handle: receipt_handle.clone(),
                    });
                }
            }
            Ok(successful_ids)
        })
    }

    // ---- Tags ----

    fn list_queue_tags(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, String>, SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            match load_queue(&mut cm, &account_id, &region, &queue_name).await? {
                Some(sq) => Ok(sq.tags),
                None => Err(queue_not_found()),
            }
        })
    }

    fn tag_queue(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        tags: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            let mut sq = load_queue(&mut cm, &account_id, &region, &queue_name)
                .await?
                .ok_or_else(queue_not_found)?;
            sq.tags.extend(tags);
            save_queue(&mut cm, &account_id, &region, &sq).await
        })
    }

    fn untag_queue(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        tag_keys: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            let mut sq = load_queue(&mut cm, &account_id, &region, &queue_name)
                .await?
                .ok_or_else(queue_not_found)?;
            for k in &tag_keys {
                sq.tags.remove(k);
            }
            save_queue(&mut cm, &account_id, &region, &sq).await
        })
    }

    // ---- Permissions ----

    fn add_permission(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        label: String,
        aws_account_ids: Vec<String>,
        actions: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            if actions.is_empty() {
                return Err(SqsError::MissingActions);
            }
            if aws_account_ids.is_empty() {
                return Err(SqsError::EmptyPrincipalId);
            }
            if actions.len() > 7 {
                return Err(SqsError::TooManyActions {
                    count: actions.len(),
                });
            }
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
            let mut sq = load_queue(&mut cm, &account_id, &region, &queue_name)
                .await?
                .ok_or_else(queue_not_found)?;
            if sq.permissions.contains_key(&label) {
                return Err(SqsError::DuplicatePermissionLabel {
                    label: label.clone(),
                });
            }
            sq.permissions.insert(
                label,
                StoredPermEntry {
                    aws_account_ids,
                    actions,
                },
            );
            rebuild_policy(&mut sq);
            save_queue(&mut cm, &account_id, &region, &sq).await
        })
    }

    fn remove_permission(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        label: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            let mut sq = load_queue(&mut cm, &account_id, &region, &queue_name)
                .await?
                .ok_or_else(queue_not_found)?;
            if !sq.permissions.contains_key(&label) {
                return Err(SqsError::PermissionLabelNotFound {
                    label: label.clone(),
                });
            }
            sq.permissions.remove(&label);
            rebuild_policy(&mut sq);
            save_queue(&mut cm, &account_id, &region, &sq).await
        })
    }

    // ---- Dead-letter queues ----

    fn list_dead_letter_source_queues(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            let target = load_queue(&mut cm, &account_id, &region, &queue_name)
                .await?
                .ok_or_else(queue_not_found)?;
            let target_arn = target.arn.clone();

            let names: Vec<String> = cm
                .smembers(k_queues(&account_id, &region))
                .await
                .map_err(redis_err)?;
            let mut source_urls = Vec::new();
            for name in names {
                if let Some(sq) = load_queue(&mut cm, &account_id, &region, &name).await? {
                    if sq.dead_letter_target_arn.as_deref() == Some(&target_arn) {
                        source_urls.push(sq.url);
                    }
                }
            }
            Ok(source_urls)
        })
    }

    // ---- Message move tasks ----

    fn start_message_move_task(
        &self,
        account_id: String,
        region: String,
        source_arn: String,
        destination_arn: Option<String>,
        max_number_of_messages_per_second: Option<i32>,
    ) -> Pin<Box<dyn Future<Output = Result<String, SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            // Verify source ARN maps to an existing queue.
            let names: Vec<String> = cm
                .smembers(k_queues(&account_id, &region))
                .await
                .map_err(redis_err)?;
            let mut source_found = false;
            for name in &names {
                if let Some(sq) = load_queue(&mut cm, &account_id, &region, name).await? {
                    if sq.arn == source_arn {
                        source_found = true;
                        break;
                    }
                }
            }
            if !source_found {
                return Err(SqsError::SourceArnNotFound { source_arn });
            }

            // Check for an already-running task on this source.
            let task_handles: Vec<String> = cm
                .lrange(k_tasks(&account_id, &region), 0, -1)
                .await
                .map_err(redis_err)?;
            for handle in &task_handles {
                let raw: Option<String> = cm
                    .get(k_task(&account_id, &region, handle))
                    .await
                    .map_err(redis_err)?;
                if let Some(json) = raw {
                    let task: StoredTask = serde_json::from_str(&json).map_err(json_err)?;
                    if task.source_arn == source_arn && task.status == "RUNNING" {
                        return Err(SqsError::MessageMoveTaskAlreadyRunning);
                    }
                }
            }

            let task_handle = Uuid::new_v4().to_string();
            let task = StoredTask {
                task_handle: task_handle.clone(),
                source_arn,
                destination_arn,
                status: "RUNNING".to_string(),
                max_number_of_messages_per_second,
                started_timestamp: Utc::now().timestamp_millis(),
                approximate_number_of_messages_moved: 0,
                approximate_number_of_messages_to_move: Some(0),
            };
            let json = serde_json::to_string(&task).map_err(json_err)?;
            let _: () = cm
                .set(k_task(&account_id, &region, &task_handle), &json)
                .await
                .map_err(redis_err)?;
            let _: () = cm
                .rpush(k_tasks(&account_id, &region), &task_handle)
                .await
                .map_err(redis_err)?;
            Ok(task_handle)
        })
    }

    fn cancel_message_move_task(
        &self,
        account_id: String,
        region: String,
        task_handle: String,
    ) -> Pin<Box<dyn Future<Output = Result<i64, SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            let raw: Option<String> = cm
                .get(k_task(&account_id, &region, &task_handle))
                .await
                .map_err(redis_err)?;
            let json = raw.ok_or_else(|| SqsError::InvalidTaskHandle {
                task_handle: task_handle.clone(),
            })?;
            let mut task: StoredTask = serde_json::from_str(&json).map_err(json_err)?;

            if task.status != "RUNNING" {
                return Err(SqsError::TaskNotCancellable);
            }

            let moved = task.approximate_number_of_messages_moved;
            task.status = "CANCELLING".to_string();
            let json = serde_json::to_string(&task).map_err(json_err)?;
            let _: () = cm
                .set(k_task(&account_id, &region, &task_handle), &json)
                .await
                .map_err(redis_err)?;
            Ok(moved)
        })
    }

    fn list_message_move_tasks(
        &self,
        account_id: String,
        region: String,
        source_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<MessageMoveTask>, SqsError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            let handles: Vec<String> = cm
                .lrange(k_tasks(&account_id, &region), 0, -1)
                .await
                .map_err(redis_err)?;
            let mut tasks = Vec::new();
            for handle in handles {
                let raw: Option<String> = cm
                    .get(k_task(&account_id, &region, &handle))
                    .await
                    .map_err(redis_err)?;
                if let Some(json) = raw {
                    let task: StoredTask = serde_json::from_str(&json).map_err(json_err)?;
                    if task.source_arn == source_arn {
                        tasks.push(task.to_task());
                    }
                }
            }
            Ok(tasks)
        })
    }

    fn snapshot(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = SqsStateView> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            let names: Vec<String> = cm
                .smembers(k_queues(&account_id, &region))
                .await
                .unwrap_or_default();
            let mut queues = HashMap::new();
            for name in names {
                let raw: Option<String> = cm
                    .get(k_queue(&account_id, &region, &name))
                    .await
                    .ok()
                    .flatten();
                if let Some(json) = raw {
                    if let Ok(sq) = serde_json::from_str::<StoredQueue>(&json) {
                        let ts = Utc.timestamp_opt(sq.created_ts, 0).single();
                        let mts = Utc.timestamp_opt(sq.modified_ts, 0).single();
                        queues.insert(
                            name,
                            QueueStateView {
                                name: sq.name,
                                url: sq.url,
                                arn: sq.arn,
                                region: sq.region,
                                account_id: sq.account_id,
                                created_timestamp: ts.map(|t| t.to_rfc3339()),
                                last_modified_timestamp: mts.map(|t| t.to_rfc3339()),
                                visibility_timeout: sq.visibility_timeout,
                                delay_seconds: sq.delay_seconds,
                                maximum_message_size: sq.maximum_message_size,
                                message_retention_period: sq.message_retention_period,
                                receive_wait_time_seconds: sq.receive_wait_time_seconds,
                                fifo_queue: sq.fifo_queue,
                                content_based_deduplication: sq.content_based_deduplication,
                                tags: sq.tags,
                                redrive_policy: sq.redrive_policy,
                                policy: sq.policy,
                            },
                        );
                    }
                }
            }
            SqsStateView {
                queues,
                message_move_tasks: Vec::new(),
            }
        })
    }

    fn restore(
        &self,
        account_id: String,
        region: String,
        view: SqsStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            // Delete all existing queues
            let names: Vec<String> = cm
                .smembers(k_queues(&account_id, &region))
                .await
                .unwrap_or_default();
            for name in &names {
                let _: () = cm
                    .del(k_queue(&account_id, &region, name))
                    .await
                    .unwrap_or(());
                let _: () = cm
                    .del(k_msgs(&account_id, &region, name))
                    .await
                    .unwrap_or(());
                let _: () = cm
                    .del(k_order(&account_id, &region, name))
                    .await
                    .unwrap_or(());
                let _: () = cm
                    .del(k_deleted(&account_id, &region, name))
                    .await
                    .unwrap_or(());
            }
            let _: () = cm.del(k_queues(&account_id, &region)).await.unwrap_or(());

            // Re-create from view
            for (name, qv) in view.queues {
                let created_ts = qv
                    .created_timestamp
                    .as_deref()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.timestamp())
                    .unwrap_or_else(|| Utc::now().timestamp());
                let modified_ts = qv
                    .last_modified_timestamp
                    .as_deref()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.timestamp())
                    .unwrap_or_else(|| Utc::now().timestamp());
                let sq = StoredQueue {
                    name: qv.name,
                    url: qv.url,
                    arn: qv.arn,
                    region: qv.region,
                    account_id: qv.account_id,
                    created_ts,
                    modified_ts,
                    visibility_timeout: qv.visibility_timeout,
                    delay_seconds: qv.delay_seconds,
                    maximum_message_size: qv.maximum_message_size,
                    message_retention_period: qv.message_retention_period,
                    receive_wait_time_seconds: qv.receive_wait_time_seconds,
                    fifo_queue: qv.fifo_queue,
                    content_based_deduplication: qv.content_based_deduplication,
                    redrive_policy: qv.redrive_policy,
                    policy: qv.policy,
                    dead_letter_target_arn: None,
                    tags: qv.tags,
                    permissions: HashMap::new(),
                };
                let json = serde_json::to_string(&sq)
                    .map_err(|e| StateViewError::Invalid(e.to_string()))?;
                let _: () = cm
                    .set(k_queue(&account_id, &region, &name), &json)
                    .await
                    .map_err(|e| StateViewError::Invalid(e.to_string()))?;
                let _: () = cm
                    .sadd(k_queues(&account_id, &region), &name)
                    .await
                    .map_err(|e| StateViewError::Invalid(e.to_string()))?;
            }
            Ok(())
        })
    }

    fn merge(
        &self,
        account_id: String,
        region: String,
        view: SqsStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>> {
        let mut cm = self.manager.clone();
        Box::pin(async move {
            for (name, qv) in view.queues {
                let created_ts = qv
                    .created_timestamp
                    .as_deref()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.timestamp())
                    .unwrap_or_else(|| Utc::now().timestamp());
                let modified_ts = qv
                    .last_modified_timestamp
                    .as_deref()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.timestamp())
                    .unwrap_or_else(|| Utc::now().timestamp());
                let sq = StoredQueue {
                    name: qv.name,
                    url: qv.url,
                    arn: qv.arn,
                    region: qv.region,
                    account_id: qv.account_id,
                    created_ts,
                    modified_ts,
                    visibility_timeout: qv.visibility_timeout,
                    delay_seconds: qv.delay_seconds,
                    maximum_message_size: qv.maximum_message_size,
                    message_retention_period: qv.message_retention_period,
                    receive_wait_time_seconds: qv.receive_wait_time_seconds,
                    fifo_queue: qv.fifo_queue,
                    content_based_deduplication: qv.content_based_deduplication,
                    redrive_policy: qv.redrive_policy,
                    policy: qv.policy,
                    dead_letter_target_arn: None,
                    tags: qv.tags,
                    permissions: HashMap::new(),
                };
                let json = serde_json::to_string(&sq)
                    .map_err(|e| StateViewError::Invalid(e.to_string()))?;
                let _: () = cm
                    .set(k_queue(&account_id, &region, &name), &json)
                    .await
                    .map_err(|e| StateViewError::Invalid(e.to_string()))?;
                let _: () = cm
                    .sadd(k_queues(&account_id, &region), &name)
                    .await
                    .map_err(|e| StateViewError::Invalid(e.to_string()))?;
            }
            Ok(())
        })
    }

    fn scopes_with_state(&self) -> Vec<(String, String)> {
        vec![]
    }
}
