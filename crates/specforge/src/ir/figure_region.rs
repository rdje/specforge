//! Typed `FigureRegion` input contract for the figure→PartialTrace
//! adapter (R16-WAVEFORM-CONTRACT-MINING.3.1 design / `.3.2`
//! implementation).
//!
//! `FigureRegion` is the typed extension projected from an enriched
//! `VisualAsset` when its timing observation carries explicit
//! tick-addressed lane structure. EvidenceIR stores the optional
//! region; raw raster/SVG bytes remain outside the typed record.
//!
//! `.3.2` refinement of the `.3.1` design: `FigureAnnotation` is
//! modelled as an enum so each variant carries exactly the fields it
//! needs (cleaner than a flat record with many `Option<…>` fields).
//! The adapter trivially maps `Delay → RelativeDelay`, `Value →
//! ValueSpan`, and `Unknown` lowers the trace confidence.
//!
//! `SPEC-TO-INTENT-ALIGNMENT.3` wires the producer into EvidenceIR and
//! grounds/mines it in SemanticIR. Artifacts without a usable timing
//! observation keep the optional field absent, preserving compatibility.

use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::ir::source::{AutomationConfidence, VisualAsset};
use crate::persisted_path::{PersistedPathOrigin, normalize_for_storage, resolve_reference};

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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_repository_image_path",
        deserialize_with = "deserialize_repository_image_path"
    )]
    pub raw_image_path: Option<PathBuf>,
    pub confidence: AutomationConfidence,
}

fn serialize_repository_image_path<S>(
    path: &Option<PathBuf>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let persisted = path
        .as_deref()
        .map(|path| normalize_for_storage(path, PersistedPathOrigin::RepositoryOwned))
        .transpose()
        .map_err(serde::ser::Error::custom)?;
    persisted.serialize(serializer)
}

fn deserialize_repository_image_path<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<PathBuf>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<PathBuf>::deserialize(deserializer)?
        .map(|path| resolve_reference(&path, PersistedPathOrigin::RepositoryOwned))
        .transpose()
        .map_err(serde::de::Error::custom)
}

impl FigureRegion {
    /// Build the typed waveform surface from one production timing-diagram observation.
    ///
    /// The VLM must supply an explicit numeric tick (a JSON integer, decimal string, or `T<n>`
    /// label) for every sample. Array order and arbitrary labels never become time authority.
    /// Only concrete HIGH/LOW values carry behavior; all other states stay `Unknown` and break
    /// spans in the downstream adapter. Free-form annotations remain `Unknown` rather than being
    /// reinterpreted as delays or values without a typed producer contract.
    pub fn from_timing_observation(
        asset: &VisualAsset,
        value: &serde_json::Value,
        confidence: AutomationConfidence,
    ) -> Option<Self> {
        let signals = value.get("signals")?.as_array()?;
        let mut lanes = BTreeMap::<String, BTreeMap<u32, LaneLevel>>::new();

        for signal in signals {
            let Some(signal_name) = signal
                .get("name")
                .and_then(serde_json::Value::as_str)
                .map(str::trim)
                .filter(|name| !name.is_empty())
            else {
                continue;
            };
            let Some(values) = signal.get("values").and_then(serde_json::Value::as_array) else {
                continue;
            };

            for sample in values {
                let Some(tick) = sample.get("cycle").and_then(parse_explicit_tick) else {
                    continue;
                };
                let Some(level) = sample
                    .get("state")
                    .and_then(serde_json::Value::as_str)
                    .map(parse_lane_level)
                else {
                    continue;
                };
                merge_sample(
                    lanes.entry(signal_name.to_string()).or_default(),
                    tick,
                    level,
                );
            }
        }

        let waveform_lanes = lanes
            .into_iter()
            .filter_map(|(signal_name, samples)| {
                (!samples.is_empty()).then(|| FigureLane {
                    signal_name,
                    samples: samples
                        .into_iter()
                        .map(|(at_tick, level)| LaneSample { at_tick, level })
                        .collect(),
                })
            })
            .collect::<Vec<_>>();
        let annotations = value
            .get("annotations")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(serde_json::Value::as_str)
            .map(str::trim)
            .filter(|text| !text.is_empty())
            .map(|text| FigureAnnotation::Unknown {
                text: text.to_string(),
                bbox: None,
            })
            .collect::<Vec<_>>();

        if waveform_lanes.is_empty() && annotations.is_empty() {
            return None;
        }

        Some(Self {
            visual_asset_id: asset.asset_id.clone(),
            bbox: None,
            annotations,
            waveform_lanes,
            tick_count: None,
            raw_image_path: asset.image_path.clone(),
            confidence,
        })
    }

    /// Retain only signal-bearing records that resolve uniquely to the document's current signal
    /// catalog, canonicalizing case back to that catalog. Ambiguous case-folded identities and
    /// VLM-only names disappear before contract mining; informational/unknown annotations remain
    /// visible so they can still lower confidence honestly.
    pub fn grounded_to(&self, known_signal_names: &HashSet<String>) -> Self {
        let mut canonical_by_fold = BTreeMap::<String, Vec<String>>::new();
        for name in known_signal_names {
            canonical_by_fold
                .entry(name.to_ascii_lowercase())
                .or_default()
                .push(name.clone());
        }
        for names in canonical_by_fold.values_mut() {
            names.sort();
            names.dedup();
        }

        let canonical = |name: &str| -> Option<String> {
            if known_signal_names.contains(name) {
                return Some(name.to_string());
            }
            let matches = canonical_by_fold.get(&name.to_ascii_lowercase())?;
            (matches.len() == 1).then(|| matches[0].clone())
        };

        let mut grounded_lanes = BTreeMap::<String, BTreeMap<u32, LaneLevel>>::new();
        for lane in &self.waveform_lanes {
            let Some(signal_name) = canonical(&lane.signal_name) else {
                continue;
            };
            let samples = grounded_lanes.entry(signal_name).or_default();
            for sample in &lane.samples {
                merge_sample(samples, sample.at_tick, sample.level.clone());
            }
        }

        let annotations = self
            .annotations
            .iter()
            .filter_map(|annotation| match annotation {
                FigureAnnotation::Delay {
                    from_signal,
                    to_signal,
                    min_cycles,
                    max_cycles,
                    text,
                    bbox,
                } => Some(FigureAnnotation::Delay {
                    from_signal: canonical(from_signal)?,
                    to_signal: canonical(to_signal)?,
                    min_cycles: *min_cycles,
                    max_cycles: *max_cycles,
                    text: text.clone(),
                    bbox: *bbox,
                }),
                FigureAnnotation::Value {
                    signal,
                    value,
                    from_tick,
                    to_tick,
                    text,
                    bbox,
                } => Some(FigureAnnotation::Value {
                    signal: canonical(signal)?,
                    value: value.clone(),
                    from_tick: *from_tick,
                    to_tick: *to_tick,
                    text: text.clone(),
                    bbox: *bbox,
                }),
                FigureAnnotation::Label { .. } | FigureAnnotation::Unknown { .. } => {
                    Some(annotation.clone())
                }
            })
            .collect();

        Self {
            visual_asset_id: self.visual_asset_id.clone(),
            bbox: self.bbox,
            annotations,
            waveform_lanes: grounded_lanes
                .into_iter()
                .map(|(signal_name, samples)| FigureLane {
                    signal_name,
                    samples: samples
                        .into_iter()
                        .map(|(at_tick, level)| LaneSample { at_tick, level })
                        .collect(),
                })
                .collect(),
            tick_count: self.tick_count,
            raw_image_path: self.raw_image_path.clone(),
            confidence: self.confidence,
        }
    }

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

fn parse_explicit_tick(value: &serde_json::Value) -> Option<u32> {
    if let Some(tick) = value.as_u64() {
        return u32::try_from(tick).ok();
    }
    let label = value.as_str()?.trim();
    label.parse::<u32>().ok().or_else(|| {
        label
            .strip_prefix('T')
            .or_else(|| label.strip_prefix('t'))?
            .parse::<u32>()
            .ok()
    })
}

fn parse_lane_level(value: &str) -> LaneLevel {
    match value.trim().to_ascii_uppercase().as_str() {
        "HIGH" | "1" => LaneLevel::High,
        "LOW" | "0" => LaneLevel::Low,
        _ => LaneLevel::Unknown,
    }
}

fn merge_sample(samples: &mut BTreeMap<u32, LaneLevel>, tick: u32, level: LaneLevel) {
    samples
        .entry(tick)
        .and_modify(|existing| {
            if *existing != level {
                *existing = LaneLevel::Unknown;
            }
        })
        .or_insert(level);
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
        let workspace = crate::project_data::tempdir().unwrap();
        let raw_image_path = workspace.path().join("fig.png");
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
            raw_image_path: Some(raw_image_path.clone()),
            confidence: AutomationConfidence::Low,
        };
        let json = serde_json::to_string(&original).unwrap();
        assert!(
            !json.contains(
                crate::project_data::repository_root()
                    .unwrap()
                    .to_string_lossy()
                    .as_ref()
            )
        );
        assert!(json.contains(".project-data/tmp/"));
        let restored: FigureRegion = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, original);
        assert_eq!(restored.raw_image_path, Some(raw_image_path));
    }

    #[test]
    fn figure_region_rejects_unlabeled_external_image_path() {
        let json = serde_json::json!({
            "visual_asset_id": "fig:external",
            "annotations": [],
            "waveform_lanes": [],
            "raw_image_path": "/tmp/fig.png",
            "confidence": "low"
        });

        let error = serde_json::from_value::<FigureRegion>(json).unwrap_err();
        assert!(error.to_string().contains("/tmp/fig.png"));
    }

    fn timing_asset() -> VisualAsset {
        VisualAsset {
            asset_id: "picture_0005".to_string(),
            asset_kind: crate::ir::source::VisualAssetKind::Figure,
            page_id: Some("page_0004".to_string()),
            image_path: Some(PathBuf::from(
                "generated/source_ir/i2s/normalized/assets/picture-0005.png",
            )),
            caption_text: Some(
                "Figure 1. Simple system configurations and basic interface timing".to_string(),
            ),
            caption_source_path: None,
            source_ref: Some("#/pictures/4".to_string()),
            placeholder_text: None,
            note: None,
            diagram_kind: crate::ir::source::DiagramKind::TimingDiagram,
        }
    }

    #[test]
    fn timing_observation_requires_explicit_ticks_and_coalesces_conflicts_honestly() {
        let value = serde_json::json!({
            "signals": [
                {
                    "name": "WS",
                    "values": [
                        {"cycle": "T2", "state": "LOW"},
                        {"cycle": 0, "state": "LOW"},
                        {"cycle": "2", "state": "HIGH"},
                        {"cycle": "address phase", "state": "LOW"}
                    ]
                },
                {"name": "", "values": [{"cycle": 0, "state": "HIGH"}]}
            ],
            "annotations": ["word-select boundary is visible"]
        });

        let region = FigureRegion::from_timing_observation(
            &timing_asset(),
            &value,
            AutomationConfidence::High,
        )
        .expect("the explicit samples should produce a region");
        assert_eq!(region.waveform_lanes.len(), 1);
        assert_eq!(region.waveform_lanes[0].signal_name, "WS");
        assert_eq!(
            region.waveform_lanes[0].samples,
            vec![
                LaneSample {
                    at_tick: 0,
                    level: LaneLevel::Low,
                },
                LaneSample {
                    at_tick: 2,
                    level: LaneLevel::Unknown,
                },
            ]
        );
        assert_eq!(region.inferred_ticks(), 3);
        assert!(matches!(
            region.annotations.as_slice(),
            [FigureAnnotation::Unknown { .. }]
        ));
    }

    #[test]
    fn timing_observation_without_explicit_samples_or_annotations_is_unavailable() {
        let value = serde_json::json!({
            "signals": [{
                "name": "WS",
                "values": [{"cycle": "left channel", "state": "LOW"}]
            }],
            "annotations": []
        });
        assert!(
            FigureRegion::from_timing_observation(
                &timing_asset(),
                &value,
                AutomationConfidence::High,
            )
            .is_none()
        );
    }

    #[test]
    fn grounding_canonicalizes_known_lanes_and_drops_vlm_only_names() {
        let value = serde_json::json!({
            "signals": [
                {"name": "ws", "values": [{"cycle": 0, "state": "LOW"}]},
                {"name": "INVENTED", "values": [{"cycle": 0, "state": "HIGH"}]}
            ],
            "annotations": []
        });
        let region = FigureRegion::from_timing_observation(
            &timing_asset(),
            &value,
            AutomationConfidence::High,
        )
        .unwrap();
        let grounded = region.grounded_to(&HashSet::from(["WS".to_string()]));
        assert_eq!(grounded.waveform_lanes.len(), 1);
        assert_eq!(grounded.waveform_lanes[0].signal_name, "WS");
    }
}
