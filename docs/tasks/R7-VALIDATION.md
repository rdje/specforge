# R7-VALIDATION: R7 Validation and Back-Annotation Hardening

## Metadata

- Tree ID: `R7-VALIDATION`
- Status: `active`
- Roadmap lane: `R7`
- Created: `2026-05-16`
- Last updated: `2026-05-16`
- Owner: repo-local workflow

## Goal

Close the remaining R7 validation gaps: extend validation findings into temporal rules and KG-quality surfaces, add adapter validation targets for `.fsm` and `.isf`, and design tracked approval evidence if canonical IR mutation is introduced.

## Non-Goals

- Replacing the existing validation infrastructure — findings are additive
- Changing IR schema — validation reads existing IR fields
- Broadening adapter validation beyond `.fsm` and `.isf`
- Implementing canonical IR mutation (blocked on future design decision)

## Acceptance Criteria

- Every remaining R7 gap has a validation finding surfaced in the report
- Both `validate_semantic_ir()` and `validate_intent_ir()` carry the new temporal findings
- KG-quality benchmarks produce measurable signals (metrics + findings)
- Adapter validation targets exist for `.fsm` and `.isf` adapter artifacts
- Focused tests pass for all new findings
- `cargo test -p specforge --lib` passes clean
- `cargo clippy` passes clean
- Live docs updated after each leaf

## Task Tree

- ID: `R7-VALIDATION`
  Status: `active`
  Goal: Close the remaining R7 validation gaps
  Children: R7-VALIDATION.1, R7-VALIDATION.2, R7-VALIDATION.3, R7-VALIDATION.4, R7-VALIDATION.5

- ID: `R7-VALIDATION.1`
  Status: `done`
  Goal: Add temporal handshake completion gap finding to validate_semantic_ir() and validate_intent_ir()
  Acceptance: >
    When temporal rules exist and interface signals carry handshake semantic roles
    (HandshakeValidLike / HandshakeReadyLike) but no temporal rule expresses a
    HandshakeComplete predicate, an Info finding surfaces the gap with affected
    signal names as related IDs. Rescan guidance is pushed for the temporal
    grounding surface. Finding exists in both semantic and intent validation.
  Verification: `passed`
  Commit: `430ccc08`

- ID: `R7-VALIDATION.2`
  Status: `done`
  Goal: Add temporal multi-predicate antecedent finding to validate_semantic_ir() and validate_intent_ir()
  Acceptance: >
    When temporal rules carry multi-predicate antecedents (>1), an Info finding
    flags the count and related rule IDs. Rescan guidance is pushed for the
    temporal grounding surface. Finding exists in both semantic and intent
    validation.
  Verification: `passed`
  Commit: `430ccc08`

- ID: `R7-VALIDATION.3`
  Status: `done`
  Goal: Add KG-quality benchmark findings to validate_semantic_ir() and validate_intent_ir()
  Acceptance: >
    Quality benchmark findings flag when key KG dimensions fall below defined
    coverage thresholds (graph direction coverage < 50%, semantic role
    resolution rate < 30%, consensus coverage rate < 50%). Each finding carries
    the current rate and the benchmark threshold as context. Findings exist in
    both semantic and intent validation.
  Verification: `passed`
  Commit: `pending`

- ID: `R7-VALIDATION.4`
  Status: `pending`
  Goal: Add adapter validation targets for .fsm and .isf adapters
  Acceptance: >
    The validate command can accept `.fsm` and `.isf` adapter artifacts,
    auto-detect the adapter kind, and run adapter-specific validation checks.
    At minimum, each adapter target has a structural well-formedness check
    and a coverage check for key properties (state graph completeness for .fsm,
    interface completeness for .isf). Findings are reported in the same
    ValidationReportRecord format.
  Verification: `pending`
  Commit: `pending`

- ID: `R7-VALIDATION.5`
  Status: `deferred`
  Goal: Design tracked approval evidence for canonical IR mutation
  Acceptance: >
    Design document defining what tracked approval evidence is, when it is
    required, and how it integrates with the validation pipeline. This leaf
    is gated on an explicit decision to introduce canonical IR mutation.
  Deferred reason: The ROADMAP gates this on canonical IR mutation being explicitly introduced. Current workflow does not mutate canonical IR — validation is read-only and rescan recommendations are additive. No implementation until the gating decision is made.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R7-VALIDATION.4` | `pending` | Adapter validation — spans multiple files, needs adapter format research |
| 2 | `R7-VALIDATION.5` | `deferred` | Gated on canonical IR mutation decision — not in frontier |

## Decisions

- `2026-05-16`: Temporal findings use `Info` severity because they signal coverage/quality gaps, not correctness errors. They do not prevent downstream adapters from running.
- `2026-05-16`: KG-quality benchmark thresholds start at 50% as a conservative floor. Thresholds are documented in the finding message so users can interpret the signal without reading source code.
- `2026-05-16`: `R7-VALIDATION.5` (tracked approval evidence) is deferred per ROADMAP gating language. Move it to `pending` and into the frontier only when canonical IR mutation is explicitly introduced.

## Open Questions

- What concrete benchmark thresholds should KG-quality use beyond the initial 50% floor? Answer will emerge from running validation against real spec documents and observing distribution of current coverage rates.
- For adapter validation (R7-VALIDATION.4): should validation be a separate command or integrated into `specforge validate`? TBD when leaf is reached.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-16` | `R7-VALIDATION.1` | `cargo test -p specforge --lib` (1016 passed) | `passed` |
| `2026-05-16` | `R7-VALIDATION.2` | `cargo test -p specforge --lib` (1016 passed) | `passed` |
| `2026-05-16` | `R7-VALIDATION.3` | `pending` | `pending` |
| `2026-05-16` | `R7-VALIDATION.4` | `pending` | `pending` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R7-VALIDATION.1` | `430ccc08` — R7-VALIDATION.1 R7-VALIDATION.2 — add temporal handshake completion gap and multi-predicate antecedent validation findings | Both leaves .1 and .2 implemented in one slice |
| `R7-VALIDATION.2` | `430ccc08` — shares commit with .1 | Both leaves .1 and .2 implemented in one slice |
| `R7-VALIDATION.3` | `pending` | `pending` |
| `R7-VALIDATION.4` | `pending` | `pending` |

## Changelog

- `2026-05-16`: Created task tree. Scoped five leaves from R7 remaining work in ROADMAP.
- `2026-05-16`: Completed leaves .1 and .2 in commit `430ccc08`. Added temporal handshake completion gap and multi-predicate antecedent findings to both validate_semantic_ir() and validate_intent_ir(). 1016 tests passing, clippy clean. Frontier advanced to .3.
