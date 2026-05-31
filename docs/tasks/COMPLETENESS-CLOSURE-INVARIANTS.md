# COMPLETENESS-CLOSURE-INVARIANTS: first completeness miss-detectors (symbol closure + register tiling)

## Metadata

- Tree ID: `COMPLETENESS-CLOSURE-INVARIANTS`
- Status: `active`
- Roadmap lane: `R15d` (arbitration/closure) — first implementation from `INTENT-COMPLETENESS-RESEARCH`
- Created: `2026-05-31`
- Last updated: `2026-05-31`
- Owner: repo-local workflow

## Goal

The **first implementation** from the completeness research program (user chose
"closure invariants" as where coding begins). Build the cheapest, EXACT,
ground-truth-free miss detectors from the catalog
([`miss-detectors-catalog.md`](../research/miss-detectors-catalog.md) C1/C2) and
surface each detected miss as an explicit `validate` finding (the established
residual surface). This proves the whole **detect-a-miss → surface-a-residual →
validate** loop end-to-end on real specs, at minimal risk, and establishes the
plumbing future detectors plug into.

Two detectors:
- **Register bit-tiling (C2, EXACT):** within a `RegisterRecord`, the documented
  fields must not overlap, and must have no *interior* gap (an uncovered bit
  between the lowest and highest documented field). Overlap = a real defect; an
  interior gap = a likely **missed field**. (No register width is declared in the
  IR, so we deliberately do NOT flag bits above the highest field — that would
  require speculating width; conservative by construction.)
- **Symbol closure (C1, GATED):** a signal *referenced* (constraint subject,
  conditional/temporal rule, actor-signal relation) that is absent from the
  canonical signal inventory = a **dangling reference** (a missed declaration or a
  doc defect). Gated to avoid flagging legitimately-external signals.

## Non-Goals

- NOT building the full `CompletenessReport` yet (that is research `.5` + a later
  tree) — these detectors surface as `validate` findings + metrics, exactly like
  the existing polarity/semantic/connectivity conflict surfaces.
- NOT inventing facts — detectors only FLAG; they never mutate the IR.
- NOT assuming a register width (avoid width-speculation false positives).

## Acceptance Criteria

- A pure, unit-tested `ir/completeness.rs` module with the two detectors as pure
  functions over the existing IR types (no I/O).
- Register tiling: overlaps + interior gaps detected; conservative (no width
  assumption); surfaced in `validate` (EvidenceIR stage, where `register_records`
  live) as findings + metrics.
- Symbol closure: dangling references detected against the canonical inventory,
  gated to suppress known-external/false-positive cases; surfaced in `validate`.
- Behavior-neutral on extraction; full `scripts/run_ci.sh` green; book subsection
  in the topically-correct chapter (`quality/validation.md` or `pipeline/*`).

## Task Tree

- ID: `COMPLETENESS-CLOSURE-INVARIANTS`
  Status: `active`
  Goal: first completeness detectors (symbol closure + register tiling) surfaced in validate
  Children: `.1`, `.2`, `.3`, `.4`

- ID: `COMPLETENESS-CLOSURE-INVARIANTS.1`
  Status: `done`
  Goal: own + design (this file); record the first-slice decision in `INTENT-COMPLETENESS-RESEARCH.7`; register. Docs-only.
  Acceptance: tree created + registered; research `.7` records the chosen backlog + first slice.
  Verification: pending
  Commit: `see Commit Log`

- ID: `COMPLETENESS-CLOSURE-INVARIANTS.2`
  Status: `pending`
  Goal: >
    Register bit-tiling detector: new `ir/completeness.rs` with a pure
    `register_tiling_residuals(&[RegisterRecord]) -> Vec<…>` (overlap + interior
    gap, conservative, no width assumption); unit tests over clean/overlap/gap/
    sparse cases; wire into `validate_evidence_ir` (findings + metrics). Full CI.
  Acceptance: detector + tests + validate surface; CI green.
  Verification: pending
  Commit: pending

- ID: `COMPLETENESS-CLOSURE-INVARIANTS.3`
  Status: `pending`
  Goal: >
    Symbol-closure / dangling-reference detector: pure fn computing referenced
    signals (constraint subjects, conditional/temporal rule targets, actor-signal
    relations) minus the canonical inventory (declared ports/signals), GATED to
    suppress legitimately-external/uppercase-noise cases; unit tests; wire into
    `validate`. Full CI.
  Acceptance: detector + tests + validate surface (gated); CI green.
  Verification: pending
  Commit: pending

- ID: `COMPLETENESS-CLOSURE-INVARIANTS.4`
  Status: `pending`
  Goal: book subsection (how the closure detectors work + why they matter, user-friendly) + close; refresh research `.7`/program status.
  Acceptance: book updated; full CI green; tree CLOSED.
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `COMPLETENESS-CLOSURE-INVARIANTS.1` | `done` | tree + design + research `.7` decision recorded |
| 2 | `COMPLETENESS-CLOSURE-INVARIANTS.2` | `pending` | register tiling — cleanest exact detector — next |
| 3 | `COMPLETENESS-CLOSURE-INVARIANTS.3` | `pending` | symbol closure (gated) |
| 4 | `COMPLETENESS-CLOSURE-INVARIANTS.4` | `pending` | book + close |

## Decisions

- `2026-05-31`: surface detectors as `validate` findings + metrics (not a new
  persisted report yet) — consistent with the existing conflict surfaces; the
  unifying `CompletenessReport` is deferred to research `.5` + a later tree.
- `2026-05-31`: register tiling checks only `[min_field, max_field]` (overlaps +
  interior gaps); no width speculation → exact, no width-induced false positives.
- `2026-05-31`: register tiling first (EXACT, lowest-risk), symbol closure second
  (GATED, needs careful inventory/reference definition + external-signal gating).

## Open Questions

- Which stage owns symbol closure — EvidenceIR (relations/constraints present) vs
  SemanticIR/IntentIR (canonical `actor_ports` inventory present)? Likely the
  stage where BOTH the inventory and the references coexist; resolve in `.3`.
- Interior-gap severity: Info vs Warning? (Lean Info — a gap is a *candidate*
  missed field, not a proven defect; overlap = Warning, a real contradiction.)

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-31` | `.1` | tree created + registered; research `.7` decision (closure-invariants first) recorded; docs-only (CI invariant) | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `COMPLETENESS-CLOSURE-INVARIANTS.1` | `COMPLETENESS-CLOSURE-INVARIANTS.1 — own the first completeness detectors + record the first-slice decision` | docs-only |

## Changelog

- `2026-05-31`: Created — first implementation from `INTENT-COMPLETENESS-RESEARCH`
  (user chose closure invariants). Register tiling (EXACT) then symbol closure
  (GATED), surfaced as `validate` findings. Frontier → `.2` (register tiling).
