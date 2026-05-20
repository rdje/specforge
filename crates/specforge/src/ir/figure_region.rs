//! Typed `FigureRegion` input contract for the figure→PartialTrace
//! adapter (R16-WAVEFORM-CONTRACT-MINING.3.1 design / `.3.2`
//! implementation).
//!
//! `FigureRegion` is the typed *extension* an upstream PDF pipeline
//! produces when it classifies a `VisualAsset` (already present in
//! `crates/specforge/src/ir/source.rs`) as a timing diagram and
//! recovers lane / annotation structure. SpecForge consumes these
//! typed records; raw raster/SVG bytes stay out-of-tree.
//!
//! `.3.2` refinement of the `.3.1` design: `FigureAnnotation` is
//! modelled as an enum so each variant carries exactly the fields it
//! needs (cleaner than a flat record with many `Option<…>` fields).
//! The adapter trivially maps `Delay → RelativeDelay`, `Value →
//! ValueSpan`, and `Unknown` lowers the trace confidence.
//!
//! No producer wiring; `SemanticIr`/`IntentIr` schemas unchanged ⇒
//! zero artifact churn until an upstream pipeline produces typed
//! `FigureRegion`s.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::ir::source::AutomationConfidence;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

/// Per-tick logic level on a recovered waveform lane.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "kind", content = "value")]
pub enum LaneLevel {
    High,
    Low,
    Unknown,
    Bus(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LaneSample {
    pub at_tick: u32,
    pub level: LaneLevel,
}

/// A recovered waveform lane: a signal + its per-tick samples.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FigureLane {
    pub signal_name: String,
    #[serde(default)]
    pub samples: Vec<LaneSample>,
}

/// A typed annotation recovered from the figure. Enum-shaped so each
/// variant carries exactly the fields the adapter needs to lower it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum FigureAnnotation {
    /// e.g. "≥ 2 cycles" between two lane events.
    Delay {
        from_signal: String,
        to_signal: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        min_cycles: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_cycles: Option<u32>,
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        bbox: Option<BoundingBox>,
    },
    /// e.g. "Q = 1 from tick 1 to tick 3".
    Value {
        signal: String,
        value: String,
        from_tick: u32,
        to_tick: u32,
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        bbox: Option<BoundingBox>,
    },
    /// A free-form lane / signal label — informational only.
    Label {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        bbox: Option<BoundingBox>,
    },
    /// An annotation upstream could not classify. The adapter does not
    /// invent semantics from `Unknown`; it lowers
    /// `PartialTrace.confidence` instead (honest dormancy).
    Unknown {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        bbox: Option<BoundingBox>,
    },
}

/// The typed figure record an upstream PDF pipeline produces when it
/// classifies a `VisualAsset` as a timing diagram + recovers lane /
/// annotation structure. References the upstream `VisualAsset` by id
/// (additive — `VisualAsset` itself is unchanged).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FigureRegion {
    pub visual_asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<BoundingBox>,
    #[serde(default)]
    pub annotations: Vec<FigureAnnotation>,
    #[serde(default)]
    pub waveform_lanes: Vec<FigureLane>,
    /// `tick_count` = number of ticks the diagram represents (when
    /// known; otherwise inferred from the maximum `at_tick`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_image_path: Option<PathBuf>,
    pub confidence: AutomationConfidence,
}

impl FigureRegion {
    /// Inferred tick count: explicit `tick_count` if present, else the
    /// max `at_tick + 1` across all lanes / Value annotations, else 0.
    pub fn inferred_ticks(&self) -> u32 {
        if let Some(t) = self.tick_count {
            return t;
        }
        let lane_max = self
            .waveform_lanes
            .iter()
            .flat_map(|l| l.samples.iter().map(|s| s.at_tick))
            .max();
        let value_max = self
            .annotations
            .iter()
            .filter_map(|a| match a {
                FigureAnnotation::Value { to_tick, .. } => Some(*to_tick),
                _ => None,
            })
            .max();
        match (lane_max, value_max) {
            (Some(a), Some(b)) => a.max(b).saturating_add(1),
            (Some(a), None) | (None, Some(a)) => a.saturating_add(1),
            (None, None) => 0,
        }
    }
}
