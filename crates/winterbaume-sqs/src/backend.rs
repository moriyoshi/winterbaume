//! Pluggable storage backend for the SQS mock service.
//!
//! The [`SqsBackend`] trait abstracts all queue and message operations so that
//! alternative implementations (e.g. file-backed, Redis-backed) can be swapped
//! in without touching the protocol layer.
//!
//! The built-in [`InMemorySqsBackend`] is the default and stores everything in
//! `HashMap`s protected by an `RwLock`, partitioned by account ID and region
//! via [`BackendState`].
//!
//! # Object safety and async
//!
//! The same `Pin<Box<dyn Future>>` pattern used by [`winterbaume_core::Vfs`] is
//! used here so that `Arc<dyn SqsBackend>` is object-safe without requiring the
//! `async-trait` crate.  All string parameters are passed as owned `String`s so
//! that futures can be `'static` without lifetime annotations.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{BackendState, StateViewError};

use crate::state::{SqsError, SqsState};
use crate::types::{Message, MessageAttribute, MessageMoveTask, Queue};
use crate::views::SqsStateView;

// ---------------------------------------------------------------------------
// Output types
// ---------------------------------------------------------------------------

/// Return value of [`SqsBackend::send_message`].
#[derive(Debug)]
pub struct SendMessageOutput {
    pub message_id: String,
    pub body_md5: String,
}

// ---------------------------------------------------------------------------
// Trait
// ---------------------------------------------------------------------------

/// Pluggable backend for SQS queue and message storage.
///
/// Implementations are responsible for partitioning data by `account_id` and
/// `region` internally.  All parameters are owned so the returned futures are
/// `'static`.
pub trait SqsBackend: Send + Sync {
    // --- Queue management ---

    fn create_queue(
        &self,
        account_id: String,
        region: String,
        name: String,
        attributes: HashMap<String, String>,
        tags: Option<HashMap<String, String>>,
    ) -> Pin<Box<dyn Future<Output = Result<Queue, SqsError>> + Send>>;

    fn delete_queue(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>>;

    fn get_queue_url(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, SqsError>> + Send>>;

    fn get_queue_attributes(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, String>, SqsError>> + Send>>;

    fn set_queue_attributes(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        attributes: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>>;

    fn list_queues(
        &self,
        account_id: String,
        region: String,
        prefix: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Queue>, SqsError>> + Send>>;

    // --- Message operations ---

    #[allow(clippy::too_many_arguments)]
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
    ) -> Pin<Box<dyn Future<Output = Result<SendMessageOutput, SqsError>> + Send>>;

    fn receive_messages(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        max: usize,
        visibility_timeout: Option<u32>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Message>, SqsError>> + Send>>;

    fn delete_message(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        receipt_handle: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>>;

    fn purge_queue(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>>;

    fn change_message_visibility(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        receipt_handle: String,
        visibility_timeout: u32,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>>;

    fn change_message_visibility_batch(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        entries: Vec<(String, String, u32)>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, SqsError>> + Send>>;

    // --- Tags ---

    fn list_queue_tags(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, String>, SqsError>> + Send>>;

    fn tag_queue(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        tags: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>>;

    fn untag_queue(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        tag_keys: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>>;

    // --- Permissions ---

    fn add_permission(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        label: String,
        aws_account_ids: Vec<String>,
        actions: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>>;

    fn remove_permission(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        label: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>>;

    // --- Dead-letter queues ---

    fn list_dead_letter_source_queues(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, SqsError>> + Send>>;

    // --- Message move tasks ---

    fn start_message_move_task(
        &self,
        account_id: String,
        region: String,
        source_arn: String,
        destination_arn: Option<String>,
        max_number_of_messages_per_second: Option<i32>,
    ) -> Pin<Box<dyn Future<Output = Result<String, SqsError>> + Send>>;

    fn cancel_message_move_task(
        &self,
        account_id: String,
        region: String,
        task_handle: String,
    ) -> Pin<Box<dyn Future<Output = Result<i64, SqsError>> + Send>>;

    fn list_message_move_tasks(
        &self,
        account_id: String,
        region: String,
        source_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<MessageMoveTask>, SqsError>> + Send>>;

    // --- Scope discovery ---

    /// Returns all `(account_id, region)` pairs that have state.
    fn scopes_with_state(&self) -> Vec<(String, String)>;

    // --- State snapshot / restore / merge ---

    /// Take a snapshot of the state for the given account/region.
    fn snapshot(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = SqsStateView> + Send>>;

    /// Replace state for the given account/region from a view.
    fn restore(
        &self,
        account_id: String,
        region: String,
        view: SqsStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>>;

    /// Merge a partial view into existing state (additive).
    fn merge(
        &self,
        account_id: String,
        region: String,
        view: SqsStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>>;
}

// ---------------------------------------------------------------------------
// In-memory implementation
// ---------------------------------------------------------------------------

/// In-memory [`SqsBackend`] backed by [`SqsState`], partitioned by account ID
/// and region via [`BackendState`].
pub struct InMemorySqsBackend {
    pub(crate) state: Arc<BackendState<SqsState>>,
}

impl InMemorySqsBackend {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
        }
    }
}

impl Default for InMemorySqsBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl SqsBackend for InMemorySqsBackend {
    fn create_queue(
        &self,
        account_id: String,
        region: String,
        name: String,
        attributes: HashMap<String, String>,
        tags: Option<HashMap<String, String>>,
    ) -> Pin<Box<dyn Future<Output = Result<Queue, SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.create_queue(&name, &region, &account_id, &attributes, tags.as_ref())
                .cloned()
        })
    }

    fn delete_queue(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.delete_queue(&queue_name)
        })
    }

    fn get_queue_url(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.get_queue_url(&queue_name).map(|u| u.to_string())
        })
    }

    fn get_queue_attributes(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, String>, SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.get_queue_attributes(&queue_name)
        })
    }

    fn set_queue_attributes(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        attributes: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.set_queue_attributes(&queue_name, &attributes)
        })
    }

    fn list_queues(
        &self,
        account_id: String,
        region: String,
        prefix: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Queue>, SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.list_queues(prefix.as_deref())
                .into_iter()
                .cloned()
                .collect())
        })
    }

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
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.send_message(
                &queue_name,
                &body,
                delay_seconds,
                message_attributes,
                message_group_id.as_deref(),
                message_deduplication_id.as_deref(),
            )
            .map(|out| SendMessageOutput {
                message_id: out.message_id,
                body_md5: out.body_md5,
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
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.receive_message(&queue_name, max, visibility_timeout)
        })
    }

    fn delete_message(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        receipt_handle: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.delete_message(&queue_name, &receipt_handle)
        })
    }

    fn purge_queue(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.purge_queue(&queue_name)
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
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.change_message_visibility(&queue_name, &receipt_handle, visibility_timeout)
        })
    }

    fn change_message_visibility_batch(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        entries: Vec<(String, String, u32)>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.change_message_visibility_batch(&queue_name, &entries)
        })
    }

    fn list_queue_tags(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, String>, SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.list_queue_tags(&queue_name)
        })
    }

    fn tag_queue(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        tags: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.tag_queue(&queue_name, &tags)
        })
    }

    fn untag_queue(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        tag_keys: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.untag_queue(&queue_name, &tag_keys)
        })
    }

    fn add_permission(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        label: String,
        aws_account_ids: Vec<String>,
        actions: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.add_permission(&queue_name, &label, aws_account_ids, actions)
        })
    }

    fn remove_permission(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
        label: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.remove_permission(&queue_name, &label)
        })
    }

    fn list_dead_letter_source_queues(
        &self,
        account_id: String,
        region: String,
        queue_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.list_dead_letter_source_queues(&queue_name)
        })
    }

    fn start_message_move_task(
        &self,
        account_id: String,
        region: String,
        source_arn: String,
        destination_arn: Option<String>,
        max_number_of_messages_per_second: Option<i32>,
    ) -> Pin<Box<dyn Future<Output = Result<String, SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.start_message_move_task(
                &source_arn,
                destination_arn.as_deref(),
                max_number_of_messages_per_second,
            )
        })
    }

    fn cancel_message_move_task(
        &self,
        account_id: String,
        region: String,
        task_handle: String,
    ) -> Pin<Box<dyn Future<Output = Result<i64, SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.cancel_message_move_task(&task_handle)
        })
    }

    fn list_message_move_tasks(
        &self,
        account_id: String,
        region: String,
        source_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<MessageMoveTask>, SqsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.list_message_move_tasks(&source_arn)
                .into_iter()
                .cloned()
                .collect())
        })
    }

    fn scopes_with_state(&self) -> Vec<(String, String)> {
        self.state.scopes_with_state()
    }

    fn snapshot(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = SqsStateView> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let guard = lock.read().await;
            SqsStateView::from(&*guard)
        })
    }

    fn restore(
        &self,
        account_id: String,
        region: String,
        view: SqsStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut guard = lock.write().await;
            *guard = SqsState::from(view);
            Ok(())
        })
    }

    fn merge(
        &self,
        account_id: String,
        region: String,
        view: SqsStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>> {
        use crate::state::QueueState;
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut guard = lock.write().await;
            for (name, queue_view) in view.queues {
                guard.queues.insert(name, QueueState::from(queue_view));
            }
            for task_view in view.message_move_tasks {
                let task = MessageMoveTask::from(task_view);
                guard.message_move_tasks.push(task);
                guard.message_move_task_counter += 1;
            }
            Ok(())
        })
    }
}
