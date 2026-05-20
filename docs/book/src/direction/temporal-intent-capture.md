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

This section explains the typed shape that every temporal-intent
record now lands in. Reading it end-to-end should leave you with
a working mental model of what an `ActorContract` is, what it
can represent, what happens when one can't be cleanly expressed,
and where the line is between "the IR captured the intent" and
"the IR honestly recorded that it couldn't."

### The problem this fixes

Picture a single, perfectly-bound obligation from a protocol
spec:

> *"`VALID` holds until `READY` is high; the payload is stable
> across exactly that interval; the transfer is the first cycle
> they overlap."*

That is **one** thing. It's a timed contract with a clear
subject (the channel), a clear shape (a ready/valid handshake),
and clear evidence (the prose says all three pieces).

Before this tree, SpecForge stored the temporal model as a *bag
of predicates*: a `TemporalRuleRecord` was "some antecedents →
some consequents + maybe one cycle window." A bound obligation
like the one above got shredded into three or four disjoint
records, and the binding *between* them — the thing that made
them one contract — was lost the moment the records were saved.

That loss looked like an extraction failure, but it wasn't —
the extractor saw the binding fine. It was a **representation
failure**: the IR had no shape for "one contract with three
bound pieces", so the binding had nowhere to land.

`R16-CONTRACT-IR` adds the missing shape.

### The mental model

> **An `ActorContract` is one timed promise about how an actor
> behaves at its boundary. Every contract carries who is
> promising (an Assume vs Guarantee role), what they're
> promising (a small, closed set of obligation shapes), what
> evidence licenses the promise, and what happens when the
> promise can't be cleanly lowered (an explicit residual with a
> reason — never a silent loss).**

Everything below is the typed surface of that sentence: the
small closed operator algebra (`EventExpr` / `Window` /
`Obligation` / `Condition`), the wrapping
`ActorContract { … }`, where it lives in the pipeline, and how
the lowering to `.isf` consumes it.

### Where `ActorContract` lives

It's a typed layer, not a new pipeline stage. A new module
`crates/specforge/src/ir/contract.rs` defines the records;
`SemanticIR` and `IntentIR` carry an additive field
`actor_contracts: Vec<ActorContract>` (named that way to avoid
colliding with the pre-existing
`SemanticIR.contracts: Vec<ContractRecord>` for
protocol-contracts — different surface, same word). The field
is serde-default and skipped-while-empty, so:

- older IR artifacts load unchanged;
- the typed surface ships before any producer populates it ⇒
  artifacts don't change shape until a real change happens;
- no new `IrStage` / CLI / validate target appears — `IntentIR`
  remains the canonical product boundary.

This matches how `temporal_rules` and the actor-relative graph
already live as typed fields on existing stages. You don't
learn a new pipeline; you learn one more field shape.

### What an `ActorContract` carries

```rust
pub struct ActorContract {
    pub contract_id: String,
    pub source_rule_id: Option<String>,    // back-reference to the originating TemporalRuleRecord
    pub actor_name: Option<String>,        // who is making the promise
    pub kind: ContractKind,                // Assume | Guarantee
    pub guard: Option<Condition>,          // a bounded "when …" clause
    pub guard_candidates: Vec<Condition>,  // all the candidate guards parity selects from
    pub obligation: Obligation,            // the actual promise (see below)
    pub clock_signal: Option<String>,
    pub edge: ClockEdge,                   // rising / falling
    pub channel: Option<String>,           // protocol channel id (populated by extraction)
    pub phase: Option<String>,             // protocol phase id (populated by extraction)
    pub provenance: ContractProvenance,    // which source text licensed this
    pub lowering: LoweringDisposition,     // Lowerable | Residual{reason}
    pub automation_confidence: AutomationConfidence,
}
```

Read it left-to-right: *who* is promising, *under what guard*,
*what specifically*, *on what clock*, *for which channel/phase
of the protocol*, *backed by what evidence*, *and how cleanly
it lowers*.

`channel` and `phase` are typed but optional — extraction trees
(`R16-KG-PROTOCOL-ONTOLOGY`, then the prose/figure extractors)
populate them; until they do, the contract still lands cleanly,
just without protocol-graph anchoring.

### The closed operator algebra (the obligation shapes)

The set of shapes a contract can express is **closed** —
deliberately finite — so it stays realizability-checkable, the
lowering stays mechanical, and the boundary between "the IR
captured it" and "the IR honestly couldn't" is structural, not
authorial.

`Obligation` variants:

- **`Eventually { target, window }`** — `target` must happen
  within `window` cycles (`Within { min, max }`, with `max ≥ 1`
  per FSMGen-strict; `0` is not a window, it's a `SameCycle`).
- **`Stable { signal, during }`** — `signal` holds its value
  across `during`.
- **`Drive { signal, value }`** — the actor drives `signal` to
  `value` (the everyday case for a `Guarantee` contract).
- **`HandshakeBarrier { valid, ready }`** — the canonical
  ready/valid transfer. Lowers to FSMGen's
  `(stage p (ready r) (valid v))` form (more on this below).
- **`Persist { hold, until }`** — `hold` persists until `until`
  is observed.
- **`Sequence { steps }`** — an ordered series of
  boundary events with per-step windows.
- **`Mutex { a, b }`** — `a` and `b` cannot be simultaneously
  asserted.
- **`OrderedBefore { earlier_phase, later_phase }`** — a phase
  ordering (KG-level, not a tick-level event).
- **`Observe { signal }`** — *"this signal participated, but the
  source gave us no value and no window."* This is the **honest
  weak fact**: it's captured in the typed KG so we don't lose
  the observation, but it lowers as `Residual{reason}` because
  there's nothing concrete to lower. **The IR never invents a
  value or window from an `Observe`.**

`EventExpr` is similarly closed: `Edge { signal, dir }`,
`Level { signal, value }`, `HandshakeFire { valid, ready }`,
`PhaseBoundary { phase, at }`, `Start`. `Window` has three
shapes: `Within { min, max ≥ 1 }`, `Between { from, to }`,
`SameCycle`. `Condition` (used for guards) is a single bounded
shape: `Eq { signal, value }`.

That's the whole algebra. If a piece of prose says something
the closed algebra can't express, the contract gets created
anyway — with `lowering = Residual { reason: "…" }` and the
intent recorded in `provenance`. **The typed KG holds the
observation; the lowering pathway honestly admits it doesn't
fit.**

### What you see in the report today

Run `specforge validate <intent.json>` against the current
corpus and you'll see, in both the SemanticIR and IntentIR
count blocks:

```
  temporal_rules: N
  actor_contracts: N
```

The two numbers match — every `TemporalRuleRecord` projects to
exactly one `ActorContract`. That projection is **lossless by
construction** (the `.3` parity gate, below).

### How the lowering stays mechanical

`SemanticIr::build` calls `contract_from_temporal_rule` on
every record and stores the result in `actor_contracts`. From
that point on, the `.isf` adapter reads `actor_contracts` —
**not** the raw `temporal_rules`. The mapping is:

- a contract whose `lowering` is `Lowerable` becomes the
  corresponding `.isf` form (an `(eventually …)`, a `(stable …)`,
  a `(rule …)` drive, or a `(stage p (ready r) (valid v))` for
  `HandshakeBarrier`);
- a contract whose `lowering` is `Residual { reason }` is
  emitted as a typed residual decision in the adapter report —
  with the reason text — and is **never** silently dropped.

`temporal_rules` is **kept** on the IR (it's load-bearing for
validation, prior-memory, and as a back-compat fallback for
pre-ContractIR artifacts), but the adapter has stopped reading
it.

### The four user-facing guarantees

The design buys you four properties you can rely on, framed
as benefits rather than restrictions:

1. **You can hold one contract in one place.** Bound
   obligations stay bound — no more re-stitching three
   predicates by hand to recover a handshake.
2. **You always know what got lowered and what didn't.**
   `lowering: Lowerable` ⇒ the `.isf` form is present;
   `lowering: Residual { reason }` ⇒ the reason text tells you
   exactly why the IR couldn't lower it. Reading the adapter
   report tells you the whole truth — no hidden drops.
3. **You can trust the lowering to be reproducible.** The
   re-point (see `.3` below) was gated by a parity test that
   demanded the post-ContractIR `.isf` be **semantically
   identical** to the pre-ContractIR `.isf` on the corpus
   before the change shipped. The adapter's behaviour on every
   contract that was already lowering cleanly didn't change at
   all.
4. **The honesty doctrine is structural.** `Observe` and
   under-determined windows lower as `Residual` by *type*, not
   by author convention. There's no "well, the validator
   inferred a 2-cycle window because it seemed reasonable"
   path. If the typed shape doesn't license it, the IR doesn't
   claim it.

### Status — delivered (`R16-CONTRACT-IR` tree closed)

The work landed across four leaves, each gated by the standard
CI bar (`scripts/run_ci.sh` green; per-leaf `COMMIT.md`
discipline):

- `.1` — the design above, recorded against the *verified*
  current types (every `TemporalRuleRecord` /
  `TemporalPredicateRecord` variant enumerated to its explicit
  ContractIR target; no assumed shapes).
- `.2` — the typed `ir/contract.rs` module + the
  `contract_from_temporal_rule` projection + unit tests.
  Additive only: the field shipped, the producer didn't.
- `.3` — the `.isf` adapter re-pointed at `actor_contracts`.
  **Parity proven three ways**:
  - a by-construction pointwise oracle test
    (`classify_actor_contract` reproduces
    `classify_temporal_rule` exactly);
  - the live `nvme` corpus emitted byte-equivalent
    `.isf` against the pre-ContractIR baseline;
  - the existing real-binary fsmgen-strict suite
    (`bounded_contract_passes_fsmgen_strict_validation`,
    `temporal_rule_isf_passes_fsmgen_strict_validation`,
    `isf_temporal_rules_reach_isf_end_to_end`) stayed green.
  A back-compat fallback projects from `temporal_rules` for
  pre-ContractIR artifacts so older runs still load.
- `.4` — enabled `HandshakeBarrier → (stage p (ready r) (valid
  v))`. This **subsumes and delivers
  `ISF-HANDSHAKE-STAGE-LOWERING`** — the separate proposal
  closes folded into here.

Two honest, recorded constraints on `.4` you should know
about:

- FSMGen's `ready_valid_barrier` form requires the stage's
  `ready` operand to be an actor **input** (verified against
  the pinned `9bfb9a20` binary). SpecForge emits `(stage …)`
  **only** when that input-direction property holds; otherwise
  it preserves an explicit residual. It never fabricates a
  strict-invalid stage.
- The `(stage …)` capability is **verified-but-dormant** on
  today's corpus: no current `temporal_rule` carries a
  `handshake_complete` predicate, so zero stages are actually
  emitted corpus-wide and your emitted `.isf` is unchanged.
  The path is unit-tested and real-binary-fsmgen-strict-
  verified; it activates the moment the extraction trees
  (`R16-WAVEFORM-CONTRACT-MINING` /
  `R16-CONSTRAINED-VERIFIED-EXTRACTION`) start grounding
  handshake completions. This is the program's thesis at
  work: typed target + honest mechanical lowering are ready;
  capture fidelity is the remaining work.

*Authoritative tracking:* `docs/tasks/R16-CONTRACT-IR.md`
(the "Design (`.1` output)" section is the full specification;
the Decisions and Verification Log record every honest catch).

## R16-KG-PROTOCOL-ONTOLOGY — how it is implemented and verified

This section explains the typed shape that gives the knowledge
graph its **protocol structure** — the channels, phases,
transactions, and handshake pairs that organise every protocol
spec but were missing from the KG before this tree. Reading it
end-to-end should leave you with a working mental model of what
each record represents, how it gets populated (and what doesn't
populate it), and what you can rely on the KG to tell you once
extraction grounds the structure.

### The problem this fixes

Pick any digital-protocol PDF. The spec is almost certainly
organised around three structural ideas:

- **Channels** — named groups of related signals that move
  together as one logical "lane" of the protocol. AXI has
  `AW` / `W` / `B` / `AR` / `R`. APB has the transfer
  bundle (`PSEL`/`PENABLE`/`PADDR`/…). TileLink has `A` /
  `B` / `C` / `D` / `E`.
- **Phases** — named stages each channel passes through. APB
  has `setup` / `access`. AXI burst protocols have address /
  data / response.
- **Transactions** — bundles of channel activity that together
  realise one protocol operation (a complete AXI write =
  AW + W + B; a burst = an ordered sequence of beats ending in
  `LAST`).

Before this tree, the SpecForge KG had actors, signals, the
actor-relative direction graph, and `TickPhase` (clock-edge
granularity). None of those structural ideas were first-class.
A reader of `IntentIR` could see *which signals exist* but
not *which channel they belong to*; the cross-modal fusion
tree (`R16-MULTIMODAL-CONTRACT-FUSION`) had nothing stable to
cluster on; the spec's own organising vocabulary was lost in
translation.

`R16-KG-PROTOCOL-ONTOLOGY` adds the missing vocabulary.

### The mental model

> **A `ProtocolGraph` is the typed projection of the spec's own
> organising vocabulary — channels, phases, transactions, and
> handshake pairs — into the KG. The extraction trees populate
> it; the rest of the pipeline reads it. When it's empty, the
> pipeline behaves exactly as before; when it's populated, an
> `ActorContract` knows which channel and phase it belongs to,
> and downstream tooling can reason about protocol structure
> instead of bare signals.**

Everything below is the typed surface of that sentence: the
records, where they live, how they get populated (mechanically
from contracts vs from PDF extraction), and the
`TickPhase ≠ ProtocolPhase` distinction that keeps clock-level
and protocol-level granularity separate.

### Where `ProtocolGraph` lives

It's a typed layer, not a new pipeline stage — same decision
as ContractIR. A new module
`crates/specforge/src/ir/protocol_graph.rs` defines the
records; `SemanticIR` and `IntentIR` carry an additive
`protocol_graph: ProtocolGraph` field, serde-default and
skipped-while-empty. Today's IR artifacts grow no shape;
populated tomorrow's gain a structured vocabulary.

### What `ProtocolGraph` carries

```rust
pub struct ProtocolGraph {
    pub channels: Vec<Channel>,
    pub phases: Vec<ProtocolPhase>,
    pub transactions: Vec<Transaction>,
    pub handshakes: Vec<HandshakePair>,
}
```

Each record is closed and typed; edges are typed references
between them (not a raw edge soup), matching the way the
actor-relative graph already models actor↔signal relations.

- **`Channel { channel_id, name, actor, signal_names, role }`**
  — a named group of related signals from one actor's
  perspective. `role` is an optional `ChannelRole`
  (`Address` / `Data` / `Response` / `Request` / `Sideband` /
  `Mixed`) — populated by extraction when the spec's
  classification is clear.
- **`ProtocolPhase { phase_id, name, channel, order }`** — a
  named stage within a channel, with a stable `order` so phase
  sequencing is mechanical, not heuristic.
- **`Transaction { transaction_id, name, channels, phases,
  ordered_before }`** — a protocol operation that binds
  channels and phases together; `ordered_before` carries a
  typed ordering edge for use cases like "address phase
  precedes data phase".
- **`HandshakePair { pair_id, valid_signal, ready_signal,
  channel }`** — the canonical transfer point of a ready/valid
  protocol. The single place a downstream consumer needs to
  look to know *"this is where one beat of the channel
  transfers."*

### `TickPhase` is **not** `ProtocolPhase` — and you need both

The pre-existing `TickPhase` (clock-edge granularity:
`pre_tick` / `post_tick`) is **not** what a protocol spec
means by "phase". A `ProtocolPhase` is protocol-stage
granularity (`setup` / `access` / `address` / `data` /
`response` / …) and can span many ticks.

The two co-exist on the IR because they answer different
questions: `TickPhase` answers *"where in the clock cycle?"*;
`ProtocolPhase` answers *"where in the protocol's
choreography?"*. A `Stable { signal, during: Between { from:
PhaseBoundary { phase: …, at: enter }, to: PhaseBoundary {
phase: …, at: exit } } }` obligation expresses *"this signal
is stable across the data phase"* — and the typed reference
to a `ProtocolPhase` makes that claim mechanically auditable.

### How `ProtocolGraph` gets populated

Two paths feed the structure, with very different trust
levels:

- **Mechanical projection from already-recovered contracts.**
  `SemanticIr::build` runs `project_handshake_pairs(
  &actor_contracts)`, which derives one `HandshakePair` from
  every `HandshakeBarrier` obligation already on the
  contracts. This is **lossless restatement** of data that's
  already on the IR — the IR isn't claiming anything new; it's
  just exposing the handshake in the place a consumer of
  protocol structure looks. Today the corpus has no
  `HandshakeBarrier` contracts, so this projection produces
  zero `HandshakePair`s — but the pathway is live and
  unit-tested.
- **Extraction (Non-Goal here; the extraction trees' job).**
  Recovering `Channel`, `ProtocolPhase`, `Transaction` from
  the PDF requires reading prose / tables / figures — that's
  the work of `R16-WAVEFORM-CONTRACT-MINING` and
  `R16-CONSTRAINED-VERIFIED-EXTRACTION`. This tree
  deliberately does **not** invent extraction; it ships the
  vocabulary so when extraction lands, the data has a typed
  place to live. *"Tree ships the vocabulary, not the
  extractor"* is the standing scope rule.

The accessors round out the surface:

- `protocol_graph.channel(id)` / `protocol_graph.phase(id)` —
  typed lookups by id.
- `protocol_graph.dangling_contract_refs(&actor_contracts)`
  — surfaces any contract whose `channel` or `phase` field
  references a node that doesn't exist. The doctrine: a
  dangling reference is an inconsistency, not an "I'll figure
  it out later"; the helper makes it observable.

### What you see in the report today

Run `specforge validate <intent.json>` and the SemanticIR and
IntentIR count blocks include:

```
  protocol_graph: channels=0 phases=0 transactions=0 handshakes=0
```

All zeros, today. That's the honest baseline: no in-tree
producer populates the `ProtocolGraph`; extraction hasn't
landed yet. The line is there so the day the numbers move,
it's visible at a glance — *"oh, the extractor wired up; the
spec's vocabulary is showing up in the IR."*

### The user-facing guarantees

This design buys you three properties you can rely on, framed
as benefits:

1. **You can reason about protocol structure, not just
   signals.** Once extraction grounds the records, asking
   *"which channel does `AWVALID` belong to?"* / *"what phase
   ordering does this transaction enforce?"* becomes a typed
   lookup — not a regex search through the spec.
2. **Mechanical projection from contracts is lossless.** A
   `HandshakePair` derived from a `HandshakeBarrier` contract
   is the **same** information in a new typed shape — never an
   embellishment. If a consumer reads the `HandshakePair`s and
   gets surprised by a `valid`/`ready` they didn't expect,
   that surprise points back to a real `HandshakeBarrier`
   obligation; there's no fabrication path.
3. **Today's pipeline is byte-identical.** The field exists,
   it's empty, it serde-skips, your reports look the same as
   they did before this tree. Zero churn; load-bearing for
   tomorrow.

### Status — delivered (`2026-05-20`)

`R16-KG-PROTOCOL-ONTOLOGY` is **closed**. All four leaves
landed under the standard CI bar:

- `.1` — the ontology design above, fixing the typed records,
  the `TickPhase ≠ ProtocolPhase` distinction, and the
  Non-Goal that extraction is the extraction trees' job.
- `.2` — the typed `protocol_graph` module + serde +
  additive empty `ProtocolGraph` field on
  `SemanticIr`/`IntentIr`. Zero artifact churn (the field
  serde-skips while empty).
- `.3` — `project_handshake_pairs` wired in
  `SemanticIr::build`; `IntentIR` carries the structure
  forward; accessors + `dangling_contract_refs` shipped.
  Lossless restatement of `HandshakeBarrier` contract data,
  with the explicit guarantee that *zero corpus contracts ⇒
  zero `HandshakePair`s ⇒ zero downstream churn*.
- `.4` — the `validate` count surface
  (`protocol_graph: channels=… phases=… transactions=…
  handshakes=…`) on both SemanticIR and IntentIR. The
  kg-bench protocol-structure fixtures are **honestly
  deferred** to the extraction trees that actually recover
  protocol structure from PDFs — a fixture here would be
  hollow (this tree ships the vocabulary, not the extractor).

Live corpus evidence: the validate block reads
`protocol_graph: channels=0 phases=0 transactions=0
handshakes=0` — the dormant-but-ready signal the design
promised. The numbers move the moment the extraction trees
land.

*Authoritative tracking:*
`docs/tasks/R16-KG-PROTOCOL-ONTOLOGY.md`.

## R16-CAPTURE-FIDELITY-GATES — how it is implemented and verified

This section explains the typed surface that turns *"how well
did we capture intent?"* into a measurable, gating number — and
turns the residual-honesty doctrine *"unverifiable intent →
explicit residual, never fabricated"* from a rule SpecForge's
authors follow into a property the code itself enforces.
Reading it end-to-end should leave you with a working mental
model of each gate, the three-valued result you should expect
to see, what the corpus baseline tells you today, and the four
properties this design buys you.

### The problem this fixes

The program thesis says the hard problem is extracting accurate
temporal intent from prose and timing diagrams into a typed
KG; the rest of the pipeline is mechanical. But there's an
honest follow-up question: *"how would we know if we got
better at it?"* If we can't measure capture fidelity, we can't
improve it — and worse, we have no structural barrier against
silently lowering a contract that shouldn't have been licensed
in the first place.

Before this tree, fidelity was a judgment SpecForge's authors
made — a residual-honesty doctrine that read *"if you can't
ground it, residual it."* That's fine as a principle. As a
property the IR enforces, it was missing. A typo or a
well-meaning shortcut in the contract producer could ship a
`Lowerable` contract whose obligation referenced a signal that
isn't on the actor's boundary — and nothing in the pipeline
would catch it until someone read the `.isf` output and
noticed.

`R16-CAPTURE-FIDELITY-GATES` adds the missing measurement and
the missing structural barrier.

### The mental model

> **A `FidelityFinding` is a typed observation about one
> contract from one of six gates. Each gate returns `Pass`,
> `Fail`, or `NotEvaluated` — honestly three-valued. A `Fail`
> on a `Lowerable` contract is mechanically rerouted to
> `Residual{reason}` before the `.isf` adapter ever sees it —
> the doctrine becomes structural, not authorial. The corpus
> baseline (`fail=0 score=1.000`) tells you `SpecForge`'s
> existing contract producer is fidelity-honest today; the
> gates remain on guard against any future drift.**

Everything below is the typed surface: the six gates and what
each one checks, the three-valued status, the producer that
runs them, the routing rule, the `FidelitySummary` and how the
per-document score is computed, the validate `fidelity:` block
you see in the report, and the four user-facing properties
the design guarantees.

### Where `fidelity` lives

It's a typed layer, not a new stage — parallel to ContractIR
and KG-ONTOLOGY. A new module
`crates/specforge/src/ir/fidelity.rs` defines the gate set,
the typed records, and the per-gate evaluators;
`SemanticIR` and `IntentIR` carry an additive
`fidelity_findings: Vec<FidelityFinding>` field, serde-default
and skipped-while-empty. Today's IR artifacts don't grow;
populated tomorrow's gain a structured per-contract fidelity
record.

### The six gates

Each gate is a focused check that returns `Pass` / `Fail` /
`NotEvaluated` for one contract:

- **`RealizableBoundary`** — every signal the contract
  references (in its obligation, guard, guard_candidates, or
  clock) is declared on the actor's boundary. *"You can't
  promise about signals that don't exist."*
- **`RealizableDirection`** — the obligation's primary signal
  direction is consistent with the contract kind. A
  `Guarantee` is about an output (or `InOut`); an `Assume` is
  about an input (or `InOut`). When the direction is unknown
  to the actor, the gate honestly returns `NotEvaluated` —
  never silently `Pass`.
- **`RealizableHandshake`** — when the obligation is a
  `HandshakeBarrier`, `ready` must be an actor input and
  `valid` an actor output. Mirrors the FSMGen
  `ready_valid_barrier` strict requirement from CONTRACT-IR.4.
  Non-handshake obligations honestly return `NotEvaluated`.
- **`ResidualHonesty`** — a `Residual{reason}` must carry a
  non-empty reason; a `Lowerable` contract must not carry an
  `Observe` obligation (which has no representable `.isf`
  form). Catches the most common dishonesty: a residual
  pretending it doesn't owe an explanation.
- **`NoStrictInvalid`** — a `Lowerable` contract whose
  obligation shape has no FSMGen-strict-valid `.isf` form
  (currently `Observe` and `OrderedBefore`) fails. Catches
  contracts that would produce invalid syntax downstream.
- **`FigureConformance`** — when a figure-derived trace is
  attached to a contract (via `R16-WAVEFORM-CONTRACT-MINING`),
  the contract's obligation must be satisfied by that trace.
  Until `#4` populates `FigureTrace`s in the corpus, this gate
  honestly runs `NotEvaluated` — never faked as `Pass`. The
  primitive is unit-tested with synthesized traces.

### Three-valued status — and why `NotEvaluated` is a real outcome

```rust
pub enum FindingStatus { Pass, Fail, NotEvaluated }
```

The third value is load-bearing. `NotEvaluated` means *"this
gate honestly cannot reach a verdict on this contract right
now"* — usually because the inputs the gate needs aren't
available yet (e.g. no figure trace; unknown signal
direction). It's **not** the same as `Pass`, and it's **not**
treated as `Pass` anywhere in the pipeline. The per-document
score excludes `NotEvaluated` from the denominator and counts
it separately. The corpus baseline today carries plenty of
`NotEvaluated` (because no figure traces exist yet); none of
those silently roll up into a "passing" number.

### How the gates run — and what they do on a `Fail`

`SemanticIr::build` runs `apply_fidelity_gates(&mut
[ActorContract], &[ActorPortRecord])` after the contract
producer has populated `actor_contracts`. Each gate evaluates
every contract; the per-contract findings get appended to
`fidelity_findings`.

The load-bearing routing rule: **if a `Lowerable` contract
gets any `Fail` finding, the producer reroutes it to**

```rust
LoweringDisposition::Residual {
    reason: format!("fidelity:<Gate>: <message>"),
}
```

**before the `.isf` adapter ever sees it.** The contract still
exists in the IR (no information is lost), but it now honestly
records that it failed a fidelity gate, with the gate name and
the gate's message embedded in the reason text. Reading the
adapter's residual decisions tells you exactly which gate
objected and why.

`Pass` and `NotEvaluated` leave the contract unchanged.
Already-`Residual` contracts are not re-routed by this pass
(their existing reason is preserved); the gates still produce
their findings, but no second rewrite happens.

### `FidelitySummary` and the per-document score

```rust
pub struct FidelitySummary {
    pub pass: u32,
    pub fail: u32,
    pub not_evaluated: u32,
}

impl FidelitySummary {
    pub fn score(&self) -> Option<f64> {
        let evaluated = self.pass + self.fail;
        if evaluated == 0 { None } else {
            Some(self.pass as f64 / evaluated as f64)
        }
    }

    pub fn meets_threshold(&self, threshold: f64) -> bool {
        match self.score() {
            Some(s) => s >= threshold && self.fail == 0,
            None => false,
        }
    }
}
```

The score is `pass / (pass + fail)` over **evaluated** gates.
`NotEvaluated` is counted separately. `meets_threshold(t)`
requires `score ≥ t` **and** `fail == 0` — the default
threshold is `1.0`, which means any `Fail` ⇒ below-threshold.
That's the disciplined-honesty default: SpecForge doesn't ship
"79% fidelity" as a victory; you either captured cleanly or you
have a known residual.

### What you see in the report today

Run `specforge validate <intent.json>` and the SemanticIR and
IntentIR count blocks include:

```
  fidelity: pass=N fail=0 not_evaluated=K  score=1.000
```

`fail=0` is the live evidence that the existing
`contract_from_temporal_rule` producer is fidelity-honest on
the nvme corpus — every contract it currently creates passes
every gate it can be evaluated against, and the residual ones
have non-empty reasons. The `score=1.000` is correspondingly
clean. The `not_evaluated=K` count is non-zero because
`FigureConformance` runs `NotEvaluated` corpus-wide until
extraction lands.

If `fail` ever goes non-zero, the report also includes:

```
  fidelity_failures (first 5):
    [Gate] contract_id: message
    …
```

so you can see exactly which contracts failed which gates, in
the report itself.

### The four user-facing guarantees

This design buys you four properties you can rely on:

1. **You can see, in a number, how clean your IR's contract
   producer is.** `fail=0 score=1.000` is the corpus baseline
   today; any drift moves the numbers visibly.
2. **The residual-honesty doctrine is structural, not
   authorial.** A future producer change that ships a
   fidelity-violating contract gets mechanically demoted to
   `Residual` with a diagnostic reason; it doesn't sneak past
   review.
3. **`NotEvaluated` never lies as `Pass`.** When a gate
   honestly cannot reach a verdict, the report says so
   explicitly. You can tell *"the gate couldn't run"* from
   *"the gate ran and approved"*.
4. **Failures carry full provenance.** The diagnostic in
   `Residual.reason` names the gate (`fidelity:<Gate>:`) and
   the gate's message; a downstream reader can trace from the
   `.isf` residual decision back to which gate objected,
   without re-running anything.

### Status — delivered (`2026-05-20`)

`R16-CAPTURE-FIDELITY-GATES` is **closed**. All four leaves
landed under the standard CI bar:

- `.1` — the gate design above (typed layer; six gates;
  three-valued status; refuse-by-default routing).
- `.2` — the typed `fidelity` module + five per-gate
  evaluators + the bounded `evaluate_figure_trace` primitive +
  `FidelitySummary` with honest `score()` and
  `meets_threshold(1.0)` default + the additive empty
  `fidelity_findings` field on `SemanticIr`/`IntentIr`.
- `.3` — `apply_fidelity_gates` wired in `SemanticIr::build`,
  with the **honesty doctrine mechanically enforced**: a
  `Lowerable` contract with any `Fail` is rerouted to
  `Residual { reason: "fidelity:<Gate>: <message>" }` *before*
  the `.isf` adapter sees it.
- `.4` — the `validate fidelity:` block (pass / fail /
  not_evaluated + score; first-5 failures when any) on both
  SemanticIR and IntentIR. The corpus baseline reads
  `fail=0 score=1.000` — the contract producer is
  fidelity-honest on nvme today; the gates remain on guard
  against any future drift.

The producer is now load-bearing: it's the third structural
honesty doctrine (with FUSION.3 disagreement-routing and
CVE.3 entailment-Fail-routing) that together make
fabrication mechanically prevented end-to-end across the
pipeline.

*Authoritative tracking:*
`docs/tasks/R16-CAPTURE-FIDELITY-GATES.md`.

## R16-MULTIMODAL-CONTRACT-FUSION — how it is implemented and verified

This section explains the typed phase that takes contract
candidates extracted from *multiple* sources — prose, a timing
table, a figure, an exception clause — and turns them into one
authoritative `ActorContract` per protocol element. Reading it
end-to-end should leave you with a working mental model of
the cluster key, the deterministic merge, the
disagreement-routing rule, and what the corpus baseline tells
you today about why the producer is dormant-but-load-bearing.

### The problem this fixes

Picture a single AXI write-channel obligation a spec might
license in *four* places:

> *Prose §3.1: "AWVALID must be held high until the slave
> asserts AWREADY."*
>
> *Timing table §3.4: "AWVALID ≥ 1 cycle before AWREADY
> sample."*
>
> *Figure 3-2: an arrow from `AWVALID↑` to `AWREADY↑` two
> ticks later.*
>
> *Exception §3.5: "AWVALID may be deasserted before AWREADY
> if AWRESETN low."*

Each of those is one piece of evidence about the **same**
protocol obligation. Before this tree they landed as four
disjoint `ActorContract`s, each with its own provenance,
none of them aware of the others. Contract-level recall got
lost at the join, and two even nastier failure modes lurked:

- **One source silently overwrites another.** If the prose
  said "≥ 1 cycle" and the table said "≥ 2 cycles," whichever
  one the pipeline processed last won — with no visible
  signal that the other source had said something different.
- **A real contradiction looked like a clean win.** A
  contradiction between two sources is a *first-class
  observation* about the spec — but the IR had no shape for
  it, so the contradiction silently disappeared.

`R16-MULTIMODAL-CONTRACT-FUSION` adds the missing typed
phase: cluster the candidates, merge the ones that agree
(preserving every source's evidence), and route the ones
that disagree to an explicit `Residual{reason="disagreement:
…"}`.

### The mental model

> **A `FusionKey` says "these contract candidates are all
> talking about the same protocol element." `merge_cluster`
> takes a cluster of candidates with the same key and returns
> one `ActorContract`. When the candidates agree, the merged
> contract is `Lowerable` and carries provenance from all
> sources. When they disagree on `obligation` / `guard` /
> `kind`, the merged contract is `Residual{reason}` — the
> doctrine is "two sources contradicting each other is a
> recorded observation, never a silent pick."**

Everything below is the typed surface: the `FusionKey`
fields, the deterministic agreement merge, the
disagreement-routing rule, the `apply_fusion` producer (which
runs **before** the fidelity gate so the gates see the fused
contracts), and the validate `fusion:` block you see in the
report.

### Where `fusion` lives

It's a typed layer, not a new pipeline stage — parallel to
ContractIR, KG-ONTOLOGY, and FIDELITY-GATES. A new module
`crates/specforge/src/ir/fusion.rs` defines `FusionKey` and
the merge primitive. The fused outcome is still an
`ActorContract` (no schema change needed); fusion is
reflected through the existing `provenance` and `lowering`
fields. `SemanticIr`/`IntentIr` carry the fused contracts in
the same `actor_contracts` field — no new vector, no new
shape to learn.

### `FusionKey` — what makes two candidates "the same protocol element"

```rust
pub struct FusionKey {
    pub actor: Option<String>,
    pub channel: Option<String>,
    pub phase: Option<String>,
    pub obligation_kind: &'static str,
    pub primary_signal: Option<String>,
}
```

Two candidates fuse iff their `FusionKey`s are equal. The
five fields together capture *"about whom, in which channel,
at which phase, what shape of obligation, primarily about
which signal."* When two extractors look at the same protocol
element from different angles, they should produce candidates
with the same key — that's the cluster the merge operates on.

`channel` and `phase` are populated by the extraction trees
(KG-ONTOLOGY + the extraction siblings). Until they
populate, most contracts have `actor=None / channel=None /
phase=None`, leaving `(obligation_kind, primary_signal)` as
the discriminator. That's still enough to keep distinct
obligations distinct (a `Drive { signal: "Q", value: "1" }`
key never clusters with a `Stable { signal: "D", … }` key);
it just means most clusters end up size-1 today, and the
producer is correspondingly an identity on the corpus —
load-bearing for the day extraction lands the
`channel`/`phase` fields.

### Agreement merge — deterministic, provenance-preserving

When two or more candidates share a `FusionKey` and **agree**
on the load-bearing fields (`obligation`, `guard`, `kind`),
`merge_cluster` returns one `ActorContract` with:

- `obligation` / `guard` / `kind` / `clock_signal` /
  `channel` / `phase` carried through (they're equal across
  candidates by construction);
- `guard_candidates` — union of all sources, in first-seen
  order, deduped;
- `provenance.supporting_statement_ids` — union of all
  sources' supporting ids, in first-seen order, deduped;
- `provenance.modality` — `Mixed` when sources differ
  (prose + table + figure = `Mixed`); the common modality
  otherwise;
- `provenance.source_text` — delimited concatenation
  (`"prose §3.1 | table §3.4 | figure 3-2"`);
- `automation_confidence` — **minimum** across the cluster
  (conservative — a chain is as strong as its weakest
  source);
- `contract_id` — `"fused:<id1>+<id2>+…"` so you can trace
  back to the original candidates by inspection.

The merge is **deterministic**: the same cluster of
candidates always produces the same fused contract.
Reproducibility is preserved.

### Disagreement routing — the second structural honesty doctrine

When two or more candidates share a `FusionKey` but **disagree**
on `obligation` / `guard` / `kind`, the merge returns:

- `lowering = Residual { reason: "disagreement: <sorted+
  deduped list of disagreeing fields>" }`;
- everything else as in the agreement path — `provenance`
  still unioned (so both sources are preserved), `Mixed`
  modality, `automation_confidence` still the minimum.

That's the second of the three structural honesty doctrines
the R16 program adds. (The first is fidelity-Fail → Residual
from `R16-CAPTURE-FIDELITY-GATES.3`; the third is
entailment-Fail → Residual from
`R16-CONSTRAINED-VERIFIED-EXTRACTION.3`.) Together they
mean: **a contract that any structural check rejects is
mechanically routed to `Residual` with the reason in the
text — fabrication is impossible end-to-end.**

A reader of the `.isf` adapter's residual decisions can now
see exactly which contracts came from a disagreement and
exactly which fields disagreed. The contradiction is
preserved in the IR as a first-class observation.

### `apply_fusion` — and why ordering matters

`SemanticIr::build` runs `apply_fusion(&mut actor_contracts)`
**before** `apply_fidelity_gates`. The ordering is
load-bearing: the fidelity gates evaluate **fused**
contracts, not pre-fusion duplicates. A multi-source
contradiction is routed by fusion first; then the fidelity
gates evaluate the now-Residual fused contract; then the
.isf adapter consumes the result. Each step's invariants
are stable because the previous step already produced its
output.

Implementation notes:

- Clustering uses a `HashMap` keyed by `FusionKey` for the
  cluster lookup, plus a parallel `Vec<FusionKey>` recording
  first-seen order so the resulting `actor_contracts` vector
  is deterministic across runs.
- For each cluster of size > 1, `merge_cluster` produces the
  fused contract; the merged contract takes the slot of the
  cluster's first member; the trailing members are dropped
  from the vector.
- The producer is **idempotent**: re-running it on already-
  fused input is a no-op (every contract is now in its own
  size-1 cluster, and `merge_cluster` returns size-1 input
  unchanged).

### What you see in the report today

Run `specforge validate <intent.json>` and the SemanticIR
and IntentIR count blocks include:

```
  fusion: groups_merged=0 disagreements=0
```

Both zero, on the nvme corpus today. That's the honest
dormancy signal: clusters are all size 1 because the
extraction trees haven't populated `channel`/`phase` or
delivered multi-source candidates yet. The producer ran
and produced its identity output; no fabrication is
hiding behind the numbers.

The counts are derived from the IR itself — no new field
was needed: `groups_merged` is the count of contracts whose
`contract_id` starts with `"fused:"`; `disagreements` is the
count of `Residual` contracts whose `reason` starts with
`"disagreement: "`. The IR is self-describing.

### The four user-facing guarantees

This design buys you four properties you can rely on:

1. **You can hold one obligation in one place even when it
   came from many sources.** When extraction lands and yields
   multi-source candidates, agreement consolidates them into
   one `ActorContract` with the provenance from all sources —
   not four contracts you'd have to re-cluster by hand.
2. **No source silently overwrites another.** Disagreements
   are routed to `Residual{reason="disagreement: …"}` with
   the disagreeing fields enumerated; the contradiction is a
   first-class IR observation, not a hidden choice.
3. **Mergers are deterministic and reproducible.** The same
   cluster always produces the same fused contract, with
   stable ordering and stable provenance.
4. **Today's pipeline is byte-identical.** The producer runs
   on every build and produces an identity result on the
   corpus; reports look the same as before this tree shipped
   — zero churn, load-bearing for tomorrow.

### Status — delivered (`2026-05-20`)

`R16-MULTIMODAL-CONTRACT-FUSION` is **closed**. All four
leaves landed under the standard CI bar:

- `.1` — the fusion design above: typed layer / no new
  stage; `FusionKey` on the five fields; agreement merge
  rule; disagreement → Residual; producer ordering before
  fidelity.
- `.2` — the typed `fusion` module + `merge_cluster`
  primitive (size-1 identity, agreement merge, multi-field
  sorted-deduped disagreement) + 6 unit tests; no producer
  wiring; zero artifact churn.
- `.3` — `apply_fusion` producer wired in `SemanticIr::build`
  **before** `apply_fidelity_gates` so the fidelity gates
  evaluate fused contracts; deterministic cluster ordering;
  idempotent on already-fused input; 3 producer tests.
- `.4` — `validate fusion: groups_merged=N disagreements=M`
  block; counts derived from `actor_contracts` via the
  `contract_id` `"fused:"` prefix and the `Residual.reason`
  `"disagreement: "` prefix.

Live corpus evidence: `fusion: groups_merged=0
disagreements=0`. Honest dormancy. The producer is
load-bearing the moment extraction starts grounding
`channel`/`phase` or yielding multi-source candidates per
protocol element.

*Authoritative tracking:*
`docs/tasks/R16-MULTIMODAL-CONTRACT-FUSION.md`.

## R16-WAVEFORM-CONTRACT-MINING — how it is implemented and verified

This section explains the typed pipeline that turns timing
diagrams from a protocol PDF into honest, ground-truth
contracts. Reading it end-to-end should leave you with a
working mental model of the typed intermediate (`PartialTrace`),
the four generalization rules that turn it into contracts, the
round-trip verifier that prevents fabrication at the
generalizer's own boundary, the typed input contract
(`FigureRegion`) the upstream PDF pipeline targets, and what
the corpus baseline tells you today.

### The problem this fixes

The program thesis says capture fidelity is the hard problem,
and timing diagrams are *the* densest source of temporal
intent in any protocol spec — they're literally the timed
automaton, drawn. A well-mined timing diagram gives you
exactly the bound obligations
(*"VALID held high for ≥ 2 cycles; READY sampled on the
third tick; payload stable across the transfer"*) that prose
struggles to articulate cleanly.

Before this tree, SpecForge's VLM-based timing extraction was
**defensive** — its main job was to guard against junk labels
and reject things it didn't recognise, rather than turn
recognised structure into contracts. That worked as a safety
floor but missed the upside: when the figure *is* clean, the
contracts the figure licenses are right there.

`R16-WAVEFORM-CONTRACT-MINING` adds the missing pipeline:
typed `PartialTrace` ⇒ deterministic generalizer ⇒ round-trip
verifier ⇒ `ActorContract`s the rest of the program (fusion,
fidelity, .isf lowering) already knows how to consume.

### The mental model

> **A `PartialTrace` is whatever a figure-extractor recovered
> from one timing diagram: the lanes, the values they held,
> the annotated delays, the causal arrows. The generalizer
> reads it and produces conservative `ActorContract`s — never
> inventing windows beyond what the trace licenses. The
> round-trip verifier double-checks that each generated
> contract is satisfied by the trace it came from — and
> demotes it to `Residual{reason="verifier disagreement:
> …"}` if it isn't. Fabrication is structurally prevented
> at the boundary, not at lowering time.**

Everything below is the typed surface: the `PartialTrace`
records the figure-extractor produces, the four conservative
generalization rules, the round-trip verifier and what it
guarantees, the cross-check delegation to FUSION, the typed
`FigureRegion` input contract (`.3.1`), the adapter (`.3.2`),
and what the corpus baseline tells you today.

### Where `waveform` lives

It's a typed layer, not a new pipeline stage — parallel to
the rest of the R16 family. A new module
`crates/specforge/src/ir/waveform.rs` defines the typed
intermediate `PartialTrace` and the generalizer + verifier
that operate on it. The figure→`PartialTrace` adapter lives
in a sibling module `crates/specforge/src/ir/figure_region.rs`
(detailed under `.3.1` / `.3.2` below). Both are additive;
nothing on `SemanticIr` / `IntentIr` changes shape until an
upstream figure-extractor produces `PartialTrace`s.

### `PartialTrace` — what a figure-extractor produces

```rust
pub struct PartialTrace {
    pub figure_id: String,
    pub signals: Vec<String>,             // signals the trace covers
    pub edges: Vec<LaneEdge>,             // bare transitions (no window licensed alone)
    pub spans: Vec<ValueSpan>,            // contiguous-value runs
    pub delays: Vec<RelativeDelay>,       // annotated bounds between events
    pub causal: Vec<CausalArrow>,         // "A's edge causes B's edge"
    pub ticks: u32,                       // length of the trace window
    pub confidence: AutomationConfidence, // upstream's confidence in the recovery
}
```

The intermediate is **deliberately structured per evidence
kind**. An extractor that's good at recovering value spans
but bad at causal arrows produces good `spans` and few
`causal`s — and that observability shows up downstream. The
generalizer doesn't need to know how the extractor works;
both sides are unit-testable against synthetic
`PartialTrace`s.

### Generalization rules — conservative, bounded, never fabricating

`generalize_partial_trace(&PartialTrace) -> Vec<ActorContract>`
applies four rules, each well-grounded in what the trace
licenses:

- **`RelativeDelay { from, to, min, max }`** ⇒
  `Eventually { target: Edge { signal: to, dir: Rose }, window:
  Within { min, max } }` / `Lowerable`. The annotated bound
  is the window.
- **Multi-tick `ValueSpan { signal, from_tick, to_tick }`** ⇒
  `Stable { signal, during: Within { max: to_tick - from_tick } }`
  / `Lowerable`. The held value across multiple ticks is the
  Stable obligation; the span length is the window.
- **Next-tick `CausalArrow`** ⇒ `Eventually { target: Edge { …,
  dir: Rose }, window: Within { min: 0, max: 1 } }` /
  `Lowerable`. An arrow with `to_tick == from_tick + 1`
  licenses a one-cycle Eventually.
- **Bare `LaneEdge`** (no enclosing delay / span / causal) ⇒
  `Observe { signal }` + `Residual { reason: "bare edge — no
  window licensed" }`. **The IR captures the observation but
  refuses to invent a window the trace doesn't license.**

Confidence is **capped** per source: a contract mined from
a single figure never gets `automation_confidence = High` on
its own (`capped_confidence` clamps it to `Medium`).
Promotion to High is a cross-modal property — it requires
agreement with a prose- or table-derived contract through
`R16-MULTIMODAL-CONTRACT-FUSION`.

### Round-trip verifier — fabrication is structurally prevented

`verify_contract_against_trace(&ActorContract, &PartialTrace)
-> FindingStatus` is the safety property that makes the
above rules trustworthy. The verifier:

1. **lifts** the `PartialTrace` to a `FigureTrace` (the
   primitive `R16-CAPTURE-FIDELITY-GATES.2` already defined)
   by replaying its samples;
2. **calls** `evaluate_figure_trace(&contract, &lifted)`
   from FIDELITY.2 — the trace must satisfy the obligation
   the contract claims;
3. returns `Pass` / `Fail` / `NotEvaluated` honestly.

A `Fail` ⇒ the generated contract is demoted to
`Residual { reason: "verifier disagreement: …" }`. The
contract still exists in the IR (the observation isn't
lost), but the round-trip oracle has refused to license it
as `Lowerable`. This is the **third place** in the R16
program where fabrication is structurally prevented:

1. `FIDELITY.3` — Fail on a Lowerable contract ⇒ Residual.
2. `FUSION.3` — disagreement across sources ⇒ Residual.
3. `WAVEFORM.3`'s round-trip verifier — generalized
   contract not satisfied by its source trace ⇒ Residual.

Together with `CVE.3`'s entailment routing (the fourth
structural-honesty enforcement, planned for prose), the IR
cannot silently fabricate.

### Cross-check with prose — delegated to FUSION

This tree does **not** re-implement prose-vs-figure
cross-checking. Figure-derived contracts cluster by
`FusionKey` like everyone else; agreement and disagreement
are `R16-MULTIMODAL-CONTRACT-FUSION`'s job. That keeps the
WAVEFORM tree focused on figure→contract; FUSION stays the
single load-bearing primitive for cross-modal
reconciliation. (See the `MULTIMODAL-CONTRACT-FUSION`
section above for the merge / disagreement rules a
WAVEFORM-derived candidate flows through.)

### `.3.1` — the typed `FigureRegion` input contract

A corpus survey (`find … *.pdf *.svg` over the SpecForge
tree) returned nothing — the test corpus is **pre-processed**;
raw PDF / SVG bytes are out-of-tree. So `WAVEFORM.3`'s
"extractor" is in fact a **typed adapter** consuming the
upstream record an out-of-tree PDF pipeline produces. The
question for `.3.1` was: *what shape should that upstream
record take so the WAVEFORM tree can consume it cleanly?*

The closest existing upstream record is
`crates/specforge/src/ir/source.rs::VisualAsset`
(`asset_id` / `asset_kind` / `page_id` / `image_path` /
`caption_text` / `diagram_kind`). `VisualAsset` is good at
*"this is a figure"* but doesn't carry the lane / annotation
structure the WAVEFORM generalizer needs. `FigureRegion` is
the typed **extension** the upstream pipeline produces when
it classifies a `VisualAsset` as a timing diagram and
recovers structure:

```rust
pub struct FigureRegion {
    pub visual_asset_id: String,             // references existing VisualAsset
    pub bbox: Option<BoundingBox>,
    pub annotations: Vec<FigureAnnotation>,  // typed: Delay / Value / Label / Unknown
    pub waveform_lanes: Vec<FigureLane>,     // recovered lane samples
    pub tick_count: Option<u32>,
    pub raw_image_path: Option<PathBuf>,     // rarely needed downstream
    pub confidence: AutomationConfidence,
}

pub enum FigureAnnotation {
    Delay { from_signal, to_signal, min_cycles, max_cycles, text, bbox },
    Value { signal, value, from_tick, to_tick, text, bbox },
    Label { text, bbox },                    // informational only
    Unknown { text, bbox },                  // upstream couldn't classify
}
```

The contract is deliberately additive: `VisualAsset` doesn't
change; `FigureRegion` is the **new typed record** an upstream
extractor populates when it has structure to record. When
upstream produces zero `FigureRegion`s (today's corpus),
the adapter is a no-op and the rest of the pipeline runs
exactly as it did before this tree.

### `.3.2` — `figure_region_to_partial_trace`

The adapter maps `FigureRegion` ⇒ `PartialTrace` cleanly:

- **`FigureLane`** ⇒ `LaneEdge`s on level transitions
  (`Low → High` = `Rising`; `High → Low` = `Falling`) +
  `ValueSpan`s on contiguous identical-value runs. `Unknown`
  lane samples **break runs without recording an edge** —
  honest dormancy at the sample level: if upstream doesn't
  know the level, the adapter doesn't claim a transition.
- **`FigureAnnotation::Delay`** ⇒ a `RelativeDelay` straight
  through.
- **`FigureAnnotation::Value`** ⇒ an extra `ValueSpan`.
- **`FigureAnnotation::Label`** is informational only; the
  adapter ignores it (lane labels are recovered through prose
  / KG already).
- **`FigureAnnotation::Unknown`** ⇒ trace confidence is
  demoted one rank (`High → Medium`, `Medium → Low`, `Low`
  stays `Low`). Honest dormancy at the annotation level: if
  upstream couldn't classify, the trace's confidence drops
  to reflect the unrecognised evidence, never silently
  treated as fine.

### What you see in the report today

Run `specforge validate <intent.json>` and the SemanticIR /
IntentIR count blocks include:

```
  waveform: figure_contracts=0 verifier_fail_residuals=0
```

Both zero, on the nvme corpus today. That's the honest
dormancy signal: no `FigureRegion`s are produced upstream
today, so the adapter generates no `PartialTrace`s, and the
generalizer mints no contracts. The typed pathway is
unit-tested end-to-end with synthetic inputs (13 waveform
tests + 6 adapter tests), so the moment upstream lands real
records, the numbers move and the rest of the pipeline
(fusion, fidelity, .isf lowering) consumes the figure-mined
contracts the same way it consumes prose-mined ones.

Counts derive from the IR itself — `figure_contracts` is the
count of contracts whose `provenance.modality` is `Figure`;
`verifier_fail_residuals` is the count of `Residual` contracts
whose `reason` starts with `"verifier disagreement: "`. The
IR is self-describing.

### Negative-fixture coverage — junk waveforms do **not** mint contracts

This is the load-bearing safety claim: the round-trip verifier
+ the conservative generalization rules together guarantee
that a junk or under-determined figure does **not** produce
a fabricated contract. Three unit tests prove it at the
synthetic-input level:

- **`bare_edge_generalizes_to_observe_residual`** — a lane
  with a single rising edge and nothing else generalizes to
  `Observe { signal } + Residual { reason: "bare edge — no
  window licensed" }`. The observation is captured; no
  contract is minted.
- **`relative_delay_missing_bounds_lowers_residual`** — a
  `RelativeDelay` without bounds (or with degenerate bounds)
  generalizes to `Observe + Residual { reason:
  "under-determined delay — bounds missing or invalid" }`.
  No invented window.
- **`adapter_demotes_confidence_on_any_unknown_annotation`** —
  any `Unknown` annotation lowers the trace's confidence by
  one rank, so downstream consumers see the reduced
  confidence even when other parts of the trace are clean.

Corpus-level proof activates the moment upstream produces
`FigureRegion`s — and when it does, those tests are the
guardrails that say *"a contract with `Lowerable` lowering
came from a trace the verifier accepted; you can trust it."*

### The four user-facing guarantees

This design buys you four properties you can rely on:

1. **Figures become contracts when they're clean enough; no
   invented windows when they're not.** The conservative
   rules + Observe-on-bare-edge default means the IR captures
   the observation without inventing structure.
2. **Round-trip verification is structural, not authorial.**
   A generated contract that the trace doesn't satisfy is
   demoted to `Residual` automatically — fabrication is
   prevented at the generalizer's own boundary.
3. **Single-source confidence is capped.** A contract
   mined from a single figure never reaches `High` on its
   own; promotion requires cross-modal agreement through
   FUSION.
4. **Today's pipeline is byte-identical.** With no
   `FigureRegion`s in the corpus, every record is empty, the
   adapter is a no-op, the validate counts read zero, and
   nothing in `IntentIR` / `.isf` changes shape.

### Status — delivered (`2026-05-20`)

`R16-WAVEFORM-CONTRACT-MINING` is **closed**. All four
leaves landed under the standard CI bar:

- `.1` — crux design fixed (typed intermediate;
  conservative generalization rules with under-determined ⇒
  `Observe`+`Residual` honesty; round-trip verifier;
  cross-check delegated to FUSION).
- `.2` — typed `ir/waveform.rs` module + the four-rule
  `generalize_partial_trace` + the round-trip
  `verify_contract_against_trace` (reuses
  `R16-CAPTURE-FIDELITY-GATES.2`'s `evaluate_figure_trace`)
  + 7 unit tests.
- `.3` — **honest-split (rule 5)** after the corpus survey
  found no raw PDFs/SVGs in tree:
  - `.3.1` — the typed `FigureRegion` input contract
    (extends upstream `VisualAsset`);
  - `.3.2` — typed `ir/figure_region.rs` (`BoundingBox` +
    `LaneLevel` + `LaneSample` + `FigureLane` +
    `FigureAnnotation` enum + `FigureRegion`) +
    `figure_region_to_partial_trace` adapter + 6 unit tests
    including the end-to-end smoke test
    (FigureRegion → PartialTrace → generalize → verify =
    `Pass`).
- `.4` — `specforge validate` `waveform: figure_contracts=N
  verifier_fail_residuals=M` block; counts derived from
  `actor_contracts` via `provenance.modality == Figure` and
  `Residual.reason` `"verifier disagreement: "` prefix —
  IR is self-describing.

**Future raster / vector handling** is honestly deferred to
a new tree, when the upstream PDF pipeline produces those
bytes. That's not a re-opened leaf of this tree — it's a
new piece of work with its own scope.

*Authoritative tracking:*
`docs/tasks/R16-WAVEFORM-CONTRACT-MINING.md`.

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
