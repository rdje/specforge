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
  Status: `pending`
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
  Verification: `pending`
  Commit: `pending`

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
| 1 | `ISF-TEMPORAL-LOWERING.1` | `pending` | Mapping must be FSMGen-spec-grounded before code |
| 2 | `ISF-TEMPORAL-LOWERING.2` | `pending` | Implement + reconcile metrics |
| 3 | `ISF-TEMPORAL-LOWERING.3` | `pending` | Prove it end-to-end incl. fsmgen strict |
| 4 | `ISF-TEMPORAL-LOWERING.4` | `pending` | Close + doc sync |

## Decisions

- `2026-05-18`: `.1` is mandatory-first and is FSMGen-spec-bound — per the
  `fsmgen-contract-authority` memory, parser acceptance ≠ support; the
  temporal construct must be explicitly supported by
  `subs/fsmgen/docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md`.

## Open Questions

- Whether some `temporal_rules` cannot map to any supported ISF construct;
  if so they must be preserved as explicit residual decisions rather than
  fabricated — resolved during `.1`.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-18` | `ISF-TEMPORAL-LOWERING.1` | `pending` | `pending` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `ISF-TEMPORAL-LOWERING.1` | `pending` | `pending` |

## Changelog

- `2026-05-18`: Created from the post-ISF-ONLY audit critical finding —
  temporal_rules extracted but never lowered to `.isf`; transaction_count
  misreports it across the entire real corpus.
