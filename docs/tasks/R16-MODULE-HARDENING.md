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

- ID: `R16-MODULE-HARDENING.2`
  Status: `done`
  Goal: >
    Audit the remaining 6 R16 modules' unit-test coverage and backfill
    the cleanest genuine gaps. (Full audit recorded in Decisions
    `2026-05-29`.)
  Acceptance: `All 6 remaining modules assessed; cve.rs recorded well-covered; the clean genuine gaps (waveform capped_confidence cap + single-tick ValueSpan skip; protocol_graph phase() None) backfilled; deeper-construction gaps recorded as .3; scripts/run_ci.sh green.`
  Verification: `passed — see Verification Log`
  Commit: `see Commit Log`

- ID: `R16-MODULE-HARDENING.3`
  Status: `done`
  Goal: >
    Backfill the fidelity gap from the `.2` audit:
    `evaluate_figure_conformance` trace=Some Pass / Fail / obligation-
    unsupported (NotEvaluated) glue (only the trace=None path was tested).
  Acceptance: `fidelity test covers the three trace=Some paths through evaluate_figure_conformance (Pass; Fail-with-message; unsupported obligation → NotEvaluated, never silently Pass); scripts/run_ci.sh green.`
  Verification: `passed — see Verification Log`
  Commit: `see Commit Log`

- ID: `R16-MODULE-HARDENING.4`
  Status: `pending`
  Goal: >
    Backfill the remaining deeper-construction gaps from the `.2` audit,
    then close the tree: (a) contract `contract_from_temporal_rule`
    untested consequent arms (windowed non-HandshakeComplete →
    empty-signal `Observe`; non-windowed `ActorMaintainsSignalStable` /
    `ActorSamplesSignal` / `SignalSampled`); (b) fusion guard_candidates
    dedup on merge + `apply_fusion` no-multi-cluster early-exit.
  Acceptance: `Each gap gets a direct test (or is recorded with evidence as already indirectly locked); scripts/run_ci.sh green; then close the tree (all 7 R16 modules assessed + gaps closed/recorded).`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R16-MODULE-HARDENING.1` | `done` | `figure_region.rs` was the only R16 module with 0 direct tests |
| 2 | `R16-MODULE-HARDENING.2` | `done` | audited all 6 remaining modules; `cve` well-covered; backfilled the 3 clean gaps |
| 3 | `R16-MODULE-HARDENING.3` | `done` | fidelity `evaluate_figure_conformance` Pass/Fail/unsupported glue |
| → | `R16-MODULE-HARDENING.4` | `pending` | **Real frontier** — contract consequent arms + fusion dedup/early-exit, then close the tree |

## Decisions

- `2026-05-29` (`.2` audit): a conservative coverage audit of the 6
  remaining R16 modules found `cve.rs` **well-covered** (23 tests, all
  branches) and **minor genuine gaps** in the other 5:
  - `waveform.rs`: `capped_confidence` (High→Medium cap) untested
    directly; single-tick `ValueSpan` (`from==to`) skip untested →
    **both backfilled in `.2`**.
  - `protocol_graph.rs`: `phase()` None (unknown-id) path untested →
    **backfilled in `.2`**.
  - `fidelity.rs`: `evaluate_figure_conformance` trace=Some Pass/Fail
    glue untested (only NotEvaluated tested) → **deferred to `.3`**
    (needs a satisfying/violating `FigureTrace` + `Stable` contract).
  - `contract.rs`: `contract_from_temporal_rule` arms (windowed
    non-HandshakeComplete → empty-signal `Observe`; non-windowed
    `ActorMaintainsSignalStable` / `ActorSamplesSignal` / `SignalSampled`)
    untested → **deferred to `.3`**.
  - `fusion.rs`: guard_candidates dedup on merge + `apply_fusion`
    no-multi-cluster early-exit untested → **deferred to `.3`**.
  These are dormant-code gaps (marginal but genuine); `.2` closed the
  clean ones, `.3` owns the deeper-construction ones. Full mutation
  testing (`cargo-mutants`, available here) remains an option for deeper
  rigor if later desired.

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
| `2026-05-29` | `R16-MODULE-HARDENING.2` | coverage audit of 6 modules + 3 backfill tests (waveform `capped_confidence` + single-tick span; protocol_graph `phase()` None); lib `1138 → 1141`; full `scripts/run_ci.sh` | `passed` |
| `2026-05-29` | `R16-MODULE-HARDENING.3` | 1 fidelity test (`evaluate_figure_conformance` Pass / Fail / unsupported); lib `1141 → 1142`; full `scripts/run_ci.sh` | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R16-MODULE-HARDENING.1` | `R16-MODULE-HARDENING.1 — backfill figure_region.rs unit tests (0 -> 10); create hardening tree` | tree created + first leaf; test-only; zero production behavior change |
| `R16-MODULE-HARDENING.2` | `R16-MODULE-HARDENING.2 — audit remaining 6 R16 modules + backfill clean gaps (3 tests)` | test-only; cve well-covered; deeper gaps → `.3`/`.4` |
| `R16-MODULE-HARDENING.3` | `R16-MODULE-HARDENING.3 — backfill fidelity evaluate_figure_conformance Pass/Fail glue test` | test-only; contract + fusion gaps → `.4` |

## Changelog

- `2026-05-29`: `.3` — backfilled the fidelity gap
  (`evaluate_figure_conformance` trace=Some Pass / Fail / unsupported-
  NotEvaluated glue); contract + fusion gaps moved to `.4`.
- `2026-05-29`: `.2` — audited the 6 remaining R16 modules; backfilled
  3 clean gaps (waveform `capped_confidence`, single-tick `ValueSpan`;
  protocol_graph `phase()` None); recorded `cve.rs` well-covered and the
  deeper-construction gaps (fidelity Pass/Fail; contract consequent arms;
  fusion dedup/early-exit) as `.3`.
- `2026-05-29`: Created. `.1` (`figure_region.rs` unit-test backfill,
  `0 → 10` tests) implemented in the same slice as tree registration
  (bounded, test-only first leaf).
