//! Per-account, per-region backend state management.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

/// Built-in fallback mock account ID.
///
/// Use [`default_account_id`] in handler code: it returns the value
/// installed by [`set_default_account_id`] (e.g. from `winterbaume-server`'s
/// `--account-id` flag) and falls back to this constant otherwise.
pub const DEFAULT_ACCOUNT_ID: &str = "123456789012";

static CONFIGURED_ACCOUNT_ID: OnceLock<&'static str> = OnceLock::new();

/// Install a process-wide default account ID. Idempotent: calling with the
/// same value is a no-op; a different value after one has been installed is
/// silently ignored (the first writer wins). Intended to be called once at
/// server / `MockAws` startup, before any service handles a request.
pub fn set_default_account_id(id: impl Into<String>) {
    let id = id.into();
    if id == DEFAULT_ACCOUNT_ID {
        return;
    }
    let leaked: &'static str = Box::leak(id.into_boxed_str());
    let _ = CONFIGURED_ACCOUNT_ID.set(leaked);
}

/// The currently effective default account ID. Returns the value installed via
/// [`set_default_account_id`] if any, otherwise [`DEFAULT_ACCOUNT_ID`].
pub fn default_account_id() -> &'static str {
    match CONFIGURED_ACCOUNT_ID.get() {
        Some(s) => s,
        None => DEFAULT_ACCOUNT_ID,
    }
}

/// Manages per-account, per-region state for a service backend.
///
/// Modeled after moto's `BackendDict` pattern: backends are lazily
/// initialized on first access for each (account_id, region) pair.
///
/// Uses `tokio::sync::RwLock` for per-region state so locks can be held
/// across `.await` points (e.g. during blob-backed snapshot/restore).
pub struct BackendState<B: Default + Send + Sync> {
    inner: RwLock<HashMap<(String, String), Arc<tokio::sync::RwLock<B>>>>,
}

impl<B: Default + Send + Sync> BackendState<B> {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }

    /// Get or create the backend state for the given account and region.
    pub fn get(&self, account_id: &str, region: &str) -> Arc<tokio::sync::RwLock<B>> {
        let key = (account_id.to_string(), region.to_string());

        // Fast path: read lock on the outer map (std sync — brief, no await)
        {
            let map = self.inner.read().unwrap();
            if let Some(backend) = map.get(&key) {
                return Arc::clone(backend);
            }
        }

        // Slow path: write lock on the outer map, create if missing
        let mut map = self.inner.write().unwrap();
        Arc::clone(
            map.entry(key)
                .or_insert_with(|| Arc::new(tokio::sync::RwLock::new(B::default()))),
        )
    }

    /// Returns sorted `(account_id, region)` pairs that have state.
    ///
    /// Read-only: does not create empty backends (unlike [`get()`](Self::get)).
    pub fn scopes_with_state(&self) -> Vec<(String, String)> {
        let map = self.inner.read().unwrap();
        let mut scopes: Vec<(String, String)> = map.keys().cloned().collect();
        scopes.sort();
        scopes
    }

    /// Reset all state (clear all backends).
    pub fn reset(&self) {
        let mut map = self.inner.write().unwrap();
        map.clear();
    }
}

impl<B: Default + Send + Sync> Default for BackendState<B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: Default + Send + Sync> FromIterator<((String, String), B)> for BackendState<B> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = ((String, String), B)>,
    {
        Self {
            inner: RwLock::new(HashMap::from_iter(
                iter.into_iter()
                    .map(|pair| (pair.0, Arc::new(tokio::sync::RwLock::new(pair.1)))),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestState {
        counter: u32,
    }

    #[tokio::test]
    async fn test_get_creates_default() {
        let state = BackendState::<TestState>::new();
        let backend = state.get("123456789012", "us-east-1");
        assert_eq!(backend.read().await.counter, 0);
    }

    #[tokio::test]
    async fn test_get_returns_same_instance() {
        let state = BackendState::<TestState>::new();
        let b1 = state.get("123456789012", "us-east-1");
        b1.write().await.counter = 42;
        let b2 = state.get("123456789012", "us-east-1");
        assert_eq!(b2.read().await.counter, 42);
    }

    #[tokio::test]
    async fn test_different_regions_different_state() {
        let state = BackendState::<TestState>::new();
        let b1 = state.get("123456789012", "us-east-1");
        b1.write().await.counter = 10;
        let b2 = state.get("123456789012", "eu-west-1");
        assert_eq!(b2.read().await.counter, 0);
    }

    #[tokio::test]
    async fn test_reset_clears_all() {
        let state = BackendState::<TestState>::new();
        let b = state.get("123456789012", "us-east-1");
        b.write().await.counter = 99;
        state.reset();
        let b2 = state.get("123456789012", "us-east-1");
        assert_eq!(b2.read().await.counter, 0);
    }
}
