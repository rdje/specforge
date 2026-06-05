# NLI-GATE-METRIC: surface the NLI gate's demotions as a validate metric

## Metadata

- Tree ID: `NLI-GATE-METRIC`
- Status: `done` (CLOSED `2026-06-05`; `.1` — `nli_demoted_contracts` validate metric)
- Roadmap lane: `R16`/`R15e`
- Created: `2026-06-05`
- Owner: repo-local workflow
- Parent context: user directive ("→ surface an nli_* count as a … metric") — the follow-up to
  `NLI-INTENT-GATE`. The gate demotes NotEntailed contracts into `residual_decisions` with
  `packet_id = nli_unentailed_<id>`; make that count visible.

## Design (read-only, hermetic)

- `nli_demoted_count(residuals: &[ResidualDecisionPacket]) -> usize` in `ir/nli_verify.rs` —
  counts packets whose `packet_id` starts with `nli_unentailed_`. Pure; no LLM call (the demotion
  is already recorded in the artifact).
- `validate_intent_ir` emits `metric("nli_demoted_contracts", …)` from that count, so
  `specforge validate <intent_ir.json>` surfaces how many contracts the NLI gate demoted.

## Acceptance Criteria

- `.1`: `nli_demoted_count` + a unit test (mixed packet ids → correct count); the
  `nli_demoted_contracts` validate metric; book mention; full CI GREEN; tree CLOSED.

## Task Tree

- ID: `NLI-GATE-METRIC` · Status: `done` (CLOSED `2026-06-05`) · Children: `.1`
- ID: `NLI-GATE-METRIC.1` · Status: `done` · Goal: the count helper + validate metric + test +
  book + close.
  Verification: passed (`2026-06-05`) — `ir/nli_verify.rs`: `NLI_RESIDUAL_PREFIX` const (reused in
  `nli_gate_contracts`) + `nli_demoted_count(&[ResidualDecisionPacket]) -> usize` (counts
  `nli_unentailed_` packets; read-only, no LLM) + a unit test (mixed ids → 2; empty → 0).
  `validate_intent_ir` emits `metric("nli_demoted_contracts", …)`. Book mention added; full
  `scripts/run_ci.sh` GREEN (1253→1254; +1). Tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `NLI-GATE-METRIC.1` | `done` | count helper + `validate` `nli_demoted_contracts` metric + test → **tree CLOSED** |

**Tree CLOSED `2026-06-05`.** `specforge validate <intent_ir.json>` now reports
`nli_demoted_contracts` (read-only count of the gate's `nli_unentailed_` residuals). CI green 1254.

## Decisions

- `2026-06-05`: surface the gate's effect as a **read-only validate metric** (count the
  `nli_unentailed_` residuals already in the artifact) — no LLM call in `validate`.

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `NLI-GATE-METRIC.1` | `NLI-GATE-METRIC.1 — nli_demoted_count + nli_demoted_contracts validate metric; close` | +1 test; CI green 1254 |

## Changelog

- `2026-06-05`: Created — surface the NLI gate's demotions as `nli_demoted_contracts` in
  `validate`.
- `2026-06-05`: **Tree CLOSED.** `nli_demoted_count` helper + `nli_demoted_contracts` metric in
  `validate_intent_ir` + a unit test + book mention. CI green 1254.
