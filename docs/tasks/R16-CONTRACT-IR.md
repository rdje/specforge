# R16-CONTRACT-IR: typed timed-contract IR layer (point #1)

## Metadata

- Tree ID: `R16-CONTRACT-IR`
- Status: `active`
- Roadmap lane: `R16`
- Program: `R16-INTENT-CAPTURE` (point #1, order 1 — DAG root)
- Created: `2026-05-19`
- Last updated: `2026-05-19`
- Owner: repo-local workflow

## Goal

Replace the predicate-bag temporal model (`TemporalRuleRecord` =
antecedents→consequents + one optional `cycle_window` + edge) with a
typed **timed-contract IR**: per-actor `assume`/`guarantee` contracts
over boundary signals, expressed in a small, closed operator fragment —
`rose/fell(sig)`, `stable(sig) throughout [e1,e2]`, `s ##[m:n] t`,
`s until t`, `eventually(s) within N`, `mutex(a,b)`,
`ordered_before(phaseA,phaseB)` — so a single bound obligation is **one
object with full provenance**, not shredded across disjoint predicates.
IntentIR becomes a typed projection of ContractIR; `.isf` lowering
becomes near-mechanical (FSMGen already ships `bounded_eventually`,
ready/valid `(stage …)`).

## Non-Goals

- Do not change extraction sources in this tree (fusion/waveform/LLM are
  points #3/#4/#6). This tree delivers the **target shape only**.
- Do not break the residual-honesty doctrine: unrepresentable temporal
  intent stays an explicit residual.
- Do not regress `ISF-TEMPORAL-LOWERING`'s shipped windowed/`(rule …)`/
  residual behavior; subsume `ISF-HANDSHAKE-STAGE-LOWERING` cleanly.

## Acceptance Criteria

- A typed ContractIR exists (stage vs. typed layer decided in `.1`);
  `TemporalRuleRecord` either migrates onto it or is expressed by it
  with no temporal-fidelity loss.
- Round-trip: every currently-lowered temporal rule still lowers (CI
  parity), plus previously-shredded bound obligations now survive as one
  contract.
- `scripts/run_ci.sh` green per leaf; every leaf via `COMMIT.md`.

## Task Tree

- ID: `R16-CONTRACT-IR`
  Status: `active`
  Goal: typed timed-contract IR; IntentIR projects it; mechanical `.isf`
  Children: `.1`, `.2`, `.3`, `.4`, `.5`

- ID: `R16-CONTRACT-IR.1`
  Status: `done`
  Goal: >
    Design leaf (docs-only): (a) placement decision — new IR stage vs.
    typed layer extending SemanticIR/IntentIR; (b) the closed operator
    grammar; (c) exact `TemporalRuleRecord` → ContractIR migration map
    proving zero temporal-fidelity loss + CI parity strategy; (d) how
    `ISF-HANDSHAKE-STAGE-LOWERING` is subsumed.
  Acceptance: `Design doc recorded in this tree (placement + grammar + migration map + parity plan + subsumption); no code; reviewed against current TemporalRuleRecord/TickPhase/cycle_window/HandshakeComplete and the .isf contract/stage/bounded_eventually surface.`
  Verification: `passed` — current types re-verified
    (`semantic.rs:805-879`, carriers `semantic.rs:79`/`intent.rs:80`,
    classifier `isf_ir.rs TemporalRuleDisposition`); full design recorded
    in "Design (`.1` output)" below; every `TemporalRuleRecord` /
    `TemporalPredicateRecord` case has an explicit ContractIR target
    (incl. the currently-residual ones — now *modeled*, not lost); CI
    parity strategy + `ISF-HANDSHAKE-STAGE-LOWERING` subsumption fixed;
    a thorough mirror section was added to the mdBook
    (`direction/temporal-intent-capture.md`) per `BOOK-METHOD-DOC`.
    Docs-only — no code.
  Commit: `see Commit Log`

- ID: `R16-CONTRACT-IR.2`
  Status: `done`
  Goal: implement the typed ContractIR model + serde per the `.1` design.
  Acceptance: `Typed model + serde + unit tests; scripts/run_ci.sh green.`
  Verification: `passed` — `crates/specforge/src/ir/contract.rs` added
    (closed algebra: `EventExpr`/`Window`/`Condition`/`Obligation`/
    `ActorContract` + serde) with `contract_from_temporal_rule` mapping
    every `TemporalRuleRecord`/`TemporalPredicateRecord` case per the
    `.1` table (residual cases modelled, not lost); 7 unit tests
    (per migration row + serde round-trip); module registered in
    `ir/mod.rs`; additive `actor_contracts` field on `SemanticIr`/
    `IntentIr` (serde default + skip-if-empty, **unpopulated** — zero
    artifact/fixture change; producers/lowering re-point is `.3`). Two
    honest in-implementation refinements (see Decisions): field named
    `actor_contracts` (collision with pre-existing `SemanticIR.contracts:
    Vec<ContractRecord>`); `Obligation::Observe` added for the no-value
    predicate case. Full `scripts/run_ci.sh` green — 1061 passed
    (1054 + 7), 0 failed, mdBook builds.
  Commit: `see Commit Log`

- ID: `R16-CONTRACT-IR.3`
  Status: `done`
  Goal: >
    **Parity-preserving re-point only** (no behaviour change). Populate
    `actor_contracts` from `temporal_rules` in the SemanticIr/IntentIr
    builders; add `classify_actor_contract` reproducing the *exact*
    current `classify_temporal_rule` Contract|Rule|Residual decisions;
    switch the `.isf` lowering call site to consume `actor_contracts`.
    `HandshakeBarrier` stays Residual here (parity) — enabling `(stage
    …)` is the deliberate behaviour change in `.4`. Then audit
    `temporal_rules` consumers (converge snapshot / validate /
    learn_priors, the `ISF-ONLY-IR-PRUNE.1` method) and project-or-migrate.
  Acceptance: `Emitted .isf is semantically identical pre/post on the real corpus (parity gate); fsmgen-strict tests green; scripts/run_ci.sh green.`
  Verification: `passed` — parity proven three independent ways:
    (1) by-construction pointwise oracle test
    `classify_actor_contract_is_parity_equivalent_to_classify_temporal_rule`
    (13-shape battery incl. multi-antecedent, windowed-stable, every
    residual kind) asserts `classify_actor_contract ==
    classify_temporal_rule`; (2) LIVE nvme corpus via the back-compat
    fallback: `transaction_count=0`, `rule_count=250`, `92` temporal
    residuals == the ISF-TEMPORAL-LOWERING.2.4 baseline exactly, emitted
    `.isf` `fsmgen --strict --check` `success:true`; (3) the existing
    e2e + fsmgen-strict tests (real markdown→IntentIR→`.isf`) green
    through the new populate+re-point. `classify_temporal_rule` + 3
    helpers retained `#[cfg(test)]` as the oracle. Back-compat fallback
    added (pre-ContractIR `IntentIR` with no `actor_contracts` projects
    from `temporal_rules` → identical `.isf`). Consumer audit → KEEP
    `temporal_rules` (load-bearing: `validate.rs` 96 refs R7/R15b
    diagnostics, `learn_priors`, + the fallback source). Full
    `scripts/run_ci.sh` green — 1063 passed, 0 failed, mdBook builds.
  Commit: `see Commit Log`

- ID: `R16-CONTRACT-IR.4`
  Status: `done`
  Goal: >
    **Behaviour change (post-parity), fsmgen-strict-verified**: enable
    `HandshakeBarrier → (stage p (ready r)(valid v))` lowering — the
    subsumed `ISF-HANDSHAKE-STAGE-LOWERING`. HandshakeComplete temporal
    rules that were residual now emit `(stage …)` (FSMGen accepts it at
    pin `9bfb9a20`); verify live on the real corpus + fsmgen-strict;
    residual count drops; metric reconciliation (`ISF-TEMPORAL-LOWERING.2.4`
    invariant) still holds.
  Acceptance: `HandshakeComplete → (stage …) live-verified fsmgen-strict on the corpus; no other .isf change; scripts/run_ci.sh green.`
  Verification: `passed` — re-added typed `IsfStage` + `stages` on
    `IsfTransaction` + `(stage <n> (ready r)(valid v))` render;
    `TemporalRuleDisposition::Stage`; `classify_actor_contract`
    `HandshakeBarrier` arm → Stage when both signals declared.
    **Picky-auditor catch via the real binary:** FSMGen rejects a stage
    whose `ready` is not an actor input ("stage … input '<r>' is not an
    actor input"); added an `input_signal_names` gate in `from_intent_ir`
    — emit `(stage …)` only when `ready` is an interface input, else an
    explicit residual (never a strict-invalid stage); test direction
    corrected to FSMGEN-SUBMODULE-BUMP.1's verified shape (ready=input /
    valid=output). 3 new tests incl. real-binary
    `temporal_stage_isf_passes_fsmgen_strict_validation` (synthesized
    `(stage …)` accepted at pin `9bfb9a20`). HONEST scope finding: NO
    corpus `temporal_rule` currently carries a `handshake_complete`
    predicate → **0 stages emitted corpus-wide; zero `.isf` change**
    (nvme still 0/250/92 == the `.3` baseline; metric==emitted; strict
    `success:true` on 4 corpus docs). `.4` is a verified-but-dormant
    capability that activates when upstream extraction (R16 #3/#4/#6)
    grounds handshake completion. The `.3` parity oracle stays green
    (handshake divergence is the intentional `.4` change, tested
    separately). Full `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

- ID: `R16-CONTRACT-IR.5`
  Status: `pending`
  Goal: close tree; sync mdBook (temporal-semantics + ISF chapters) +
  ROADMAP R16; finalize `ISF-HANDSHAKE-STAGE-LOWERING` disposition
  (delivered via `.4`).
  Acceptance: `Tree done; docs synced; scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R16-CONTRACT-IR.1` | `done` | Design fixed (placement + grammar + migration/parity + subsumption); book mirror added |
| 2 | `R16-CONTRACT-IR.2` | `done` | Typed `ir/contract.rs` + serde + conversion + 7 tests; additive field; CI 1061/0 |
| 3 | `R16-CONTRACT-IR.3` | `done` | Parity re-point complete: classify_actor_contract + populate + fallback; parity proven 3 ways (oracle test / live nvme baseline / e2e+fsmgen-strict); temporal_rules kept (audit); CI 1063/0 |
| 4 | `R16-CONTRACT-IR.4` | `done` | `HandshakeBarrier → (stage …)` enabled + fsmgen-strict-verified (ready=input gate); honest finding: 0 corpus handshake_complete → dormant, zero `.isf` change |
| 5 | `R16-CONTRACT-IR.5` | `pending` | **Next** — close tree; sync mdBook (isf-adapter/temporal-semantics) + ROADMAP R16; finalize `ISF-HANDSHAKE-STAGE-LOWERING` disposition |

## Dependencies / Order

- DAG root (no deps). Blocks `R16-KG-PROTOCOL-ONTOLOGY`,
  `R16-CAPTURE-FIDELITY-GATES`, and transitively all extraction trees —
  nothing can extract accurately into a shape that does not exist.

## Design (`.1` output, 2026-05-19)

Grounded against the verified current types: `TemporalRuleRecord`
(`semantic.rs:867`), `TemporalPredicateRecord` 7 variants
(`:828`), `CycleWindowRecord` (`:819`), `TickPhase{PreTick,PostTick}`,
`ClockEdge`; carriers `semantic.rs:79` + `intent.rs:80`; lowering
`classify_temporal_rule → TemporalRuleDisposition{Contract|Rule|Residual}`
(`isf_ir.rs`).

### (a) Placement — typed layer, NOT a new IR stage

ContractIR is a new typed module `crates/specforge/src/ir/contract.rs`
defining `ActorContract` + the operator algebra. `SemanticIR` and
`IntentIR` carry `actor_contracts: Vec<ActorContract>` as an additive
(`#[serde(default, skip_serializing_if = "Vec::is_empty")]`) field —
named `actor_contracts` (NOT `contracts`) because `SemanticIR` already
has `contracts: Vec<ContractRecord>` (semantic protocol contracts);
`.2` honest refinement. **No new pipeline stage / CLI / `IrStage`.**
Rationale: (1) the standing doctrine "keep `IntentIR` the canonical
product boundary"; (2) the `ISF-ONLY-*` ethos favours fewer stages; (3)
`temporal_rules`/`actor_*` already live as typed fields the same way;
(4) `.isf` lowering already consumes `IntentIR`. A separate stage would
add large surface for zero capture benefit.

### (b) Closed operator algebra (small, verifiable, lowerable)

```
EventExpr   = Edge{signal, Rose|Fell}
            | Level{signal, value}                  // state, used in guards
            | HandshakeFire{valid, ready}           // the transfer cycle
            | Start                                  // txn/phase start
            | PhaseBoundary{phase, Enter|Exit}
Window      = Within{min:Option<u32>, max:u32 (>=1)} // FSMGen (within N), N>=1
            | Between{from:EventExpr, to:EventExpr}   // for stability/throughout
            | SameCycle                               // 0-cycle/combinational
                                                       //   (the within-0 case,
                                                       //    MODELLED not lost)
Obligation  = Eventually{target:EventExpr, window:Within}
            | Stable{signal, during:Between}
            | Drive{signal, value}
            | HandshakeBarrier{valid, ready}
            | Persist{hold:EventExpr, until:EventExpr}    // "remain X until Y"
            | Sequence{steps:Vec<(EventExpr,Window)>}
            | Mutex{a,b} | OrderedBefore{phaseA,phaseB}
            | Observe{signal}    // .2 refinement: weak no-value/no-window
                                 //   boundary fact; captured, residual-
                                 //   lowered, never fabricated into a value
Condition   = Eq{signal,value}  // the existing bounded (== sig val) guard form
ActorContract = { contract_id, actor:ActorRef, kind:Assume|Guarantee,
                  guard:Option<Condition>, obligation:Obligation,
                  clock:Option<Signal>, edge:ClockEdge,
                  channel:Option<Ref>, phase:Option<Ref>,
                  provenance:{statement_ids, source_text, modality},
                  lowering:Lowerable|Residual{reason},
                  confidence:AutomationConfidence }
```

The algebra is **closed** (finite operator set) so it is realizability-
checkable (`R16-CAPTURE-FIDELITY-GATES`), mechanically lowerable, and the
residual boundary is crisp (`lowering: Residual{reason}` is explicit, not
silent loss).

### (c) `TemporalRuleRecord` → ContractIR migration map (zero capture loss)

Every existing predicate/disposition has an explicit ContractIR target;
the **currently-residual cases become modelled** (captured in the typed
KG even if their `.isf` lowering is still residual — exactly the R16
thesis):

| Current shape | ContractIR | `.isf` lowering |
| --- | --- | --- |
| windowed `SignalValue` consequent (`max_cycles≥1`) | `Guarantee Eventually{Level(sig=val), Within{max=N}}` | `(contract … (eventually sig (within N)))` (parity) |
| non-windowed `SignalValue` consequent + repr. antecedent | `Guarantee Drive{sig,val}` `guard=(== ante v)` | `(rule … (sig val))` (parity) |
| `HandshakeComplete{valid,ready}` | `Guarantee HandshakeBarrier{valid,ready}` | `(stage p (ready r)(valid v))` — **subsumes `ISF-HANDSHAKE-STAGE-LOWERING`** (FSMGen accepts it at pin `9bfb9a20`) |
| `SignalStable`/`ActorMaintainsSignalStable{from→to}` | `Guarantee Stable{sig, Between{from,to}}` | checked stability where FSMGen supports, else `Residual{reason}` — **but the intent is now in the typed KG** (was lost) |
| `cycle_window` with `max_cycles==0` | `SameCycle` window | explicit `Residual` (never `(within 0)`) — modelled, not a silent skip |
| `ActorDrivesSignal`/`Samples`/`SignalSampled` (no value) | `Assume`/`Guarantee` with `Edge`/sampling `EventExpr` | residual/metadata; captured |
| `antecedents` (multi) | `guard` = conjunction of representable `Condition`s; non-representable parts → contract still emitted, guard flagged partial-residual | as today |
| `TickPhase` Pre/Post, `ClockEdge` | `PhaseBoundary`/sampling `EventExpr` operands; `edge` field | preserved |

A pure `contract_from_temporal_rule(&TemporalRuleRecord) -> ActorContract`
is lossless for representable cases and emits explicit
`Obligation`+`Residual{reason}` for the rest.

### (c2) Migration phases + CI-parity gate

- `.2` (done): add `ir/contract.rs` + serde + `contract_from_temporal_rule`;
  add `actor_contracts` field additively+unpopulated. No behaviour
  change (temporal_rules still drives `.isf`; zero artifact churn).
- `.3` — **parity-preserving re-point ONLY** (corrected: a behaviour
  change cannot share a leaf with a byte-identical parity gate).
  Populate `actor_contracts`; add `classify_actor_contract` that
  reproduces the **exact** current `classify_temporal_rule`
  Contract|Rule|Residual decisions (`HandshakeBarrier` **stays
  Residual** here for parity); switch the `.isf` lowering call site to
  `actor_contracts`. **Parity gate:** emitted `.isf` semantically
  identical pre/post on the real corpus; the fsmgen-strict tests
  (`bounded_contract_passes_…`, `temporal_rule_isf_passes_…`,
  `isf_temporal_rules_reach_isf_end_to_end`) stay green; add a
  transition parity test. Then audit `temporal_rules` consumers
  (converge snapshot / validate / learn_priors — the
  `ISF-ONLY-IR-PRUNE.1` method) and project-or-migrate.
- `.4` — **deliberate behaviour change, post-parity**, fsmgen-strict-
  verified on the corpus: enable `HandshakeBarrier → (stage …)` (the
  subsumed `ISF-HANDSHAKE-STAGE-LOWERING`). Only the handshake disposition
  changes; everything else stays parity.
- `.5`: close + sync book (`pipeline/isf-adapter.md`,
  `domain/temporal-semantics.md`) + ROADMAP R16.

### (d) `ISF-HANDSHAKE-STAGE-LOWERING` subsumption

That `proposed` tree (HandshakeComplete → `(stage …)`) is **folded** into
ContractIR: `HandshakeComplete` → `HandshakeBarrier` obligation, lowered
to `(stage p (ready r)(valid v))` in **`R16-CONTRACT-IR.4`** (the
post-parity, fsmgen-strict-verified behaviour-change leaf — corrected
from the original `.3`, which is parity-only). The standalone tree is
marked `superseded` → `R16-CONTRACT-IR`.

## Decisions

- `2026-05-19`: Highest program leverage (per `R16-INTENT-CAPTURE`
  thesis): representation loss is currently misdiagnosed as extraction
  loss. Created `proposed`; first to be promoted.
- `2026-05-19` (`.3` slice 2 — re-point complete + 2 more honest catches):
  (a) the re-point made `.isf` depend on `actor_contracts`, which a
  pre-ContractIR persisted `IntentIR` lacks → it would emit *no*
  temporal `.isf` for old artifacts. Added a back-compat fallback:
  empty `actor_contracts` ⇒ project from `temporal_rules` via the same
  lossless conversion (parity for old & new). (b) the 4 old fns became
  production-dead; retained `#[cfg(test)]` as the parity oracle (test
  build only), `TemporalRuleRecord`/`TemporalPredicateRecord` imports
  test-gated. Added `guard_candidates` to `ActorContract` so
  `classify_actor_contract` reproduces `temporal_antecedent_condition`
  EXACTLY for the multi-antecedent case (parity by construction, not
  gate-and-hope). Consumer audit (`ISF-ONLY-IR-PRUNE.1` method):
  `temporal_rules` is KEPT — load-bearing for `validate.rs` (96 refs,
  R7/R15b diagnostics), `learn_priors`, and the fallback; `actor_contracts`
  is the additive typed projection lowering consumes.
- `2026-05-19` (`.3` parity finding — picky-auditor catch before any
  re-point): the classifier's `temporal_consequent_signal` treats
  `SignalStable`/`ActorMaintainsSignalStable`/`ActorSamplesSignal`/
  `SignalSampled`/`ActorDrivesSignal` as signal-bearing, so a *windowed*
  such rule currently lowers to a **Contract**. The `.2`
  `contract_from_temporal_rule` wrongly mapped those to `Stable`/`Observe`
  + Residual, dropping the window — a latent corpus-wide `.isf` parity
  break the re-point would have silently introduced. Fixed: the
  conversion is now **window-first, structurally parallel to
  `classify_temporal_rule`** (`consequent_signal` mirrors
  `temporal_consequent_signal`); windowed signal-bearing → `Eventually`
  + `Lowerable` for every predicate kind. Added `source_rule_id`
  provenance so `classify_actor_contract` can reproduce identical
  `.isf` naming. New unit test
  `windowed_signalstable_is_eventually_lowerable_parity_finding`; CI
  1062/0; zero `.isf`/artifact change (still unpopulated/unconsumed).
- `2026-05-19` (precise parity definition for `.3`): the parity gate is
  **the FSMGen-facing emitted `.isf` is byte-identical pre/post, AND the
  set of Contract/Rule emissions and the set of residual `rule_id`s is
  identical**. Residual *reason wording* is SPECFORGE-internal
  adapter-metadata (in `adapter.json`, not `.isf`) and may become the
  (more accurate) ContractIR reason — an accepted, documented
  consequence, not a regression. This avoids contorting the typed model
  to leak window-ness through `HandshakeBarrier`.
- `2026-05-19` (honest split — TASK_TREE rule 5): the `.1` design
  conflated two mutually-exclusive things in one leaf — a
  byte-identical corpus **parity** re-point and the
  `HandshakeBarrier → (stage …)` **behaviour change** (parity forbids
  any `.isf` change; the stage-enable *is* an `.isf` change). Split:
  `.3` = parity-preserving re-point only (HandshakeBarrier stays
  Residual); `.4` = the post-parity fsmgen-strict-verified
  handshake-stage behaviour change (the subsumed
  `ISF-HANDSHAKE-STAGE-LOWERING`); `.5` = close. Design `(c2)`/`(d)` +
  frontier corrected to match. Recorded as an honest tree update before
  any `.3` code.
- `2026-05-19` (`.2` honest refinements during implementation): (1) the
  carried field is `actor_contracts`, not `contracts` — `SemanticIR`
  already has `contracts: Vec<ContractRecord>` (semantic protocol
  contracts); a duplicate field name would not compile and would collide
  in serde. Caught by the picky-auditor compile check before any commit.
  (2) Added `Obligation::Observe{signal}` for predicates that name a
  boundary signal but carry no concrete value/window
  (`ActorDrivesSignal`/`ActorSamplesSignal`/`SignalSampled`): an honest,
  closed, residual-lowered variant — captured in the typed KG, never
  fabricated into a value. The `.1` design text + book mirror were
  corrected to match (docs-track-code discipline).
- `2026-05-19` (`.1`): placement = typed layer, no new stage (canonical-
  IntentIR doctrine + fewer-stages ethos). Closed operator algebra fixed.
  Migration is additive-then-reproint with a corpus CI-parity gate.
  `ISF-HANDSHAKE-STAGE-LOWERING` subsumed (→ `superseded`). The
  currently-residual temporal cases become *modelled* in the typed KG
  even when their `.isf` lowering stays residual — directly serving the
  thesis (accurate typed KG first; lowering mechanical/honest).
- `2026-05-19`: **Promoted `proposed → active`** by `R16-INTENT-CAPTURE.2`
  after the extraction methodology was presented and accepted. Frontier
  is `.1` — a docs-only design leaf (no code until the placement +
  operator grammar + `TemporalRuleRecord` migration/parity map are
  recorded), honoring the no-code-without-an-agreed-design discipline.

## Blockers

- None. Active; frontier `R16-CONTRACT-IR.2` (implement the typed model).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-19` | `R16-CONTRACT-IR.1` | current types re-verified; full design recorded (placement/grammar/migration/parity/subsumption); every `TemporalRuleRecord`/`TemporalPredicateRecord` case mapped incl. residual-now-modelled; book mirror added; mdBook builds | `passed` (docs-only) |
| `2026-05-19` | `R16-CONTRACT-IR.2` | `ir/contract.rs` typed model + serde + `contract_from_temporal_rule` + 7 unit tests; additive unpopulated `actor_contracts` field; collision + Observe refinements; full `scripts/run_ci.sh` | `passed` (1061 passed, 0 failed; mdBook builds; zero artifact churn) |
| `2026-05-19` | `R16-CONTRACT-IR.3` (slice 1) | parity-faithful window-first conversion rewrite (windowed signal-bearing → Eventually for all predicate kinds) + `source_rule_id`; +1 parity unit test; full `scripts/run_ci.sh` | `passed` (1062 passed, 0 failed; mdBook builds; zero `.isf`/artifact change — unpopulated/unconsumed) |
| `2026-05-19` | `R16-CONTRACT-IR.3` (slice 2 — done) | `classify_actor_contract` + `guard_candidates` + populate builders + back-compat fallback + re-point `.isf` loop; pointwise oracle parity test; LIVE nvme baseline match (0/250/92, strict `success:true`); consumer audit (keep `temporal_rules`); full `scripts/run_ci.sh` | `passed` (1063 passed, 0 failed; mdBook builds; parity proven 3 ways) |
| `2026-05-19` | `R16-CONTRACT-IR.4` | re-added `IsfStage`/render + `HandshakeBarrier`→Stage + ready-is-input gate (real-binary catch); 3 new tests incl. fsmgen-strict on synthesized `(stage …)`; LIVE 4-corpus (0 stages — no corpus handshake_complete; zero `.isf` change, nvme==`.3` baseline, strict `success:true`); full `scripts/run_ci.sh` | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R16-CONTRACT-IR.1` | `R16-CONTRACT-IR.1 — ContractIR design (placement + operator algebra + migration/parity + subsumption)` | docs-only; book mirror per `BOOK-METHOD-DOC` |
| `R16-CONTRACT-IR.2` | `R16-CONTRACT-IR.2 — implement typed ir/contract.rs model + serde + conversion` | first R16 code; additive unpopulated field; CI 1061/0 |
| `R16-CONTRACT-IR.3` (s1) | `R16-CONTRACT-IR.3 (slice 1) — parity-faithful window-first conversion + source_rule_id` (`4c5d34e0`) | CI 1062/0; zero `.isf` change |
| `R16-CONTRACT-IR.3` (s2) | `R16-CONTRACT-IR.3 (slice 2) — classify_actor_contract + re-point + fallback; parity proven` | CI 1063/0; `.3` DONE |
| `R16-CONTRACT-IR.4` | `R16-CONTRACT-IR.4 — enable HandshakeBarrier → (stage …); ready-is-input gate; fsmgen-strict-verified` | dormant on corpus (no handshake_complete); zero `.isf` change |

## Changelog

- `2026-05-19`: Created `proposed` as program point #1 / DAG root.
- `2026-05-19`: Promoted to `active` (frontier `.1`, docs-only design)
  after the user accepted the extraction methodology and authorized the
  program to begin. Concrete `.1`–`.4` leaves defined.
- `2026-05-19`: `.1` done — ContractIR design fixed (typed layer / no new
  stage; closed operator algebra; lossless `TemporalRuleRecord` migration
  map with residual-cases-now-modelled; corpus CI-parity gate;
  `ISF-HANDSHAKE-STAGE-LOWERING` subsumed). Thorough mirror added to the
  mdBook per `BOOK-METHOD-DOC`. Frontier → `.2` (implement typed model).
- `2026-05-19`: `.4` DONE — `HandshakeBarrier → (stage …)` enabled
  (re-added `IsfStage`/render; `TemporalRuleDisposition::Stage`;
  classify arm). Real-binary picky catch: FSMGen requires the stage
  `ready` to be an actor INPUT → added an `input_signal_names` gate
  (emit stage only then, else explicit residual — never strict-invalid);
  test shape corrected (ready=input/valid=output per
  FSMGEN-SUBMODULE-BUMP.1). 3 new tests incl. real-binary strict on the
  synthesized `(stage …)`. Honest finding: NO corpus `temporal_rule`
  has `handshake_complete` → 0 stages corpus-wide, zero `.isf` change
  (nvme == `.3` baseline) — a verified-but-dormant capability,
  activates when R16 #3/#4/#6 ground handshakes. `ISF-HANDSHAKE-STAGE-
  LOWERING` thereby delivered (folded). Frontier → `.5` (close).
- `2026-05-19`: `.3` DONE — slice 2 landed `classify_actor_contract`
  (reproduces `classify_temporal_rule` exactly via obligation +
  `guard_candidates` + `source_rule_id`), populated `actor_contracts`
  in both builders, added the pre-ContractIR back-compat fallback, and
  re-pointed the `.isf` lowering loop. Parity proven 3 ways (pointwise
  oracle test / live nvme baseline 0/250/92 strict-valid / e2e+fsmgen
  CI). Consumer audit → keep `temporal_rules`. CI 1063/0. Frontier →
  `.4` (post-parity `HandshakeBarrier → (stage …)` behaviour change).
- `2026-05-19`: `.3` slice 1 — picky-auditor caught a latent corpus-wide
  parity break (windowed `SignalStable`/sample/actor-drive currently →
  Contract, but `.2` conversion mapped them to Stable/Observe+Residual).
  Rewrote `contract_from_temporal_rule` window-first parallel to
  `classify_temporal_rule`; added `source_rule_id`; +1 parity unit test;
  CI 1062/0; zero `.isf` change. Precise `.3` parity definition recorded
  (FSMGen-facing `.isf` + Contract/Rule/residual-set identity; residual
  reason wording is internal metadata). `.3` `in_progress` — re-point +
  corpus gate next.
- `2026-05-19`: honest tree split (rule 5) — `.3` parity-only re-point;
  new `.4` = post-parity fsmgen-strict-verified `HandshakeBarrier →
  (stage …)` (subsumed `ISF-HANDSHAKE-STAGE-LOWERING`); `.5` close.
  Design/frontier corrected. No code.
- `2026-05-19`: `.2` done — `ir/contract.rs` typed model + serde +
  `contract_from_temporal_rule` + 7 unit tests; additive unpopulated
  `actor_contracts` field (zero artifact churn). Honest refinements:
  field renamed to avoid the `SemanticIR.contracts` collision; added
  `Obligation::Observe` for no-value predicates; design doc + book
  corrected to match. Full CI 1061/0. Frontier → `.3` (populate +
  re-point lowering with the real-corpus CI-parity gate).
