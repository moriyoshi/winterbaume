//! Warm execution environments, reused between invocations of the same
//! function version and configuration, as Lambda reuses them.

use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex, Weak};
use std::time::Duration;

use tokio::sync::{Notify, OwnedSemaphorePermit, Semaphore};
use winterbaume_lambda::StubReason;

use crate::instance::Instance;

/// What an environment is reusable for.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PoolKey {
    pub function_arn: String,
    pub function_version: String,
    /// Hash of everything else the environment was started with: code,
    /// image, configuration, environment.
    pub fingerprint: u64,
}

struct PerFunction {
    permits: Arc<Semaphore>,
    /// Woken when an environment of the function is returned.
    returned: Arc<Notify>,
    generation: u64,
}

pub(crate) struct Pool {
    idle: Mutex<HashMap<PoolKey, Vec<Instance>>>,
    global: Arc<Semaphore>,
    functions: Mutex<HashMap<String, PerFunction>>,
    max_per_function: usize,
    warm_ttl: Duration,
    reaper: std::sync::OnceLock<()>,
}

/// An environment checked out for one invocation.  Dropping a lease without
/// [`Pool::release`] discards the environment, which kills its VM.
pub(crate) struct Lease {
    pub key: PoolKey,
    pub instance: Instance,
}

impl Pool {
    pub(crate) fn new(max_vms: usize, max_per_function: usize, warm_ttl: Duration) -> Arc<Self> {
        Arc::new(Self {
            idle: Mutex::new(HashMap::new()),
            global: Arc::new(Semaphore::new(max_vms)),
            functions: Mutex::new(HashMap::new()),
            max_per_function,
            warm_ttl,
            reaper: std::sync::OnceLock::new(),
        })
    }

    fn lock_idle(&self) -> std::sync::MutexGuard<'_, HashMap<PoolKey, Vec<Instance>>> {
        self.idle.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn function(&self, arn: &str) -> (Arc<Semaphore>, Arc<Notify>, u64) {
        let mut functions = self.functions.lock().unwrap_or_else(|e| e.into_inner());
        let entry = functions
            .entry(arn.to_string())
            .or_insert_with(|| PerFunction {
                permits: Arc::new(Semaphore::new(self.max_per_function)),
                returned: Arc::new(Notify::new()),
                generation: 0,
            });
        (
            entry.permits.clone(),
            entry.returned.clone(),
            entry.generation,
        )
    }

    fn take_idle(&self, key: &PoolKey) -> Option<Instance> {
        let mut idle = self.lock_idle();
        let instances = idle.get_mut(key)?;
        while let Some(instance) = instances.pop() {
            if instance.is_usable() {
                return Some(instance);
            }
        }
        None
    }

    /// Drop the least recently used idle environment of any function, to
    /// make room.  Whether there was one.
    fn evict_one(&self) -> bool {
        let mut idle = self.lock_idle();
        let oldest = idle
            .iter()
            .flat_map(|(key, list)| {
                list.iter()
                    .enumerate()
                    .map(move |(i, inst)| (key, i, inst.last_used))
            })
            .min_by_key(|(_, _, used)| *used)
            .map(|(key, i, _)| (key.clone(), i));
        match oldest {
            Some((key, index)) => {
                if let Some(list) = idle.get_mut(&key) {
                    list.remove(index);
                }
                true
            }
            None => false,
        }
    }

    /// An environment for `key`: an idle one, or a new one from `start`
    /// once the per-function and global limits allow.
    pub(crate) async fn acquire<F, Fut>(
        self: &Arc<Self>,
        key: PoolKey,
        start: F,
    ) -> Result<Lease, StubReason>
    where
        F: FnOnce(u64, Vec<OwnedSemaphorePermit>) -> Fut,
        Fut: Future<Output = Result<Instance, StubReason>>,
    {
        self.ensure_reaper();
        let (per_function, returned, generation) = self.function(&key.function_arn);
        let function_permit = loop {
            if let Some(instance) = self.take_idle(&key) {
                return Ok(Lease { key, instance });
            }
            let notified = returned.notified();
            tokio::select! {
                permit = per_function.clone().acquire_owned() => {
                    break permit.map_err(|_| StubReason::VmUnavailable)?;
                }
                _ = notified => continue,
            }
        };
        // Someone may have returned an environment meanwhile.
        if let Some(instance) = self.take_idle(&key) {
            return Ok(Lease { key, instance });
        }
        let global_permit = loop {
            match self.global.clone().try_acquire_owned() {
                Ok(permit) => break permit,
                Err(_) if self.evict_one() => continue,
                Err(_) => {
                    break self
                        .global
                        .clone()
                        .acquire_owned()
                        .await
                        .map_err(|_| StubReason::VmUnavailable)?;
                }
            }
        };
        let instance = start(generation, vec![function_permit, global_permit]).await?;
        Ok(Lease { key, instance })
    }

    /// Return an environment after an invocation.  It is kept for reuse
    /// unless it failed or its function was retired meanwhile.
    pub(crate) fn release(&self, lease: Lease) {
        let (_, returned, generation) = self.function(&lease.key.function_arn);
        if lease.instance.is_usable() && lease.instance.generation == generation {
            self.lock_idle()
                .entry(lease.key)
                .or_default()
                .push(lease.instance);
        }
        returned.notify_one();
    }

    /// Forget the environments of a function.
    pub(crate) fn retire(&self, function_arn: &str) {
        {
            let mut functions = self.functions.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(entry) = functions.get_mut(function_arn) {
                entry.generation += 1;
            }
        }
        self.lock_idle()
            .retain(|key, _| key.function_arn != function_arn);
    }

    /// Drop idle environments that outlived the warm TTL or died.
    pub(crate) fn reap(&self) {
        let ttl = self.warm_ttl;
        let mut idle = self.lock_idle();
        for list in idle.values_mut() {
            list.retain(|instance| instance.is_usable() && instance.last_used.elapsed() < ttl);
        }
        idle.retain(|_, list| !list.is_empty());
    }

    /// Number of idle environments, for tests.
    pub(crate) fn idle_count(&self) -> usize {
        self.lock_idle().values().map(Vec::len).sum()
    }

    fn ensure_reaper(self: &Arc<Self>) {
        self.reaper.get_or_init(|| {
            let weak: Weak<Self> = Arc::downgrade(self);
            let period =
                (self.warm_ttl / 4).clamp(Duration::from_millis(100), Duration::from_secs(10));
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(period);
                loop {
                    interval.tick().await;
                    match weak.upgrade() {
                        Some(pool) => pool.reap(),
                        None => break,
                    }
                }
            });
        });
    }
}
