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
