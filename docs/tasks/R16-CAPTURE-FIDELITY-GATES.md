# R16-CAPTURE-FIDELITY-GATES: objective capture-fidelity metric (point #5)

## Metadata

- Tree ID: `R16-CAPTURE-FIDELITY-GATES`
- Status: `active`
- Roadmap lane: `R16`
- Program: `R16-INTENT-CAPTURE` (point #5, **pulled to order 3** — the
  program's objective function)
- Created: `2026-05-19`
- Last updated: `2026-05-20`
- Owner: repo-local workflow

## Goal

Make "how well did we capture intent?" an objective, gating number,
because per the program thesis the hard problem (prose + timing-diagram →
typed KG) cannot be improved if it cannot be measured. Two gates over
ContractIR:

1. **Realizability / consistency** — automaton-product or SMT check that
   some implementation satisfies all `assume`/`guarantee` contracts and
   the handshake set is deadlock-free / latency bounds are mutually
   satisfiable. Unrealizable captured intent ⇒ strongest "capture is
   wrong" signal ⇒ high-value repair residual, never silent lowering.
2. **Figure conformance** — replay the spec's **own** example waveforms /
   sequence diagrams as traces against the mined contracts. "Figure 3-2
   satisfies the contract extracted for Figure 3-2" is near-ground-truth
   and free (the PDF ships its own test vectors).

The pair becomes the objective function that gates and steers #3/#4/#6.

## Non-Goals

- Not full formal verification of a design — a bounded realizability /
  conformance harness, not a model checker product.
- Does not itself improve extraction; it measures and gates it.

## Acceptance Criteria

- A realizability check + a figure-conformance replay run over ContractIR
  and emit a calibrated per-contract + per-document fidelity score with
  provenance; failures route to residual/repair, never fabrication.
- A corpus fidelity report (analogous to `kg-bench`, but semantic
  conformance) regression-locked.
- `scripts/run_ci.sh` green per leaf; every leaf via `COMMIT.md`.

## Task Tree

- ID: `R16-CAPTURE-FIDELITY-GATES.1`
  Status: `done`
  Goal: typed fidelity-gate set + finding shape + scoring/calibration +
  report shape design (docs-only, parallels `R16-CONTRACT-IR.1` /
  `R16-KG-PROTOCOL-ONTOLOGY.1`).
  Acceptance: `Design recorded in this tree + mirrored in mdBook per BOOK-METHOD-DOC; docs-only; scripts/run_docs_ci.sh green.`
  Verification: `passed` — see "Design (`.1` output)" section below;
    book mirror under *Temporal-Intent Capture* chapter; `mdbook build`
    green.
  Commit: `see Commit Log`

- ID: `R16-CAPTURE-FIDELITY-GATES.2`
  Status: `done`
  Goal: implement the typed `fidelity` module (`FidelityGate`,
  `FindingStatus`, `FidelityFinding`) + per-gate evaluators that act on
  `actor_contracts` + trace-replay primitive. Additive empty
  `fidelity_findings: Vec<FidelityFinding>` field on SemanticIr/IntentIr.
  Acceptance: `Typed module + per-gate evaluators + trace-replay primitive + additive empty field + unit tests; zero artifact churn; scripts/run_ci.sh green.`
  Verification: `passed` — `crates/specforge/src/ir/fidelity.rs` added:
    `FidelityGate` (6 variants), `FindingStatus = Pass | Fail |
    NotEvaluated` (NotEvaluated never silently Pass — honesty),
    `FidelityFinding`, `FigureTrace`, per-gate evaluators
    (`evaluate_realizable_boundary`, `evaluate_realizable_direction`,
    `evaluate_realizable_handshake` matching the CONTRACT-IR.4 gate,
    `evaluate_residual_honesty` (empty-reason / Lowerable-Observe),
    `evaluate_no_strict_invalid` (Lowerable + (Observe|OrderedBefore)
    ⇒ Fail), `evaluate_figure_conformance` honestly NotEvaluated when
    no trace), bounded `evaluate_figure_trace` primitive
    (`.2` scope: Stable+Drive only; missing signal ⇒ NotEvaluated, not
    Pass), `FidelitySummary` with `score()` = pass/(pass+fail) over
    *evaluated* gates only (None when no gates evaluated) +
    `meets_threshold(t)` requiring score≥t AND fail==0 (threshold 1.0
    default = honest discipline). 9 unit tests covering Pass/Fail/
    NotEvaluated paths per gate + summary/threshold behaviour. Module
    registered in `ir/mod.rs`. Additive `fidelity_findings` field on
    `SemanticIr` (empty `Vec::new()`) and `IntentIr` (carried forward
    from `SemanticIR`, parallel to `actor_contracts`/`protocol_graph`),
    serde-default + `skip_serializing_if = Vec::is_empty` ⇒ **zero
    artifact/fixture churn**. Full `scripts/run_ci.sh` green (clippy
    too: rewrote `iter().any(|v| *v == want)` → `contains(&want)`).
  Commit: `see Commit Log`

- ID: `R16-CAPTURE-FIDELITY-GATES.3`
  Status: `done`
  Goal: wire the producer — `SemanticIr::build` runs the gate
  evaluators over `actor_contracts`, populating `fidelity_findings`;
  failures route to residual (a `Fail` on a contract marked
  `Lowerable` becomes a `Residual` with the gate message —
  honesty doctrine mechanically enforced). IntentIR carries findings
  forward (already done in `.2`).
  Acceptance: `Producer wired; per-leaf Fail-to-residual routing tested; corpus parity preserved (a Fail downgrades to Residual rather than fabricating a Lowerable — honesty doctrine); scripts/run_ci.sh green.`
  Verification: `passed` — `apply_fidelity_gates(&mut [ActorContract],
    &[ActorPortRecord])` added (private helper in
    `crates/specforge/src/ir/semantic.rs`): builds per-actor
    `(declared, inputs, outputs)` sets from `actor_ports`
    (Input/Output/InOut/Unknown) + global declared fallback when
    `actor_name` is `None`/unknown (direction-bearing gates run
    `NotEvaluated`, honestly). For each contract: 6 evaluators run
    (`RealizableBoundary`, `RealizableDirection`, `RealizableHandshake`,
    `ResidualHonesty`, `NoStrictInvalid`, `FigureConformance` =
    NotEvaluated until #4). Honesty doctrine: a `Lowerable` contract
    with any `Fail` ⇒ rerouted to `Residual{reason =
    "fidelity:<Gate>: <message>"}` BEFORE the `.isf` adapter consumes
    it; already-Residual contracts and Pass-only contracts left
    untouched. Findings recorded reflect pre-routing observations (a
    `Fail` paired with a now-Residual contract is the doctrine
    working). `SemanticIr::build` populates
    `fidelity_findings`; `IntentIR` carries it forward (from `.2`). 3
    new routing unit tests (Lowerable+Observe→Residual; clean Drive
    stays Lowerable; preexisting Residual untouched). Clippy-clean
    (`ActorSigSets` type alias; `or_default`). Full
    `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

- ID: `R16-CAPTURE-FIDELITY-GATES.4`
  Status: `pending`
  Goal: corpus fidelity report — `specforge validate` adds a `fidelity:`
  block (pass/fail/not_evaluated counts + score + first-N failures)
  for SemanticIR and IntentIR; baseline-lock the corpus fidelity
  numbers; close tree + book + ROADMAP R16.
  Acceptance: `validate prints fidelity block; corpus baseline locked (e.g., nvme); tree marked done; ROADMAP R16 closed for CAPTURE-FIDELITY-GATES; mdBook "Status — delivered" subsection per BOOK-METHOD-DOC; scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R16-CAPTURE-FIDELITY-GATES.1` | `done` | Gate design fixed; book mirror added |
| 2 | `R16-CAPTURE-FIDELITY-GATES.2` | `done` | Typed `fidelity` module + 5 per-gate evaluators + trace-replay primitive + summary/threshold + 9 tests; additive empty field; zero artifact churn |
| 3 | `R16-CAPTURE-FIDELITY-GATES.3` | `done` | Producer wired in `SemanticIr::build`; Fail-on-Lowerable → Residual routing (honesty doctrine MECHANICALLY enforced); 3 routing tests |
| 4 | `R16-CAPTURE-FIDELITY-GATES.4` | `pending` | **Next** — corpus `fidelity:` block in `specforge validate` + baseline-lock + close |

## Design (`.1` output, 2026-05-20)

The hard problem is captured intent fidelity (per the program thesis);
this tree makes "how well did we capture intent?" an **objective,
gating number** so the extraction trees (`#3`/`#4`/`#6`) have a
feedback signal to optimize against. Design parallels
`R16-CONTRACT-IR.1` / `R16-KG-PROTOCOL-ONTOLOGY.1`: typed layer, no new
stage, additive serde-skipped-while-empty field, mechanical projection,
honest residual routing.

### Placement — typed layer, no new stage

A new `crates/specforge/src/ir/fidelity.rs` defines `FidelityGate`,
`FindingStatus`, `FidelityFinding`. An additive
`fidelity_findings: Vec<FidelityFinding>` field is added to
`SemanticIr`/`IntentIr` (serde-default + `skip_serializing_if =
Vec::is_empty` ⇒ zero artifact churn while no gate has run). Producer
runs in `SemanticIr::build` (after `actor_contracts` exist); IntentIR
carries findings forward (parallel to `actor_contracts` /
`protocol_graph`).

### Typed gate set

```rust
pub enum FidelityGate {
    RealizableBoundary,   // every referenced signal declared on the actor boundary
    RealizableDirection,  // guarantee→output, assume→input (per actor direction)
    RealizableHandshake,  // HandshakeBarrier: ready∈inputs, valid∈outputs (matches CONTRACT-IR.4 gate)
    ResidualHonesty,      // Residual ⇒ non-empty reason; Lowerable ⇒ no classifier-residual provenance
    NoStrictInvalid,      // the contract, when lowered, would not produce FSMGen-strict-invalid syntax
    FigureConformance,    // a figure-derived trace assigned to this contract satisfies it (deferred trace plumbing)
}
```

### Finding shape

```rust
pub enum FindingStatus { Pass, Fail, NotEvaluated }
pub struct FidelityFinding {
    pub gate: FidelityGate,
    pub status: FindingStatus,
    pub contract_id: Option<String>,
    pub message: String,         // human-readable; for Fail, the residual reason
}
```

### Scoring / calibration

Per-document fidelity score =
`pass / (pass + fail)` over **evaluated** gates (denominator excludes
`NotEvaluated`); `NotEvaluated` is **counted separately**, never
silently treated as `Pass`. A document is "at fidelity X" iff
`score ≥ X` AND `fail == 0`. Default threshold = `1.0` (any Fail =
below-threshold) — the disciplined honesty default (residual not
fabricate, per the project doctrine).

### Trace-replay (figure conformance)

A `FigureTrace { signals: BTreeMap<String, Vec<u64>>, ticks: u32 }`
expresses a cycle-accurate sample sequence (one value per signal per
tick). `evaluate_figure_trace(&ActorContract, &FigureTrace) -> FindingStatus`
is a bounded structural check: every obligation's witness within the
trace's tick window must hold. Until `R16-WAVEFORM-CONTRACT-MINING`
(#4) extracts `FigureTrace` from PDF figures, the `FigureConformance`
gate runs `NotEvaluated` corpus-wide (honest dormant capability — the
trace-replay primitive is unit-tested with synthesized traces in
`.2`).

### Residual routing (the honesty doctrine, mechanically enforced)

A `Fail` on a contract marked `Lowerable` is the strongest signal that
captured intent is wrong. `.3` routes such failures to **Residual**
with the gate message as the reason; the contract is NOT silently
lowered. This makes the doctrine ("unverifiable temporal intent →
explicit residual, never fabricated") mechanically enforced rather
than only authorial. Pre-existing Residual contracts with empty
reasons surface as `ResidualHonesty` Fail.

### Report shape (`.4`, `validate`)

Additive lines in the SemanticIR + IntentIR count blocks of `specforge
validate`:

```
  fidelity: pass=… fail=… not_evaluated=…  score=…
  fidelity_failures (first 5):
    [gate] contract_id: message
    …
```

Structured-metric/JSON shape is intentionally **not** touched (kept
bounded, mirrors the `R16-KG-PROTOCOL-ONTOLOGY.4` precedent).

### Non-Goals (recorded)

- Not a model-checker product (bounded algebraic gates only).
- Does not improve extraction itself (measurement + residual routing).
- `FigureTrace` recovery from PDFs is the extraction trees' job
  (`#4 WAVEFORM-CONTRACT-MINING`); until then `FigureConformance` is
  honestly `NotEvaluated` corpus-wide (not faked `Pass`).

## Decisions

- `2026-05-19`: Re-ordered from point #5 to program order 3 — measuring
  the hard problem must precede pouring effort into it; the spec's own
  figures are near-ground-truth. Created `proposed`.
- `2026-05-20`: **Promoted `proposed → active`** by
  `R16-INTENT-CAPTURE.2` (umbrella) after DAG predecessor
  `R16-CONTRACT-IR` closed and sibling `R16-KG-PROTOCOL-ONTOLOGY` (#2)
  closed. `.1` design fixed (docs-only): typed layer / no new stage
  (parallels CONTRACT-IR/KG-ONTOLOGY); typed gate set; `Pass / Fail /
  NotEvaluated` (honestly distinct — `NotEvaluated` never silently
  Pass); per-document score with default threshold 1.0; `Fail` on a
  `Lowerable` contract → routed to `Residual` in `.3` (honesty doctrine
  mechanically enforced); `FigureConformance` honestly dormant until
  `#4 WAVEFORM-CONTRACT-MINING` produces `FigureTrace`s from PDF
  figures; structured-metric/JSON shape intentionally not touched
  (bounded, KG-ONTOLOGY.4 precedent). Book mirror per BOOK-METHOD-DOC.

## Blockers

- None. Active; frontier `R16-CAPTURE-FIDELITY-GATES.2`.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-20` | `R16-CAPTURE-FIDELITY-GATES.1` | gate set / finding shape / scoring / trace-replay / residual routing / report shape recorded; book mirror per BOOK-METHOD-DOC; mdBook builds | `passed` (docs-only) |
| `2026-05-20` | `R16-CAPTURE-FIDELITY-GATES.2` | typed `fidelity` module (`FidelityGate`/`FindingStatus`/`FidelityFinding`/`FigureTrace` + 5 per-gate evaluators + bounded `evaluate_figure_trace` + `FidelitySummary`) + 9 unit tests; additive `fidelity_findings` field; clippy-clean (`contains(&want)` rewrite); full `scripts/run_ci.sh` | `passed` (zero artifact churn) |
| `2026-05-20` | `R16-CAPTURE-FIDELITY-GATES.3` | `apply_fidelity_gates` producer in `SemanticIr::build` (per-actor + global declared/inputs/outputs from `actor_ports`); Fail-on-Lowerable → `Residual{reason="fidelity:<Gate>: <message>"}` routing; 3 routing tests; clippy-clean; full `scripts/run_ci.sh` | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R16-CAPTURE-FIDELITY-GATES.1` | `R16-CAPTURE-FIDELITY-GATES.1 — gate design (promote #5/order-3)` (`4d0b2207`) | docs-only; book mirror; also the `R16-INTENT-CAPTURE.2` #5/order-3 promotion |
| `R16-CAPTURE-FIDELITY-GATES.2` | `R16-CAPTURE-FIDELITY-GATES.2 — typed fidelity module + per-gate evaluators + trace primitive + additive empty field` (`a111cd43`) | first FIDELITY code; zero artifact churn; producer wiring deferred to `.3` |
| `R16-CAPTURE-FIDELITY-GATES.3` | `R16-CAPTURE-FIDELITY-GATES.3 — producer wiring + Fail-on-Lowerable → Residual routing` | honesty doctrine mechanically enforced |

## Changelog

- `2026-05-19`: Created `proposed` as program point #5, ordered 3rd.
- `2026-05-20`: Promoted to `active` (DAG predecessor `R16-CONTRACT-IR`
  done; sibling `R16-KG-PROTOCOL-ONTOLOGY` (#2) done); `.1` gate design
  fixed + book mirror; concrete `.1`–`.4` leaves defined. Frontier →
  `.2` (implement typed `fidelity` module).
- `2026-05-20`: `.3` done — producer wired:
  `apply_fidelity_gates(&mut [ActorContract], &[ActorPortRecord])` in
  `SemanticIr::build` builds per-actor `(declared, inputs, outputs)`
  sets (Input/Output/InOut/Unknown) with global declared fallback when
  `actor_name` is `None`/unknown (direction-bearing gates honestly
  `NotEvaluated`); for each contract runs all 6 evaluators; a
  `Lowerable` contract with any `Fail` is rerouted to
  `Residual{reason = "fidelity:<Gate>: <message>"}` BEFORE the `.isf`
  adapter consumes it. The honesty doctrine ("unverifiable temporal
  intent → explicit residual, never fabricated") becomes
  **structural**, not only authorial. 3 routing unit tests
  (Lowerable+Observe→Residual; clean Drive stays Lowerable; preexisting
  Residual untouched). Clippy-clean (`ActorSigSets` type alias;
  `or_default`). Full CI green. Frontier → `.4` (corpus `fidelity:`
  block in `specforge validate` + baseline-lock + close).
- `2026-05-20`: `.2` done — `ir/fidelity.rs` typed module
  (`FidelityGate{6}`/`FindingStatus{Pass,Fail,NotEvaluated}`/
  `FidelityFinding`/`FigureTrace`) + 5 per-gate evaluators (boundary/
  direction/handshake/residual-honesty/no-strict-invalid) + bounded
  `evaluate_figure_trace` primitive (`.2` scope: Stable+Drive only;
  missing-signal ⇒ NotEvaluated, never silently Pass) +
  `FidelitySummary{pass,fail,not_evaluated}` with `score()` over
  *evaluated* gates and `meets_threshold(1.0)` requiring `fail == 0`
  (honest discipline default). 9 unit tests. Additive
  `fidelity_findings` field on SemanticIr(empty)/IntentIr(carried) ⇒
  zero artifact churn (CONTRACT-IR.2/KG-ONTOLOGY.2 discipline). Full
  CI green (clippy too — `samples.iter().any(|v| *v == want)` rewritten
  to `samples.contains(&want)`). Frontier → `.3` (wire producer in
  `SemanticIr::build` + Fail-on-Lowerable → Residual routing).

## Dependencies / Order

- Depends on `R16-CONTRACT-IR`. Pulled ahead of #3/#4/#6: it is their
  objective function and feedback driver. Gates (does not block creation
  of) `R16-MULTIMODAL-CONTRACT-FUSION` / `R16-WAVEFORM-CONTRACT-MINING` /
  `R16-CONSTRAINED-VERIFIED-EXTRACTION`.
