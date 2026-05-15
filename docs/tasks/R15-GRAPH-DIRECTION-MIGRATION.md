# R15-GRAPH-DIRECTION-MIGRATION: Complete actor-relative graph direction migration

## Metadata

- Tree ID: `R15-GRAPH-DIRECTION-MIGRATION`
- Status: `active`
- Roadmap lane: `R15`
- Created: `2026-05-15`
- Last updated: `2026-05-15`
- Owner: repo-local workflow

## Goal

Finish replacing the remaining flat `direction_hint` consumers with actor-relative graph semantics across the adapter, validation, and semantic stages. This is the highest-priority remaining gap per LIVE_ACHIEVEMENT_STATUS.md.

## Non-Goals

- Do not widen backend scope (no new adapter targets).
- Do not remove the `direction_hint` field — it remains as a compatibility surface.
- Do not change the canonical data model.
- Do not break existing tests or fixtures.

## Background

`SemanticIR` and `IntentIR` now carry `actor_signal_relations`, `actor_ports`, and `signal_connectivity` alongside flat `direction_hint`. The `.fsm` adapter already has a `preferred_signal_direction_hint` resolver that prefers graph direction over flat. The remaining work is ensuring every consumer path uses graph-first resolution, particularly in renderability checks (size entries, output registration, control-read direction, etc.).

## Acceptance Criteria

- All identified remaining `direction_hint` consumers are audited.
- Consumers that should use graph-first resolution are converted.
- All 1014 tests continue to pass.
- All 151 kg-bench fixtures continue to pass.
- No production behavior regressions.

## Task Tree

- ID: `R15-GRAPH-DIRECTION-MIGRATION`
  Status: `complete`
  Goal: `Complete graph-first direction migration.`
  Children: `R15-GRAPH-DIRECTION-MIGRATION.1`, `R15-GRAPH-DIRECTION-MIGRATION.2`, `R15-GRAPH-DIRECTION-MIGRATION.3`

### Batch 1: Audit remaining consumers

- ID: `R15-GRAPH-DIRECTION-MIGRATION.1`
  Status: `complete`
  Goal: `Audit all production-code direction_hint consumers across the codebase to identify those not yet behind graph-first resolution.`
  Acceptance: `Inventory of all direction_hint reads outside test code, categorized by whether they use graph-first resolution or raw flat direction.`
  Files to audit: `adapters.rs`, `validate.rs`, `semantic.rs`, `kg_bench.rs`, `learn_priors.rs`
  Findings:
  - **adapters.rs**: 125 total accesses. Production code (~42) categorized: GRAPH-FIRST (8: lines 1416-1432, 5136, 2845-2880, 5264), COMPAT (28: data ingestion, evidence merging, conflict detection, metrics), TO-MIGRATE (2: lines 2917-2919 resolved port direction, line 5462 render_top_port_token). Test code (~83 lines 10731-27138).
  - **semantic.rs**: 43 total accesses. All STRUCTURAL (field definitions, parsing, accumulation). Test code at lines 10718+.
  - **validate.rs**: 38 total accesses. All COMPAT (metrics/reporting on compat vs graph direction coverage). Test code at lines 7913+.
  - **kg_bench.rs**: 16 total accesses. All TEST/FIXTURE (fixture patch application, assertion checks).
  - **learn_priors.rs**: 7 total accesses. All TEST (test fixture setup).
  - **VERDICT**: Only 2 TO-MIGRATE sites, both in adapters.rs. The top-port resolved direction at line 2917 should prefer graph_top_port_directions over top_port_directions. Downstream consumer render_top_port_token (line 5462) is automatically fixed.

### Batch 2: Convert adapter renderability consumers

- ID: `R15-GRAPH-DIRECTION-MIGRATION.2`
  Status: `complete`
  Goal: `Convert remaining adapter renderability direction_hint consumers to use preferred_signal_direction_hint.`
  Acceptance: `All size-entry, output-registration, and control-read consumers in adapters.rs use graph-first resolution.`
  Depends on: `R15-GRAPH-DIRECTION-MIGRATION.1`
  Implementation: `Changed resolved_port.direction_hint in analyze_top_renderability (line ~2917) to prefer graph_top_port_directions over top_port_directions, with conflict-aware resolution mirroring preferred_signal_direction_hint(). Returns None when flat and graph disagree or either is conflicted.`

### Batch 3: Convert validation and semantic consumers

- ID: `R15-GRAPH-DIRECTION-MIGRATION.3`
  Status: `complete`
  Goal: `Convert remaining validation and semantic direction_hint consumers where appropriate.`
  Acceptance: `Validation metrics correctly report graph coverage. Semantic consumers prefer graph direction.`
  Depends on: `R15-GRAPH-DIRECTION-MIGRATION.2`
  Resolution: `Audit confirmed no TO-MIGRATE consumers in validate.rs (all metric/reporting), semantic.rs (all structural/parsing/accumulation), kg_bench.rs (all test/fixture), or learn_priors.rs (all test). No code changes required.`

## Current Frontier

- Tree complete. All three leaves resolved.
