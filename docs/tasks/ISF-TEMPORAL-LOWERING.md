# ISF-TEMPORAL-LOWERING: lower IntentIR.temporal_rules into the `.isf` adapter

## Metadata

- Tree ID: `ISF-TEMPORAL-LOWERING`
- Status: `active`
- Roadmap lane: `R15b` (explicit clock-tick temporal model → must reach the adapter)
- Created: `2026-05-18`
- Last updated: `2026-05-18`
- Owner: repo-local workflow

## Goal

Make the typed temporal-rule surface (`IntentIR.temporal_rules`, R15b's
flagship deliverable) actually reach the only adapter. Today
`IsfIr::from_intent_ir` builds ISF rules/transactions from
`conditional_rules` + `signal_constraints` + `temporal_invariants` +
`transactions` + `control_blocks`, and **never reads `temporal_rules`** —
yet `assess_isf_renderability` gates on it and
`build_isf_adapter_artifact` sets `transaction_count =
intent_ir.temporal_rules.len() + cb_tx_count`. Across all 10 real corpus
IntentIRs, `temporal_rules` is 7–109 while `temporal_invariants`,
`control_blocks`, and `IntentIR.transactions` are 0 — so the entire typed
temporal/clock-tick behavior is extracted and then silently dropped from
`.isf`, while `transaction_count` misreports it.

## Non-Goals

- Do not change how `temporal_rules` are extracted upstream
  (SemanticIR/IntentIR builders are correct; this is an adapter gap).
- Do not invent ISF syntax — the ISF construct used MUST be one FSMGen's
  downstream integration spec actually supports (see Decisions).
- Do not regress the existing conditional/constraint/transaction lowering
  or the `fsmgen --strict --check --json` test.

## Acceptance Criteria

- `IsfIr::from_intent_ir` lowers `intent_ir.temporal_rules` into ISF using
  a FSMGen-spec-supported construct.
- `build_isf_adapter_artifact` metrics (`transaction_count` and any other
  count) reflect what the emitter actually renders — no metric counts a
  surface the emitter ignores.
- A regression fixture/test proves a temporal-rules-bearing IntentIR
  yields non-empty ISF temporal behavior, the metric matches, and the
  emitted `.isf` still passes `fsmgen --strict --check --json`.
- `scripts/run_ci.sh` green at every code-touching leaf; every leaf
  committed through `COMMIT.md`.

## Task Tree

- ID: `ISF-TEMPORAL-LOWERING`
  Status: `active`
  Goal: `temporal_rules reach .isf; metrics honest; fsmgen-strict valid.`
  Children: `.1`, `.2`, `.3`, `.4`

- ID: `ISF-TEMPORAL-LOWERING.1`
  Status: `done`
  Goal: >
    Design the `temporal_rules → ISF` mapping. Per the
    `fsmgen-contract-authority` rule, FIRST read
    `subs/fsmgen/docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md` and determine
    which supported ISF construct represents a typed temporal rule
    (antecedents/consequents, clock/edge, cycle window, HandshakeComplete,
    actor-grounded drive/stability). Record the decision and the exact
    supported source shape; do NOT pick a construct the spec does not
    list as supported. Docs/decision only.
  Acceptance: `Mapping decision recorded with the exact FSMGen-spec source shape cited; no code change.`
  Verification: `passed` — FSMGen ISF spec §6/§11/§11.8/§12 + TemporalRuleRecord read; spec-grounded mapping recorded in Decisions; `(handshake …)` rejected as deprecated; docs-only
  Commit: `ISF-TEMPORAL-LOWERING.1 — FSMGen-spec-grounded temporal_rules→ISF mapping decision`

- ID: `ISF-TEMPORAL-LOWERING.2`
  Status: `pending`
  Goal: >
    Implement `temporal_rules` lowering in `IsfIr::from_intent_ir` per the
    `.1` decision, and reconcile `build_isf_adapter_artifact` so every
    reported count matches emitted content (no double counting vs
    conditional/transaction sources). Focused tests.
  Acceptance: `temporal_rules emit ISF per .1; counts == emitted; scripts/run_ci.sh green. May split.`
  Verification: `pending`
  Commit: `pending`

- ID: `ISF-TEMPORAL-LOWERING.3`
  Status: `pending`
  Goal: >
    Regression: a temporal-rules-bearing IntentIR (via the generic
    markdown pipeline) yields non-empty ISF temporal behavior, the
    artifact metric matches the emitted content, and the emitted `.isf`
    passes `fsmgen --strict --check --json`.
  Acceptance: `Self-contained regression test added; fsmgen strict green; scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

- ID: `ISF-TEMPORAL-LOWERING.4`
  Status: `pending`
  Goal: `Close tree; sync live docs + the mdBook ISF chapter.`
  Acceptance: `Tree done; book/ROADMAP reflect temporal lowering; scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `ISF-TEMPORAL-LOWERING.1` | `done` | Spec-grounded mapping recorded |
| 2 | `ISF-TEMPORAL-LOWERING.2` | `pending` | Next — implement + reconcile metrics |
| 3 | `ISF-TEMPORAL-LOWERING.3` | `pending` | Prove it end-to-end incl. fsmgen strict |
| 4 | `ISF-TEMPORAL-LOWERING.4` | `pending` | Close + doc sync |

## Decisions

- `2026-05-18`: `.1` is mandatory-first and is FSMGen-spec-bound — per the
  `fsmgen-contract-authority` memory, parser acceptance ≠ support; the
  temporal construct must be explicitly supported by
  `subs/fsmgen/docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md`.
- `2026-05-18` (`.1` outcome — FSMGen ISF spec read; `TemporalRuleRecord`
  shape read). Mapping decision, grounded in the spec:
  - **DO NOT** emit `(handshake name (valid s)(ready s))` — spec §6
    lines 248–253 mark it **deprecated: "validated for shape and
    ignored. It does not lower to ready/valid behavior."** Emitting it
    would be the exact `fsmgen-contract-authority` trap (acceptance ≠
    support).
  - Supported temporal vehicles are **transaction-internal** clauses
    (spec §11 / §11.8): `(when cond body)`, `(await port (watchdog N))`,
    `(stage phase (ready r)(valid v))` (shipped `ready_valid_barrier`),
    `(contract name (eventually signal within N))` (shipped
    `bounded_eventually`), `(latency (min N)(max M))`; plus actor-level
    `(rule name cond (sig val))` (spec §12, already emitter-used and
    fsmgen-strict-passing).
  - `TemporalRuleRecord` → ISF mapping:
    1. has `cycle_window` (bounded "within N") + a consequent
       `SignalValue`/`ActorDrivesSignal` signal → synthetic
       `(transaction txn_temporal_<rule_id> (on start) … (contract
       <rule_id> (eventually <consequent_signal> within <N>)))`
       (`bounded_eventually`).
    2. consequent `HandshakeComplete{valid,ready}` → transaction
       `(stage <phase> (ready <ready>)(valid <valid>))`
       (`ready_valid_barrier`) — the supported replacement for the
       deprecated `(handshake …)`.
    3. value/guard→drive at a phase, no `cycle_window` → actor-level
       `(rule <rule_id> <antecedent_condition> (<sig> <val>))` (same
       supported `(rule …)` form already emitted).
    4. predicates with no representable supported ISF construct
       (e.g. bare `ActorMaintainsSignalStable` with no window) →
       **preserve as an explicit residual decision; do not fabricate
       syntax** (`fsmgen-contract-authority`).
  - Metric reconciliation (`build_isf_adapter_artifact`): replace the
    blind `transaction_count = temporal_rules.len() + cb_tx_count` with
    counts derived from what the emitter actually produced (emitted
    transactions incl. temporal-synthesized ones; a separate
    `temporal_*` count for rules emitted as `(rule …)` / residual).
    No metric may count a surface the emitter ignores.

## Open Questions

- Whether some `temporal_rules` cannot map to any supported ISF construct;
  if so they must be preserved as explicit residual decisions rather than
  fabricated — resolved during `.1`.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-18` | `ISF-TEMPORAL-LOWERING.1` | FSMGen ISF spec + TemporalRuleRecord read; mapping recorded | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `ISF-TEMPORAL-LOWERING.1` | `ISF-TEMPORAL-LOWERING.1 — FSMGen-spec-grounded temporal_rules→ISF mapping decision` | Docs only; rejected deprecated `(handshake …)` |

## Changelog

- `2026-05-18`: Created from the post-ISF-ONLY audit critical finding —
  temporal_rules extracted but never lowered to `.isf`; transaction_count
  misreports it across the entire real corpus.
