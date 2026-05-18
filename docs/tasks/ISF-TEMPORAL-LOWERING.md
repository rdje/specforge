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
  Status: `active`
  Goal: `Implement temporal_rules lowering + metric reconciliation.`
  Children: `.2.1`, `.2.2`, `.2.3`, `.2.4`
  Split rationale (2026-05-18): real corpus = 387 temporal_rules (only 28
  with cycle_window), predominantly `SignalStable`/`SignalValue`
  invariant-shaped — no large trivially-safe `(rule …)` subset. The fix
  genuinely requires new FSMGen transaction-internal constructs
  (`(contract …)`, `(stage …)`) that must pass `fsmgen --strict --check
  --json`, so this is split into independently CI-greenable sub-leaves
  rather than one risky multi-construct slice.

- ID: `ISF-TEMPORAL-LOWERING.2.1`
  Status: `done`
  Goal: >
    Extend the typed `IsfIr` model + `render` with the spec-supported
    transaction-internal constructs `(contract <name> (eventually
    <signal> within <N>))` (`bounded_eventually`) and `(stage <phase>
    (ready <r>)(valid <v>))` (`ready_valid_barrier`), wired only into
    `render`/`render_transaction` (no `from_intent_ir` behavior yet).
    Add a focused render unit test proving exact emitted text shape, and
    a hand-built fixture proving the emitted forms pass
    `fsmgen --strict --check --json`.
  Acceptance: `Typed contract/stage constructs render to the spec shapes; fsmgen-strict accepts a hand-built sample; scripts/run_ci.sh green.`
  Verification: `passed` — fsmgen-strict-verified: `(contract … (eventually s (within N)))` accepted (2 unit tests incl. real-binary strict); `(stage …)` proven strict-rejected → dropped + FSMGen feedback logged; scripts/run_ci.sh green
  Commit: `ISF-TEMPORAL-LOWERING.2.1 — typed bounded `(contract …)` render construct (fsmgen-strict-verified; stage dropped)`

- ID: `ISF-TEMPORAL-LOWERING.2.2`
  Status: `done`
  Goal: >
    Wire `temporal_rules` with a `cycle_window` → synthetic
    `(transaction … (contract <rule_id> (eventually <consequent_signal>
    within <N>)))`; consequent `HandshakeComplete{valid,ready}` →
    transaction `(stage <phase> (ready <ready>)(valid <valid>))`. Per
    `.1` mapping #1/#2.
  Acceptance: `Windowed + handshake temporal_rules lowered per .1; fsmgen-strict green; scripts/run_ci.sh green.`
  Verification: `passed` — windowed temporal_rules (`max_cycles>=1`, signal-bearing consequent, in-interface) emit `(contract <id> (eventually <sig> (within <N>)))`; verified LIVE on the AMBA CXS corpus IntentIR; picky self-verification caught + fixed a real bug: `(within 0)` is strict-REJECTED, so `max_cycles==0` is skipped (→ .2.3 residual) not emitted invalid; 15 isf tests + both fsmgen-strict tests green; full scripts/run_ci.sh green
  Commit: `ISF-TEMPORAL-LOWERING.2.2 — wire windowed temporal_rules → (contract …)`

- ID: `ISF-TEMPORAL-LOWERING.2.3`
  Status: `done`
  Goal: >
    Wire value/guard→drive temporal_rules (no window) → actor
    `(rule <rule_id> <antecedent_condition> (<sig> <val>))` per `.1` #3;
    temporal_rules whose predicates have no representable supported ISF
    construct (e.g. bare `SignalStable` with no window) → explicit
    residual decision per `.1` #4 (do NOT fabricate syntax).
  Acceptance: `Guard→drive temporal_rules emit (rule …); unrepresentable → residual decision; fsmgen-strict green; scripts/run_ci.sh green.`
  Verification: `passed` — single `classify_temporal_rule` classifier
    (Contract | Rule | Residual) shared by `from_intent_ir` (emit) and
    `build_isf_adapter_artifact` (residual append, single ISF model built
    once); guard `(== <declared-sig> <lit>)` / conditionless forms
    empirically fsmgen-strict-verified (exact declared name, never
    `sanitize_isf_name`d); 11 new unit tests incl. real-binary strict;
    LIVE on nvme IntentIR (109 temporal_rules, 0 windowed → 17 Rule [13
    emitted + 4 dedup-dropped] + 92 explicit residual = 109 fully
    accounted; emitted `.isf` `fsmgen --strict --check` `success:true`);
    full `scripts/run_ci.sh` green
  Commit: `see Commit Log`

- ID: `ISF-TEMPORAL-LOWERING.2.4`
  Status: `done`
  Goal: >
    Reconcile `build_isf_adapter_artifact` so every reported count
    reflects emitted content: `transaction_count` counts emitted
    transactions (incl. temporal-synthesized) only; temporal rules
    emitted as `(rule …)` count under rules; unrepresentable count as
    residual. No metric counts a surface the emitter ignores.
  Acceptance: `Every count == emitted content; regression locks it; scripts/run_ci.sh green.`
  Verification: `passed` — `transaction_count`/`rule_count` now derive
    from the single ISF model (`emitted_transaction_count()` /
    `emitted_rule_count()`), replacing the blind
    `temporal_rules.len()+cb_tx_count` / `conditional_rules+
    signal_constraints`; unrepresentable temporal rules already counted
    as `residual_decisions` (`.2.3`). Regression
    `isf_adapter_counts_equal_emitted_content` locks metric ==
    `\n  (transaction `/`\n  (rule ` occurrences in `source_text`. LIVE
    nvme: `transaction_count` 109 (old blind) → 0 (== emitted);
    `rule_count` 250 == 250 emitted; 92 temporal residuals intact. Full
    `scripts/run_ci.sh` green (1053 lib tests). Scope note in Decisions:
    constant/enum counts are a separate pre-existing non-temporal
    concern, surfaced not silently changed.
  Commit: `see Commit Log`

- ID: `ISF-TEMPORAL-LOWERING.3`
  Status: `done`
  Goal: >
    Regression: a temporal-rules-bearing IntentIR (via the generic
    markdown pipeline) yields non-empty ISF temporal behavior, the
    artifact metric matches the emitted content, and the emitted `.isf`
    passes `fsmgen --strict --check --json`.
  Acceptance: `Self-contained regression test added; fsmgen strict green; scripts/run_ci.sh green.`
  Verification: `passed` — `isf_temporal_rules_reach_isf_end_to_end`:
    self-contained markdown spec → full SourceIR→…→IntentIR pipeline
    recovers 2 `temporal_rules`; emitted `.isf` carries BOTH a windowed
    `(contract temporal_signal_constraint_sigcon_0002 (eventually PREADY
    (within 2)))` and a value `(rule temporal_…)`; asserts non-empty
    temporal behavior (contract|rule|residual — never silent loss),
    `transaction_count`/`rule_count` == emitted occurrences, and
    `fsmgen --strict --check --json` `success:true`. Full
    `scripts/run_ci.sh` green (1054 lib tests).
  Commit: `see Commit Log`

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
| — | `ISF-TEMPORAL-LOWERING.2` | `active` | Container — split into `.2.1`–`.2.4` |
| 2 | `ISF-TEMPORAL-LOWERING.2.1` | `done` | `(contract …)` render verified strict; stage dropped |
| 3 | `ISF-TEMPORAL-LOWERING.2.2` | `done` | Windowed→`(contract …)` live on corpus; `(within 0)` guarded |
| 4 | `ISF-TEMPORAL-LOWERING.2.3` | `done` | guard→drive `(rule …)` + residual classifier; live on nvme; strict-valid |
| 5 | `ISF-TEMPORAL-LOWERING.2.4` | `done` | metrics now == emitted content; nvme 109→0 fixed; regression locks it |
| 6 | `ISF-TEMPORAL-LOWERING.3` | `done` | e2e regression green: markdown→IntentIR→`.isf` with contract+temporal rule, metric match, fsmgen-strict |
| 7 | `ISF-TEMPORAL-LOWERING.4` | `pending` | Next — close tree; sync live docs + mdBook ISF chapter + ROADMAP R15b |

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

- `2026-05-18` (`.2.1` fsmgen-strict verification — corrects `.1`):
  binary `fsmgen --strict --check` proved two §11.8 doc claims wrong
  (logged in `docs/FSMGEN_FEEDBACK.md`):
  - `(contract … (eventually s within N))` flat form is REJECTED; the
    strict-valid shape is nested `(contract … (eventually s (within N)))`.
    `.1` mapping #1 stands with the corrected shape.
  - `(stage … (ready)(valid))` is REJECTED despite §11.8. `.1` mapping
    #2 (HandshakeComplete → `(stage …)`) is **retired**; HandshakeComplete
    temporal_rules now map to explicit residual decisions (`.1` #4).
    `IsfStage` is not modelled. `.2.2` handles only the windowed
    `(contract …)` case; HandshakeComplete → residual in `.2.3`.
- `2026-05-18` (`.2.3` implementation + empirical fsmgen-strict
  verification): mapping #3/#4 wired through one `classify_temporal_rule`
  classifier returning `Contract | Rule | Residual` (subsumes the inline
  `.2.2` windowed loop — single source of truth). Empirically verified
  against the pinned binary BEFORE trusting: `(rule n (== <declared-sig>
  <lit>) (<sig> <lit>))` and conditionless `(rule n (<sig> <lit>))` both
  `--strict --check` `success:true`. Picky fix: the guard references the
  EXACT declared signal name (FSMGen is case-sensitive) — it is NOT
  `sanitize_isf_name`d (that lowercases and would reference an undeclared
  signal). Non-windowed-without-value, HandshakeComplete, `(within 0)`,
  undeclared-signal, non-literal-value → explicit residual
  (`isf_temporal_unrepresentable_*`), never fabricated. Temporal rules
  join `rules` BEFORE dedup so conflicting drives are dropped, not
  emitted invalid. Residuals appended in `build_isf_adapter_artifact`
  from the same single ISF model (`isf_model.temporal_residuals()`),
  replacing the throwaway `build_isf_source_text`. `transaction_count`/
  `rule_count` deliberately NOT touched — that is `.2.4`.
- `2026-05-18` (`.2.4` metric reconciliation): `transaction_count` and
  `rule_count` now come from the single emitted ISF model
  (`emitted_transaction_count()`/`emitted_rule_count()`), so they count
  exactly what `render()` produces (incl. temporal-synthesized
  `(contract …)` transactions and temporal `(rule …)`, post-dedup).
  Regression `isf_adapter_counts_equal_emitted_content` locks
  metric == emitted-text occurrences. Scope honesty: `render()` does NOT
  emit constants/types/enums (they are `#[allow(dead_code)]` in `IsfIr`),
  so `constant_count`/`enum_count` derived from IntentIR symbol defs are
  a SEPARATE pre-existing non-temporal "extracted but not rendered" gap
  analogous to the temporal one. It is deliberately NOT changed in this
  temporal tree (scope discipline) and is surfaced here as an Open
  Question for a future audit/tree rather than silently zeroed or hidden.
- `2026-05-18` (`.2` honest outcome — PNT rule 5): `.2` split into
  `.2.1`–`.2.4`. Corpus evidence (387 temporal_rules, 28 windowed,
  mostly `SignalStable`/`SignalValue`) showed no large trivially-safe
  subset; the fix requires new fsmgen-strict-validated transaction
  constructs, so it must land as independently CI-greenable sub-leaves
  (render constructs → wire windowed/handshake → wire guard→drive +
  residual → reconcile metrics) rather than one risky multi-construct
  slice.

## Open Questions

- Whether some `temporal_rules` cannot map to any supported ISF construct;
  if so they must be preserved as explicit residual decisions rather than
  fabricated — `.1` decided this (mapping #4); `.2.3` implements it.
- `IsfIr` populates `constants`/`types`/`enums` from IntentIR but
  `render()` does NOT emit them (`#[allow(dead_code)]`); the artifact's
  `constant_count`/`enum_count` therefore still count an unrendered
  surface. This is a SEPARATE, pre-existing, non-temporal "extracted but
  not rendered" gap (analogous to the temporal one this tree fixed). It
  is out of scope for ISF-TEMPORAL-LOWERING and is surfaced here for a
  future audit/tree — owner: next codebase-vs-roadmap audit; does NOT
  block this tree's frontier (`.3`).

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-18` | `ISF-TEMPORAL-LOWERING.1` | FSMGen ISF spec + TemporalRuleRecord read; mapping recorded | `passed` |
| `2026-05-18` | `ISF-TEMPORAL-LOWERING.2.1` | 2 unit tests incl. real fsmgen `--strict --check`; full `scripts/run_ci.sh` | `passed` |
| `2026-05-18` | `ISF-TEMPORAL-LOWERING.2.2` | live corpus adapt (CXS) + `(within 0)` strict-reject caught/guarded; 15 isf tests; full `scripts/run_ci.sh` | `passed` |
| `2026-05-18` | `ISF-TEMPORAL-LOWERING.2.3` | 11 new classifier/residual/render unit tests + real-binary `(rule …)` strict test; empirical pre-verify of guard/conditionless forms; LIVE nvme adapt (109→17 Rule/92 residual, strict `success:true`); full `scripts/run_ci.sh` | `passed` |
| `2026-05-18` | `ISF-TEMPORAL-LOWERING.2.4` | `isf_adapter_counts_equal_emitted_content` regression + LIVE nvme metric check (txn 109→0, rule 250==250, 92 residual intact); full `scripts/run_ci.sh` (1053 lib tests) | `passed` |
| `2026-05-18` | `ISF-TEMPORAL-LOWERING.3` | `isf_temporal_rules_reach_isf_end_to_end` (markdown→pipeline→IntentIR 2 temporal_rules→`.isf` contract+rule, metric==emitted, fsmgen-strict `success:true`); full `scripts/run_ci.sh` (1054 lib tests) | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `ISF-TEMPORAL-LOWERING.1` | `ISF-TEMPORAL-LOWERING.1 — FSMGen-spec-grounded temporal_rules→ISF mapping decision` | Docs only; rejected deprecated `(handshake …)` |
| `ISF-TEMPORAL-LOWERING.2.1` | `ISF-TEMPORAL-LOWERING.2.1 — typed bounded (contract …) render construct` | strict-verified nested `(within N)`; `(stage …)` dropped + FSMGen feedback logged |
| `ISF-TEMPORAL-LOWERING.2.2` | `ISF-TEMPORAL-LOWERING.2.2 — wire windowed temporal_rules → (contract …)` | `0ccdc8f0`; live CXS; `(within 0)` guarded |
| `ISF-TEMPORAL-LOWERING.2.3` | `ISF-TEMPORAL-LOWERING.2.3 — guard→drive (rule …) + residual classifier` | classifier shared by emit + residual; live nvme strict-valid |
| `ISF-TEMPORAL-LOWERING.2.4` | `ISF-TEMPORAL-LOWERING.2.4 — reconcile artifact metrics to emitted content` | txn/rule counts from emitted model; regression locks it; nvme 109→0 |
| `ISF-TEMPORAL-LOWERING.3` | `ISF-TEMPORAL-LOWERING.3 — end-to-end markdown→.isf temporal regression` | full pipeline; contract+temporal rule emitted; metric match; fsmgen-strict |

## Changelog

- `2026-05-18`: Created from the post-ISF-ONLY audit critical finding —
  temporal_rules extracted but never lowered to `.isf`; transaction_count
  misreports it across the entire real corpus.
