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
