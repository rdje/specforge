# ISF-RULE-CONFLICT-RESIDUAL: surface dropped value-conflicting rules as explicit residuals (not silent loss)

## Metadata

- Tree ID: `ISF-RULE-CONFLICT-RESIDUAL`
- Status: `done`
- Roadmap lane: `R6` (`.isf` adapter — semantic-truthfulness hardening)
- Created: `2026-05-31`
- Last updated: `2026-05-31`
- Owner: repo-local workflow

## Goal

Close a residual-honesty gap in the `.isf` emitter. `IsfIr::from_intent_ir`
dedups rules that conflict on the same signal+guard (two rules driving one
signal to different values under the same guard — FSMGen strict rejects that),
keeping the first and **silently `continue`-ing past the rest**
(`isf_ir.rs` dedup block). Every other "dropped obligation" path in the emitter
already records a `ResidualDecisionPacket` into `temporal_residuals` (surfaced
by the adapter as `residual_decisions`, per `adapters.rs`) precisely so nothing
is lost silently — the conflict-dedup is the one place that bypasses it. Make
it record a residual too.

The emitted `.isf` is **unchanged** (the conflicting rule is still dropped, so
output stays FSMGen-strict-valid); only the previously-silent conflict now
appears as an explicit `residual_decisions` entry — consistent with the
project's core thesis ("never hide ambiguity; residual decision packets for
irreducible ambiguity").

## Non-Goals

- No change to which rules are emitted / to the `.isf` source text (strict
  validity preserved — the fsmgen-strict tests must stay green unchanged).
- Not arbitration/priority resolution of the conflict — just honest surfacing.

## Acceptance Criteria

- The dedup is extracted into a pure, testable
  `dedup_conflicting_rules(Vec<IsfRule>) -> (Vec<IsfRule>, Vec<ResidualDecisionPacket>)`;
  `from_intent_ir` calls it and extends `temporal_residuals` with the returned
  conflict residuals.
- Each dropped conflict yields a `ResidualDecisionPacket` (mirroring
  `temporal_residual_packet`): id `isf_rule_conflict_<rule>`, the conflicting
  signal/guard/values named, two candidate interpretations (keep-earlier vs
  keep-dropped), `Low` confidence.
- Unit tests: conflicting rules → first kept + the dropped one recorded as a
  residual; non-conflicting → all kept, no residual.
- Emitted `.isf` byte-identical for non-conflicting corpora; full
  `scripts/run_ci.sh` green (incl. the fsmgen-strict tests).

## Task Tree

- ID: `ISF-RULE-CONFLICT-RESIDUAL`
  Status: `done`
  Goal: surface dropped value-conflicting rules as residuals
  Children: `.1`

- ID: `ISF-RULE-CONFLICT-RESIDUAL.1`
  Status: `done`
  Goal: >
    Extract `dedup_conflicting_rules` + add `rule_conflict_residual_packet`;
    route `from_intent_ir` through it (extend `temporal_residuals`); 2 unit
    tests. Book method-doc subsection in `pipeline/isf-adapter.md`; close.
  Acceptance: dedup records residuals; `.isf` unchanged; tests + `scripts/run_ci.sh` green; book subsection added.
  Verification: >
    passed (`2026-05-31`) — extracted pure `dedup_conflicting_rules(Vec<IsfRule>)
    -> (kept, Vec<ResidualDecisionPacket>)` + `rule_conflict_residual_packet`
    (mirrors `temporal_residual_packet`: id `isf_rule_conflict_<rule>`, names
    signal/guard/dropped+kept values, 2 candidate interpretations, `Low`
    confidence). `from_intent_ir` routes through it and extends
    `temporal_residuals` (surfaced by `adapters.rs` as `residual_decisions`).
    Emitted `.isf` is byte-identical (the conflict is still dropped) — the
    fsmgen-strict tests pass unchanged. 2 unit tests (conflict → first kept +
    residual recorded naming the signal/DROPPED; same-signal-different-guard →
    all kept, no residual). Book subsection added to `pipeline/isf-adapter.md`.
    Lib `1167 → 1169`; full `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

## Current Frontier

**Tree CLOSED `2026-05-31`** — single fix leaf `done`.

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `ISF-RULE-CONFLICT-RESIDUAL.1` | `done` | dropped conflicts now surfaced as residuals; `.isf` unchanged; CI green |

## Decisions

- `2026-05-31`: reuse the existing `temporal_residuals` →
  `residual_decisions` mechanism (no new field, no construction-site churn);
  extract the dedup into a pure helper for direct unit testing. Chosen as a
  server-independent semantic-truthfulness improvement while the live
  confirmations are gated on the busy Ollama server.

## Open Questions

- None. (Arbitrating the conflict — picking the "right" value — is future work;
  this tree only makes the conflict honest/visible.)

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-31` | `ISF-RULE-CONFLICT-RESIDUAL.1` | pure `dedup_conflicting_rules` + `rule_conflict_residual_packet`; `from_intent_ir` extends `temporal_residuals`; emitted `.isf` byte-identical (fsmgen-strict unchanged); 2 unit tests; book subsection; lib `1167 → 1169`; full `scripts/run_ci.sh` | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `ISF-RULE-CONFLICT-RESIDUAL.1` | `ISF-RULE-CONFLICT-RESIDUAL.1 — surface dropped value-conflicting rules as residual decisions; close tree` | `.isf` unchanged; conflict no longer silently lost |

## Changelog

- `2026-05-31`: `.1` — extracted `dedup_conflicting_rules`; dropped
  value-conflicting rules now recorded as `ResidualDecisionPacket`s (surfaced
  as `residual_decisions`) instead of silently `continue`-d; `.isf` emission
  unchanged; +2 unit tests; book subsection; **TREE CLOSED**.
- `2026-05-31`: Created — surface dropped value-conflicting `.isf` rules as
  explicit residual decisions instead of silent loss.
