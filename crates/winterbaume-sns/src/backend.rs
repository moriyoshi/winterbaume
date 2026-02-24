//! Pluggable storage backend for the SNS mock service.
//!
//! The [`SnsBackend`] trait abstracts all topic, subscription, and messaging
//! operations so that alternative implementations can be swapped in without
//! touching the protocol layer.
//!
//! The built-in [`InMemorySnsBackend`] is the default and stores everything in
//! `HashMap`s protected by an `RwLock`, partitioned by account ID and region
//! via [`BackendState`].
//!
//! # Object safety and async
//!
//! The same `Pin<Box<dyn Future>>` pattern used by [`winterbaume_core::Vfs`] is
//! used here so that `Arc<dyn SnsBackend>` is object-safe without requiring the
//! `async-trait` crate.  All string parameters are passed as owned `String`s so
//! that futures can be `'static` without lifetime annotations.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{BackendState, StateViewError};

use crate::state::{SnsError, SnsState};
use crate::types::{
    PlatformApplicationState, PlatformEndpointState, SmsSandboxPhoneNumber, Subscription, Topic,
};
use crate::views::SnsStateView;

// ---------------------------------------------------------------------------
// Trait
// ---------------------------------------------------------------------------

/// Pluggable backend for SNS topic, subscription, and messaging storage.
///
/// Implementations are responsible for partitioning data by `account_id` and
/// `region` internally.  All parameters are owned so the returned futures are
/// `'static`.
pub trait SnsBackend: Send + Sync {
    // --- Topics ---

    /// Create a topic; returns the topic ARN.  Idempotent: re-creating an
    /// existing topic by the same name returns its existing ARN.
    fn create_topic(
        &self,
        account_id: String,
        region: String,
        name: String,
        tags: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<String, SnsError>> + Send>>;

    fn delete_topic(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    fn list_topics(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Topic>, SnsError>> + Send>>;

    fn get_topic_attributes(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, String>, SnsError>> + Send>>;

    fn set_topic_attributes(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        attribute_name: String,
        attribute_value: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    fn tag_resource(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
        tags: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    fn untag_resource(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
        tag_keys: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    fn list_tags_for_resource(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, String>, SnsError>> + Send>>;

    // --- Permissions ---

    fn add_permission(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        label: String,
        aws_account_ids: Vec<String>,
        action_names: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    fn remove_permission(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        label: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    // --- Subscriptions ---

    /// Subscribe to a topic; returns the subscription ARN.
    fn subscribe(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        protocol: String,
        endpoint: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, SnsError>> + Send>>;

    fn unsubscribe(
        &self,
        account_id: String,
        region: String,
        subscription_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    fn list_subscriptions(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Subscription>, SnsError>> + Send>>;

    fn list_subscriptions_by_topic(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Subscription>, SnsError>> + Send>>;

    /// Confirm a subscription; returns the confirmed subscription ARN.
    fn confirm_subscription(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        token: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, SnsError>> + Send>>;

    fn get_subscription_attributes(
        &self,
        account_id: String,
        region: String,
        subscription_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, String>, SnsError>> + Send>>;

    fn set_subscription_attributes(
        &self,
        account_id: String,
        region: String,
        subscription_arn: String,
        attribute_name: String,
        attribute_value: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    // --- Publishing (stub — no delivery) ---

    /// Publish a message to a topic; returns a generated message ID.
    fn publish(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        message: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, SnsError>> + Send>>;

    /// Publish a batch of messages; returns `(batch_entry_id, message_id)` pairs.
    fn publish_batch(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        entries: Vec<(String, String)>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<(String, String)>, SnsError>> + Send>>;

    // --- Data protection ---

    fn get_data_protection_policy(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>, SnsError>> + Send>>;

    fn put_data_protection_policy(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        policy: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    // --- SMS attributes ---

    fn get_sms_attributes(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, String>, SnsError>> + Send>>;

    fn set_sms_attributes(
        &self,
        account_id: String,
        region: String,
        attributes: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    // --- Platform applications ---

    /// Create a platform application; returns the application ARN.
    fn create_platform_application(
        &self,
        account_id: String,
        region: String,
        name: String,
        platform: String,
        attributes: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<String, SnsError>> + Send>>;

    fn delete_platform_application(
        &self,
        account_id: String,
        region: String,
        platform_application_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    fn get_platform_application_attributes(
        &self,
        account_id: String,
        region: String,
        platform_application_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<PlatformApplicationState, SnsError>> + Send>>;

    fn set_platform_application_attributes(
        &self,
        account_id: String,
        region: String,
        platform_application_arn: String,
        attributes: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    fn list_platform_applications(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<PlatformApplicationState>, SnsError>> + Send>>;

    // --- Platform endpoints ---

    /// Create a platform endpoint; returns the endpoint ARN.
    fn create_platform_endpoint(
        &self,
        account_id: String,
        region: String,
        platform_application_arn: String,
        token: String,
        custom_user_data: Option<String>,
        attributes: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<String, SnsError>> + Send>>;

    fn delete_endpoint(
        &self,
        account_id: String,
        region: String,
        endpoint_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    fn get_endpoint_attributes(
        &self,
        account_id: String,
        region: String,
        endpoint_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<PlatformEndpointState, SnsError>> + Send>>;

    fn set_endpoint_attributes(
        &self,
        account_id: String,
        region: String,
        endpoint_arn: String,
        attributes: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    fn list_endpoints_by_platform_application(
        &self,
        account_id: String,
        region: String,
        platform_application_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<PlatformEndpointState>, SnsError>> + Send>>;

    // --- SMS sandbox ---

    fn create_sms_sandbox_phone_number(
        &self,
        account_id: String,
        region: String,
        phone_number: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    fn delete_sms_sandbox_phone_number(
        &self,
        account_id: String,
        region: String,
        phone_number: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    fn verify_sms_sandbox_phone_number(
        &self,
        account_id: String,
        region: String,
        phone_number: String,
        one_time_password: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    fn list_sms_sandbox_phone_numbers(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<SmsSandboxPhoneNumber>, SnsError>> + Send>>;

    fn get_sms_sandbox_account_status(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<bool, SnsError>> + Send>>;

    // --- SMS opt-out ---

    fn check_if_phone_number_is_opted_out(
        &self,
        account_id: String,
        region: String,
        phone_number: String,
    ) -> Pin<Box<dyn Future<Output = Result<bool, SnsError>> + Send>>;

    fn list_phone_numbers_opted_out(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, SnsError>> + Send>>;

    fn opt_in_phone_number(
        &self,
        account_id: String,
        region: String,
        phone_number: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>>;

    // --- Scope discovery ---

    /// Returns sorted `(account_id, region)` pairs that have state.
    fn scopes_with_state(&self) -> Vec<(String, String)>;

    // --- State snapshot / restore / merge ---

    /// Take a snapshot of the state for the given account/region.
    fn snapshot(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = SnsStateView> + Send>>;

    /// Replace state for the given account/region from a view.
    fn restore(
        &self,
        account_id: String,
        region: String,
        view: SnsStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>>;

    /// Merge a partial view into existing state (additive).
    fn merge(
        &self,
        account_id: String,
        region: String,
        view: SnsStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>>;
}

// ---------------------------------------------------------------------------
// In-memory implementation
// ---------------------------------------------------------------------------

fn not_found(msg: impl Into<String>) -> SnsError {
    SnsError::ResourceNotFound(msg.into())
}

/// In-memory [`SnsBackend`] backed by [`SnsState`], partitioned by account ID
/// and region via [`BackendState`].
pub struct InMemorySnsBackend {
    pub(crate) state: Arc<BackendState<SnsState>>,
}

impl InMemorySnsBackend {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
        }
    }
}

impl Default for InMemorySnsBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl SnsBackend for InMemorySnsBackend {
    fn create_topic(
        &self,
        account_id: String,
        region: String,
        name: String,
        tags: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<String, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.create_topic(&name, &account_id, &region, tags)
                .map(|t| t.arn.clone())
        })
    }

    fn delete_topic(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.delete_topic(&topic_arn)
        })
    }

    fn list_topics(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Topic>, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.list_topics().into_iter().cloned().collect())
        })
    }

    fn get_topic_attributes(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, String>, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.get_topic_attributes(&topic_arn)
                .map(|t| t.attributes.clone())
        })
    }

    fn set_topic_attributes(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        attribute_name: String,
        attribute_value: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            match s.topics.get_mut(&topic_arn) {
                Some(topic) => {
                    topic.attributes.insert(attribute_name, attribute_value);
                    Ok(())
                }
                None => Err(not_found(format!("Topic does not exist: {topic_arn}"))),
            }
        })
    }

    fn tag_resource(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
        tags: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            match s.topics.get_mut(&resource_arn) {
                Some(topic) => {
                    topic.tags.extend(tags);
                    Ok(())
                }
                None => Err(not_found(format!(
                    "Resource does not exist: {resource_arn}"
                ))),
            }
        })
    }

    fn untag_resource(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
        tag_keys: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            match s.topics.get_mut(&resource_arn) {
                Some(topic) => {
                    for k in &tag_keys {
                        topic.tags.remove(k);
                    }
                    Ok(())
                }
                None => Err(not_found(format!(
                    "Resource does not exist: {resource_arn}"
                ))),
            }
        })
    }

    fn list_tags_for_resource(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, String>, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            match s.topics.get(&resource_arn) {
                Some(topic) => Ok(topic.tags.clone()),
                None => Err(not_found(format!(
                    "Resource does not exist: {resource_arn}"
                ))),
            }
        })
    }

    fn add_permission(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        label: String,
        aws_account_ids: Vec<String>,
        action_names: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.add_permission(&topic_arn, &label, aws_account_ids, action_names)
        })
    }

    fn remove_permission(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        label: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.remove_permission(&topic_arn, &label)
        })
    }

    fn subscribe(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        protocol: String,
        endpoint: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.subscribe(&topic_arn, &protocol, &endpoint, &account_id, &region)
                .map(|sub| sub.arn.clone())
        })
    }

    fn unsubscribe(
        &self,
        account_id: String,
        region: String,
        subscription_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.unsubscribe(&subscription_arn)
        })
    }

    fn list_subscriptions(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Subscription>, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.list_subscriptions().into_iter().cloned().collect())
        })
    }

    fn list_subscriptions_by_topic(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Subscription>, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.list_subscriptions_by_topic(&topic_arn)
                .map(|v| v.into_iter().cloned().collect())
        })
    }

    fn confirm_subscription(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        token: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.confirm_subscription(&topic_arn, &token)
        })
    }

    fn get_subscription_attributes(
        &self,
        account_id: String,
        region: String,
        subscription_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, String>, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.get_subscription_attributes(&subscription_arn).cloned()
        })
    }

    fn set_subscription_attributes(
        &self,
        account_id: String,
        region: String,
        subscription_arn: String,
        attribute_name: String,
        attribute_value: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.set_subscription_attributes(&subscription_arn, &attribute_name, &attribute_value)
        })
    }

    fn publish(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        message: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.publish(&topic_arn, &message)
        })
    }

    fn publish_batch(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        entries: Vec<(String, String)>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<(String, String)>, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            let pairs: Vec<(&str, &str)> = entries
                .iter()
                .map(|(id, msg)| (id.as_str(), msg.as_str()))
                .collect();
            s.publish_batch(&topic_arn, &pairs)
        })
    }

    fn get_data_protection_policy(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.get_data_protection_policy(&topic_arn)
                .map(|opt| opt.map(|s| s.to_string()))
        })
    }

    fn put_data_protection_policy(
        &self,
        account_id: String,
        region: String,
        topic_arn: String,
        policy: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.put_data_protection_policy(&topic_arn, &policy)
        })
    }

    fn get_sms_attributes(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, String>, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.get_sms_attributes().clone())
        })
    }

    fn set_sms_attributes(
        &self,
        account_id: String,
        region: String,
        attributes: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.set_sms_attributes(attributes);
            Ok(())
        })
    }

    fn create_platform_application(
        &self,
        account_id: String,
        region: String,
        name: String,
        platform: String,
        attributes: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<String, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            let app =
                s.create_platform_application(&name, &platform, attributes, &account_id, &region);
            Ok(app.arn.clone())
        })
    }

    fn delete_platform_application(
        &self,
        account_id: String,
        region: String,
        platform_application_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.delete_platform_application(&platform_application_arn)
        })
    }

    fn get_platform_application_attributes(
        &self,
        account_id: String,
        region: String,
        platform_application_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<PlatformApplicationState, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.get_platform_application_attributes(&platform_application_arn)
                .cloned()
        })
    }

    fn set_platform_application_attributes(
        &self,
        account_id: String,
        region: String,
        platform_application_arn: String,
        attributes: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.set_platform_application_attributes(&platform_application_arn, attributes)
        })
    }

    fn list_platform_applications(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<PlatformApplicationState>, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.list_platform_applications()
                .into_iter()
                .cloned()
                .collect())
        })
    }

    fn create_platform_endpoint(
        &self,
        account_id: String,
        region: String,
        platform_application_arn: String,
        token: String,
        custom_user_data: Option<String>,
        attributes: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<String, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.create_platform_endpoint(
                &platform_application_arn,
                &token,
                custom_user_data.as_deref(),
                attributes,
                &account_id,
                &region,
            )
            .map(|ep| ep.arn.clone())
        })
    }

    fn delete_endpoint(
        &self,
        account_id: String,
        region: String,
        endpoint_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.delete_endpoint(&endpoint_arn)
        })
    }

    fn get_endpoint_attributes(
        &self,
        account_id: String,
        region: String,
        endpoint_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<PlatformEndpointState, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.get_endpoint_attributes(&endpoint_arn).cloned()
        })
    }

    fn set_endpoint_attributes(
        &self,
        account_id: String,
        region: String,
        endpoint_arn: String,
        attributes: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.set_endpoint_attributes(&endpoint_arn, attributes)
        })
    }

    fn list_endpoints_by_platform_application(
        &self,
        account_id: String,
        region: String,
        platform_application_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<PlatformEndpointState>, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.list_endpoints_by_platform_application(&platform_application_arn)
                .map(|v| v.into_iter().cloned().collect())
        })
    }

    fn create_sms_sandbox_phone_number(
        &self,
        account_id: String,
        region: String,
        phone_number: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.create_sms_sandbox_phone_number(&phone_number)
        })
    }

    fn delete_sms_sandbox_phone_number(
        &self,
        account_id: String,
        region: String,
        phone_number: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.delete_sms_sandbox_phone_number(&phone_number)
        })
    }

    fn verify_sms_sandbox_phone_number(
        &self,
        account_id: String,
        region: String,
        phone_number: String,
        one_time_password: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.verify_sms_sandbox_phone_number(&phone_number, &one_time_password)
        })
    }

    fn list_sms_sandbox_phone_numbers(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<SmsSandboxPhoneNumber>, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.list_sms_sandbox_phone_numbers().to_vec())
        })
    }

    fn get_sms_sandbox_account_status(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<bool, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.get_sms_sandbox_account_status())
        })
    }

    fn check_if_phone_number_is_opted_out(
        &self,
        account_id: String,
        region: String,
        phone_number: String,
    ) -> Pin<Box<dyn Future<Output = Result<bool, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.check_if_phone_number_is_opted_out(&phone_number))
        })
    }

    fn list_phone_numbers_opted_out(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.list_phone_numbers_opted_out().to_vec())
        })
    }

    fn opt_in_phone_number(
        &self,
        account_id: String,
        region: String,
        phone_number: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnsError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.opt_in_phone_number(&phone_number)
        })
    }

    fn scopes_with_state(&self) -> Vec<(String, String)> {
        self.state.scopes_with_state()
    }

    fn snapshot(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = SnsStateView> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let guard = lock.read().await;
            SnsStateView::from(&*guard)
        })
    }

    fn restore(
        &self,
        account_id: String,
        region: String,
        view: SnsStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut guard = lock.write().await;
            *guard = SnsState::from(view);
            Ok(())
        })
    }

    fn merge(
        &self,
        account_id: String,
        region: String,
        view: SnsStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>> {
        use crate::types::{PlatformApplicationState, PlatformEndpointState, Subscription, Topic};
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut guard = lock.write().await;
            for (arn, topic_view) in view.topics {
                guard.topics.insert(arn, Topic::from(topic_view));
            }
            for (arn, sub_view) in view.subscriptions {
                guard
                    .subscriptions
                    .insert(arn, Subscription::from(sub_view));
            }
            for (arn, app_view) in view.platform_applications {
                guard
                    .platform_applications
                    .insert(arn, PlatformApplicationState::from(app_view));
            }
            for (arn, ep_view) in view.platform_endpoints {
                guard
                    .platform_endpoints
                    .insert(arn, PlatformEndpointState::from(ep_view));
            }
            guard.sms_attributes.extend(view.sms_attributes);
            Ok(())
        })
    }
}
