# R1-R5-FOUNDATION-BACKFILL: own + audit the foundational pipeline milestones (delivered pre-task-tree-system)

## Metadata

- Tree ID: `R1-R5-FOUNDATION-BACKFILL`
- Status: `done`
- Roadmap lane: `R1-R5`
- Created: `2026-05-31`
- Last updated: `2026-05-31`
- Owner: repo-local workflow

## Goal

Backfill task-tree ownership + a meticulous audit for ROADMAP milestones
**R1–R5** (the canonical pipeline foundation: CLI/IntentIR pivot → SourceIR →
EvidenceIR → SemanticIR → IntentIR), which were delivered before the
task-tree system existed. Driven by `ROADMAP-TASKTREE-COVERAGE.2`. Each
milestone is an owned, audited leaf; the audit records the delivering code +
tests + book coverage and confirms the ROADMAP completion criteria are met.
(R0 is already owned by the R0-lane trees; R6+ already have their own trees.)

## Non-Goals

- No code change — audit + ownership backfill only. Any gap an audit uncovers
  becomes its own normal task-tree (no code change without one).
- Not re-documenting the milestones — links to the `ROADMAP.md` sections;
  records only the audit + ownership.

## Acceptance Criteria

- R1–R5 each owned by an audited leaf (delivering modules/commands + test
  counts + book pages verified, ROADMAP criteria confirmed met).
- Registered in `docs/TASK_TREE.md`; this tree `done`.

## Task Tree

- ID: `R1-R5-FOUNDATION-BACKFILL`
  Status: `done`
  Goal: own + audit R1–R5
  Children: `.1`, `.2`, `.3`, `.4`, `.5`

- ID: `R1-R5-FOUNDATION-BACKFILL.1`
  Status: `done`
  Goal: own + audit **R1 — IntentIR pivot and CLI identity** (ROADMAP §R1)
  Acceptance: crate/binary = `specforge`; IntentIR canonical; `.isf` sole adapter.
  Verification: >
    AUDIT passed (`2026-05-31`) — crate/binary name = `specforge`
    (`crates/specforge/Cargo.toml`); `IntentIR` is the canonical endpoint and
    `.isf` the sole adapter target (`IrStage` = …`/IntentIr/IsfAdapter` in
    `ir/mod.rs`; `INTENTIR_SPEC.md` + `README.md` frame `IntentIR` as canonical
    and `.fsm`/HDL as out-of-scope/downstream). Reinforced by
    `ISF-ONLY-CONSOLIDATION` (removed the `.fsm` product boundary). Book:
    `introduction.md` / `architecture-rationale.md` / `pipeline/intentir.md`.
    ROADMAP §R1 completion criteria met.
  Commit: `see Commit Log`

- ID: `R1-R5-FOUNDATION-BACKFILL.2`
  Status: `done`
  Goal: own + audit **R2 — SourceIR** (ROADMAP §R2)
  Acceptance: `ingest` materializes SourceIR; Docling ingest; cleanup; manifests; residuals.
  Verification: >
    AUDIT passed (`2026-05-31`) — `ir/source.rs` (12 `#[test]`s) +
    `ir/source/docling_backend.rs` (real Docling-backed PDF normalization) +
    `commands/ingest.rs` (materializes `generated/source_ir/<key>/source_ir.json`
    + normalized bundles with atomic replace). First-class cleanup via
    `commands/clean.rs`; Docling readiness via `commands/doctor.rs`. Hardened by
    `R6-SOURCE-HARDENING`. Book: `pipeline/sourceir.md` + `runtime-and-doctor.md`.
    ROADMAP §R2 completion criteria met.
  Commit: `see Commit Log`

- ID: `R1-R5-FOUNDATION-BACKFILL.3`
  Status: `done`
  Goal: own + audit **R3 — EvidenceIR** (ROADMAP §R3)
  Acceptance: real EvidenceIR with anchors/spans/visual evidence/statement classification + provenance.
  Verification: >
    AUDIT passed (`2026-05-31`) — `ir/evidence.rs` (120 `#[test]`s) +
    `commands/evidence.rs` (materializes
    `generated/evidence_ir/<key>/evidence_ir.json`): section anchors, evidence
    spans, typed visual evidence, figure/caption linkage, statement
    classification — all provenance-bearing + inspectable. Hardened by
    `R6-EVIDENCE-HARDENING`. Book: `pipeline/evidenceir.md`. ROADMAP §R3 met.
  Commit: `see Commit Log`

- ID: `R1-R5-FOUNDATION-BACKFILL.4`
  Status: `done`
  Goal: own + audit **R4 — SemanticIR** (ROADMAP §R4)
  Acceptance: real backend-neutral SemanticIR (actors/interfaces/typed signals/control fragments/etc.).
  Verification: >
    AUDIT passed (`2026-05-31`) — `ir/semantic.rs` (381 `#[test]`s) +
    `commands/semantic.rs` (materializes
    `generated/semantic_ir/<key>/semantic_ir.json`): actors, interfaces, typed
    signal records, backend-neutral control fragments, system/init records,
    phases, invariants, contracts, gates, abstractions, decomposition
    candidates, residuals — backend-neutral + actor-first. Hardened by
    `R6-SEMANTIC-HARDENING`. Book: `pipeline/semanticir.md`. ROADMAP §R4 met.
  Commit: `see Commit Log`

- ID: `R1-R5-FOUNDATION-BACKFILL.5`
  Status: `done`
  Goal: own + audit **R5 — IntentIR** (ROADMAP §R5)
  Acceptance: real versioned/serializable IntentIR; canonical interface inventory + system contract + fragments carried.
  Verification: >
    AUDIT passed (`2026-05-31`) — `ir/intent.rs` (52 `#[test]`s) +
    `commands/intent.rs` (materializes
    `generated/intent_ir/<key>/intent_ir.json`): canonical intent identity,
    actor responsibilities, interface inventory, backend-neutral system
    contract + init, guarded/action fragments, behaviors, constraints,
    assumptions, residuals — versioned + serde-serializable. Hardened by
    `R6-INTENT-HARDENING` (+ `R6-CONVERGE-HARDENING` for the fixed-point
    entrypoint, `R6-PRIOR-MEMORY-HARDENING` for the prior store). Book:
    `pipeline/intentir.md`. ROADMAP §R5 met.
  Commit: `see Commit Log`

## Current Frontier

**Tree CLOSED `2026-05-31`** — R1–R5 each owned + audited (all delivered).

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R1-R5-FOUNDATION-BACKFILL.1` | `done` | R1 audited |
| 2 | `R1-R5-FOUNDATION-BACKFILL.2` | `done` | R2 audited |
| 3 | `R1-R5-FOUNDATION-BACKFILL.3` | `done` | R3 audited |
| 4 | `R1-R5-FOUNDATION-BACKFILL.4` | `done` | R4 audited |
| 5 | `R1-R5-FOUNDATION-BACKFILL.5` | `done` | R5 audited |

## Decisions

- `2026-05-31`: realized the umbrella's per-milestone ownership as one
  phase-group backfill tree with a leaf per milestone (each milestone is an
  owned, audited leaf-node) — faithful to the doctrine and maintainable. The
  audits are verification-only (these milestones are long delivered + further
  hardened by the R6-*-HARDENING trees).

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-31` | `R1-R5-FOUNDATION-BACKFILL.1` | crate=specforge; IntentIR canonical + `.isf` sole adapter; book covered; ROADMAP §R1 criteria met | `passed` |
| `2026-05-31` | `R1-R5-FOUNDATION-BACKFILL.2` | `ir/source.rs`(12) + docling_backend + `ingest`/`clean`/`doctor`; book covered; ROADMAP §R2 met | `passed` |
| `2026-05-31` | `R1-R5-FOUNDATION-BACKFILL.3` | `ir/evidence.rs`(120) + `evidence`; book covered; ROADMAP §R3 met | `passed` |
| `2026-05-31` | `R1-R5-FOUNDATION-BACKFILL.4` | `ir/semantic.rs`(381) + `semantic`; book covered; ROADMAP §R4 met | `passed` |
| `2026-05-31` | `R1-R5-FOUNDATION-BACKFILL.5` | `ir/intent.rs`(52) + `intent`; book covered; ROADMAP §R5 met | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R1-R5-FOUNDATION-BACKFILL.{1..5}` | `ROADMAP-TASKTREE-COVERAGE.2 — backfill+audit R1–R5 foundation milestones` | audit/ownership only; all delivered + book-covered |

## Changelog

- `2026-05-31`: Created + CLOSED — owned + audited R1–R5 (foundational
  pipeline); all delivered, tested, and book-covered; ROADMAP criteria
  confirmed met. (`ROADMAP-TASKTREE-COVERAGE.2`.)
