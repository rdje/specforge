use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::evidence::{EvidenceIr, StatementClass, VisualEvidenceRole};
use crate::ir::source::{
    AutomationConfidence, CandidateInterpretation, ResidualDecisionPacket, document_key,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticIr {
    pub schema_version: u32,
    pub stage: IrStage,
    pub evidence_ir_path: PathBuf,
    pub artifact_layout: SemanticArtifactLayout,
    pub document_identity: SemanticDocumentIdentity,
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

impl SemanticIr {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(AppError::MissingPath(path.to_path_buf()));
        }

        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn build(evidence_ir_path: &Path, artifact_base_root: &Path) -> Result<Self> {
        let evidence_ir_path = canonicalize_existing_path(evidence_ir_path)?;
        let evidence_ir = EvidenceIr::load_from_path(&evidence_ir_path)?;

        if !matches!(evidence_ir.stage, IrStage::EvidenceIr) {
            return Err(AppError::InvalidStageArtifact(format!(
                "artifact at {} must be an EvidenceIR document before building SemanticIR",
                evidence_ir_path.display()
            )));
        }

        let artifact_root = artifact_base_root.join(&evidence_ir.document_identity.document_key);
        let semantic_ir_path = artifact_root.join("semantic_ir.json");
        let artifact_layout = SemanticArtifactLayout {
            artifact_root,
            semantic_ir_path,
        };
        let document_identity = SemanticDocumentIdentity {
            document_key: evidence_ir.document_identity.document_key.clone(),
            display_name: evidence_ir.document_identity.display_name.clone(),
        };

        let context = SemanticContext::from_evidence_ir(&evidence_ir);
        let interfaces = build_interfaces(&context);
        let actor_build = build_actors(&context, &interfaces);
        let phases = build_phases(&context);
        let interface_ids_by_signal = interface_ids_by_signal(&interfaces);
        let invariants = build_invariants(&context, &interface_ids_by_signal);
        let contracts = build_contracts(&context, &actor_build.actor_id_by_term);
        let gates = build_gates(&context, &interface_ids_by_signal);
        let assertions = build_assertions(&context);
        let abstractions = build_abstractions(&context);
        let decomposition_candidates = build_decomposition_candidates(&context);
        let residual_decisions =
            build_residual_decisions(&context, &interfaces, actor_build.explicit_actor_count);

        Ok(Self {
            schema_version: 1,
            stage: IrStage::SemanticIr,
            evidence_ir_path,
            artifact_layout,
            document_identity,
            actors: actor_build.actors,
            interfaces,
            phases,
            invariants,
            contracts,
            gates,
            assertions,
            abstractions,
            decomposition_candidates,
            residual_decisions,
        })
    }

    pub fn to_pretty_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn write_to_disk(&self) -> Result<()> {
        fs::create_dir_all(&self.artifact_layout.artifact_root)?;
        fs::write(
            &self.artifact_layout.semantic_ir_path,
            self.to_pretty_json()?,
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticArtifactLayout {
    pub artifact_root: PathBuf,
    pub semantic_ir_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticDocumentIdentity {
    pub document_key: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActorRecord {
    pub actor_id: String,
    pub role_summary: String,
    pub supporting_statement_ids: Vec<String>,
    pub supporting_section_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InterfaceRecord {
    pub interface_id: String,
    pub signals: Vec<String>,
    pub supporting_statement_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PhaseRecord {
    pub phase_id: String,
    pub summary: String,
    pub supporting_statement_ids: Vec<String>,
    pub supporting_section_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InvariantRecord {
    pub invariant_id: String,
    pub statement: String,
    pub supporting_statement_ids: Vec<String>,
    pub related_interface_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContractRecord {
    pub contract_id: String,
    pub statement: String,
    pub supporting_statement_ids: Vec<String>,
    pub actor_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GateRecord {
    pub gate_id: String,
    pub condition: String,
    pub supporting_statement_ids: Vec<String>,
    pub related_interface_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssertionRecord {
    pub assertion_id: String,
    pub statement: String,
    pub supporting_statement_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AbstractionRecord {
    pub abstraction_id: String,
    pub description: String,
    pub supporting_statement_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DecompositionCandidate {
    pub candidate_id: String,
    pub summary: String,
    pub supporting_statement_ids: Vec<String>,
    pub supporting_section_ids: Vec<String>,
}

#[derive(Debug, Clone)]
struct SemanticContext {
    statements: Vec<StatementContext>,
    section_anchors: Vec<SemanticSectionContext>,
    visual_roles_by_id: HashMap<String, VisualEvidenceRole>,
}

impl SemanticContext {
    fn from_evidence_ir(evidence_ir: &EvidenceIr) -> Self {
        let spans_by_id: HashMap<String, (Option<u32>, Option<u32>)> = evidence_ir
            .evidence_spans
            .iter()
            .map(|span| (span.span_id.clone(), (span.line_start, span.line_end)))
            .collect();

        let mut section_statement_ids: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let statements = evidence_ir
            .extracted_statements
            .iter()
            .map(|statement| {
                let section_ids = section_ids_for_statement(
                    statement.evidence_span_ids.as_slice(),
                    &spans_by_id,
                    &evidence_ir.section_anchors,
                );
                for section_id in &section_ids {
                    section_statement_ids
                        .entry(section_id.clone())
                        .or_default()
                        .push(statement.statement_id.clone());
                }

                StatementContext {
                    statement_id: statement.statement_id.clone(),
                    class: statement.class,
                    text: statement.text.clone(),
                    related_visual_evidence_ids: statement.related_visual_evidence_ids.clone(),
                    section_ids,
                    signals: extract_signal_tokens(&statement.text),
                }
            })
            .collect();

        let section_anchors = evidence_ir
            .section_anchors
            .iter()
            .map(|anchor| SemanticSectionContext {
                section_id: anchor.section_id.clone(),
                title: anchor.title.clone(),
                supporting_statement_ids: section_statement_ids
                    .get(&anchor.section_id)
                    .cloned()
                    .unwrap_or_default(),
            })
            .collect();

        let visual_roles_by_id = evidence_ir
            .visual_evidence
            .iter()
            .map(|visual| (visual.evidence_id.clone(), visual.role))
            .collect();

        Self {
            statements,
            section_anchors,
            visual_roles_by_id,
        }
    }
}

#[derive(Debug, Clone)]
struct StatementContext {
    statement_id: String,
    class: StatementClass,
    text: String,
    related_visual_evidence_ids: Vec<String>,
    section_ids: Vec<String>,
    signals: Vec<String>,
}

#[derive(Debug, Clone)]
struct SemanticSectionContext {
    section_id: String,
    title: String,
    supporting_statement_ids: Vec<String>,
}

#[derive(Debug, Clone)]
struct ActorBuildResult {
    actors: Vec<ActorRecord>,
    actor_id_by_term: HashMap<String, String>,
    explicit_actor_count: usize,
}

#[derive(Debug, Clone)]
struct ActorAccumulator {
    role_summary: String,
    supporting_statement_ids: BTreeSet<String>,
    supporting_section_ids: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct InterfaceAccumulator {
    signals: BTreeSet<String>,
    supporting_statement_ids: BTreeSet<String>,
}

fn build_interfaces(context: &SemanticContext) -> Vec<InterfaceRecord> {
    let mut accumulators: BTreeMap<String, InterfaceAccumulator> = BTreeMap::new();

    for statement in &context.statements {
        if !should_emit_interface_candidate(statement.signals.as_slice()) {
            continue;
        }

        let key = statement.signals.join("__");
        let entry = accumulators
            .entry(key)
            .or_insert_with(|| InterfaceAccumulator {
                signals: statement.signals.iter().cloned().collect(),
                supporting_statement_ids: BTreeSet::new(),
            });
        entry
            .supporting_statement_ids
            .insert(statement.statement_id.clone());
    }

    accumulators
        .into_values()
        .map(|entry| {
            let signals: Vec<String> = entry.signals.into_iter().collect();
            InterfaceRecord {
                interface_id: format!("interface_{}", document_key(&signals.join("_"))),
                signals,
                supporting_statement_ids: entry.supporting_statement_ids.into_iter().collect(),
            }
        })
        .collect()
}

fn build_actors(context: &SemanticContext, interfaces: &[InterfaceRecord]) -> ActorBuildResult {
    const ACTOR_TERMS: &[&str] = &[
        "transmitter",
        "receiver",
        "sender",
        "requester",
        "responder",
        "producer",
        "consumer",
        "controller",
        "manager",
        "device",
        "host",
        "client",
        "server",
        "initiator",
        "target",
        "source",
        "sink",
        "agent",
        "arbiter",
        "scheduler",
        "decoder",
        "encoder",
        "channel",
        "state machine",
    ];

    let mut accumulators: BTreeMap<String, ActorAccumulator> = BTreeMap::new();
    let mut actor_id_by_term = HashMap::new();

    for statement in &context.statements {
        let lowered_text = statement.text.to_ascii_lowercase();
        for term in ACTOR_TERMS {
            if !contains_phrase(&lowered_text, term) {
                continue;
            }

            let actor_id = format!("actor_{}", document_key(term));
            actor_id_by_term.insert((*term).to_string(), actor_id.clone());
            let entry = accumulators
                .entry(actor_id)
                .or_insert_with(|| ActorAccumulator {
                    role_summary: format!("semantic role inferred around `{term}` evidence"),
                    supporting_statement_ids: BTreeSet::new(),
                    supporting_section_ids: BTreeSet::new(),
                });
            entry
                .supporting_statement_ids
                .insert(statement.statement_id.clone());
            entry
                .supporting_section_ids
                .extend(statement.section_ids.iter().cloned());
        }
    }

    let explicit_actor_count = accumulators.len();
    if accumulators.is_empty() {
        for interface in interfaces {
            let actor_id = format!(
                "actor_{}_channel",
                document_key(&interface.signals.join("_"))
            );
            let supporting_section_ids = statement_ids_to_section_ids(
                context,
                interface.supporting_statement_ids.as_slice(),
            );
            accumulators.insert(
                actor_id,
                ActorAccumulator {
                    role_summary: format!(
                        "semantic channel inferred from grouped interface signals: {}",
                        interface.signals.join(", ")
                    ),
                    supporting_statement_ids: interface
                        .supporting_statement_ids
                        .iter()
                        .cloned()
                        .collect(),
                    supporting_section_ids: supporting_section_ids.into_iter().collect(),
                },
            );
        }
    }

    let actors = accumulators
        .into_iter()
        .map(|(actor_id, entry)| ActorRecord {
            actor_id,
            role_summary: entry.role_summary,
            supporting_statement_ids: entry.supporting_statement_ids.into_iter().collect(),
            supporting_section_ids: entry.supporting_section_ids.into_iter().collect(),
        })
        .collect();

    ActorBuildResult {
        actors,
        actor_id_by_term,
        explicit_actor_count,
    }
}

fn build_phases(context: &SemanticContext) -> Vec<PhaseRecord> {
    context
        .section_anchors
        .iter()
        .filter(|section| {
            !section.supporting_statement_ids.is_empty()
                && (phase_like_title(&section.title)
                    || section
                        .supporting_statement_ids
                        .iter()
                        .filter_map(|statement_id| statement_by_id(context, statement_id))
                        .any(|statement| sequencing_language(&statement.text)))
        })
        .map(|section| PhaseRecord {
            phase_id: format!("phase_{}", document_key(&section.title)),
            summary: format!("semantic phase derived from section `{}`", section.title),
            supporting_statement_ids: section.supporting_statement_ids.clone(),
            supporting_section_ids: vec![section.section_id.clone()],
        })
        .collect()
}

fn build_invariants(
    context: &SemanticContext,
    interface_ids_by_signal: &HashMap<String, BTreeSet<String>>,
) -> Vec<InvariantRecord> {
    let mut invariants = Vec::new();
    let mut seen = BTreeSet::new();

    for statement in &context.statements {
        if !is_invariant_like(statement, context) {
            continue;
        }

        let dedupe_key = normalize_text_key(&statement.text);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        invariants.push(InvariantRecord {
            invariant_id: format!("invariant_{}", document_key(&dedupe_key)),
            statement: statement.text.clone(),
            supporting_statement_ids: vec![statement.statement_id.clone()],
            related_interface_ids: related_interface_ids(
                statement.signals.as_slice(),
                interface_ids_by_signal,
            ),
        });
    }

    invariants
}

fn build_contracts(
    context: &SemanticContext,
    actor_id_by_term: &HashMap<String, String>,
) -> Vec<ContractRecord> {
    let mut contracts = Vec::new();
    let mut seen = BTreeSet::new();

    for statement in &context.statements {
        let lowered_text = statement.text.to_ascii_lowercase();
        if !contains_any_phrase(
            &lowered_text,
            &[
                "must",
                "shall",
                "required",
                "responsible",
                "may",
                "allowed",
                "forbidden",
                "prohibited",
            ],
        ) {
            continue;
        }

        let actor_ids = actor_ids_for_text(&lowered_text, actor_id_by_term);
        if actor_ids.is_empty() {
            continue;
        }

        let dedupe_key = normalize_text_key(&statement.text);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        contracts.push(ContractRecord {
            contract_id: format!("contract_{}", document_key(&dedupe_key)),
            statement: statement.text.clone(),
            supporting_statement_ids: vec![statement.statement_id.clone()],
            actor_ids,
        });
    }

    contracts
}

fn build_gates(
    context: &SemanticContext,
    interface_ids_by_signal: &HashMap<String, BTreeSet<String>>,
) -> Vec<GateRecord> {
    let mut gates = Vec::new();
    let mut seen = BTreeSet::new();

    for statement in &context.statements {
        let lowered_text = statement.text.to_ascii_lowercase();
        if !contains_any_phrase(
            &lowered_text,
            &[
                "if",
                "when",
                "unless",
                "only when",
                "while",
                "after",
                "before",
                "until",
            ],
        ) {
            continue;
        }

        let dedupe_key = normalize_text_key(&statement.text);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        gates.push(GateRecord {
            gate_id: format!("gate_{}", document_key(&dedupe_key)),
            condition: statement.text.clone(),
            supporting_statement_ids: vec![statement.statement_id.clone()],
            related_interface_ids: related_interface_ids(
                statement.signals.as_slice(),
                interface_ids_by_signal,
            ),
        });
    }

    gates
}

fn build_assertions(context: &SemanticContext) -> Vec<AssertionRecord> {
    let mut assertions = Vec::new();
    let mut seen = BTreeSet::new();

    for statement in &context.statements {
        let lowered_text = statement.text.to_ascii_lowercase();
        if !contains_any_phrase(
            &lowered_text,
            &[
                "assert",
                "assertion",
                "must not",
                "shall not",
                "must never",
                "shall never",
                "illegal",
                "forbidden",
            ],
        ) {
            continue;
        }

        let dedupe_key = normalize_text_key(&statement.text);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        assertions.push(AssertionRecord {
            assertion_id: format!("assertion_{}", document_key(&dedupe_key)),
            statement: statement.text.clone(),
            supporting_statement_ids: vec![statement.statement_id.clone()],
        });
    }

    assertions
}

fn build_abstractions(context: &SemanticContext) -> Vec<AbstractionRecord> {
    context
        .statements
        .iter()
        .filter(|statement| matches!(statement.class, StatementClass::ExplicitAbstraction))
        .map(|statement| AbstractionRecord {
            abstraction_id: format!("abstraction_{}", document_key(&statement.statement_id)),
            description: statement.text.clone(),
            supporting_statement_ids: vec![statement.statement_id.clone()],
        })
        .collect()
}

fn build_decomposition_candidates(context: &SemanticContext) -> Vec<DecompositionCandidate> {
    context
        .section_anchors
        .iter()
        .filter(|section| {
            section.supporting_statement_ids.len() >= 2 || decomposition_like_title(&section.title)
        })
        .map(|section| DecompositionCandidate {
            candidate_id: format!("candidate_{}", document_key(&section.title)),
            summary: format!("semantic cluster around section `{}`", section.title),
            supporting_statement_ids: section.supporting_statement_ids.clone(),
            supporting_section_ids: vec![section.section_id.clone()],
        })
        .collect()
}

fn build_residual_decisions(
    context: &SemanticContext,
    interfaces: &[InterfaceRecord],
    explicit_actor_count: usize,
) -> Vec<ResidualDecisionPacket> {
    let mut packets = Vec::new();

    if explicit_actor_count == 0 && !interfaces.is_empty() {
        packets.push(ResidualDecisionPacket {
            packet_id: "semantic_actor_boundary_inference".to_string(),
            question: "Which actor boundary should own the inferred interface semantics?".to_string(),
            why_unresolved: "The first-pass SemanticIR builder inferred interface-level structure from evidence statements, but the evidence does not name stable endpoint actors explicitly.".to_string(),
            automation_confidence: AutomationConfidence::Medium,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "channel_actor".to_string(),
                    description: "Represent the grouped signals as a channel-like actor owned by the interface semantics.".to_string(),
                    downstream_impact: "SemanticIR remains conservative and backend-neutral, but later IntentIR stages may need endpoint decomposition.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "endpoint_pair".to_string(),
                    description: "Split the semantics into two endpoint actors that exchange the inferred interface signals.".to_string(),
                    downstream_impact: "Later stages gain clearer endpoint responsibilities, but actor invention risk is higher without explicit textual support.".to_string(),
                },
            ],
        });
    }

    let ambiguous_visual_ids: Vec<String> = context
        .statements
        .iter()
        .flat_map(|statement| statement.related_visual_evidence_ids.iter())
        .filter(|visual_id| {
            context
                .visual_roles_by_id
                .get(*visual_id)
                .is_some_and(|role| {
                    matches!(
                        role,
                        VisualEvidenceRole::Ambiguous | VisualEvidenceRole::Unknown
                    )
                })
        })
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if !ambiguous_visual_ids.is_empty() {
        packets.push(ResidualDecisionPacket {
            packet_id: "semantic_ambiguous_visual_grounding".to_string(),
            question: "Do the ambiguous visual artifacts carry normative semantics that must be lifted into SemanticIR?".to_string(),
            why_unresolved: format!(
                "EvidenceIR links the current semantic slice to ambiguous visual evidence items ({}) whose role is not safely classifiable as purely illustrative.",
                ambiguous_visual_ids.join(", ")
            ),
            automation_confidence: AutomationConfidence::Low,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "normative_visual".to_string(),
                    description: "Treat the ambiguous visual evidence as normatively relevant and lift additional semantic structure from it.".to_string(),
                    downstream_impact: "Later stages may need richer figure parsing, OCR, or visual-sequence extraction before IntentIR is complete.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "illustrative_visual".to_string(),
                    description: "Treat the ambiguous visual evidence as explanatory context only and rely on the current text-derived semantics.".to_string(),
                    downstream_impact: "The pipeline remains deterministic now, but there is a risk of missing figure-only constraints or sequencing information.".to_string(),
                },
            ],
        });
    }

    let overlapping_signals = overlapping_interface_signals(interfaces);
    if !overlapping_signals.is_empty() {
        packets.push(ResidualDecisionPacket {
            packet_id: "semantic_interface_grouping".to_string(),
            question: "Should overlapping signal groups be merged into one interface or kept as coupled interfaces?".to_string(),
            why_unresolved: format!(
                "The first-pass interface extractor observed overlapping signal participation for {} across multiple grouped statements.",
                overlapping_signals.join(", ")
            ),
            automation_confidence: AutomationConfidence::Medium,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "single_bus_interface".to_string(),
                    description: "Merge the overlapping signals into a broader bus/interface abstraction.".to_string(),
                    downstream_impact: "SemanticIR becomes simpler, but protocol subchannels may be flattened too early.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "multiple_coupled_interfaces".to_string(),
                    description: "Keep the overlapping signals in multiple related interface records.".to_string(),
                    downstream_impact: "SemanticIR preserves local structure, but later IntentIR stages must model coupling explicitly.".to_string(),
                },
            ],
        });
    }

    packets
}

fn section_ids_for_statement(
    evidence_span_ids: &[String],
    spans_by_id: &HashMap<String, (Option<u32>, Option<u32>)>,
    section_anchors: &[crate::ir::evidence::SectionAnchor],
) -> Vec<String> {
    let mut section_ids = BTreeSet::new();

    for span_id in evidence_span_ids {
        let Some((line_start, line_end)) = spans_by_id.get(span_id) else {
            continue;
        };

        let resolved_line_start = line_start.or(*line_end);
        let resolved_line_end = line_end.or(resolved_line_start);
        let Some(line_start) = resolved_line_start else {
            continue;
        };
        let Some(line_end) = resolved_line_end else {
            continue;
        };

        for anchor in section_anchors {
            let Some(anchor_start) = anchor.line_start else {
                continue;
            };
            let Some(anchor_end) = anchor.line_end else {
                continue;
            };

            if line_start <= anchor_end && line_end >= anchor_start {
                section_ids.insert(anchor.section_id.clone());
            }
        }
    }

    section_ids.into_iter().collect()
}

fn extract_signal_tokens(text: &str) -> Vec<String> {
    let mut signals = BTreeSet::new();
    let mut current = String::new();

    for character in text.chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            current.push(character);
        } else {
            maybe_add_signal_token(&mut signals, &current);
            current.clear();
        }
    }
    maybe_add_signal_token(&mut signals, &current);

    signals.into_iter().collect()
}

fn maybe_add_signal_token(signals: &mut BTreeSet<String>, token: &str) {
    if !looks_like_signal_token(token) {
        return;
    }

    let upper_token = token.to_ascii_uppercase();
    if signal_stop_words().contains(&upper_token.as_str()) {
        return;
    }

    signals.insert(upper_token);
}

fn looks_like_signal_token(token: &str) -> bool {
    if token.len() < 2 {
        return false;
    }

    let has_alpha = token
        .chars()
        .any(|character| character.is_ascii_alphabetic());
    if !has_alpha {
        return false;
    }

    let is_upper = token.chars().all(|character| {
        character.is_ascii_uppercase() || character.is_ascii_digit() || character == '_'
    });
    if is_upper
        && token
            .chars()
            .any(|character| character.is_ascii_uppercase())
    {
        return true;
    }

    let lowered_token = token.to_ascii_lowercase();
    lowered_token.ends_with("_n") || lowered_token.ends_with("_b")
}

fn signal_stop_words() -> BTreeSet<&'static str> {
    [
        "IR",
        "PDF",
        "JSON",
        "CLI",
        "README",
        "DOCS",
        "LLM",
        "RTL",
        "FSM",
        "VHDL",
        "DOCLING",
        "SOURCEIR",
        "EVIDENCEIR",
        "SEMANTICIR",
        "INTENTIR",
        "API",
        "URL",
    ]
    .into_iter()
    .collect()
}

fn should_emit_interface_candidate(signals: &[String]) -> bool {
    if signals.len() >= 2 {
        return true;
    }

    signals.first().is_some_and(|signal| {
        signal.ends_with("_N")
            || signal.ends_with("_B")
            || signal.contains("RST")
            || signal.contains("RESET")
            || signal.contains("CLK")
            || signal.contains("CLOCK")
    })
}

fn interface_ids_by_signal(interfaces: &[InterfaceRecord]) -> HashMap<String, BTreeSet<String>> {
    let mut ids_by_signal = HashMap::new();

    for interface in interfaces {
        for signal in &interface.signals {
            ids_by_signal
                .entry(signal.clone())
                .or_insert_with(BTreeSet::new)
                .insert(interface.interface_id.clone());
        }
    }

    ids_by_signal
}

fn related_interface_ids(
    signals: &[String],
    interface_ids_by_signal: &HashMap<String, BTreeSet<String>>,
) -> Vec<String> {
    let mut related_ids = BTreeSet::new();

    for signal in signals {
        if let Some(interface_ids) = interface_ids_by_signal.get(signal) {
            related_ids.extend(interface_ids.iter().cloned());
        }
    }

    related_ids.into_iter().collect()
}

fn phase_like_title(title: &str) -> bool {
    contains_any_phrase(
        &title.to_ascii_lowercase(),
        &[
            "phase",
            "sequence",
            "flow",
            "timing",
            "transaction",
            "handshake",
            "operation",
            "mode",
            "startup",
            "shutdown",
            "reset",
            "request",
            "response",
            "transport",
            "state",
            "write",
            "read",
        ],
    )
}

fn decomposition_like_title(title: &str) -> bool {
    contains_any_phrase(
        &title.to_ascii_lowercase(),
        &[
            "channel",
            "interface",
            "control",
            "data",
            "read",
            "write",
            "timing",
            "state",
            "reset",
            "transaction",
            "arbiter",
            "decoder",
            "encoder",
            "path",
        ],
    )
}

fn sequencing_language(text: &str) -> bool {
    contains_any_phrase(
        &text.to_ascii_lowercase(),
        &[
            "before", "after", "until", "during", "then", "next", "once", "when", "while",
        ],
    )
}

fn is_invariant_like(statement: &StatementContext, context: &SemanticContext) -> bool {
    if matches!(statement.class, StatementClass::ExplicitAbstraction) {
        return false;
    }

    let lowered_text = statement.text.to_ascii_lowercase();
    if contains_any_phrase(
        &lowered_text,
        &[
            "must",
            "shall",
            "always",
            "never",
            "required",
            "remains",
            "remain",
            "until",
            "only when",
            "cannot",
            "must not",
            "shall not",
        ],
    ) {
        return true;
    }

    if !statement.signals.is_empty()
        && contains_any_phrase(
            &lowered_text,
            &[
                "handshake",
                "asserted",
                "deasserted",
                "transition",
                "state",
                "timing",
                "observed",
            ],
        )
    {
        return true;
    }

    statement
        .related_visual_evidence_ids
        .iter()
        .any(|visual_id| {
            context
                .visual_roles_by_id
                .get(visual_id)
                .is_some_and(|role| {
                    matches!(
                        role,
                        VisualEvidenceRole::Normative | VisualEvidenceRole::Ambiguous
                    )
                })
        })
}

fn actor_ids_for_text(
    lowered_text: &str,
    actor_id_by_term: &HashMap<String, String>,
) -> Vec<String> {
    actor_id_by_term
        .iter()
        .filter_map(|(term, actor_id)| {
            contains_phrase(lowered_text, term).then_some(actor_id.clone())
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn overlapping_interface_signals(interfaces: &[InterfaceRecord]) -> Vec<String> {
    let mut counts: HashMap<String, usize> = HashMap::new();

    for interface in interfaces {
        for signal in &interface.signals {
            *counts.entry(signal.clone()).or_insert(0) += 1;
        }
    }

    counts
        .into_iter()
        .filter_map(|(signal, count)| (count > 1).then_some(signal))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn statement_ids_to_section_ids(
    context: &SemanticContext,
    statement_ids: &[String],
) -> Vec<String> {
    let mut section_ids = BTreeSet::new();

    for statement_id in statement_ids {
        if let Some(statement) = statement_by_id(context, statement_id) {
            section_ids.extend(statement.section_ids.iter().cloned());
        }
    }

    section_ids.into_iter().collect()
}

fn statement_by_id<'a>(
    context: &'a SemanticContext,
    statement_id: &str,
) -> Option<&'a StatementContext> {
    context
        .statements
        .iter()
        .find(|statement| statement.statement_id == statement_id)
}

fn normalize_text_key(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn contains_any_phrase(text: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| contains_phrase(text, phrase))
}

fn contains_phrase(text: &str, phrase: &str) -> bool {
    let phrase = phrase.to_ascii_lowercase();

    for (index, _) in text.match_indices(&phrase) {
        let prefix_ok = text[..index]
            .chars()
            .next_back()
            .map(|character| !character.is_ascii_alphanumeric())
            .unwrap_or(true);
        let suffix_index = index + phrase.len();
        let suffix_ok = text[suffix_index..]
            .chars()
            .next()
            .map(|character| !character.is_ascii_alphanumeric())
            .unwrap_or(true);

        if prefix_ok && suffix_ok {
            return true;
        }
    }

    false
}

fn canonicalize_existing_path(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Err(AppError::MissingPath(path.to_path_buf()));
    }

    Ok(fs::canonicalize(path)?)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use crate::error::Result;
    use crate::ir::evidence::EvidenceIr;
    use crate::ir::source::{SourceIr, VisualAsset, VisualAssetKind};

    use super::SemanticIr;

    #[test]
    fn builds_semantic_ir_from_handshake_evidence() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("handshake.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            "# Channel Operation\nThe transmitter asserts VALID when data is available.\n\nThe receiver asserts READY when it can accept data.\n\nVALID must remain asserted until READY is observed.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        assert_eq!(semantic_ir.stage.as_str(), "semantic_ir");
        assert!(
            semantic_ir
                .actors
                .iter()
                .any(|actor| actor.actor_id == "actor_transmitter")
        );
        assert!(
            semantic_ir
                .actors
                .iter()
                .any(|actor| actor.actor_id == "actor_receiver")
        );
        assert!(semantic_ir.interfaces.iter().any(|interface| {
            interface.signals.contains(&"VALID".to_string())
                && interface.signals.contains(&"READY".to_string())
        }));
        assert!(semantic_ir.invariants.iter().any(|invariant| {
            invariant
                .statement
                .contains("VALID must remain asserted until READY is observed.")
        }));
        assert!(!semantic_ir.gates.is_empty());
        assert!(semantic_ir.residual_decisions.is_empty());

        Ok(())
    }

    #[test]
    fn emits_residual_decision_for_ambiguous_visual_grounding() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("control.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let asset_path = tempdir.path().join("assets").join("figure-0001.png");

        fs::create_dir_all(asset_path.parent().expect("asset parent should exist"))?;
        fs::write(&asset_path, b"png")?;
        fs::write(
            &source,
            "# Control Path\nFigure 1: Controller block diagram.\n\n![Image](assets/figure-0001.png)\n\nThe controller behavior is shown in Figure 1.\n",
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "figure_0001".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: Some(asset_path),
            caption_text: Some("Figure 1: Controller block diagram.".to_string()),
            caption_source_path: None,
            source_ref: Some("#/pictures/0".to_string()),
            placeholder_text: None,
            note: None,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        assert!(
            semantic_ir
                .actors
                .iter()
                .any(|actor| actor.actor_id == "actor_controller")
        );
        assert!(
            semantic_ir
                .residual_decisions
                .iter()
                .any(|packet| { packet.packet_id == "semantic_ambiguous_visual_grounding" })
        );
        assert!(
            semantic_ir
                .artifact_layout
                .semantic_ir_path
                .ends_with("generated/semantic_ir/control/semantic_ir.json")
        );

        Ok(())
    }
}
