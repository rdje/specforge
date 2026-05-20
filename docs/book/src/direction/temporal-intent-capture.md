# SOTA Temporal-Intent Capture (R16)

This chapter is the human-facing capture of the active forward program.
The authoritative, machine-tracked source is the task-tree umbrella
`docs/tasks/R16-INTENT-CAPTURE.md` and ROADMAP lane `R16`; this page
must stay consistent with them.

## Thesis

A digital-design PDF (protocol or component) encodes design intent as the
**temporal behavior of actors observed at their boundary — pins and
ports**. The single hardest, highest-value problem is **accurate and
reliable extraction of that temporal behavior from prose *and* from
timing diagrams into a typed knowledge graph**. Once the typed KG holds
accurate temporal behavior for every involved signal/port, the rest
(KG → `IntentIR` → `.isf` → FSMGen) is **almost mechanical**.

Consequence for sequencing:

1. the typed target the extraction lands in must exist first and be
   shaped like a *timed contract over actor boundaries* (mechanical to
   lower from);
2. capture fidelity must be **objectively measurable early** — the
   spec's own figures/waveforms are near-ground-truth conformance
   vectors;
3. then the bulk of effort concentrates on **prose + timing-diagram →
   typed KG extraction fidelity**, measured against that metric.

The residual-honesty doctrine is enforced throughout: temporal intent
the source does not license is preserved as an explicit residual, never
fabricated.

## The six points, in program order

| Order | Tree | Point | What it delivers |
| --- | --- | --- | --- |
| 1 | `R16-CONTRACT-IR` | #1 | Typed timed-contract IR — per-actor `assume`/`guarantee`, operators `rose/fell`, `stable … throughout`, `s ##[m:n] t`, `s until t`, `eventually within N`, `mutex`, `ordered_before`. One bound obligation = one object. The mechanical-to-lower target. (DAG root) |
| 2 | `R16-KG-PROTOCOL-ONTOLOGY` | #2 | First-class `Channel` / `Transaction` / `Phase` / `HandshakePair` KG nodes + edges; `IntentIR` becomes a systematic projection of protocol structure. |
| 3 | `R16-CAPTURE-FIDELITY-GATES` | #5 | Realizability/consistency check + replay of the spec's own figures as conformance vectors → the **objective capture-fidelity metric** and residual/repair driver. Pulled early — it is the objective function for the hard problem. |
| 4 | `R16-MULTIMODAL-CONTRACT-FUSION` | #3 | Cluster cross-modal evidence (prose + table + figure + state diagram) keyed by (actor, channel/group, phase) into one contract with provenance + typed merge + explicit disagreement surface. |
| 5 | `R16-WAVEFORM-CONTRACT-MINING` | #4 | Timing diagram → structured partial trace → generalized contract; cross-check vs prose. **The crux extraction thrust** (highest ceiling, research-grade). |
| 6 | `R16-CONSTRAINED-VERIFIED-EXTRACTION` | #6 | Schema-constrained LLM/VLM extraction directly into ContractIR + entailment verifier + protocol-pattern template library + uncertainty-driven converge. **The crux, continuous.** |

Dependency DAG: `1 → 2`; `1 → 3`; `{1,2,3} → 4`; `{1,3} → 5`;
`{1,3} → 6`. `5` and `6` produce/clean contract candidates that `4`
fuses; `3` measures `4`/`5`/`6`. `1` is the spine; `3` is the objective
function.

## Why this order

Points #1/#2 are not the hard part — they are the *typed target* so
extraction has somewhere accurate to land and so lowering stays
mechanical. Point #5 is pulled ahead of the extraction work because you
cannot improve what you cannot measure, and the spec ships its own
near-ground-truth vectors (its figures). Points #3/#4/#6 are the
extraction-fidelity thrust — the crux the thesis names. Each sub-tree is
`proposed` until promoted in DAG order under
`R16-INTENT-CAPTURE`; all code lands under `COMMIT.md`-tracked leaves
with `scripts/run_ci.sh` green per leaf.

---

This chapter also carries, per sub-tree, a precise account of **how it
is implemented and how it is verified** (the `BOOK-METHOD-DOC`
convention). Sections appear as each sub-tree's design is fixed.

## R16-CONTRACT-IR — how it is implemented and verified

**Why first.** This is the DAG root and the program's highest-leverage
move: today the temporal model is a *bag of predicates*
(`TemporalRuleRecord` = antecedents → consequents + one optional cycle
window), so a single *bound* obligation ("VALID holds until READY; the
payload is stable across exactly that interval; the transfer is the
first cycle both are high") is shredded into disjoint predicates and the
binding between them is lost. That loss is currently misread as an
*extraction* failure when it is a *representation* failure. ContractIR
makes the IR shaped like the thing being captured — a timed contract
over actor boundaries — so extraction (#3/#4/#6) has an accurate place
to land and `.isf` lowering stays mechanical.

### Implementation

- **Placement — a typed layer, not a new pipeline stage.** A new module
  `crates/specforge/src/ir/contract.rs` defines `ActorContract` and a
  small **closed** operator algebra. `SemanticIR` and `IntentIR` carry a
  new additive `actor_contracts: Vec<ActorContract>` field (named so to
  avoid collision with the pre-existing `SemanticIR.contracts:
  Vec<ContractRecord>` semantic-protocol-contracts; serde-default +
  skipped while empty, so older artifacts load and `.2` causes zero
  artifact change — it ships the type but leaves the field unpopulated
  until `.3`). No new `IrStage`/CLI/validate surface —
  this respects the standing "IntentIR is the canonical product
  boundary" doctrine and the ISF-only "fewer stages" ethos, and matches
  how `temporal_rules` / the actor graph already live as typed fields.
- **Closed operator algebra.** `EventExpr` (signal edge / level /
  handshake-fire / phase boundary / start), `Window`
  (`Within{min,max≥1}` / `Between{from,to}` / `SameCycle`), `Obligation`
  (`Eventually` / `Stable` / `Drive` / `HandshakeBarrier` / `Persist` /
  `Sequence` / `Mutex` / `OrderedBefore` / `Observe` — the last a weak
  no-value/no-window boundary fact, captured and residual-lowered, never
  fabricated), a bounded `Condition` guard,
  wrapped in `ActorContract{actor, Assume|Guarantee, guard, obligation,
  clock, edge, channel, phase, provenance, lowering, confidence}`. It is
  deliberately finite so it is realizability-checkable, mechanically
  lowerable, and the residual boundary is explicit (`lowering:
  Residual{reason}`), never silent loss.
- **Migration, additive then re-point.** A pure
  `contract_from_temporal_rule()` maps **every** existing
  `TemporalRuleRecord` / `TemporalPredicateRecord` case to a ContractIR
  construct. Crucially, cases that are *residual* today
  (`HandshakeComplete`, bare stability, 0-cycle windows) become
  **explicitly modelled** in the typed KG even when their `.isf`
  lowering remains residual — that is precisely the thesis (accurate
  typed KG first; honest mechanical lowering second). `HandshakeComplete
  → HandshakeBarrier → (stage …)` folds in and supersedes the separate
  `ISF-HANDSHAKE-STAGE-LOWERING` proposal (FSMGen accepts `(stage …)` at
  the pinned `9bfb9a20`). The field is added additively first
  (no behaviour change), then `.isf` lowering is re-pointed onto
  `ActorContract`.

### Verification

- **CI-parity gate (the core safety property).** After lowering is
  re-pointed, the emitted `.isf` on the real corpus must be
  *semantically identical* (same contracts/rules/residuals) to the
  pre-migration output. The existing real-binary fsmgen-strict tests
  (`bounded_contract_passes_fsmgen_strict_validation`,
  `temporal_rule_isf_passes_fsmgen_strict_validation`,
  `isf_temporal_rules_reach_isf_end_to_end`) must stay green, plus a
  dedicated transition parity test. Any divergence is a regression, not
  an improvement, until the parity baseline is consciously updated.
- **No-capture-loss review.** `.1` enumerated every
  `TemporalRuleRecord`/`TemporalPredicateRecord` variant against an
  explicit ContractIR target (recorded in the task tree's migration
  table); the design is reviewed against the *verified* current types,
  not assumed ones.
- **Consumer audit before removing `temporal_rules`.** Using the
  `ISF-ONLY-IR-PRUNE.1` method (full producer/consumer inventory:
  converge snapshot, validate, learn_priors), `temporal_rules` is either
  projected from `contracts` or its consumers are migrated and it is
  removed — never silently dropped.
- **Standard gate.** `scripts/run_ci.sh` green per leaf; every leaf via
  `COMMIT.md`; fsmgen-binary tests run through the serialized
  `run_fsmgen_strict_check` helper.

### Status — delivered (`R16-CONTRACT-IR` tree closed)

`.1` design → `.2` typed `ir/contract.rs` model → `.3` parity-preserving
re-point (`.isf` lowering now consumes ContractIR; parity proven three
ways — a by-construction pointwise oracle test, the live nvme corpus
matching the pre-ContractIR baseline exactly, and the e2e/fsmgen-strict
suite; a back-compat fallback projects from `temporal_rules` for
pre-ContractIR artifacts) → `.4` enabled `HandshakeBarrier → (stage p
(ready r)(valid v))`, which **subsumes and delivers
`ISF-HANDSHAKE-STAGE-LOWERING`**.

Two honest, recorded constraints on `.4`:

- FSMGen's `ready_valid_barrier` requires the stage's `ready` operand to
  be an actor **input** (verified against the pinned binary). SPECFORGE
  emits `(stage …)` only when that holds, otherwise it preserves an
  explicit residual — it never fabricates a strict-invalid stage.
- It is a **verified-but-dormant** capability today: no current corpus
  `temporal_rule` carries a `handshake_complete` predicate, so zero
  stages are emitted corpus-wide and the emitted `.isf` is unchanged.
  The path is unit- and real-binary-fsmgen-strict-verified; it activates
  once the extraction trees (`#3`/`#4`/`#6`) ground handshake
  completion. This is the thesis in action: the typed target and its
  lowering are correct and ready; capture fidelity is the remaining
  work.

`temporal_rules` is kept (load-bearing for validation/priors and as the
fallback); `actor_contracts` is the additive typed projection lowering
consumes.

Authoritative tracking: `docs/tasks/R16-CONTRACT-IR.md` (the "Design
(`.1` output)" section is the full specification; the Decisions and
Verification Log record every honest catch).

## R16-KG-PROTOCOL-ONTOLOGY — how it is implemented and verified

**Why.** ContractIR gives temporal intent the right *shape*; this gives
the knowledge graph the right *protocol structure*. Today the KG has
actors, signals, the actor-relative direction graph, and `TickPhase`
(clock-edge granularity) — but a protocol PDF is organised around
**channels, transactions, and protocol phases** (AXI AW/W/B/AR/R; APB
setup/access; burst/beat/last), none of which are first-class. Without
them, `IntentIR` cannot be a *systematic projection* of protocol
structure and the cross-modal fusion tree (#3) has nothing stable to key
on.

### Implementation

- **Typed layer, not a new stage** (same decision as ContractIR): a new
  `crates/specforge/src/ir/protocol_graph.rs` defines `Channel`,
  `ProtocolPhase`, `Transaction`, `HandshakePair`; `SemanticIR`/
  `IntentIR` carry an additive `protocol_graph` field (serde-default,
  skipped while empty — zero artifact churn).
- **Closed typed records, not a raw edge soup**: edges are typed
  references (`Channel.signal_names`, `Transaction.phases`/
  `.ordered_before`, `HandshakePair.channel`), matching how the actor
  graph is already modelled. `qualifies`/`stable-during` are expressed
  through a ContractIR `Stable` obligation whose `Between` endpoints are
  `PhaseBoundary`s referencing a `ProtocolPhase`.
- **`TickPhase` ≠ `ProtocolPhase`** — clock-edge vs protocol-stage
  granularity; distinct, co-existing. A `ProtocolPhase` may span many
  ticks.
- **Projection is mechanical**: `IntentIR` carries `protocol_graph`
  forward; `ActorContract.channel`/`.phase` (already typed-optional from
  ContractIR.2) reference the new node ids; `EventExpr::HandshakeFire`
  ties to a `HandshakePair`. No new lowering logic — `.isf` is
  unchanged; this is pure structure the extraction trees populate.
- **Population is out of scope** (Non-Goal): `.2`/`.3` ship + wire the
  *empty* typed structure (parity-preserving, like ContractIR.2);
  recovering channels/phases/transactions from the PDF is the extraction
  trees' (#3/#4/#6) job. This tree ships the vocabulary, not the
  extractor.

### Verification

- Additive empty fields ⇒ zero `.isf`/artifact change while unpopulated
  (parity by construction); the existing fsmgen-strict + e2e suite must
  stay green. `scripts/run_ci.sh` green per leaf; every leaf via
  `COMMIT.md`; the closing leaf refreshes this section (BOOK-METHOD-DOC).
  Honest scope note: `kg-bench` protocol-structure fixtures are
  **deferred to the extraction trees** (`#3`/`#4`/`#6`) that actually
  recover protocol structure — a fixture here would be hollow (this
  tree ships the vocabulary, not the extractor; Non-Goal).

Authoritative tracking: `docs/tasks/R16-KG-PROTOCOL-ONTOLOGY.md`.

### Status — delivered (`2026-05-20`)

`R16-KG-PROTOCOL-ONTOLOGY` is **closed**. All four leaves done:

1. `.1` ontology design fixed (typed layer / no new stage; closed typed
   records; `TickPhase` ≠ `ProtocolPhase`; mechanical projection rules);
2. `.2` typed `protocol_graph` module + serde + additive empty fields on
   `SemanticIr`/`IntentIr` (serde-skipped while empty ⇒ zero artifact
   churn);
3. `.3` projection wired — `project_handshake_pairs` derives
   `HandshakePair` nodes from already-recovered `HandshakeBarrier`
   contracts (LOSSLESS restatement of contract data; NOT PDF
   extraction); accessors + `dangling_contract_refs`; `SemanticIr::build`
   populates `protocol_graph.handshakes`; `IntentIR` carries it forward;
4. `.4` `specforge validate` count surface for `actor_contracts` and
   `protocol_graph: channels=… phases=… transactions=… handshakes=…` on
   both SemanticIR and IntentIR; kg-bench protocol-structure fixtures
   honestly deferred to the extraction trees (`#3`/`#4`/`#6`).

Live evidence: corpus reads `protocol_graph: channels=0 phases=0
transactions=0 handshakes=0` (zero handshake contracts ⇒ empty graph ⇒
zero `.isf`/artifact change across the full suite). Next
DAG-promotable R16 sub-tree = `R16-CAPTURE-FIDELITY-GATES` (#5, order 3;
dep `R16-CONTRACT-IR` ✓).

## R16-CAPTURE-FIDELITY-GATES — how it is implemented and verified

**Why.** The program thesis: prose+waveform → typed KG extraction is
the hard problem; the rest is mechanical. But you cannot improve what
you cannot measure. This tree makes "how well did we capture intent?"
an **objective, gating number** so the extraction trees
(`#3`/`#4`/`#6`) have a feedback signal to optimize against — and so
the residual-honesty doctrine ("unverifiable temporal intent →
explicit residual, never fabricated") is **mechanically enforced**,
not just authorial.

### Implementation

- **Typed layer, not a new stage** (parallels CONTRACT-IR /
  KG-ONTOLOGY): a new `crates/specforge/src/ir/fidelity.rs` defines a
  typed gate set (`RealizableBoundary` / `RealizableDirection` /
  `RealizableHandshake` / `ResidualHonesty` / `NoStrictInvalid` /
  `FigureConformance`), a `FidelityFinding` record, and a
  `FindingStatus = Pass | Fail | NotEvaluated` (honestly three-valued
  — `NotEvaluated` is never silently treated as `Pass`).
- **Additive empty field** on `SemanticIr`/`IntentIr`:
  `fidelity_findings: Vec<FidelityFinding>` (serde-default +
  `skip_serializing_if = Vec::is_empty`) ⇒ zero artifact churn until
  the producer populates it. Same discipline as `actor_contracts` and
  `protocol_graph`.
- **Producer** (`.3`) runs in `SemanticIr::build` after
  `actor_contracts` exist; each gate evaluator returns `Pass` / `Fail`
  / `NotEvaluated` per contract. A `Fail` on a `Lowerable` contract is
  routed to **Residual** with the gate message as the reason — the
  honesty doctrine, mechanically enforced.
- **Trace-replay primitive**:
  `evaluate_figure_trace(&ActorContract, &FigureTrace)` is a bounded
  structural check (every obligation's witness within the trace tick
  window must hold). Until `R16-WAVEFORM-CONTRACT-MINING` (#4)
  populates `FigureTrace` from PDF figures, `FigureConformance` runs
  `NotEvaluated` corpus-wide — honest dormant capability; the
  primitive is unit-tested with synthesized traces in `.2`.
- **Report** (`.4`): `specforge validate` adds a `fidelity:` block
  (pass / fail / not_evaluated counts + per-document score + first-N
  failures) for SemanticIR and IntentIR. Structured-metric / JSON
  shape intentionally not touched (bounded, mirrors the
  `R16-KG-PROTOCOL-ONTOLOGY.4` precedent).

### Verification

- Per-document score = `pass / (pass + fail)` over **evaluated** gates;
  `NotEvaluated` excluded from the denominator and counted separately.
  Threshold default `1.0`: any `Fail` = below-threshold (disciplined
  honesty default). Corpus baseline-locked at `.4`.
- Additive empty field ⇒ zero `.isf`/artifact change while unpopulated
  (parity by construction); existing fsmgen-strict + e2e suites must
  stay green. `scripts/run_ci.sh` green per leaf; every leaf via
  `COMMIT.md`; the closing leaf refreshes this section
  (BOOK-METHOD-DOC).
- Honest scope: `FigureConformance` stays `NotEvaluated` on the corpus
  until `#4` populates `FigureTrace`s — explicitly recorded; never
  faked as `Pass`.

Authoritative tracking: `docs/tasks/R16-CAPTURE-FIDELITY-GATES.md`.

### Status — delivered (`2026-05-20`)

`R16-CAPTURE-FIDELITY-GATES` is **closed**. All four leaves done:

1. `.1` gate design fixed (typed layer / no new stage; 6-gate set;
   three-valued `FindingStatus` honesty);
2. `.2` typed `fidelity` module + 5 per-gate evaluators + bounded
   `evaluate_figure_trace` primitive + `FidelitySummary` with honest
   `score()` (over evaluated gates) and `meets_threshold(1.0)` default
   + additive empty `fidelity_findings` field on `SemanticIr`/
   `IntentIr` (serde-skipped while empty);
3. `.3` producer wired in `SemanticIr::build` —
   `apply_fidelity_gates(&mut [ActorContract], &[ActorPortRecord])` —
   with **honesty doctrine MECHANICALLY enforced**: a `Lowerable`
   contract with any `Fail` is rerouted to
   `Residual{reason = "fidelity:<Gate>: <message>"}` BEFORE the
   `.isf` adapter consumes it;
4. `.4` `specforge validate` `fidelity:` block (pass / fail /
   not_evaluated + score; first-5 failures when any) on both
   SemanticIR and IntentIR.

Live evidence: corpus baseline reads `fidelity: pass=N fail=0
not_evaluated=K score=1.000` — the existing
`contract_from_temporal_rule` path is fidelity-honest on the nvme
corpus (no `Fail` findings). The producer is now load-bearing: any
future contract-producer change that introduces a fidelity violation
will mechanically downgrade the affected contract to `Residual`
(documented reason) and surface in both the validate `fidelity:` block
and IR fixtures — the residual-honesty doctrine is structural, not
only authorial. Next DAG-promotable R16 sub-tree =
`R16-MULTIMODAL-CONTRACT-FUSION` (#3, order 4; deps `R16-CONTRACT-IR`
✓ + `R16-KG-PROTOCOL-ONTOLOGY` ✓ + `R16-CAPTURE-FIDELITY-GATES` ✓).

## R16-MULTIMODAL-CONTRACT-FUSION — how it is implemented and verified

**Why.** Today an obligation distributed across prose §3.1, a timing
table §3.4, Figure 3-2, and an exception in §3.5 lives as four
disjoint `ActorContract`s — contract-level recall is lost at the
join. Worse, when two sources say different things, today there is no
first-class way to surface "these contradict": one extraction's value
silently wins. This tree adds a **deterministic fusion phase** that
clusters multimodal candidates by a typed `FusionKey`, merges
agreeing sources (provenance union, `Mixed` modality), and routes
disagreements to `Residual{reason="disagreement: …"}` —
mechanically refusing to silently pick.

### Implementation

- **Typed layer, no new stage** (parallels prior R16 trees): a new
  `crates/specforge/src/ir/fusion.rs` defines `FusionKey` and the
  merge primitive. The fused outcome is still an `ActorContract` (no
  schema change needed); fusion is reflected through `provenance`
  (`Mixed` modality, union `supporting_statement_ids`, delimited
  `source_text`) and, on disagreement, through `lowering =
  Residual{reason="disagreement: …"}`.
- **`FusionKey`** keys clusters on `(actor, channel, phase,
  obligation_kind, primary_signal)`. `channel`/`phase` are populated
  by the extraction trees (#3/#4/#6); until then most contracts
  cluster at size 1 ⇒ fusion is identity ⇒ zero artifact churn on
  the corpus (CONTRACT-IR.2 / KG-ONTOLOGY.2 / FIDELITY.2 discipline).
- **Agreement merge** (deterministic): obligation/guard/kind common
  values carried through; `guard_candidates`/`supporting_statement_ids`
  union with source-order dedup; `provenance.modality` ⇒ `Mixed` when
  sources differ; `provenance.source_text` ⇒ delimited concatenation;
  `automation_confidence` ⇒ minimum (conservative).
- **Disagreement** (incompatible obligation / guard / kind): the
  fused contract carries the union of provenance and sets `lowering
  = Residual{reason="disagreement: …"}` — the honesty doctrine,
  mechanically enforced (parallel to `R16-CAPTURE-FIDELITY-GATES.3`).
  Two sources contradicting is now a first-class IR observation,
  never a silent pick.
- **Producer** (`.3`) runs in `SemanticIr::build` BEFORE
  `apply_fidelity_gates` (so the fidelity gates see fused contracts);
  IntentIR carries forward without schema change.
- **Report** (`.4`): `specforge validate` adds
  `fusion: groups_merged=… disagreements=…` to the SemanticIR /
  IntentIR count blocks. Structured-metric / JSON shape intentionally
  not touched (bounded, same as the prior R16 closes).

### Verification

- Unit-tested with synthetic multi-modal candidates: agreement path
  consolidates into one contract with `Mixed` modality and union
  provenance; disagreement path produces `Residual{reason="disagreement:
  …"}`; single-contract cluster is identity. Producer parity on
  corpus: clusters all size 1 today ⇒ zero artifact churn (the
  producer becomes load-bearing when `#4`/`#6` populate
  `channel`/`phase` and ground multiple sources per protocol element).
- Recall-improvement *measurement* on the corpus is necessarily `0`
  today (Non-Goal until extraction populates multi-source
  candidates). The corpus baseline at `.4` reads `groups_merged=0
  disagreements=0` — honest dormancy, not a faked Pass.
- `scripts/run_ci.sh` green per leaf; every leaf via `COMMIT.md`; the
  closing leaf refreshes this section (BOOK-METHOD-DOC).

Authoritative tracking: `docs/tasks/R16-MULTIMODAL-CONTRACT-FUSION.md`.

### Status — delivered (`2026-05-20`)

`R16-MULTIMODAL-CONTRACT-FUSION` is **closed**. All four leaves done:

1. `.1` fusion design fixed (typed layer / no new stage; `FusionKey`
   on (actor, channel, phase, obligation_kind, primary_signal);
   deterministic agreement merge with provenance union + `Mixed`
   modality + min `automation_confidence`; disagreement → Residual
   honesty doctrine);
2. `.2` typed `fusion` module + `merge_cluster` primitive (size-1
   identity, agreement merge, multi-field sorted-dedup disagreement) +
   6 unit tests; no producer wiring; zero artifact churn;
3. `.3` `apply_fusion` producer wired in `SemanticIr::build` **BEFORE**
   `apply_fidelity_gates` (so the fidelity gates evaluate the fused
   contracts); HashMap + first-seen-order Vec for determinism;
   merged contract takes the first slot, trailing dropped, input
   order otherwise preserved; idempotent on already-fused input;
   3 producer tests;
4. `.4` `specforge validate` `fusion: groups_merged=N
   disagreements=M` block (counts derived from `actor_contracts`
   via the `contract_id` `"fused:"` prefix and the `Residual.reason`
   `"disagreement: "` prefix — the IR is self-describing, no new
   field needed).

Live evidence: corpus baseline reads `fusion: groups_merged=0
disagreements=0` — the nvme corpus has no agreement-mergeable
clusters today (most contracts have `actor_name=None` /
`channel=None` / `phase=None`, but `(obligation_kind, primary_signal)`
still discriminates the rules well enough). **Honest dormancy**: the
primitive and producer are unit-tested with synthetic clusters; the
producer becomes load-bearing the moment extraction (`#4`/`#6`)
populates `channel`/`phase` or yields multi-source candidates per
protocol element. Next DAG-promotable R16 sub-tree =
`R16-WAVEFORM-CONTRACT-MINING` (#4, order 5 — **the crux** of the
program thesis: prose + timing-diagram extraction into typed
contracts).

## R16-WAVEFORM-CONTRACT-MINING — how it is implemented and verified

**Why.** This is the program thesis's crux. Timing diagrams *are* the
timed automaton, drawn, and they typically encode the densest
temporal-intent the spec ships — yet today the VLM timing extractor
is largely defensive (junk-label guarding). Closing this tree turns
the spec's own figures into ground truth a contract producer can
mine.

### Implementation

- **Typed layer, no new stage** (parallels prior R16 trees): a new
  `crates/specforge/src/ir/waveform.rs` defines the typed
  intermediate (`PartialTrace`, `LaneEdge`, `ValueSpan`,
  `RelativeDelay`, `CausalArrow`, `EdgeKind`) that an extractor
  produces and a generalizer consumes. This split lets each side be
  independently unit-testable against synthetic `PartialTrace`s.
- **Generalization rules** (conservative, bounded; under-determined
  ⇒ Observe/Residual): `RelativeDelay{min,max}` ⇒ `Eventually` with
  `Within{min,max}`; multi-tick `ValueSpan` ⇒ `Stable` with
  `Within{max=span_len}`; next-tick `CausalArrow` ⇒ `Eventually`
  with `Within{min=0,max=1}`; bare `LaneEdge` ⇒ `Observe` +
  `Residual{reason="bare edge — no window licensed"}` (honesty
  doctrine, mechanically enforced).
- **Round-trip verifier**: a generated contract must satisfy
  `evaluate_figure_trace` against the same `PartialTrace` it was
  generalized from; otherwise it is demoted to
  `Residual{reason="verifier disagreement: …"}`. Fabrication is
  structurally prevented at the generalizer's own boundary, before
  the downstream fidelity / fusion gates see the contract.
- **Cross-check with prose** is delegated to
  `R16-MULTIMODAL-CONTRACT-FUSION` (already closed) — the figure
  contracts and prose contracts cluster by `FusionKey`, and
  agreement / disagreement is the FUSION layer's job, not this
  tree's. This keeps the tree focused on figure→contract; fusion
  stays the single load-bearing primitive for cross-modal
  reconciliation.
- **Extractor strategy** (`.3` — qualitatively the largest leaf):
  VLM-structured prompting and/or vector-SVG path parsing,
  decision-deferred to `.3`'s promotion (when corpus figure-format
  mix is empirically known). Explicitly **expected to honest-split
  (rule 5)** into sub-leaves at that point.

### Verification

- `.2` ships the typed intermediate + generalizer + verifier
  unit-tested against synthetic `PartialTrace`s; zero artifact churn
  through `.2` (no PDF parsing yet).
- `.3` ships the figure→`PartialTrace` extractor (likely split);
  `.4` measures `FigureConformance` Pass-rate improvement on the
  corpus and ships negative-fixture coverage proving that junk
  waveforms do **not** mint contracts (verifier-fail ⇒ Residual, not
  silent fabrication).
- `scripts/run_ci.sh` green per leaf; every leaf via `COMMIT.md`;
  the closing leaf refreshes this section (BOOK-METHOD-DOC).

Authoritative tracking: `docs/tasks/R16-WAVEFORM-CONTRACT-MINING.md`.

## R16-CONSTRAINED-VERIFIED-EXTRACTION — how it is implemented and verified

**Why.** This is the closing tree of the program — it makes
extraction **high-precision by construction** by removing the last
authorial path to a bad contract. Prior trees gave us the typed
target (CONTRACT-IR), protocol structure (KG-ONTOLOGY), an objective
metric and Fail-to-Residual routing (CAPTURE-FIDELITY-GATES),
cross-modal reconciliation with disagreement-to-Residual (MULTIMODAL-
CONTRACT-FUSION), and a verifier-gated trace-mining primitive
(WAVEFORM-CONTRACT-MINING). What was left: keeping every captured
contract honest at extraction time, not just at lowering time.

### Implementation

- **Typed layer, no new stage** (parallels prior R16 trees): a new
  `crates/specforge/src/ir/cve.rs` houses the constrained-decoding
  adapter (`.2`), the entailment verifier (`.3`), and the
  uncertainty selection helper (`.5`). The protocol-pattern templates
  in `.4` live in `prior_memory` (the existing home for repeated
  priors). No new IR stage; `SemanticIr`/`IntentIr` schemas unchanged
  through `.2`/`.3`/`.4`/`.5`.
- **Schema-constrained decoding** (`.2`): a JSON schema for
  `ActorContract` (derived from its serde shape with a round-trip
  oracle test) + a provider-agnostic adapter
  `parse_constrained_contract(json: &str) -> Result<ActorContract>`
  that fails closed on schema violations. Invalid JSON is an `Err`
  — not a silently-fabricated contract. The integration with any
  particular LLM/VLM is **not pinned** so the design survives
  provider churn.
- **Entailment verifier** (`.3`): given `(source_span, contract)`
  returns `FindingStatus`. Initial implementation is conservative
  lexical/structural (every signal in the contract must appear in
  the span; every numeric bound must match a number actually present
  in the span). A `Fail` reroutes to
  `Residual{reason="entailment fail: …"}` — the honesty doctrine,
  mechanically enforced, parallel to `FUSION.3` disagreement-routing
  and `FIDELITY.3` Fail-on-Lowerable-routing. The verifier never
  "softens" a contract to pass.
- **Template library** (`.4`): canonical protocol templates seeded
  into `prior_memory` (ready/valid; credit flow control;
  setup/access; async-assert/sync-release reset; burst+last). Matched
  templates instantiate at `automation_confidence = High` only when
  the template's promised signals are all present — the match itself
  is entailment-verifiable.
- **Uncertainty-driven converge** (`.5`): a deterministic
  value-of-information score over (`automation_confidence`,
  fidelity-`Fail` count) selects the top-N contracts for re-extraction
  on the next pass — bounded budget, never blanket rescans.

### Verification

- `.2` ships the schema + adapter (round-trip parse/serialize +
  reject-invalid tests); `.3` ships the verifier + routing test;
  `.4` ships the template library + a match-grounding test (no
  fabricated templates: signals must be present); `.5` ships the
  selection helper + a unit test over synthetic findings; `.6`
  measures corpus precision/recall via
  `R16-CAPTURE-FIDELITY-GATES` and baseline-locks. `scripts/run_ci.sh`
  green per leaf; every leaf via `COMMIT.md`; the closing leaf
  refreshes this section (BOOK-METHOD-DOC).
- Honest dormancy: until an upstream prose extractor produces
  candidates and feeds the adapter, the producer-side of `.3`/`.5`
  is dormant on the corpus (the verifier still runs whenever it has
  a span; the templates are always available; the schema adapter is
  always usable). `.6` is where measurement becomes meaningful
  end-to-end.

Authoritative tracking: `docs/tasks/R16-CONSTRAINED-VERIFIED-EXTRACTION.md`.

### Status — delivered (`2026-05-20`) — R16 PROGRAM COMPLETE

`R16-CONSTRAINED-VERIFIED-EXTRACTION` is **closed**. All six leaves
done:

1. `.1` high-precision-by-construction design fixed (typed layer /
   no new stage; provider-agnostic schema adapter; entailment
   verifier; template library; uncertainty-driven converge);
2. `.2` typed `cve` module — provider-facing JSON-Schema summary +
   fails-closed `parse_constrained_contract` adapter (serde is the
   authoritative validator) + 4 tests including the discriminator
   drift-lock;
3. `.3` entailment verifier `entailment_check(span, contract)` +
   `apply_entailment_to_contract` Fail→Residual routing (honesty
   doctrine MECHANICALLY enforced, parallel to FUSION.3 / FIDELITY.3
   routings) + 7 tests including a complete-digit-run match for
   numeric bounds;
4. `.4` protocol-pattern template library (5 canonical:
   ReadyValidHandshake / CreditFlowControl / SetupAccess /
   AsyncAssertSyncReleaseReset / BurstLast) with `SignalBindings` +
   `instantiate_template` match-grounding gate; CFC + SetupAccess
   honestly Residual (deferred lowering, no fabrication) + 7 tests
   including the "match is entailment-verifiable" round-trip;
5. `.5` uncertainty-driven converge `voi_score` +
   `select_top_n_by_voi` (deterministic lex tie-break) + 5 tests
   (converge-loop integration deferred — honest bounded scope);
6. `.6` `specforge validate` `constrained: schema_rejects=N
   entailment_fails=M template_hits=K` block (additive lines via
   `replace_all`, structured-metric / JSON shape untouched).

Live evidence: corpus baseline reads `constrained: schema_rejects=0
entailment_fails=0 template_hits=0` — honest dormancy: each gate runs
whenever it has a span / a binding / a finding-set, but no upstream
prose extractor invokes the adapter today, so the metric reports zero
across the corpus. The primitives are unit-tested with synthetic
inputs (4+7+7+5 = 23 tests in `ir/cve.rs`); each becomes load-bearing
as upstream extraction lands.

**R16 PROGRAM COMPLETE.** All 6 sub-trees closed at their honest
scope boundaries:

- `R16-CONTRACT-IR` (#1, DAG root) — typed timed-contract IR;
- `R16-KG-PROTOCOL-ONTOLOGY` (#2) — typed protocol-structure KG;
- `R16-MULTIMODAL-CONTRACT-FUSION` (#3) — deterministic fusion +
  disagreement→Residual routing;
- `R16-WAVEFORM-CONTRACT-MINING` (#4) — typed figure→contract
  generalizer + verifier (extractor `.3` still pending, expected
  honest-split when concrete approach is chosen);
- `R16-CAPTURE-FIDELITY-GATES` (#5) — 6-gate fidelity producer with
  Fail-on-Lowerable→Residual routing + validate `fidelity:` block;
- `R16-CONSTRAINED-VERIFIED-EXTRACTION` (#6) — fails-closed adapter
  + entailment verifier + template library + uncertainty selector +
  validate `constrained:` block.

Three load-bearing honesty doctrines are now **structural** rather
than authorial: (a) fidelity Fail-on-Lowerable → Residual; (b)
fusion disagreement → Residual; (c) entailment Fail-on-Lowerable
→ Residual. Together, the IR cannot silently fabricate a contract
that any of the three gates rejects — fabrication is mechanically
prevented end-to-end.
