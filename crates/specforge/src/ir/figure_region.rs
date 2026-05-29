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

#[cfg(test)]
mod tests {
    use super::*;

    fn lane(signal: &str, ticks: &[u32]) -> FigureLane {
        FigureLane {
            signal_name: signal.into(),
            samples: ticks
                .iter()
                .map(|&at_tick| LaneSample {
                    at_tick,
                    level: LaneLevel::High,
                })
                .collect(),
        }
    }

    fn value_annotation(to_tick: u32) -> FigureAnnotation {
        FigureAnnotation::Value {
            signal: "Q".into(),
            value: "1".into(),
            from_tick: 0,
            to_tick,
            text: "Q held".into(),
            bbox: None,
        }
    }

    fn region(
        tick_count: Option<u32>,
        waveform_lanes: Vec<FigureLane>,
        annotations: Vec<FigureAnnotation>,
    ) -> FigureRegion {
        FigureRegion {
            visual_asset_id: "fig:t".into(),
            bbox: None,
            annotations,
            waveform_lanes,
            tick_count,
            raw_image_path: None,
            confidence: AutomationConfidence::Medium,
        }
    }

    #[test]
    fn inferred_ticks_prefers_explicit_tick_count() {
        // Explicit tick_count wins even when lanes / Value annotations
        // would derive a different (larger) number.
        let r = region(
            Some(7),
            vec![lane("clk", &[0, 2])],
            vec![value_annotation(10)],
        );
        assert_eq!(r.inferred_ticks(), 7);
    }

    #[test]
    fn inferred_ticks_from_lane_samples_max_plus_one() {
        let r = region(None, vec![lane("clk", &[0, 1, 3])], vec![]);
        assert_eq!(r.inferred_ticks(), 4);
    }

    #[test]
    fn inferred_ticks_from_value_annotation_max_plus_one() {
        let r = region(None, vec![], vec![value_annotation(5)]);
        assert_eq!(r.inferred_ticks(), 6);
    }

    #[test]
    fn inferred_ticks_uses_max_of_lanes_and_values() {
        // Value dominates: lane max 3, Value to_tick 6 -> max(3,6)+1 = 7.
        let r = region(None, vec![lane("clk", &[3])], vec![value_annotation(6)]);
        assert_eq!(r.inferred_ticks(), 7);
        // Lane dominates: lane max 8, Value to_tick 6 -> max(8,6)+1 = 9.
        let r2 = region(None, vec![lane("clk", &[8])], vec![value_annotation(6)]);
        assert_eq!(r2.inferred_ticks(), 9);
    }

    #[test]
    fn inferred_ticks_ignores_delay_label_unknown_annotations() {
        // Only Value annotations (and lane samples) drive tick inference.
        // Delay min/max_cycles, Label, and Unknown must contribute nothing.
        let r = region(
            None,
            vec![],
            vec![
                FigureAnnotation::Delay {
                    from_signal: "A".into(),
                    to_signal: "B".into(),
                    min_cycles: Some(99),
                    max_cycles: Some(99),
                    text: "≥ 99 cycles".into(),
                    bbox: None,
                },
                FigureAnnotation::Label {
                    text: "addr phase".into(),
                    bbox: None,
                },
                FigureAnnotation::Unknown {
                    text: "??".into(),
                    bbox: None,
                },
            ],
        );
        assert_eq!(r.inferred_ticks(), 0);
    }

    #[test]
    fn inferred_ticks_empty_region_is_zero() {
        let r = region(None, vec![], vec![]);
        assert_eq!(r.inferred_ticks(), 0);
    }

    #[test]
    fn inferred_ticks_saturates_instead_of_overflowing() {
        // at_tick = u32::MAX -> saturating_add(1) must not panic/overflow.
        let r = region(None, vec![lane("clk", &[u32::MAX])], vec![]);
        assert_eq!(r.inferred_ticks(), u32::MAX);
    }

    #[test]
    fn lane_level_serializes_with_kind_content_tagging() {
        // The upstream pipeline must emit this exact tagged shape.
        assert_eq!(
            serde_json::to_value(LaneLevel::High).unwrap(),
            serde_json::json!({ "kind": "high" })
        );
        assert_eq!(
            serde_json::to_value(LaneLevel::Bus("addr".into())).unwrap(),
            serde_json::json!({ "kind": "bus", "value": "addr" })
        );
    }

    #[test]
    fn optional_fields_are_skipped_when_none() {
        // The skip_serializing_if discipline keeps the wire format lean
        // and preserves zero artifact churn until extraction populates it.
        let r = region(
            None,
            vec![],
            vec![FigureAnnotation::Label {
                text: "x".into(),
                bbox: None,
            }],
        );
        let json = serde_json::to_value(&r).unwrap();
        assert!(json.get("bbox").is_none());
        assert!(json.get("tick_count").is_none());
        assert!(json.get("raw_image_path").is_none());
        // The Label annotation's None bbox is skipped too.
        let ann = &json["annotations"][0];
        assert!(ann.get("bbox").is_none());
        assert_eq!(ann["kind"], "label");
    }

    #[test]
    fn figure_region_json_round_trips() {
        // A fully-populated region (every annotation variant, every
        // LaneLevel variant) must survive serialize -> deserialize intact,
        // locking the contract the upstream PDF pipeline must produce.
        let original = FigureRegion {
            visual_asset_id: "fig:full".into(),
            bbox: Some(BoundingBox {
                x: 1.0,
                y: 2.0,
                w: 3.0,
                h: 4.0,
            }),
            annotations: vec![
                FigureAnnotation::Delay {
                    from_signal: "REQ".into(),
                    to_signal: "ACK".into(),
                    min_cycles: Some(1),
                    max_cycles: Some(3),
                    text: "1..3 cycles".into(),
                    bbox: Some(BoundingBox {
                        x: 0.0,
                        y: 0.0,
                        w: 1.0,
                        h: 1.0,
                    }),
                },
                FigureAnnotation::Value {
                    signal: "DATA".into(),
                    value: "0xAB".into(),
                    from_tick: 2,
                    to_tick: 5,
                    text: "DATA stable".into(),
                    bbox: None,
                },
                FigureAnnotation::Label {
                    text: "T0".into(),
                    bbox: None,
                },
                FigureAnnotation::Unknown {
                    text: "glyph".into(),
                    bbox: None,
                },
            ],
            waveform_lanes: vec![FigureLane {
                signal_name: "BUS".into(),
                samples: vec![
                    LaneSample {
                        at_tick: 0,
                        level: LaneLevel::Low,
                    },
                    LaneSample {
                        at_tick: 1,
                        level: LaneLevel::High,
                    },
                    LaneSample {
                        at_tick: 2,
                        level: LaneLevel::Unknown,
                    },
                    LaneSample {
                        at_tick: 3,
                        level: LaneLevel::Bus("0x4".into()),
                    },
                ],
            }],
            tick_count: Some(4),
            raw_image_path: Some(PathBuf::from("/tmp/fig.png")),
            confidence: AutomationConfidence::Low,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: FigureRegion = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, original);
    }
}
