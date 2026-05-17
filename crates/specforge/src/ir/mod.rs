pub mod adapters;
pub mod evidence;
pub mod intent;
pub mod isf_ir;
pub mod prior_memory;
pub mod semantic;
pub mod source;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IrStage {
    SourceIr,
    EvidenceIr,
    SemanticIr,
    IntentIr,
    FsmAdapter,
    IsfAdapter,
}

impl IrStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourceIr => "source_ir",
            Self::EvidenceIr => "evidence_ir",
            Self::SemanticIr => "semantic_ir",
            Self::IntentIr => "intent_ir",
            Self::FsmAdapter => "fsm_adapter",
            Self::IsfAdapter => "isf_adapter",
        }
    }
}
