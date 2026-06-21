# ISF-VALUE-WIDTH-EMIT: width-align ISF value literals to the declared signal width

## Metadata

- Tree ID: `ISF-VALUE-WIDTH-EMIT`
- Status: `done` (CLOSED `2026-06-21`)
- Roadmap lane: `R6` × `R15`/`R16` (ISF emitter fidelity; KG-ISF-COMPLETENESS bar #6)
- Created: `2026-06-21`
- Last updated: `2026-06-21`
- Closed: `2026-06-21`
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
  Status: `done` (CLOSED `2026-06-21`)
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
  Status: `done` (`2026-06-21`)
  Goal: emit (CODE) — width recovery completeness + value-literal width-alignment + honest residual
  Acceptance: the `.2` gates above all green; compile-gated (needs `cargo build` — held until the host
    has RAM headroom)
  Verification: `passed 2026-06-21` — two faithful, ADR-0006 numeric-only fixes in `ir/isf_ir.rs`:
    (1) a new `interface_widths` aggregate (single unambiguous concrete `Numeric > 1` width across ALL of
    a signal's `interfaces[].signal_records`, conflict→width-1) chained ahead of the `.2a.i` `port_widths`
    in the width fallback, so a width living in a NON-FIRST interface record is recovered; (2) an
    `align_rule_drive_widths` post-pass (before `dedup_conflicting_rules`) with `parse_sized_literal` /
    `align_value_to_width` / `ValueAlign` / `value_width_residual_packet` — a based literal (`0b…`/`0x…`)
    whose notation width ≠ the signal's emitted width is re-rendered `W'd<v>` when `value < 2^W`, else the
    rule is DROPPED with an honest `isf_value_width_*` residual (never truncate). Verified with the real
    `subs/fsmgen/bin/fsmgen --strict --check --json`: **DTI `(ATST 1'd1)`** (was `2'b1`) and **trace-bus
    `(output ATID (width 7))` + `(ATID 7'd125)`** (was width-1 + `8'h7D`) both now report
    `has_diagnostics: false` — the OperandContract value-width error is CLEARED. The 4 wire golds carry
    **0 NEW** FSMGen diagnostics (APB 0/0, SWD 0/0 unchanged; AHB 1/1 + AXI 1/1 are the pre-existing
    ORTHOGONAL `isf_conflicting_rule_writes`/`(port expr)` issues, count unchanged). 47/47 `isf_ir` unit
    tests (incl. 3 new + 5 `*_passes_fsmgen_strict_validation` canaries); full suite **1682 passed / 0
    failed**; `scripts/run_ci.sh` GREEN (fmt + clippy `-D warnings` + rustdoc + mdBook); `kg-bench`
    **156/156**. WIRE-BASED-100 orthogonal (emitter-only — measures extraction F1, not `.isf` bytes).
  Commit: `ISF-VALUE-WIDTH-EMIT.2 — emit value-width-aligned ISF literals + complete width recovery (CLOSE)`

## Current Frontier

**Tree CLOSED `2026-06-21`.** All leaves done; the emit slice `.2` landed with every gate green
(DTI + TRACE clear the OperandContract value-width error on the real FSMGen `--strict --check`; 0 new
wire-gold diagnostics; CI + `kg-bench` 156/156). The book close-rule subsection is in
`docs/book/src/pipeline/isf-adapter.md`. No frontier remains.

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `ISF-VALUE-WIDTH-EMIT.2` | `done` (`2026-06-21`) | the emit slice — landed once the host had RAM headroom; all `.2` gates green (CLOSES the tree) |

## Decisions

- `2026-06-21`: **GO** with the two-part faithful fix (width recovery completeness + value-literal
  width-alignment-or-residual). The naive "truncate the value to fit width" is REJECTED — it would
  corrupt genuine values (ATID `0x7D` = 125 is real; ATID's true width is 7, dropped at emit). FSMGen
  reads a literal's width by **notation digit count** (`0x7D` = 8 bits, `0b00` = 2 bits), not value, and
  requires an exact width-cast `W'…` match; bare decimals are unsized and safe.
- `2026-06-21`: dedicated tree (not a `KG-ISF-COMPLETENESS` leaf) mirroring the closed
  `ISF-REGISTER-RESET-EMIT` `ISF-*-EMIT` precedent — emitter-fidelity is its own ownership boundary.

## Open Questions

- None. `.2` landed and the tree is closed.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-21` | `.0`/`.1` | read-only corpus scan (86 `.isf` + intent_ir) + FSMGen `--strict --check` probes (Perl, RAM-safe) + `scripts/check_memory_architecture.sh` + knowledge-map derive-and-diff | DONE — GO; DTI/TRACE confirmed FAIL on OperandContract; fix design probe-validated; docs-only, no code/CI/`.isf` change |
| `2026-06-21` | `.2` | `cargo build` (debug, single-job, RAM-safe @ 81% free) → 47/47 `isf_ir` tests; regenerated DTI + TRACE `.isf` and ran real `subs/fsmgen/bin/fsmgen --strict --check --json`; regenerated 4 wire golds and re-checked diagnostics; `scripts/run_ci.sh`; `kg-bench` | PASSED — DTI `(ATST 1'd1)` + TRACE `(output ATID (width 7))`/`(ATID 7'd125)` both `has_diagnostics: false` (value-width error CLEARED); wire golds 0 NEW diagnostics (APB 0/0, SWD 0/0, AHB 1/1 + AXI 1/1 pre-existing orthogonal); full suite 1682 passed / 0 failed; CI GREEN; `kg-bench` 156/156 |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0`/`.1` | `ISF-VALUE-WIDTH-EMIT.0/.1 — own + measure ISF value-literal width-alignment (GO)` | docs-only measurement-first slice |
| `.2` | `ISF-VALUE-WIDTH-EMIT.2 — emit value-width-aligned ISF literals + complete width recovery (CLOSE)` | CODE — emitter-only; closes the tree |

## Changelog

- `2026-06-21`: Created task tree; `.0` (ownership/scoping) + `.1` (measurement → GO) done as one
  measurement-first docs-only slice. Surfaced by the `CORPUS-COVERAGE.2` re-ingest sweep ("Lever A").
- `2026-06-21`: **Tree CLOSED.** `.2` landed (CODE, emitter-only) once the host had RAM headroom (81% free
  by `memory_pressure`). Two ADR-0006 numeric-only fixes in `ir/isf_ir.rs`: an `interface_widths`
  aggregate that recovers a grounded width from a NON-FIRST interface `signal_record` (trace-bus `ATID`
  width 7), chained ahead of the `.2a.i` `port_widths`; and an `align_rule_drive_widths` post-pass that
  re-renders an over-wide-but-fitting based literal as `W'd<v>` (DTI `0B01`→`1'd1`, trace `0x7D`→`7'd125`)
  or DROPS the rule with an honest `isf_value_width_*` residual when the value overflows the signal width
  (never truncate). Verified on the real FSMGen `--strict --check --json`: DTI + TRACE both clear the
  OperandContract value-width error (`has_diagnostics: false`); 0 new wire-gold diagnostics; full test
  suite 1682 passed / 0 failed; `scripts/run_ci.sh` GREEN; `kg-bench` 156/156. The AXI `(port expr)`
  grammar lever and the DTI ATST upstream mis-attribution stay spun out (Non-Goals). Book close-rule
  subsection added to `docs/book/src/pipeline/isf-adapter.md`.
