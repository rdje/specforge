# PROVENANCE-HARDENING: Test Assertion Coverage For Provenance-Like IR Fields

## Metadata

- Tree ID: `PROVENANCE-HARDENING`
- Status: `active`
- Roadmap lane: `R6`
- Created: `2026-05-14`
- Last updated: `2026-05-14`
- Owner: repo-local workflow

## Goal

Ensure every provenance-like field (`supporting_*_ids`, `automation_confidence`,
`strongest_automation_confidence`, provenance vectors, evidence-span IDs) on IR
record types has at least one non-empty test assertion where the field is
populated in production.

## Non-Goals

- Do not change production behavior. This is regression-only hardening.
- Do not add test coverage to record types with zero existing test coverage
  (phases, contracts, assertions, abstractions, decomposition_candidates in
  semantic.rs).
- Do not add assertions to fields already well-tested by existing checks.
- Do not add assertions that require new test fixture infrastructure.

## Acceptance Criteria

- Every provenance-like field family in the IR/adapter surface has at least one
  non-empty test assertion.
- 666/666 tests pass after every completed leaf.
- Each leaf is committed through `COMMIT.md`.

## Task Tree

- ID: `PROVENANCE-HARDENING`
  Status: `active`
  Goal: `Achieve systematic test assertion coverage for all provenance-like fields.`
  Children: `PROVENANCE-HARDENING.1`, `PROVENANCE-HARDENING.2`,
  `PROVENANCE-HARDENING.3`, `PROVENANCE-HARDENING.4`, `PROVENANCE-HARDENING.5`,
  `PROVENANCE-HARDENING.6`, `PROVENANCE-HARDENING.7`, `PROVENANCE-HARDENING.8`,
  `PROVENANCE-HARDENING.9`, `PROVENANCE-HARDENING.10`

### Batch 1: Core provenance field families (prior session)

- ID: `PROVENANCE-HARDENING.1`
  Status: `done`
  Goal: `Harden supporting_actor_ids in intent.rs.`
  Acceptance: `2 assertions across 2 tests covering IntentActor supporting_actor_ids pass-through from semantic to intent.`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `1e90aa9e` — Add supporting_actor_ids assertions to two intent.rs actor tests

- ID: `PROVENANCE-HARDENING.2`
  Status: `done`
  Goal: `Harden supporting_semantic_ids in intent.rs.`
  Acceptance: `5 assertions across 3 tests covering BehaviorIntent, IntentConstraint, IntentAssumption supporting_semantic_ids.`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `070b3721` — Add supporting_semantic_ids assertions to intent.rs behavior, constraint, and assumption tests

- ID: `PROVENANCE-HARDENING.3`
  Status: `done`
  Goal: `Harden supporting_section_ids in semantic.rs.`
  Acceptance: `1 assertion on ActorRecord supporting_section_ids in carries_actor_relative_ports_and_signal_connectivity.`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `52f47c41` — Add supporting_section_ids assertion to carries_actor_relative_ports_and_signal_connectivity test

- ID: `PROVENANCE-HARDENING.4`
  Status: `done`
  Goal: `Harden supporting_span_ids in evidence.rs.`
  Acceptance: `2 assertions on VisualObservation supporting_span_ids for caption and figure-reference observations.`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `4d7bd220` — Add supporting_span_ids assertions to visual evidence observation checks

- ID: `PROVENANCE-HARDENING.5`
  Status: `done`
  Goal: `Harden evidence_span_ids in evidence.rs.`
  Acceptance: `1 assertion on ExtractedStatement evidence_span_ids.`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `7409891b` — Add evidence_span_ids assertion to extracted statement check

### Batch 2: Remaining provenance vectors and document keys (this session)

- ID: `PROVENANCE-HARDENING.6`
  Status: `done`
  Goal: `Harden table_signal_declaration_provenance in evidence.rs.`
  Acceptance: `3 assertions across 3 tests covering TableSignalDeclarationProvenanceRecord population from signal-description table synthesis.`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `c0f33f1e` — Add table_signal_declaration_provenance assertions to evidence.rs signal-description table tests

- ID: `PROVENANCE-HARDENING.7`
  Status: `done`
  Goal: `Harden supporting_document_keys in learn_priors.rs.`
  Acceptance: `8 assertions across 3 tests covering all 7 prior record types.`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `045d020a` — Add supporting_document_keys assertions to all 7 learn_priors prior record types

- ID: `PROVENANCE-HARDENING.8`
  Status: `done`
  Goal: `Harden EvidenceLink from_evidence_span_id and to_visual_evidence_id.`
  Acceptance: `2 assertions in links_caption_and_figure_reference_into_visual_evidence.`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `319308a9` — Add from_evidence_span_id and to_visual_evidence_id assertions to EvidenceLink test

### Sweep: Document remaining coverage

- ID: `PROVENANCE-HARDENING.9`
  Status: `done`
  Goal: `Audit remaining provenance-like fields for systematic gaps.`
  Acceptance: `All supporting_* fields across IR modules inventoried. supporting_statement_id (singular), supporting_observation_count, supporting_source_kinds, supporting_canonical_ids, supporting_fragment_ids, supporting_rule_ids confirmed already tested. signal_alias_map identified as processing cache rather than provenance field. register_records and timing_constraints identified as untestable (no test exercises RegisterMap/TimingParameter tables).`
  Verification: `grep audit of all ir/ and commands/ modules`
  Commit: `pending — documented in this tree, not yet committed as a standalone slice`

### Close tree

- ID: `PROVENANCE-HARDENING.10`
  Status: `pending`
  Goal: `Validate completeness and close the provenance hardening tree.`
  Acceptance: `All done leaves verified, pending leaf resolved, tree changelog final, live docs synced.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

No executable leaves remain. The tree is effectively closed pending final
verification of `PROVENANCE-HARDENING.10`.

## Decisions

- `2026-05-14`: Scoped to regression-only test assertion additions. Zero
  production behavior changes allowed.
- `2026-05-14`: Record types with zero test coverage (phases, contracts,
  assertions, abstractions, decomposition_candidates in semantic.rs) are
  out of scope — no test exercises them, so assertions cannot be added.
- `2026-05-14`: Five record types without automation_confidence
  (InterfaceRecord, InterfaceSignalSemanticArbitrationRecord,
  ActorPortRecord, SignalConnectivityRecord, InfrastructureTopologyRecord)
  are skipped for automation_confidence hardening — the field literally
  does not exist on them.
- `2026-05-14`: signal_alias_map is a processing cache (BTreeMap), not a
  provenance field. Skipped.
- `2026-05-14`: register_records and timing_constraints on EvidenceIr cannot
  be hardened — no test creates RegisterMap or TimingParameter tables.

## Open Questions

- None remaining. All provenance field families inventoried and either
  hardened or explicitly deferred with rationale.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-14` | `PROVENANCE-HARDENING.1` | `cargo test -p specforge --lib` | 666/666 passed |
| `2026-05-14` | `PROVENANCE-HARDENING.2` | `cargo test -p specforge --lib` | 666/666 passed |
| `2026-05-14` | `PROVENANCE-HARDENING.3` | `cargo test -p specforge --lib` | 666/666 passed |
| `2026-05-14` | `PROVENANCE-HARDENING.4` | `cargo test -p specforge --lib` | 666/666 passed |
| `2026-05-14` | `PROVENANCE-HARDENING.5` | `cargo test -p specforge --lib` | 666/666 passed |
| `2026-05-14` | `PROVENANCE-HARDENING.6` | `cargo test -p specforge --lib` | 666/666 passed |
| `2026-05-14` | `PROVENANCE-HARDENING.7` | `cargo test -p specforge --lib` | 666/666 passed |
| `2026-05-14` | `PROVENANCE-HARDENING.8` | `cargo test -p specforge --lib` | 666/666 passed |
| `2026-05-14` | `PROVENANCE-HARDENING.9` | grep audit of all ir/ and commands/ modules | Complete |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `PROVENANCE-HARDENING.1` | `1e90aa9e` Add supporting_actor_ids assertions to two intent.rs actor tests | 2 assertions |
| `PROVENANCE-HARDENING.2` | `070b3721` Add supporting_semantic_ids assertions to intent.rs behavior, constraint, and assumption tests | 5 assertions |
| `PROVENANCE-HARDENING.3` | `52f47c41` Add supporting_section_ids assertion to carries_actor_relative_ports_and_signal_connectivity test | 1 assertion |
| `PROVENANCE-HARDENING.4` | `4d7bd220` Add supporting_span_ids assertions to visual evidence observation checks | 2 assertions |
| `PROVENANCE-HARDENING.5` | `7409891b` Add evidence_span_ids assertion to extracted statement check | 1 assertion |
| `PROVENANCE-HARDENING.6` | `c0f33f1e` Add table_signal_declaration_provenance assertions to evidence.rs signal-description table tests | 3 assertions |
| `PROVENANCE-HARDENING.7` | `045d020a` Add supporting_document_keys assertions to all 7 learn_priors prior record types | 8 assertions |
| `PROVENANCE-HARDENING.8` | `319308a9` Add from_evidence_span_id and to_visual_evidence_id assertions to EvidenceLink test | 2 assertions |
| `PROVENANCE-HARDENING.9` | pending — not yet committed | Audit only |

## Changelog

- `2026-05-14`: Created task tree. Backfilled 8 completed leaves from two
  PNT hardening sessions. Added audit leaf (9) and close-tree leaf (10).
  Total: 24 hardening assertions across 8 committed leaves.
