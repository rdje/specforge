use serde::Serialize;

use crate::ir::IrStage;
use crate::ir::source::ResidualDecisionPacket;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct IntentIr {
    pub schema_version: u32,
    pub stage: IrStage,
    pub semantic_ir_ref: String,
    pub intent_identity: IntentIdentity,
    pub actors: Vec<IntentActor>,
    pub behaviors: Vec<BehaviorIntent>,
    pub constraints: Vec<IntentConstraint>,
    pub assumptions: Vec<IntentAssumption>,
    pub residual_decisions: Vec<ResidualDecisionPacket>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct IntentIdentity {
    pub intent_id: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct IntentActor {
    pub actor_id: String,
    pub responsibilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BehaviorIntent {
    pub behavior_id: String,
    pub statement: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct IntentConstraint {
    pub constraint_id: String,
    pub statement: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct IntentAssumption {
    pub assumption_id: String,
    pub statement: String,
}
