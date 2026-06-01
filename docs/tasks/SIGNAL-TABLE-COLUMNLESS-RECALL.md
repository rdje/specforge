# SIGNAL-TABLE-COLUMNLESS-RECALL: capture signals from column-less Signal|Description tables

## Metadata

- Tree ID: `SIGNAL-TABLE-COLUMNLESS-RECALL`
- Status: `active` (`.1` done — confirmed diagnosis; `.2` BLOCKED on a design decision)
- Roadmap lane: `R12`/`R15e` (ingest/extraction recall / completeness)
- Created: `2026-06-01`
- Owner: repo-local workflow
- Parent context: `CORPUS-HARDENING` follow-up ("11 unexplained intent-bearing CHI tables").

## Discovery (the virtuous loop in action)

The region-accounting detector flagged **11 unexplained intent-bearing tables** on
CHI. Read-only diagnosis split them into two classes:

- **Class A (4 "timing_parameter" tables: 0036, 0084, 0123, 0136)** — these are
  protocol **requirement / state-transition matrices** ("Requester CompAck
  requirement", "Permitted Write + CMO combinations", "Cache state transitions",
  "Snoop response …"), not timing tables, and Docling marked **every row as a
  header row** (`body_rows` empty), so any extractor reads nothing. A combined
  kind-mismatch + Docling-structure quirk. (Lower value; not this tree.)
- **Class B (signal_description tables: 0233–0236)** — genuine, well-formed
  `Signal | Description` interface tables for the REQ/RSP/SNP/DAT channels
  (`REQFLITPEND`, `REQFLITV`, `REQLCRDV`, …) that produced **0 records**. A real
  recall miss of **core CHI interface signals**. **This tree owns Class B.**

### Confirmed live (failing test, not a stale artifact)

Added `two_column_signal_description_table_synthesizes_declarations`
(`crates/specforge/src/ir/evidence.rs`, `#[ignore]`d) — a minimal reproduction of
`table_0233` (2-col `Signal | Description`, body `REQFLITPEND` / `REQFLITV` /
`REQFLIT[(R-1):0]` / `REQLCRDV`). It builds an `EvidenceIR` and asserts the clean
names are declared. **It fails on current code: `got []`** — zero declarations.

### Root cause

Two gates, both requiring direction OR width:

1. `synthesize_signal_declarations` (evidence.rs ~6273): the text match ends with
   `_ => continue` — a row with **no direction AND no width** is skipped.
2. `parse_explicit_signal_declaration` (semantic.rs ~4448): same — returns `None`
   when `direction_hint.is_none() && width_hint.is_none()`.

For `table_0233`: there is no Width column (→ width `None`), no Direction/Source
column, and `infer_signal_direction_from_section("REQ channel interface signals")`
returns `None` (a *channel*, not a Requester/Completer actor). So every row is
`(None, None)` → skipped → **0 records → table flagged unexplained**.

### The design subtlety (why `.2` is BLOCKED, not auto-fixable)

The signals here are driven by **transmitter / receiver** — and the codebase
**deliberately excludes** abstract-transport signal tables from top-level interface
signals (`table_looks_like_abstract_transport_signal_table`,
`is_abstract_transport_actor_term` = `tx/rx/transmitter/receiver`). So capturing
these would *reverse* an existing precision philosophy. The direction is also only
partially recoverable from prose ("the transmitter sets this signal HIGH" →
`REQFLITV`/`REQLCRDV`; but `REQFLITPEND` has no such verb). And a broad relaxation
of the direction-OR-width contract **cannot be corpus-validated right now**
(`.venv-docling` absent + corpus tree moved → no re-ingest; most `normalized`
bundles reclaimed). Hence a user design decision is required before implementing.

## Candidate approaches (for the design decision)

- **A. Prose-direction inference (safe, partial).** When no direction/width column,
  scan the Description cell for "transmitter/receiver/driven by/asserted by sets
  this signal" → direction. No contract change (still requires direction). Captures
  `REQFLITV`/`REQLCRDV`; leaves `REQFLITPEND` an honest residual. Unit-testable; no
  re-ingest needed. Clears the region-accounting finding (≥1 record per table).
- **B. Bare-existence capture (full recall, contract change).** Relax both gates to
  emit a directionless/widthless declaration for valid signal tokens in tables that
  pass the strong `should_treat_…` gate. Captures all three. Broad precision
  implication; not corpus-validatable now.
- **C. Leave as deliberate residual.** Treat transmitter/receiver flit-level link
  signals as out of scope for the top-level interface inventory (consistent with the
  abstract-transport exclusion); the region-accounting flag is then a *true*
  honest residual, not a bug. Document and move on.

## Non-Goals

- Class A (timing-matrix mis-kind + empty body) — separate finding, not this tree.
- Changing the region-accounting detector — it is correct; the question is capture.

## Task Tree

- ID: `SIGNAL-TABLE-COLUMNLESS-RECALL`
  Status: `active`
  Children: `.1`, `.2`

- ID: `SIGNAL-TABLE-COLUMNLESS-RECALL.1`
  Status: `done`
  Goal: own + diagnose Class B; confirm live (failing `#[ignore]`d reproduction);
    root-cause the dual direction-OR-width gate; surface the design fork. Docs+test.
  Verification: >
    passed (`2026-06-01`) — diagnosed the 11 unexplained CHI tables into Class A
    (timing-matrix mis-kind + empty body) and Class B (real 2-col signal tables).
    Confirmed live via `two_column_signal_description_table_synthesizes_declarations`
    (fails `got []` un-ignored). Root cause = direction-OR-width gate at both the
    evidence (`synthesize_signal_declarations`) and semantic
    (`parse_explicit_signal_declaration`) stages. Abstract-transport-philosophy
    nuance + no-re-ingest validation constraint documented → `.2` needs a decision.
  Commit: `see Commit Log`

- ID: `SIGNAL-TABLE-COLUMNLESS-RECALL.2`
  Status: `blocked` (awaiting design decision: approach A / B / C)
  Goal: implement the chosen approach; un-ignore the reproduction test; add unit
    tests; (corpus-validate if feasible); CI; book note; close.
  Acceptance: chosen approach implemented; reproduction test un-ignored + passing
    (or, for C, the residual documented and the test removed/kept as a known-by-
    design marker); CI green; book; tree CLOSED.
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `SIGNAL-TABLE-COLUMNLESS-RECALL.1` | `done` | owned + confirmed-live diagnosis + design fork surfaced |
| 2 | `SIGNAL-TABLE-COLUMNLESS-RECALL.2` | `blocked` | needs the user's precision/recall design decision (A/B/C) |

## Decisions

- `2026-06-01`: scope this tree to Class B (real signal tables); Class A stays a
  separate `CORPUS-HARDENING` follow-up.
- `2026-06-01`: do NOT auto-implement — the fix reverses a deliberate
  abstract-transport precision philosophy and cannot be corpus-validated now;
  surface the fork for a user decision.

## Open Questions

- Which approach (A / B / C)? Materially changes the pipeline's precision/recall
  contract for column-less signal tables across every spec.

## Blockers

- `.2`: design decision (A/B/C). Full corpus re-validation also blocked on Docling
  availability (`.venv-docling` absent; corpus tree moved).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-01` | `.1` | diagnosed 11 unexplained CHI tables (Class A/B); confirmed live recall gap via `#[ignore]`d failing reproduction; root-caused dual direction-OR-width gate; documented design fork; CI green (test ignored) | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `SIGNAL-TABLE-COLUMNLESS-RECALL.1` | `SIGNAL-TABLE-COLUMNLESS-RECALL.1 — own + confirm live column-less signal-table recall gap (CHI channels); surface design fork` | docs + ignored reproduction test |

## Changelog

- `2026-06-01`: Created — diagnosed CHI's unexplained signal tables to a confirmed
  live recall gap (column-less `Signal|Description` tables yield 0 declarations);
  `.2` blocked on a precision/recall design decision.
