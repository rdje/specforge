# Source-to-Intent Completeness Contract

SpecForge calls a document “understood” only when its materially relevant, source-grounded meaning reaches
canonical `IntentIR` or remains visible as a source-linked residual. A stable pipeline, a green test suite, and
a strict-valid `.isf` file are valuable checks, but none proves that the PDF was understood.

This chapter defines the category-aware acceptance contract used by the
[`SPEC-TO-INTENT-ALIGNMENT`](../../tasks/SPEC-TO-INTENT-ALIGNMENT.md) program. Its machine-readable authority is
`doctrine/spec_to_intent_category_contract.json`. The controller described in
[Trajectory and Automatic Steering](quality/trajectory.md) will consume that contract in a later leaf; this
chapter explains it for reviewers and users.

## The boundary being measured

The contract measures:

```text
SourceIR -> EvidenceIR -> SemanticIR -> IntentIR
```

ISF lowering is a separate, later measurement. It becomes relevant only after the upstream contract has shown
that the required source semantics are present in `IntentIR`. An emitted `.isf` file cannot compensate for a
missed table, unexamined timing diagram, dropped field record, or ungrounded relation.

The six document categories remain the dominant-purpose taxonomy described in
[Chip-Spec Document Categories](document-categories.md). The automatic recognizer may return the combined
`register-or-platform` or an honest `unresolved`; a reviewed category label selects the acceptance contract.
An unresolved automatic result never becomes a forced guess.

## Floors for a support claim

A category is not declared supported from one convenient document or one blended score. A support claim needs
at least two held-out documents for that category. For every applicable semantic family and every required
modality present in those documents, the reviewed gold must be exhaustive for its stated scope and include
positive and negative cases.

The initial floors are deliberately strict:

| Dimension | Floor | Denominator or interpretation |
| --- | ---: | --- |
| Reviewed-gold precision | `1.000` | Every canonical fact in each document × family × modality cell |
| Reviewed-gold recall | `1.000` | Every reviewed gold fact in that same cell |
| Intent-bearing source-region disposition | `1.000` | Every such region links to a canonical fact or a source-linked residual |
| Required-modality accounting | `1.000` | Every category-required modality independently found in the source is attempted or residualized |
| Canonical provenance closure | `1.000` | Every promoted fact, conflict, and residual is traceable to evidence |
| Stage conservation or residual | `1.000` | Every source-grounded fact survives each IR boundary or gains an explanatory residual |
| Residual actionability | `1.000` | Every residual names provenance, reason, first failing stage, and review/replay route |
| Fabricated canonical facts | `0` | No unsupported fact is allowed |
| Unexplained stage drops | `0` | A disappearance without a residual is a hard failure |
| Silently removed conflicts | `0` | Disagreement cannot be averaged or selected away |
| High-confidence category precision | `1.000` | Every high-confidence automatic category claim in the held-out population |
| Forced category errors | `0` | Ambiguity must remain unresolved rather than become a wrong label |

These are fixture-level acceptance floors, not a claim of population-wide statistical certainty. Every result
also reports its document count, fact count, modality, oracle, and uncertainty. If a denominator is missing or
too small, the result is `unmeasurable`, not passing. Floors apply to each cell before any category roll-up, so
many easy signals cannot hide one missed reset rule or one fabricated field.

## Universal outcomes

Two obligations apply to every category, including honest non-targets.

First, provenance must close. A promoted fact needs record-specific source identifiers, and a withheld fact
needs a residual that points back to the evidence. Second, conflict and uncertainty must remain typed. The
applicable `IntentIR` surfaces include interface, connectivity, polarity, semantic, and temporal conflicts plus
`assumptions` and `residual_decisions`. “No conflict” is non-applicable only when the reviewed source oracle found
none; an empty extractor result is not evidence of absence.

## Category 1 — wire protocol

Required source modalities are normative and definitional prose; signal/interface, timing, encoding, frame,
and field tables when present; section/list structure; captions and interface figures; and any timing or state
diagram in the source.

The content families are:

| Family | Canonical `IntentIR` surface | Applicability |
| --- | --- | --- |
| Interface inventory and shape | `interfaces`, `actor_ports`, `signal_polarities` | Boundary signals or pins are declared |
| Actors and connectivity | `actors`, `actor_signal_relations`, `signal_connectivity` | Producer/consumer or endpoint responsibilities are stated |
| Normative and temporal behavior | `signal_constraints`, `conditional_rules`, `temporal_rules`, `actor_contracts`, `fidelity_findings` | Values, stability, ordering, latency, sampling, or handshakes are stated |
| Protocol structure and state | `transactions`, `protocol_graph`, `regular_states`, `state_transitions`, `control_blocks` | Transactions, phases, handshakes, states, or transitions are named |
| Frame and protocol observations | `serial_frame_fields`, `protocol_operations`, `protocol_states`, `interface_edge_timings` | Frames, response branches, states, or edge timing are stated |
| Symbols and secondary storage | `symbol_definitions`, `register_records` | Encodings or register-backed secondary content exists |

Packet, flit, header, and message fields expose a current conservation failure. They exist as
`EvidenceIR.message_field_records` and `EvidenceIR.message_field_constraints`, but no matching `IntentIR`
carrier exists. Until that carrier lands, each source-grounded record needs a residual and this family cannot
pass, regardless of whether unrelated wire content lowers successfully.

## Category 2 — register IP

Required modalities are register and field tables; bit-layout figures and geometric labels; register/field
description prose; address, reset, access, and encoding tables; and section structure that scopes register
blocks.

The register map must reach `register_records` and `register_records[].fields`. When the source supplies the
information, field access, reset, and enumerated values must populate the corresponding field members and
`symbol_definitions`; access side effects and update/ordering obligations must reach the appropriate constraint,
control, or contract surface. A hardware interface is required only when the document itself specifies one.

Descriptors, queues, page tables, contexts, commands, and other packed programming structures currently share
the message-field carrier gap: their typed EvidenceIR records stop before `IntentIR`. A source-linked residual
makes the loss honest, but does not satisfy completeness.

## Category 3 — platform and system IP

Required modalities are component/integration prose; block, connectivity, clock, reset, and topology diagrams
with their captions; interface/distribution tables; register maps and bit layouts; and the section hierarchy
that scopes components and domains.

The current canonical surfaces cover component and boundary evidence through `actors`, `explicit_modules`,
`explicit_tops`, `interfaces`, and `actor_ports`; integration evidence through `actor_signal_relations`,
`signal_connectivity`, `infrastructure_signals`, and `system_contract`; and programming/behavioral content
through registers, symbols, constraints, temporal rules, and actor contracts.

Static component topology, hierarchy, first-class clock domains, and asynchronous-domain relationships do not
yet have a complete canonical carrier. Sparse signal connectivity is not a substitute. A document containing
that content therefore requires explicit residuals and cannot pass the topology family until a typed carrier
and measured producer exist.

## Category 4 — CPU ISA and privileged architecture

Required modalities are CSR/register tables, register bit-layout figures, encoding tables and diagrams,
normative state/ordering prose, and exception/privilege/state diagrams when present.

The buildable hardware subset uses existing typed surfaces: CSRs and debug registers belong in
`register_records`, their fields in `register_records[].fields`, and encodings in `symbol_definitions`.
Implementation-relevant hardware obligations use constraints, temporal rules, control blocks, and actor
contracts as applicable.

Instruction semantics, privilege policy, abstract exceptions, and software-visible memory ordering are honest
non-applicable content for executable `IntentIR` when the source gives no hardware boundary contract. Every such
region still needs an explicit non-applicable disposition. It must not be ignored silently or translated into
invented pins, states, or rules.

## Category 5 — physical and link layer

Required modalities are electrical/timing and pin/lane tables; eye, waveform, package, mechanical, and
link-training figures; normative physical/electrical prose; captions; and units.

A source-stated digital boundary subset can use `interfaces`, `actor_ports`, `timing_constraints`, and
`signal_constraints`. Physical, analog, package, and mechanical content is otherwise non-applicable to
executable digital intent and must receive a source-linked non-applicable or unsupported disposition. The
additional hard floor is zero false executable-intent facts: a near-empty `IntentIR` can be correct, but only
after source-presence accounting proves why.

The current scalar boundary implements one deliberately narrow part of that contract. An explicit decibel-domain
unit (`dB`/`dBc` as the first unit token) is sufficient evidence that a captured row is a logarithmic physical
measurement rather than executable digital timing. The same `TimingConstraintRecord` crosses all three promoted
stages with a typed `non_applicable` disposition and actionable reason/boundary/replay fields, while canonical
temporal derivation excludes it. UI- and time-unit rows remain canonical. Other physical quantity domains remain
open until their units provide an equally closed, source-grounded grammar.

## Category 6 — methodology and guide

The classifier must inspect enough front matter, structure, prose, tables, and figures to distinguish guidance
from an embedded hardware contract. For a genuine methodology or guide, the required result is a source-backed
non-contract disposition and zero false executable-intent facts.

If a bounded region really defines a hardware interface or behavior, that region is evaluated under the
applicable category 1–5 contract. A document that is mostly a guide does not authorize the pipeline to discard
an embedded contract, and a few spurious extracted records do not authorize it to relabel the guide as a chip
contract.

## Non-applicable is an evidenced outcome

Non-applicable is allowed only when an independent source-presence review establishes that a semantic family or
modality is absent or outside executable digital intent. The record must name the category, family or modality,
source scope, reason, oracle or reviewer, and evidence identifiers.

None of these are valid shortcuts:

- zero extracted records;
- blocked adapter output;
- a strict-valid empty `.isf`; or
- a low-confidence automatic category guess.

For an unresolved category, no contract pass is possible. The pipeline preserves the category residual and
applies the union of plausible modality checks until review resolves the dominant purpose.

## The vertical evaluator

`SPEC-TO-INTENT-ALIGNMENT.4a` ships the deterministic oracle that applies this contract. Its public dataset
schema is `doctrine/spec_to_intent_vertical_eval_schema.json`; the matching Rust API is
`ir::source_to_intent_eval`. The evaluator itself does not extract facts and contains no document, vendor,
protocol, signal, or page-layout exceptions.

Each dataset pins the review-selection Git boundary and gives every source a SHA-256 identity. Repository
sources use repository-relative paths. A necessary read-only external source uses only a portable id, digest,
and necessity statement; host paths are rejected. Each document then carries bounded JSON snapshots of all
four IR stages with the digest of the original artifact. Unknown JSON fields and unsafe paths fail closed.

Review cells are data, not Rust branches. A cell declares category, semantic family, modality, oracle, bounded
review scope, source/evidence queries, and either canonical or residual queries. Queries select arrays and
records with JSON pointers and data-defined predicates, then compare exhaustive key sets as multisets. A
duplicate prediction remains a false positive. Precision or recall with a zero denominator is JSON `null`,
never an invented perfect score.

The report keeps exact TP/FP/FN counts for each stage, names the first failing boundary, and separately totals
source disposition, required-modality accounting, provenance closure, conservation-or-residual, residual
actionability, fabrication, and unexplained drops. A residual counts only when it survives SemanticIR and
IntentIR and every required actionability field is populated. A category is `supported` only when its minimum
document count, complete review scopes, and all hard floors pass. Incomplete gold or too few documents is
`unmeasurable`, even if provisional scores already reveal faults; measured failures become `incomplete` only
for a complete review population.

Mutation controls prove the oracle notices omission, fabrication, provenance loss, a silent inter-stage drop,
a missing required modality, and an inactionable residual. They prove evaluator adequacy, not category support.
The reviewed population and its product conclusions belong to `.4b` and `.4c`.

## The first reviewed population

`SPEC-TO-INTENT-ALIGNMENT.4b` locks twelve documents—exactly two per category—without changing extractor
behavior. This is a retrospective baseline frozen before gold construction and held out from extractor changes
at or after selection commit `a3e9757d63ca5499a2393864fb503d6537de0035`; it is not claimed to have been
historically unseen.

| Category | Documents | Reviewed source modality |
| --- | --- | --- |
| wire protocol | Arm AMBA APB; NXP I2S | prose; table |
| register IP | Arm GIC-400 TRM; AMD IOMMU | tables |
| platform/system IP | Arm CoreSight Base System Architecture; RISC-V IOMMU | figures |
| CPU/ISA | RISC-V Advanced Interrupt Architecture; Arm Debug Interface v6 | tables |
| physical/link | OpenCAPI 25 Gbps PHY Signaling; OpenCAPI 32G PHY Signaling | tables |
| methodology/guide | Cortex-A76 Software Optimization Guide; GIC Overview Guide | prose |

Four PDFs are repository inputs. Eight are necessary caller-authorized read-only inputs; the dataset stores only
their portable filename, SHA-256 digest, and necessity. The tracked dataset and deterministic builder live under
`crates/specforge/test_data/source_to_intent_vertical/`. The builder authenticates every source and all four
original stage artifacts, then emits bounded reviewed projections rather than copying host paths or full IRs.

There are 14 complete review cells because each physical-link source has separate digital-boundary and analog
non-applicable dispositions. Every cell declares its exact region, family, modality, oracle, provenance, and
exhaustive bounded-scope gold. Rebuilding the fixture or evaluating it twice is deterministic, all six
categories have two complete documents, and no category is unmeasurable for missing review authority.

Population construction and result publication remain separate: `.4b` owns this frozen gold, while `.4c` owns
the following product conclusions. That boundary prevents a disappointing score from changing selection or
reviewed truth.

## The first measured result

The complete persisted report is
`crates/specforge/test_data/source_to_intent_vertical/result_snapshot.json`. It is generated by the generic
`source_to_intent_eval` example and compared byte for byte in a focused test. It is not a blended health score:
each category, cell, stage, numerator, denominator, hard failure, and first failing stage remains visible.

| Category | Documents / cells | Hard failures | Status |
| --- | ---: | ---: | --- |
| wire protocol | 2 / 2 | 8 | `incomplete` |
| register IP | 2 / 2 | 9 | `incomplete` |
| platform/system IP | 2 / 2 | 4 | `incomplete` |
| CPU/ISA | 2 / 2 | 9 | `incomplete` |
| physical/link | 2 / 4 | 12 | `incomplete` |
| methodology/guide | 2 / 2 | 4 | `incomplete` |

No category is `supported`, and none is `unmeasurable`: all 12 documents have complete review scope and every
category has the required two documents. The source-region and required-modality capture queries are exact in
all 14 cells. The failure begins after the relevant region has been found, when its meaning must become the
correct typed EvidenceIR facts or an explicit non-applicable/residual disposition.

| Global dimension | Exact result |
| --- | ---: |
| canonical IntentIR precision | 7 / 48 = 14.5833% |
| canonical IntentIR recall | 7 / 40 = 17.5% |
| canonical provenance closure | 3 / 48 = 6.25% |
| source-region disposition | 0 / 14 = 0% |
| required-modality accounting | 0 / 12 = 0% |
| stage conservation or residual | 21 / 54 = 38.8889% |
| residual actionability | 0 / 24 = 0% |
| fabricated canonical facts | 41 |
| unexplained stage drops | 33 |

The stage split localizes the constraint. SourceIR → EvidenceIR conserves 7 of 40 reviewed canonical facts and
owns all 33 unexplained drops. Those seven true positives then remain 7/7 through EvidenceIR → SemanticIR and
7/7 through SemanticIR → IntentIR. Ten cells first fail at SourceIR → EvidenceIR; four first fail at EvidenceIR
→ SemanticIR because a captured figure or prose region never receives its required actionable residual. No
cell first fails at SemanticIR → IntentIR.

Concrete examples show why a category average would mislead. APB preserves three of four reviewed setup-state
facts without fabrication but misses one. Both OpenCAPI digital-lane cells preserve their two reviewed values,
yet their promoted records lack the required provenance. The RISC-V AIA table-of-contents region becomes 19
false canonical facts instead of a non-applicable disposition. The two platform figures and both guide prose
regions are captured, but their required residuals never appear in SemanticIR or IntentIR.

The evidence-ranked next constraint is therefore upstream source-to-evidence fact formation plus explicit
residualization. This result demonstrates no ISF/FSMGen expressiveness blocker: it ends at IntentIR and contains
no reviewed value that is correct and fully provenanced through IntentIR but fails only at adapter lowering.

## Current status

The strict contract, evaluator, frozen reviewed population, and first product result now ship. The result does
not mean every unreviewed region has the same rates, and it does not claim the retrospective population was
historically unseen. It does establish a reproducible baseline and a first-failing-stage direction without
extractor tuning. `SPEC-TO-INTENT-ALIGNMENT.5` is next: turn these exact dimensions into reviewable automatic
trajectory state and task proposals without allowing one aggregate score to hide a hard failure.
