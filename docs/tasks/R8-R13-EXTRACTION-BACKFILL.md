# R8-R13-EXTRACTION-BACKFILL: own + audit the extraction-SOTA milestones (delivered pre-task-tree-system)

## Metadata

- Tree ID: `R8-R13-EXTRACTION-BACKFILL`
- Status: `done`
- Roadmap lane: `R8-R13`
- Created: `2026-05-31`
- Last updated: `2026-05-31`
- Owner: repo-local workflow

## Goal

Backfill task-tree ownership + a meticulous audit for ROADMAP milestones
**R8–R13** (the extraction-SOTA tiers: SourceIR Tier-1 capture → EvidenceIR
Tier-2 typed evidence → Tier-3 VLM visual → NLP-L3 enrichment → multi-spec
validation → Tier-2 relation extraction), delivered before the task-tree
system. Driven by `ROADMAP-TASKTREE-COVERAGE.3`. Each milestone is an owned,
audited leaf recording the delivering code + tests + book coverage and
confirming the ROADMAP criteria.

## Non-Goals

- No code change — audit + ownership only; any gap → its own normal tree.

## Acceptance Criteria

- R8–R13 each owned by an audited leaf (delivering symbols/commands + book
  pages verified); registered in `docs/TASK_TREE.md`; this tree `done`.

## Task Tree

- ID: `R8-R13-EXTRACTION-BACKFILL`
  Status: `done`
  Goal: own + audit R8–R13
  Children: `.1`, `.2`, `.3`, `.4`, `.5`, `.6`

- ID: `R8-R13-EXTRACTION-BACKFILL.1`
  Status: `done`
  Goal: own + audit **R8 — SourceIR SOTA capture / Tier 1** (ROADMAP §R8)
  Verification: >
    AUDIT passed (`2026-05-31`) — `ir/source.rs` carries the full Tier-1 capture
    surface (`structured_tables` cell grids, `content_elements` typed text in
    reading order, `document_sections` `SectionKind` hierarchy,
    `document_profile`, `StructuredTableRecord.table_kind` classification — 15
    symbol hits), materialized via the Docling backend (`docling_backend.rs`).
    Hardened by `R6-SOURCE-HARDENING`. Book: `pipeline/sourceir.md`. ROADMAP §R8
    completion criteria met. (Follow-up robustness — varied-PDF benchmarking /
    ugly-layout fallback — is ongoing R9-style hardening, not an R8 gap.)
  Commit: `see Commit Log`

- ID: `R8-R13-EXTRACTION-BACKFILL.2`
  Status: `done`
  Goal: own + audit **R9 — EvidenceIR SOTA typed evidence / Tier 2** (ROADMAP §R9)
  Verification: >
    AUDIT passed (`2026-05-31`) — `ir/evidence.rs`: signal + enum + register
    (`synthesize_register_records`) + timing (`synthesize_timing_constraints`)
    table synthesis; monotone `converge_evidence_extractions` loop;
    `NormativeStatement` class (30 hits); polarity refinement;
    `TableSignalDeclarationProvenanceRecord` provenance to source tables.
    Hardened by `R6-EVIDENCE-HARDENING`. Book: `pipeline/evidenceir.md`.
    **ROADMAP §R9 = "Mostly Done"**: the typed-evidence + convergent-enrichment
    core landed; the remaining work is *incremental robustness hardening*
    (benchmark varied real PDFs; honest ugly-layout/OCR/table-split fallback) —
    captured here as the milestone's residual; would be its own hardening tree
    if/when pursued (not a missing deliverable).
  Commit: `see Commit Log`

- ID: `R8-R13-EXTRACTION-BACKFILL.3`
  Status: `done`
  Goal: own + audit **R10 — EvidenceIR visual content / Tier 3 (VLM)** (ROADMAP §R10)
  Verification: >
    AUDIT passed (`2026-05-31`) — `DiagramKind` caption classification +
    `commands/enrich.rs` (VLM via Ollama/OpenAI/LM-Studio) → `VisualObservation`
    (`TimingDiagramExtraction` / `StateMachineExtraction`) in `ir/evidence.rs` →
    `ir/semantic.rs` merges VLM-sourced timing constraints + state/transition
    records. Production default = Ollama + `qwen2.5vl:7b`. Book:
    `pipeline/evidenceir.md` + `runtime-and-doctor.md`. ROADMAP §R10 met.
  Commit: `see Commit Log`

- ID: `R8-R13-EXTRACTION-BACKFILL.4`
  Status: `done`
  Goal: own + audit **R11 — NLP Level 3 enrichment + feedback loops** (ROADMAP §R11)
  Verification: >
    AUDIT passed (`2026-05-31`) — `commands/nlp_enrich.rs` (`run` +
    `build_nlp_prompt` + `apply_alias_reclassification` + Layers A–E +
    Form-1 backannotation + Form-2 alias learning), Ollama/OpenAI/LM-Studio with
    default `qwen2.5vl:7b`, `--dry-run`/`--max-sentences`/`--grounding-signals`.
    Book: `quality/*` + `commands/overview.md`. ROADMAP §R11 met (Done
    `2026-04-03`).
  Commit: `see Commit Log`

- ID: `R8-R13-EXTRACTION-BACKFILL.5`
  Status: `done`
  Goal: own + audit **R12 — Multi-spec validation + quick fixes** (ROADMAP §R12)
  Verification: >
    AUDIT passed (`2026-05-31`) — `extract_alias_phrase()` garbage filter
    (`nlp_enrich.rs`: markdown/list/outline/link/caption-prefix rejection);
    AMBA-5 `Source`/`Driver`/`Destination` + role-name direction parsing
    (`evidence.rs`/`semantic.rs`); width-only/parametric-width honesty;
    recorded validation baselines (APB 94 / AHB 94 / AXI 85). Book:
    `quality/validation.md` (+ `VALIDATION_SNAPSHOT.md`). ROADMAP §R12 met.
  Commit: `see Commit Log`

- ID: `R8-R13-EXTRACTION-BACKFILL.6`
  Status: `done`
  Goal: own + audit **R13 — Actor-signal relation extraction: Tier 2 prose** (ROADMAP §R13)
  Verification: >
    AUDIT passed (`2026-05-31`) — `RelationKind` + `ActorSignalRelation`
    (`source.rs`); `extract_actor_signal_relations()` (`evidence.rs`) for
    active/passive drive/read verb patterns + signal-table `Source`/`Driver`
    columns; KG-derived direction synthesized back into EvidenceIR; carried to
    `SemanticIR`/`IntentIR` via `actor_signal_relations`/`actor_ports`/
    `signal_connectivity` (consumed by the R15 graph-first model). Book:
    `domain/actor-connectivity.md`. ROADMAP §R13 met. (Tier-3 LLM extension =
    R14, delivered separately as `R14-SIGNAL-RESOLVE`.)
  Commit: `see Commit Log`

## Current Frontier

**Tree CLOSED `2026-05-31`** — R8–R13 each owned + audited (all delivered; R9
core delivered with residual robustness-hardening noted).

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R8-R13-EXTRACTION-BACKFILL.1` | `done` | R8 audited |
| 2 | `R8-R13-EXTRACTION-BACKFILL.2` | `done` | R9 audited (Mostly Done; residual noted) |
| 3 | `R8-R13-EXTRACTION-BACKFILL.3` | `done` | R10 audited |
| 4 | `R8-R13-EXTRACTION-BACKFILL.4` | `done` | R11 audited |
| 5 | `R8-R13-EXTRACTION-BACKFILL.5` | `done` | R12 audited |
| 6 | `R8-R13-EXTRACTION-BACKFILL.6` | `done` | R13 audited |

## Decisions

- `2026-05-31`: phase-group backfill tree with a leaf per milestone (same
  pattern as `R1-R5-FOUNDATION-BACKFILL`). R9's "Mostly Done" residual
  (incremental robustness hardening) is recorded in its audit rather than
  blocking the ownership/audit closure.

## Open Questions

- None. (R9's robustness hardening could become its own future tree if a real
  PDF exposes a capture bottleneck — per the ROADMAP §R9 follow-up guidance.)

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-31` | `.1` R8 | source.rs Tier-1 capture (15 symbols) + Docling; book `pipeline/sourceir`; §R8 met | `passed` |
| `2026-05-31` | `.2` R9 | evidence.rs synth/converge/NormativeStatement/polarity/provenance; book `pipeline/evidenceir`; §R9 Mostly-Done core landed (residual = robustness hardening) | `passed` |
| `2026-05-31` | `.3` R10 | DiagramKind + enrich VLM + VisualObservation + semantic merge; book `pipeline/evidenceir`+`runtime-and-doctor`; §R10 met | `passed` |
| `2026-05-31` | `.4` R11 | nlp_enrich (run/prompt/alias/Layers A–E); Ollama default qwen2.5vl:7b; book `quality/*`+`commands`; §R11 met | `passed` |
| `2026-05-31` | `.5` R12 | extract_alias_phrase filter + AMBA direction parsing + baselines (APB94/AHB94/AXI85); book `quality/validation`; §R12 met | `passed` |
| `2026-05-31` | `.6` R13 | RelationKind/ActorSignalRelation + extract_actor_signal_relations + KG carry; book `domain/actor-connectivity`; §R13 met | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R8-R13-EXTRACTION-BACKFILL.{1..6}` | `ROADMAP-TASKTREE-COVERAGE.3 — backfill+audit R8–R13 extraction-SOTA milestones` | audit/ownership only; all delivered + book-covered |

## Changelog

- `2026-05-31`: Created + CLOSED — owned + audited R8–R13 (extraction SOTA:
  Tier-1 capture / Tier-2 typed evidence / Tier-3 VLM / NLP-L3 / multi-spec
  validation / Tier-2 relations); all delivered + book-covered; R9 core landed
  with residual robustness-hardening noted. (`ROADMAP-TASKTREE-COVERAGE.3`.)
