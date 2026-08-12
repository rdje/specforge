//! Specification-instance-neutral digital-chip intent extraction core.
//!
//! This crate is the compiler-visible production boundary. It may encode universal document
//! grammar and digital-design semantics, but cannot depend on conformance fixtures, reviewed
//! corpora, named examples, replay controllers, or calibrated evaluation policy.

#[path = "../../specforge/src/error.rs"]
pub mod error;
#[path = "../../specforge/src/ir/mod.rs"]
pub mod ir;
#[path = "../../specforge/src/commands/llm_text.rs"]
pub mod llm_text;
#[path = "../../specforge/src/persisted_path.rs"]
pub mod persisted_path;
#[path = "../../specforge/src/project_data.rs"]
pub mod project_data;
#[path = "../../specforge/src/provider.rs"]
pub mod provider;

#[cfg(test)]
pub(crate) mod test_support {
    use std::sync::{Mutex, MutexGuard, OnceLock};

    pub(crate) fn env_var_lock() -> MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}
