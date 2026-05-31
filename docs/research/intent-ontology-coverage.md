# Design-Intent Ontology & Coverage Matrix

> Owned by `INTENT-COMPLETENESS-RESEARCH.2`. Companion to
> [`intent-capture-completeness.md`](intent-capture-completeness.md) §2 — this is
> the **concrete, code-grounded** ontology + coverage matrix (the recall
> *denominator*). Built from a code audit of `crates/specforge/src/ir/*` +
> `commands/*`, with the load-bearing gap claims **verified against the code**
> (audit findings that failed verification are recorded as corrections, per the
> AUDIT-DOC-RECONCILE / residual-honesty doctrine — code is truth).

## Why this document exists

Recall = captured ÷ (captured + missed). The denominator is undefined until the
*universe of intent categories* is explicit and closed, and until we know which
extractor is supposed to produce each one from which modality. An intent
category with **no producer** is a *systematic, silent, total* blind spot — the
worst miss class, because nothing ever fires. A category produced from only one
modality is a *partial* blind spot. This matrix makes both visible.

## The closed ontology (categories the code models)

Grouped; each maps to ≥1 IR field below. This is the candidate closed set —
additions require evidence the code (or a planned extractor) models them.

- **Structural:** signals/ports (name, direction, width, polarity); actors
  (role); actor↔signal connectivity (drives/reads); interface grouping.
- **State surface:** registers + fields (offset, bit-range, reset value, access);
  enumerations/encodings + value atoms; parameters/constants.
- **Temporal/behavioral:** clocks (edges; *domains — gap*); resets (polarity,
  sync/async); timing constraints (min/typ/max, units); temporal rules
  (cycle-tick); handshake/protocol semantics; normative rules (must/shall/
  until/while); state machines (states, transitions, guards); stability/sampling
  obligations; temporal invariants.
- **Meaning & arbitration:** semantic role hints/candidates/consensus; conflicts
  (polarity / semantic / interface / connectivity / temporal); residuals /
  assumptions; ActorContracts (R16 constrained-verified); protocol graph;
  fidelity findings.

## Coverage matrix (category → stage:field → producer → modality)

Condensed from the audit (representative producer named; line numbers approximate
— treat as locators, not contracts):

| Category | Stage : field | Producer | Modality |
| --- | --- | --- | --- |
| Signal direction | Semantic : `actor_ports[].direction` | `build_actor_ports` + `synthesize_directions_from_relations` | prose relations + tables |
| Signal width | Semantic : `actor_ports[].width_hint` | `build_actor_ports` | tables (+ weak prose) |
| Signal polarity | Evidence : `signal_polarities[]` | `extract_signal_polarity_from_prose` / `…_from_signal_tables` | prose + signal tables |
| Actors | Semantic : `actors[]` | `build_semantic_actors` | prose role keywords |
| Connectivity (drives/reads) | Evidence : `actor_signal_relations[]` | `extract_actor_signal_relations` (+ R14 `signal-resolve` LLM) | prose verbs + tables |
| Registers + fields | Evidence : `register_records[].fields[]` | `synthesize_register_records` | `RegisterMap` tables |
| Enumerations/encodings | Evidence : `extracted_statements[]` (synthetic) | `synthesize_encoding_declarations` | `Encoding` tables |
| Timing constraints | Evidence : `timing_constraints[]` | `synthesize_timing_constraints` | `TimingParameter` tables |
| Temporal rules (cycle-tick) | Semantic : `temporal_rules[]` | NLP + VLM timing diagrams | prose + figures (VLM) |
| Handshake/protocol | Intent : `transactions[]` + Semantic : `protocol_graph.handshakes` | `recognize_digital_patterns` / `project_handshake_pairs` | signal-name patterns + contracts |
| Normative rules | Evidence : `signal_constraints[]`, `conditional_rules[]` | Level-2 NLP (+ `nlp-enrich` L3) | prose |
| State machines | Semantic : `regular_states[]`, `state_transitions[]` | `build_regular_states` / `…_transitions` + `extract_records_from_vlm_observations` | prose patterns + figures (VLM) |
| Stability/sampling | Evidence : `signal_constraints` (`MustBeStable`…) | Level-2 NLP | prose |
| Reset (polarity + kind) | Semantic : `…reset_kind: SystemResetKind` | infrastructure-signal synthesis | prose + naming |
| Semantic role hints | Evidence : `signal_semantic_hints[]` ; Semantic : `…semantic_candidates/consensus` | `synthesize_signal_semantic_hints_from_{tables,prose}` + arbitration | tables + prose + captions/VLM |
| Conflicts (5 kinds) | Evidence/Semantic/Intent : `*_conflicts[]` | per-kind collectors | multi-source divergence |
| Residuals/assumptions | all : `residual_decisions[]`, Intent : `assumptions[]` | explicit tagging + `build_assumptions` | synthesis |
| ActorContracts (CVE) | Evidence : `extracted_contracts[]` → Semantic : `actor_contracts[]` | `extract-contracts` (Qwen) + entailment fusion | prose (LLM) |
| Protocol graph | Semantic/Intent : `protocol_graph` | `project_handshake_pairs(&actor_contracts)` | derived from contracts |
| Fidelity findings | Semantic : `fidelity_findings[]` | `apply_fidelity_gates` | self-assessment |

## (A) Confirmed ontology gaps — categories with no first-class producer

These are *systematic* blind spots (the recall-denominator's missing rows).
**Verified against the code:**

1. **Clock domains** — no `ClockDomain` type/field anywhere (`grep` confirms
   none). Clock *edges* appear inside `temporal_rules` (`ClockEdge`), but
   "signal X belongs to clock domain CLK_A; CLK_A and CLK_B are asynchronous" is
   not a captured intent category. Multi-clock/CDC is explicitly FSMGen-layer
   scope, but *which domain a signal lives in* is SpecForge intent and is
   currently uncaptured. → candidate first-class category + producer.
2. **Enumerations/encodings as a queryable record** — no `EnumRecord` /
   `enum_records` type (`grep` confirms none). Enums are recovered two ways but
   neither is a first-class, queryable IR record: (a) synthetic
   `ExtractedStatement`s from `Encoding` tables; (b) the recovered symbol surface
   the `.isf` adapter emits as `(enums …)`. So "what are all values of HBURST,
   with encodings?" is not directly answerable from a typed field. → candidate
   first-class category.

**Audit claims that FAILED verification (corrections — code is truth):**
- *protocol_graph is NOT dead.* `semantic.rs:371` populates it:
  `protocol_graph: ProtocolGraph { handshakes: project_handshake_pairs(&actor_contracts), … }`.
  It is a *derived* surface whose richness is gated on `actor_contracts` — not an
  empty/unwired field. (Coverage concern is contract recall, not a dead surface.)
- *reset synchronicity IS modeled.* `semantic.rs:972` `enum SystemResetKind {
  Synchronous, Asynchronous }`, field `reset_kind` at `semantic.rs:860`. Not a
  gap.

## (B) Single-modality blind spots — audit-flagged, to be confirmed empirically

Categories the audit flagged as extracted from one modality where the same intent
commonly appears in others. **These are hypotheses, not confirmed misses** — the
region-accounting (`.3`) and cross-modal (`.4`) detectors are precisely what will
*confirm or refute* each on real documents. Listed as the candidate target set:

| Category | Captured from | Plausibly also in | Detector that will confirm |
| --- | --- | --- | --- |
| Signal direction | prose + tables | block/IO diagrams (VLM) | cross-modal (§5) |
| Signal width | tables | prose ("32-bit address"), RTL params | region accounting on prose |
| Timing constraints | `TimingParameter` tables | prose ("within 2 cycles"), timing diagrams | region accounting + cross-modal |
| Enumerations | `Encoding` tables | prose ("HTRANS is IDLE or NONSEQ") | region accounting on prose |
| Register semantics | `RegisterMap` tables | prose register descriptions | region accounting on prose |
| Actor roles | prose keywords | figure captions/labels | cross-modal (caption) |
| Polarity | prose + tables | timing-diagram waveforms (VLM) | cross-modal (VLM) |
| Handshake semantics | name heuristics + temporal | explicit prose protocol text | region accounting on prose |

> Note the elegance: the (B) hypotheses do not need to be hand-verified now. The
> §3 region-accounting detector ("intent-bearing prose region that produced no
> fact") will *automatically surface* any prose timing/width/enum/register
> statement that the table-only extractors missed — turning this hypothesis list
> into measured residuals. That is the program working as designed.

## How this feeds the program

- **(A) confirmed gaps** → candidate new-category + producer trees (clock
  domains; first-class enum record), prioritized in `.7`.
- **(B) blind spots** → the *test set* for the §3/§4 detectors: a good
  region-accounting + cross-modal implementation should light these up on real
  specs. They are how we will *measure* the detectors' value.
- The full matrix is the **denominator**: per-category recall (gold fixtures) and
  capture–recapture estimates are reported *per row*, so "accuracy" is never a
  single opaque number.

## Honest status

This matrix is **audit-derived and partially verified**. The category list and
producer mapping reflect the code as of `2026-05-31`; the (A) gaps are verified;
the (B) blind spots are hypotheses the detectors will confirm. It is a living
denominator — to be reconciled whenever an extractor or IR type changes (no
drift: same discipline as the ROADMAP↔code↔mdBook lock).
