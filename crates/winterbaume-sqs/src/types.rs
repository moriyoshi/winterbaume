use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// An SQS queue.
#[derive(Debug, Clone)]
pub struct Queue {
    pub name: String,
    pub url: String,
    pub arn: String,
    pub region: String,
    pub account_id: String,
    pub created_timestamp: DateTime<Utc>,
    pub last_modified_timestamp: DateTime<Utc>,
    pub visibility_timeout: u32,
    pub delay_seconds: u32,
    pub maximum_message_size: u32,
    pub message_retention_period: u32,
    pub receive_wait_time_seconds: u32,
    pub fifo_queue: bool,
    /// FIX(terraform-e2e): Added — FIFO queue deduplication setting. Without this,
    /// GetQueueAttributes always returned "false" regardless of the CreateQueue input.
    pub content_based_deduplication: bool,
}

/// An SQS message in a queue.
#[derive(Debug, Clone)]
pub struct Message {
    pub message_id: String,
    pub body: String,
    pub body_md5: String,
    pub receipt_handle: Option<String>,
    /// All receipt handles ever issued for this message (for old-receipt-handle deletion).
    pub all_receipt_handles: Vec<String>,
    pub sender_id: String,
    pub sent_timestamp: i64,
    pub approximate_first_receive_timestamp: Option<i64>,
    pub approximate_receive_count: u32,
    pub visible_at: i64,
    pub message_attributes: HashMap<String, MessageAttribute>,
}

impl Message {
    /// Check if the message is currently visible.
    pub fn is_visible(&self) -> bool {
        let now_ms = Utc::now().timestamp_millis();
        now_ms >= self.visible_at
    }
}

/// An SQS message attribute.
#[derive(Debug, Clone)]
pub struct MessageAttribute {
    pub data_type: String,
    pub string_value: Option<String>,
    pub binary_value: Option<Vec<u8>>,
}

/// An SQS message move task.
#[derive(Debug, Clone)]
pub struct MessageMoveTask {
    pub task_handle: String,
    pub source_arn: String,
    pub destination_arn: Option<String>,
    pub status: String,
    pub max_number_of_messages_per_second: Option<i32>,
    pub started_timestamp: i64,
    pub approximate_number_of_messages_moved: i64,
    pub approximate_number_of_messages_to_move: Option<i64>,
}
