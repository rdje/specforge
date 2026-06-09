---
id: extractor-path-architecture
title: The EvidenceIR extractor path is a flat bank of ~60 free functions wired in one build() — coherent IR target, ad-hoc producers
answers:
  - "how is the EvidenceIR extractor path / extraction layer structured and wired"
  - "where do the signal / FSM / register / constraint / actor extractors live and how are they merged"
  - "why does adding a new extractor feel fragile / erratic (god-orchestrator + inline dedup loops)"
  - "what is the proposed Extractor framework (registry / driver / SurfacePolicy / run manifest)"
  - "is the SpecForge extraction robust and can it grow to a vast set of chip-spec PDFs"
date: 2026-06-09
tags: [architecture, evidence-ir, extractor, refactor, coherence, pdf-variant-digestion, extractor-architecture]
evidence: docs/tasks/EXTRACTOR-ARCHITECTURE.md (.1 audit); crates/specforge/src/ir/evidence.rs (EvidenceIr::build ≈500–1040; synthesize_signal_semantic_hints; the four FSM extractors)
reverify: grep -nE '^fn (extract_|synthesize_)' crates/specforge/src/ir/evidence.rs | wc -l   # ~40+ producer fns; build() at the `impl EvidenceIr` is the ~500-line orchestrator
---

**Coherent target, ad-hoc producers.** The typed IR (`EvidenceIR → SemanticIR → IntentIR → .isf`) is a
properly-defined, stable, backend-neutral set of records — every reader funnels into one of
`InterfaceSignalRecord` / `ActorSignalRelation` / `SignalConstraint` / `TemporalRuleRecord` /
`ProtocolStateRecord` / `SerialFrameField` / `RegisterRecord` / `ProtocolActorRecord` and downstream is clean.
The *producer layer* is the weak point: `crates/specforge/src/ir/evidence.rs` (~16.6k lines) holds ~60 free
extractor/synthesizer functions clustered by surface (signals, relations, constraints, polarity, semantic
hints, FSM, registers, actors, timing, encoding), each with 1–4 strategy variants (table / prose / visual /
quoted / transition-bound), all invoked imperatively inside one ~500-line `EvidenceIr::build()` with
**hand-rolled, per-surface merge/dedup** and **heterogeneous, implicit applicability gates** (keyword gate,
content gate, a magic `table_signal_count < 8` threshold, structural self-gate).

The **FSM cluster is the canonical failure mode**: four sibling functions (`extract_protocol_states`,
`extract_swd_line_states`, `extract_quoted_mode_states`, `extract_transition_bound_states`) glued by *four
separate inline dedup-by-name loops at the call site* (the 4th added by `PDF-VARIANT-DIGESTION.9.7`). The
**semantic-hints cluster is the embryonic good pattern**: `synthesize_signal_semantic_hints` is a real
dispatcher (table/prose/visual strategies) + a `signal_semantic_hint_key` dedup + a conflict detector. Two
more gaps: provenance (`fact_provenance`) is tagged for only 2 of ~10 surfaces, and there is **no run
manifest** — you cannot ask "which extractors were eligible / fired / contributed / conflicted" without
reading code.

**Verdict:** growth is *additively safe* (why nothing regresses) but **not elegant** — accretion of free
functions + `build()` edits = "erratic growth in every direction" (owner's term, `2026-06-09`). Fix =
refactor, not rewrite (grammars + IR are sound): an `Extractor` trait (`name`/`surface`/`tier`/`applies_to`/
`run`) + one shared `ExtractionContext` + one `SurfacePolicy` per surface (key + conflict) + ONE driver that
merges uniformly and emits an inspectable `ExtractionRun` manifest. New PDF families then grow by registering
*one unit in one place*; the LLM/VLM tiers are the same frame (`tier=Llm/Vlm`, provider-availability
`applies_to`, "model proposes → declared signals/structure decide" as a shared post-filter). Migration is
incremental + behavior-preserving (FSM cluster first, byte-identical via kg-bench + eval). Tracked in
`docs/tasks/EXTRACTOR-ARCHITECTURE.md`; build gated on owner confirmation of the framework shape.

**Status update (`2026-06-09`, post `.2`–`.8` + `.9a`):** the framework exists (`ir/extractor.rs`:
`Extractor` trait + `run_surface` key-merge + `run_surface_concat` + `ExtractionManifest` persisted on
`EvidenceIr` and surfaced in `validate`) and SIX surfaces are registered and byte-identical-proven: FSM
(`.3`), semantic hints (`.4`), registers (`.6`, concat + post-passes), actors (`.7`), and serial-frame +
SWD-operations (`.9a` — serial-frame is a clean key-merge by field name because both strategies emit
name-unique lists; operations is single-strategy concat). The "no run manifest" claim above is therefore
HISTORICAL (the audit state), as is the four-inline-FSM-loops description. Still open: the converge-loop
surfaces (constraints/relations/polarity/conditional-rules) and retiring the `build()` god-orchestrator;
signal-declaration stays a deliberate stateful-assembly orchestrator (`.5`).

**Status update (`2026-06-10`, post `.9b` + `.9c`):** EIGHT surfaces — the two migratable
**convergence-loop** surfaces are on the framework. Signal polarity (`.9b`): two observation strategies
(`signal_polarity.prose` / `signal_polarity.tables`) run via `run_surface_concat` per fixed-point pass,
with the unchanged per-signal accumulate-and-arbitrate post-pass (consensus → `SignalPolarityRecord`,
disagreement → `SignalPolarityConflictRecord`). Actor-signal relations (`.9c`): `relations.prose` +
`relations.tables` (the table list is build-precomputed and re-emitted per pass) via `run_surface_concat`,
then the two ORDERED legacy post-passes — check-signal augmentation over the full pre-dedup list, THEN
first-wins dedup by `(actor, signal, is_drives)`; the dedup must stay a post-pass because it runs after
augmentation. `ExtractionManifest::record`'s replace-per-surface-name semantics mean the manifest holds
exactly the FINAL converged pass's run, no special casing — and it now distinguishes table-driven vs
prose-driven relation recovery per document (AXI: 356 table + 20 prose → 348; SWD/I2C: pure prose).
**Constraints + conditional rules** stay stateful-assembly by design (one per-pass `constraint_counter`
mints ids ACROSS the three extractors, plus a cross-surface polarity post-pass) → their own in-loop
orchestration, like the signal-declaration seed. The constraint-family categorization is now encoded in
the `ir/extractor.rs` module doc ("Two phases, three categories", `.9d`). Remaining: the `build()`
god-orchestrator retirement assessment.
