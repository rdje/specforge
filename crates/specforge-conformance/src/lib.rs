//! Downstream evaluation, replay, calibration, reviewed-oracle, and named-fixture plane.
//!
//! This crate may depend on [`specforge_core`]. The reverse dependency is forbidden.

pub use specforge_core::{error, persisted_path, project_data};

pub mod behavioral_genericity;
#[path = "../../specforge/src/eval.rs"]
pub mod eval;
pub mod ir;
pub mod test_support;
