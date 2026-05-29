# R16-MODULE-HARDENING: unit-test signoff hardening for the R16 IR modules

## Metadata

- Tree ID: `R16-MODULE-HARDENING`
- Status: `active`
- Roadmap lane: `R16`
- Created: `2026-05-29`
- Last updated: `2026-05-29`
- Owner: repo-local workflow

## Goal

Bring the seven typed IR modules added by the R16 program
(`contract` / `protocol_graph` / `fidelity` / `fusion` / `waveform` /
`figure_region` / `cve`) to the same unit-test signoff bar the rest of
the codebase holds, by backfilling **direct** unit tests for genuine
logic and coverage gaps. This parallels the `R6-*-HARDENING` trees that
did the same for the mature modules. The R16 surface is dormant today
(no live producer) but load-bearing the moment upstream extraction
lands, so locking each module's behavior now prevents silent corruption
later.

## Non-Goals

- No behavior change to production code. This tree adds tests (plus only
  the minimal idiomatic refactor a test legitimately exposes, if any);
  it does not alter R16 contract/extraction semantics.
- No speculative test padding. Every test must lock a real behavior —
  branching, precedence, an edge case, or a serde wire contract. Tests
  for trivial getters are not a goal.
- Does not wire any R16 producer or build the upstream extractors — that
  is the honestly-deferred future-tree work recorded by
  `R16-INTENT-CAPTURE` (waveform raster/vector extraction + CVE
  prose-extractor producer wiring).

## Acceptance Criteria

- Each R16 module either gains direct unit tests covering its genuine
  logic/contract surface, or is recorded here as already-sufficiently
  covered with explicit evidence (existing test count + what it covers).
- `scripts/run_ci.sh` green per leaf (clippy `-D warnings` includes test
  code).
- The tree closes once every R16 module has been assessed and every
  genuine gap closed.

## Task Tree

- ID: `R16-MODULE-HARDENING`
  Status: `active`
  Goal: unit-test signoff hardening across the 7 R16 IR modules
  Children: `R16-MODULE-HARDENING.1`, …

- ID: `R16-MODULE-HARDENING.1`
  Status: `done`
  Goal: >
    Backfill direct unit tests for `ir/figure_region.rs` (0 tests
    today — the only R16 module with none). Lock `inferred_ticks()`
    behavior and the enum-tagged serde wire contract.
  Acceptance: `figure_region.rs carries a #[cfg(test)] module covering inferred_ticks() precedence / derivation / ignore-non-Value / saturation / empty + LaneLevel tagged shape + skip-if-none discipline + full round-trip; scripts/run_ci.sh green.`
  Verification: `passed — see Verification Log`
  Commit: `see Commit Log`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R16-MODULE-HARDENING.1` | `done` | `figure_region.rs` was the only R16 module with 0 direct tests |
| → | (assess remaining 6 modules) | `pending` | audit `contract` / `protocol_graph` / `fidelity` / `fusion` / `waveform` / `cve` coverage; backfill genuine gaps or record as already-sufficient, then close |

## Decisions

- `2026-05-29`: created to apply the established `R6-*-HARDENING`
  unit-test discipline to the newest (R16) modules. Started with
  `figure_region.rs` because it was the single R16 module with **0**
  direct unit tests, and its `inferred_ticks()` carries real branching
  (explicit-`tick_count` precedence, `max+1` derivation from lanes vs
  `Value` annotations, ignoring `Delay`/`Label`/`Unknown` annotations,
  and `saturating_add` overflow safety) whose silent breakage would
  corrupt downstream waveform tick math once extraction lands.
- `2026-05-29`: tree registration and the first leaf (`.1`) landed in the
  same slice — `.1` is bounded and test-only, so separating a docs-only
  scaffold commit from the test commit would add ceremony without review
  value.

## Open Questions

- Whether `cargo-mutants` is available / worth running on the other six
  modules to find genuine missed mutants, vs. an eyeball coverage audit.
  Resolved per-leaf as each module is assessed.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-29` | `R16-MODULE-HARDENING.1` | 10 new `figure_region.rs` unit tests (lib `1128 → 1138`); full `scripts/run_ci.sh` (fmt / clippy-`D` / test-`D` / rustdoc / mdBook) | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R16-MODULE-HARDENING.1` | `R16-MODULE-HARDENING.1 — backfill figure_region.rs unit tests (0 -> 10); create hardening tree` | tree created + first leaf; test-only; zero production behavior change |

## Changelog

- `2026-05-29`: Created. `.1` (`figure_region.rs` unit-test backfill,
  `0 → 10` tests) implemented in the same slice as tree registration
  (bounded, test-only first leaf).
