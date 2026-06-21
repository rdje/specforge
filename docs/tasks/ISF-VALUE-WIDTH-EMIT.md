# ISF-VALUE-WIDTH-EMIT: width-align ISF value literals to the declared signal width

## Metadata

- Tree ID: `ISF-VALUE-WIDTH-EMIT`
- Status: `active`
- Roadmap lane: `R6` × `R15`/`R16` (ISF emitter fidelity; KG-ISF-COMPLETENESS bar #6)
- Created: `2026-06-21`
- Last updated: `2026-06-21`
- Owner: repo-local workflow

## Goal

Make the emitted `.isf` strict-valid for `(rule … (SIGNAL value))` clauses where the value literal's
notation width differs from the target signal's declared width. FSMGen's OperandContract (pin
`030f8c273`) blocks implicit truncation/extension and requires an "explicit width-aligned source
expression". The faithful fix is to (1) emit the signal at its grounded width and (2) re-render the value
literal width-aligned to that width **when the value fits**, else **residualize** — never truncate or
fabricate. Directly serves the `KG-ISF-COMPLETENESS` north star bar #6 (every IntentIR fact reaches the
`.isf` or an explicit residual).

## Non-Goals

- The DTI **ATST mis-attribution** (the IntentIR constraint binds SHCFG's `0b01` to ATST) — an upstream
  extraction concern, spun out, not an emitter fix.
- The AXI **`(port expr)` rule-assignment grammar** failure (the `ISF-RULE-CONFLICT` / constraint-action
  class) that masks the AXI width cases — a separate emitter lever.
- Any signal/value name list — ADR-0006: numeric parsing + width arithmetic only.

## Acceptance Criteria

- A value literal whose notation width ≠ the signal's emitted width is re-rendered as a width-aligned
  width-cast `W'<radix><digits>` when `value < 2^W`, else the clause is dropped with an explicit adapter
  residual (`ISF-RULE-CONFLICT-RESIDUAL`-style). Bare/unsized decimals untouched.
- The signal's emitted width recovers a single unambiguous concrete width from any interface
  `signal_records` **and** `actor_ports` (not just the first-seen record); a conflict keeps width-1.
- DTI + TRACE `.isf` clear the OperandContract value-width error for the affected clauses (verified with
  the real `subs/fsmgen/bin/fsmgen --strict --check --json`).
- WIRE-BASED-100 held 1.000 (orthogonal — emitter-only); `kg-bench` 156/156; `scripts/run_ci.sh` GREEN.
- Book `pipeline/isf-adapter.md` "Closed task trees" subsection refreshed on close (BOOK-METHOD-DOC).
- Each completed leaf committed through `COMMIT.md`.

## Task Tree

- ID: `ISF-VALUE-WIDTH-EMIT`
  Status: `active`
  Goal: width-align ISF value literals to the declared signal width (honest residual otherwise)
  Children: `.0` `.1` `.2`

- ID: `ISF-VALUE-WIDTH-EMIT.0`
  Status: `done`
  Goal: own the value-width emitter-fidelity candidate (ownership/scoping; new tree)
  Acceptance: tree file with the checkable bar + honest-residual policy + the FSMGen-contract framing
  Verification: `done 2026-06-21` (docs-only)
  Commit: `ISF-VALUE-WIDTH-EMIT.0/.1`

- ID: `ISF-VALUE-WIDTH-EMIT.1`
  Status: `done`
  Goal: corpus measurement + FSMGen probe → GO/NO-GO + fix design (read-only, docs-only)
  Acceptance: per-case classification across the 86 `.isf`; the exact FSMGen contract probed; the fix
    design and gates recorded; report written
  Verification: `done 2026-06-21` — report `docs/research/isf-value-width-alignment-measurement.md`;
    KM `[[isf-value-width-operand-contract]]`
  Commit: `ISF-VALUE-WIDTH-EMIT.0/.1`

- ID: `ISF-VALUE-WIDTH-EMIT.2`
  Status: `pending`
  Goal: emit (CODE) — width recovery completeness + value-literal width-alignment + honest residual
  Acceptance: the `.2` gates above all green; compile-gated (needs `cargo build` — held until the host
    has RAM headroom)
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `ISF-VALUE-WIDTH-EMIT.2` | `pending` | the emit slice — `.1` measured GO with a complete, probe-validated fix design; needs a `cargo build`, so held until the host has RAM headroom (the 6.3 GB dev host is RAM-tight; this measurement-first slice is RAM-safe) |

## Decisions

- `2026-06-21`: **GO** with the two-part faithful fix (width recovery completeness + value-literal
  width-alignment-or-residual). The naive "truncate the value to fit width" is REJECTED — it would
  corrupt genuine values (ATID `0x7D` = 125 is real; ATID's true width is 7, dropped at emit). FSMGen
  reads a literal's width by **notation digit count** (`0x7D` = 8 bits, `0b00` = 2 bits), not value, and
  requires an exact width-cast `W'…` match; bare decimals are unsized and safe.
- `2026-06-21`: dedicated tree (not a `KG-ISF-COMPLETENESS` leaf) mirroring the closed
  `ISF-REGISTER-RESET-EMIT` `ISF-*-EMIT` precedent — emitter-fidelity is its own ownership boundary.

## Open Questions

- None blocking. `.2` is fully designed; the only gate is host RAM for the `cargo build`.

## Blockers

- None (the `.2` build is held on RAM headroom, not blocked — it is a scheduling constraint, not an
  ambiguity).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-21` | `.0`/`.1` | read-only corpus scan (86 `.isf` + intent_ir) + FSMGen `--strict --check` probes (Perl, RAM-safe) + `scripts/check_memory_architecture.sh` + knowledge-map derive-and-diff | DONE — GO; DTI/TRACE confirmed FAIL on OperandContract; fix design probe-validated; docs-only, no code/CI/`.isf` change |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0`/`.1` | `ISF-VALUE-WIDTH-EMIT.0/.1 — own + measure ISF value-literal width-alignment (GO)` | docs-only measurement-first slice |
| `.2` | `pending` | pending (compile-gated; held on RAM) |

## Changelog

- `2026-06-21`: Created task tree; `.0` (ownership/scoping) + `.1` (measurement → GO) done as one
  measurement-first docs-only slice. Surfaced by the `CORPUS-COVERAGE.2` re-ingest sweep ("Lever A").
