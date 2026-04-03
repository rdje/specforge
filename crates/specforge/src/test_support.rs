use std::sync::{Mutex, MutexGuard, OnceLock};

/// Serializes tests that mutate process-global environment variables.
pub(crate) fn env_var_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}
