use serde::Serialize;

use crate::ir::IrStage;
use crate::ir::source::ResidualDecisionPacket;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SemanticIr {
    pub schema_version: u32,
    pub stage: IrStage,
    pub evidence_ir_ref: String,
    pub actors: Vec<ActorRecord>,
    pub interfaces: Vec<InterfaceRecord>,
    pub phases: Vec<PhaseRecord>,
    pub invariants: Vec<InvariantRecord>,
    pub contracts: Vec<ContractRecord>,
    pub gates: Vec<GateRecord>,
    pub assertions: Vec<AssertionRecord>,
    pub abstractions: Vec<AbstractionRecord>,
    pub decomposition_candidates: Vec<DecompositionCandidate>,
    pub residual_decisions: Vec<ResidualDecisionPacket>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ActorRecord {
    pub actor_id: String,
    pub role_summary: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct InterfaceRecord {
    pub interface_id: String,
    pub signals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PhaseRecord {
    pub phase_id: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct InvariantRecord {
    pub invariant_id: String,
    pub statement: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ContractRecord {
    pub contract_id: String,
    pub statement: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GateRecord {
    pub gate_id: String,
    pub condition: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AssertionRecord {
    pub assertion_id: String,
    pub statement: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AbstractionRecord {
    pub abstraction_id: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct DecompositionCandidate {
    pub candidate_id: String,
    pub summary: String,
}
