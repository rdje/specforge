use serde::{Deserialize, Serialize};

use crate::ir::IrStage;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AdapterTarget {
    Fsm,
    SystemVerilog,
    Verilog,
    Vhdl,
}

impl AdapterTarget {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fsm => "fsm",
            Self::SystemVerilog => "system_verilog",
            Self::Verilog => "verilog",
            Self::Vhdl => "vhdl",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AdapterStatus {
    Planned,
    Deferred,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterPlan {
    pub target: AdapterTarget,
    pub required_input_stage: IrStage,
    pub status: AdapterStatus,
    pub notes: Vec<String>,
}

pub fn default_adapter_plans() -> Vec<AdapterPlan> {
    vec![
        AdapterPlan {
            target: AdapterTarget::Fsm,
            required_input_stage: IrStage::IntentIr,
            status: AdapterStatus::Planned,
            notes: vec!["adapter target only; IntentIR remains the canonical endpoint".to_string()],
        },
        AdapterPlan {
            target: AdapterTarget::SystemVerilog,
            required_input_stage: IrStage::IntentIr,
            status: AdapterStatus::Planned,
            notes: vec!["direct RTL adapter planned after IntentIR stabilization".to_string()],
        },
        AdapterPlan {
            target: AdapterTarget::Verilog,
            required_input_stage: IrStage::IntentIr,
            status: AdapterStatus::Planned,
            notes: vec!["target-neutral lowering should support Verilog later".to_string()],
        },
        AdapterPlan {
            target: AdapterTarget::Vhdl,
            required_input_stage: IrStage::IntentIr,
            status: AdapterStatus::Planned,
            notes: vec!["target-neutral lowering should support VHDL later".to_string()],
        },
    ]
}
