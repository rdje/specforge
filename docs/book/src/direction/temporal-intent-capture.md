# SOTA Temporal-Intent Capture (R16)

This chapter is the human-facing capture of the R16 design-intent-capture
program (complete; see the program-status note below).
The authoritative, machine-tracked source is the task-tree umbrella
`docs/tasks/R16-INTENT-CAPTURE.md` and ROADMAP lane `R16`; this page
must stay consistent with them.

> **Program status (`2026-05-29`): COMPLETE.** All six sub-trees were
> delivered in DAG order and closed at their honest scope boundaries
> (`2026-05-20`); the `R16-INTENT-CAPTURE` governance umbrella that
> sequenced them is now **closed** (`2026-05-29`). What this program
> built is the *mechanical-to-lower typed target*, the *objective
> capture-fidelity metric*, and *structural honesty enforcement* (four
> doctrines) — see the per-sub-tree sections below and the **R16 PROGRAM
> COMPLETE** recap at the end of this chapter.
>
> **Current activation note (`2026-08-11`):** the production VLM timing-note
> path now projects usable observations into an optional EvidenceIR
> `FigureRegion`, grounds its lanes to document-known signals, converts it
> through `PartialTrace`, and admits an `ActorContract` only after round-trip
> verification. A reviewed retained-PDF fixture proves the persisted path
> through IntentIR without claiming a live VLM run. The current corpus still
> has zero persisted VLM notes, so operational visual recall remains
> unmeasured rather than being inferred from the fixture. Prose constrained-
> contract provider wiring remains separate work.

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
extraction-fidelity thrust — the crux the thesis names. Each sub-tree
was promoted in DAG order under `R16-INTENT-CAPTURE`, with all code
landing under `COMMIT.md`-tracked leaves and `scripts/run_ci.sh` green
per leaf; all six are now `done`.

---

This chapter also carries, per sub-tree, a precise account of **how it
is implemented and how it is verified** (the `BOOK-METHOD-DOC`
convention). All six sub-tree sections are present below.

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

```rust,ignore
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

```text
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

```rust,ignore
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

```text
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

```rust,ignore
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

```text
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

```text
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

```text
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
(detailed under `.3.1` / `.3.2` below). The production builder now stores an
optional `FigureRegion` on the matching EvidenceIR visual item; SemanticIR
mines it before fusion/fidelity, and IntentIR carries the resulting contracts.
Older and unenriched artifacts remain compatible because the field defaults
to absent and is skipped while empty.

### `PartialTrace` — what a figure-extractor produces

```rust,ignore
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

At the original `WAVEFORM.3` delivery boundary, the extractor was a typed
adapter waiting for an upstream producer. That historical scope has now been
activated at the existing production seam: `enrich` reads the region crop and
writes a bounded timing JSON note on `VisualAsset`; rebuilding EvidenceIR
turns a usable note into `FigureRegion`. Raw image bytes still stay outside the
IR record, and the full-page pass remains deliberately absent because the
page-capture study found no useful intent-recall case for it.

The closest existing upstream record is
`crates/specforge/src/ir/source.rs::VisualAsset`
(`asset_id` / `asset_kind` / `page_id` / `image_path` /
`caption_text` / `diagram_kind`). `VisualAsset` is good at
*"this is a figure"* but doesn't carry the lane / annotation
structure the WAVEFORM generalizer needs. `FigureRegion` is
the typed **extension** the upstream pipeline produces when
it classifies a `VisualAsset` as a timing diagram and
recovers structure:

```rust,ignore
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

The contract remains additive: `VisualAsset` does not change. The matching
`VisualEvidenceItem` carries `figure_region: Option<FigureRegion>`. Projection
accepts only samples with explicit integer or `T<n>` tick addresses tied to a
visible clock grid/edge; array position is never time authority. `HIGH` and
`LOW` become concrete levels, all other states remain `Unknown`, and untyped
annotation text remains `Unknown` instead of being guessed into bounds.

`raw_image_path` is optional but already follows the repository-owned persisted-path contract. A serialized
record stores a repository-relative value; deserialization resolves it at the current repository root, and an
unlabeled external absolute value is rejected. This is enforced before an upstream figure extractor exists, so
activating the dormant record cannot reintroduce workstation-specific paths.

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

EvidenceIR validation now distinguishes raw timing observations from usable
typed regions:

```text
  timing_diagram_extractions: N
  typed_figure_regions: A
  typed_figure_regions_unavailable: U
```

SemanticIR / IntentIR validation also includes:

```text
  waveform: figure_contracts=0 verifier_fail_residuals=0
```

The retained corpus still reports zero because it contains no persisted VLM
timing notes. That is an operational-data absence, not producer dormancy. A
reviewed NXP UM11732 fixture now exercises a real retained PDF identity through
the persisted SourceIR, EvidenceIR, SemanticIR, and IntentIR boundaries. It
keeps the raw model-only lane visible in EvidenceIR, proves semantic grounding
removes it, and proves the grounded `WS` span reaches a verified figure contract.
The fixture is explicitly not presented as live-provider output.

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

The reviewed real-PDF vertical fixture supplies the first producer-level proof;
held-out live-provider recall/precision remains the next measurement boundary.
These tests are the guardrails that say *"a contract with `Lowerable` lowering
came from a grounded trace the verifier accepted; you can trust that bounded
claim."*

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
4. **Unenriched artifacts stay compatible.** With no timing note, the optional
   `FigureRegion` is omitted and the pipeline behaves as before. When a note is
   present, availability and unavailability are both counted explicitly.

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

**Later activation (`SPEC-TO-INTENT-ALIGNMENT.3`, 2026-08-11):** the existing
VLM region-crop observation is now the producer for the typed record. Direct
vector geometry and richer typed delay/value annotation recovery remain future
accuracy work; they are not prerequisites for the conservative lane path that
ships today.

*Authoritative tracking:*
`docs/tasks/R16-WAVEFORM-CONTRACT-MINING.md`.

## R16-CONSTRAINED-VERIFIED-EXTRACTION — how it is implemented and verified

This is the closing tree of the R16 program — the one that
removes the last authorial path to a bad contract from the
prose side of extraction. Reading it end-to-end should leave
you with a working mental model of how SpecForge will safely
consume LLM/VLM-extracted contracts (when those extractors
land), the four typed pieces (`parse_constrained_contract`,
`entailment_check`, the template library, the
uncertainty-driven converge selector), and why this completes
the four-doctrine structural-honesty story.

### The problem this fixes

The prior R16 trees gave us a lot: a typed target
(`ContractIR`), protocol structure (`KG-ONTOLOGY`),
mechanically-enforced fidelity (`FIDELITY-GATES`),
deterministic multi-source fusion with disagreement routing
(`MULTIMODAL-CONTRACT-FUSION`), and a verifier-gated
figure-mining pipeline (`WAVEFORM-CONTRACT-MINING`). What was
left: keeping every prose-extracted contract honest at
**extraction time**, not just at lowering time.

The dangerous case looks something like this: an LLM is asked
to extract contracts from a paragraph of prose. It returns a
JSON blob that *looks* like an `ActorContract`. Without typed
constraints, the parsing pipeline tolerantly assembles a
`Lowerable` contract from a slightly-malformed blob; without
an entailment check, the contract claims a signal that doesn't
appear in the source paragraph; without protocol templates, a
familiar shape (ready/valid handshake!) gets recognised
heuristically rather than via a typed match. All three of
those failure modes produce contracts that *look* fine but
aren't grounded in the source.

`R16-CONSTRAINED-VERIFIED-EXTRACTION` adds four typed pieces
that make each of those failure modes a `Residual` instead of
a silent pass — and lays down the **fourth structural honesty
doctrine** (`entailment Fail → Residual`) that completes the
program's four-doctrine guarantee.

### The mental model

> **An LLM/VLM emits JSON that the schema-constrained adapter
> parses into an `ActorContract` (or rejects, closed-fail). An
> entailment verifier checks the contract is actually licensed
> by its source span (or demotes it to `Residual`). Canonical
> protocol templates short-circuit familiar shapes (ready/valid,
> burst+last, …) only when their promised signals are present in
> the boundary. An uncertainty-driven converge selector decides
> which low-confidence / failing contracts deserve the next
> extraction pass's budget. This bounded design now ships through
> `specforge extract-contracts`; every response still crosses the
> schema and entailment gates before it can enter the typed IR.**

Everything below starts with the typed design of the four pieces
(adapter, verifier, templates, selector), then records their
delivery and the live producer. For current command behavior, see
[`extract-contracts` and `signal-resolve`](../commands/quality-and-learning.md#extract-contracts-and-signal-resolve).

### Where `cve` lives

It's a typed layer, not a new pipeline stage — parallel to
prior R16 trees. A new module
`crates/specforge/src/ir/cve.rs` houses the
constrained-decoding adapter (`.2`), the entailment verifier
(`.3`), and the uncertainty selection helper (`.5`). The
protocol-pattern templates (`.4`) live in `prior_memory`
(the existing home for repeated priors). No new IR stage;
`SemanticIr`/`IntentIr` schemas unchanged.

### `parse_constrained_contract` — fails-closed JSON-Schema decoding

```rust,ignore
pub fn parse_constrained_contract(json: &str) -> Result<ActorContract>;
```

A provider-agnostic adapter for the LLM/VLM-emitted JSON case.
Two safety properties:

- **Serde is the authoritative validator.** The function uses
  `serde_json::from_str::<ActorContract>` against the typed
  Rust shape; invalid JSON or shape violations return `Err`
  — never a partially-assembled contract.
- **The JSON-Schema "summary" surface is documentation, not
  the validator.** A discriminator drift-lock test
  (`actor_contract_summary_schema_lists_obligation_kinds`)
  guarantees that as the typed `Obligation` enum grows, the
  documented schema summary either grows with it or the test
  fails — preventing the doc from silently drifting from the
  code.
- **Provider-agnostic.** No SDK is pinned; the function just
  parses JSON. Which LLM, which prompting strategy, which
  schema-constrained generation mode — all out-of-scope for
  this tree.

### `entailment_check` — and the **fourth structural honesty doctrine**

```rust,ignore
pub fn entailment_check(source_span: &str, contract: &ActorContract)
    -> FindingStatus;
```

The entailment verifier answers a single question: *"is this
contract actually licensed by this source paragraph?"* The
initial implementation is deliberately conservative:

- every signal the contract references must appear in
  `source_span` (case-preserving substring);
- every numeric bound the obligation carries must match a
  complete digit-run in `source_span` (so `7` does not
  spuriously match `70`);
- otherwise `Pass`. (Future iterations may consult an
  LLM-as-judge gated behind the same API — but never to
  *soften* a `Fail`; only to lift `NotEvaluated` to `Pass`
  with a separate `automation_confidence` adjustment.)

The companion `apply_entailment_to_contract(&mut contract,
source_span)` enforces the doctrine: a `Lowerable` contract
that fails entailment is demoted to:

```rust,ignore
LoweringDisposition::Residual {
    reason: "entailment fail: <details>",
}
```

The contract still exists in the IR (the observation isn't
lost); the diagnostic reason tells you exactly which span
failed to license it. This is the **fourth structural
honesty enforcement**, completing the set:

1. `FIDELITY.3` — fidelity gate Fail on a `Lowerable`
   contract ⇒ `Residual`.
2. `FUSION.3` — disagreement across sources ⇒ `Residual`.
3. `WAVEFORM.3` — generalized contract not satisfied by its
   source trace ⇒ `Residual`.
4. `CVE.3` — entailment Fail on a `Lowerable` contract ⇒
   `Residual`.

The four together mean: **a contract that any structural
check rejects is mechanically routed to `Residual` with the
reason in the text — silent fabrication is impossible
end-to-end.**

### Protocol-pattern template library — known shapes, grounded matches

```rust,ignore
pub enum ProtocolTemplate {
    ReadyValidHandshake,
    CreditFlowControl,
    SetupAccess,
    AsyncAssertSyncReleaseReset,
    BurstLast,
}

pub fn instantiate_template(
    template: ProtocolTemplate,
    bindings: &SignalBindings,
) -> Option<ActorContract>;
```

Five canonical templates seed `prior_memory` so familiar
protocol shapes don't have to be re-inferred from prose every
time. The **match-grounding rule** is what keeps the
templates honest: `instantiate_template` returns `None` when
the template's promised signals (`SignalBindings`) aren't
present on the actor's boundary — never a fabricated match.

Where a template instantiates as a concrete obligation
(`ReadyValidHandshake` → `HandshakeBarrier`;
`AsyncAssertSyncReleaseReset` → `Drive`; `BurstLast` →
`Drive`), the contract lowers as `Lowerable`. Where a
template's full semantics aren't expressible in the closed
`Obligation` algebra today (`CreditFlowControl` would need a
counter primitive; `SetupAccess` would need protocol-phase
ordering grounded by extraction), the template
**honestly instantiates as `Observe + Residual { reason: "…
not yet representable as a single ContractIR obligation" }`**.
The template existence is recorded; the lowering is honestly
deferred. The doctrine pattern from `WAVEFORM.2`'s bare-edge
case extends here naturally.

The match itself is **entailment-verifiable**: an instantiated
template's signals appear in `bindings.bindings.values()`;
those signals must appear in any source span the template was
matched against. The verifier from `.3` will pass the match;
the doctrine that prevents wrong matches is the same one that
catches wrong free-form contracts.

### Uncertainty-driven converge — `voi_score` + `select_top_n_by_voi`

```rust,ignore
pub fn voi_score(
    contract: &ActorContract,
    findings: &[FidelityFinding],
) -> f64;

pub fn select_top_n_by_voi(
    inputs: &ConvergeInputs<'_>,
    n: usize,
) -> Vec<String>;
```

When an LLM/VLM extraction loop runs multiple passes, each
pass has a bounded budget. `voi_score` is a deterministic
value-of-information ranking:

```text
voi(c) = w_conf * (1 - rank(c.automation_confidence) / 2)
       + w_fail * count_fail_findings(c)
```

with `w_conf = w_fail = 1.0` initial. Higher VoI = more
worth re-extracting (lower confidence, more failed
fidelity findings). `select_top_n_by_voi` picks the top-N
contracts deterministically — ties break by `contract_id`
lex-ascending so the next pass is reproducible across runs.

The integration into the converge loop is deferred (it's a
small follow-up when the LLM/VLM extractor lands); the
selection primitive is the load-bearing piece this tree
delivers.

### What you see in the report today

Run `specforge validate <intent.json>` and the SemanticIR /
IntentIR count blocks include:

```text
  constrained: schema_rejects=0 entailment_fails=0 template_hits=0
```

All zero, on the nvme corpus today. That's the honest
dormancy signal: no upstream LLM/VLM extractor is invoking
`parse_constrained_contract`, no source spans are being fed
to `entailment_check`, no template matches have been
attempted. The metric reports zero across the corpus because
the four typed pieces are *available* but not yet *driven*.

Counts derive from the IR itself — `schema_rejects` is
sourced when the adapter is invoked; `entailment_fails` is
the count of `Residual` contracts whose `reason` starts with
`"entailment fail: "`; `template_hits` is the count of
contracts whose `contract_id` carries the template prefix
the templates emit. The IR is self-describing.

The primitives are unit-tested with synthetic inputs (4 + 7 +
7 + 5 = 23 tests in `ir/cve.rs`); each becomes load-bearing
as upstream extraction lands.

### The four user-facing guarantees

This design buys you four properties you can rely on:

1. **An LLM/VLM extractor can never silently fabricate a
   contract.** The schema-constrained adapter fails closed on
   shape violations; the entailment verifier demotes
   contracts that aren't licensed by their source span; the
   template match-grounding refuses fabricated matches.
2. **The four-doctrine guarantee is complete.** Fidelity +
   fusion + waveform-verifier + entailment together make
   structural fabrication-prevention end-to-end.
3. **The extraction loop converges deterministically.**
   `select_top_n_by_voi` is reproducible across runs with
   stable tie-breaking; the next pass is the same next pass.
4. **Today's pipeline is byte-identical.** All four pieces
   are dormant under the corpus's current no-LLM-extractor
   conditions; reports look the same as they did before this
   tree.

### Status — delivered (`2026-05-20`) — R16 PROGRAM COMPLETE

`R16-CONSTRAINED-VERIFIED-EXTRACTION` is **closed**. All six
leaves landed under the standard CI bar:

- `.1` — high-precision-by-construction design (typed layer
  / no new stage; provider-agnostic schema adapter;
  entailment verifier; template library; uncertainty-driven
  converge).
- `.2` — typed `cve` module: provider-facing JSON-Schema
  summary + fails-closed `parse_constrained_contract`
  adapter (serde is the authoritative validator) + 4 tests
  including the obligation-discriminator drift-lock.
- `.3` — entailment verifier `entailment_check(span,
  contract)` + `apply_entailment_to_contract`
  Fail→Residual routing (the **fourth structural-honesty
  enforcement**, parallel to FUSION.3 / FIDELITY.3 /
  WAVEFORM.3) + 7 tests including a complete-digit-run
  match for numeric bounds.
- `.4` — protocol-pattern template library (5 canonical:
  `ReadyValidHandshake` / `CreditFlowControl` /
  `SetupAccess` / `AsyncAssertSyncReleaseReset` /
  `BurstLast`) with `SignalBindings` +
  `instantiate_template` match-grounding gate; CFC +
  SetupAccess honestly Residual (deferred lowering, no
  fabrication) + 7 tests including the "match is
  entailment-verifiable" round-trip.
- `.5` — uncertainty-driven converge `voi_score` +
  `select_top_n_by_voi` (deterministic lex tie-break) + 5
  tests (converge-loop integration deferred — honest
  bounded scope).
- `.6` — `specforge validate` `constrained:
  schema_rejects=N entailment_fails=M template_hits=K`
  block; counts derived from `actor_contracts` (the IR is
  self-describing).

Corpus baseline: `constrained: schema_rejects=0
entailment_fails=0 template_hits=0` — honest dormancy until
upstream LLM/VLM extraction lands.

#### R16 PROGRAM COMPLETE — what you can rely on now

All six R16 sub-trees closed at their honest scope
boundaries:

- **`R16-CONTRACT-IR` (#1, DAG root)** — typed
  timed-contract IR. One bound obligation lives as one
  `ActorContract` with a small closed operator algebra; the
  `.isf` lowering reads the typed projection (`actor_contracts`)
  and produces byte-identical output to the pre-program
  baseline, with `Lowerable`/`Residual` lowering disposition
  carrying every honesty boundary.
- **`R16-KG-PROTOCOL-ONTOLOGY` (#2)** — typed protocol
  structure (`Channel`, `ProtocolPhase`, `Transaction`,
  `HandshakePair`). The spec's organising vocabulary is now
  first-class in the IR; `TickPhase ≠ ProtocolPhase` is
  enforced; mechanical `HandshakePair` projection from
  `HandshakeBarrier` contracts is wired and tested.
- **`R16-MULTIMODAL-CONTRACT-FUSION` (#3)** — deterministic
  multi-source fusion. Agreement merges preserve provenance;
  disagreements route to `Residual` with the disagreeing
  fields named. The producer runs before fidelity, so
  cross-modal reconciliation is a first-class IR phase.
- **`R16-WAVEFORM-CONTRACT-MINING` (#4)** — typed figure →
  contract pipeline. `PartialTrace` is the typed handoff;
  four conservative generalization rules + a round-trip
  verifier prevent fabrication at the mining boundary; the
  `FigureRegion` input contract targets the upstream PDF
  pipeline.
- **`R16-CAPTURE-FIDELITY-GATES` (#5)** — six-gate fidelity
  producer + `validate fidelity:` block. The doctrine
  becomes structural; the corpus baseline `fail=0
  score=1.000` is the live evidence that today's contract
  producer is fidelity-honest.
- **`R16-CONSTRAINED-VERIFIED-EXTRACTION` (#6)** — the four
  CVE pieces (adapter / entailment / templates / selector) +
  `validate constrained:` block. The fourth structural
  honesty doctrine closes the set.

**Four load-bearing honesty doctrines** are now structural
rather than authorial:

| # | Doctrine | Tree |
| --- | --- | --- |
| 1 | fidelity Fail-on-Lowerable → Residual | `R16-CAPTURE-FIDELITY-GATES.3` |
| 2 | fusion disagreement → Residual | `R16-MULTIMODAL-CONTRACT-FUSION.3` |
| 3 | trace-verifier disagreement → Residual | `R16-WAVEFORM-CONTRACT-MINING.2`/`.3.2` |
| 4 | entailment Fail-on-Lowerable → Residual | `R16-CONSTRAINED-VERIFIED-EXTRACTION.3` |

Together, the IR cannot silently fabricate a contract that
any of the four checks rejects. Fabrication is mechanically
prevented end-to-end.

#### Program governance — closed (`2026-05-29`)

The `R16-INTENT-CAPTURE` umbrella that owned the ordering, the
dependency DAG, and the cross-tree invariants closed on
`2026-05-29`, once all six sub-trees were `done`. Closing it
records an honest boundary, not a finished crux: the *typed
target and its honesty enforcement are complete*, while the
*extraction that fills the target* waits on two upstream-blocked
future trees —

- **waveform raster/vector extraction** — decode real PDF
  timing-diagram bytes into the typed `FigureRegion` the `.3.1`
  section above defines, so `figure_region_to_partial_trace` has
  real input;
- **constrained-extraction producer wiring** — connect a prose
  LLM/VLM provider to `parse_constrained_contract` so the
  entailment-verified adapter has real candidates.

Both are recorded as deferred future trees rather than re-opened
leaves of any closed R16 tree, in keeping with the
residual-honesty doctrine: we do not pretend the crux is solved
before the evidence to solve it exists.

*Authoritative tracking:*
`docs/tasks/R16-CONSTRAINED-VERIFIED-EXTRACTION.md`.

## R16-MODULE-HARDENING — how the R16 modules are unit-test hardened

The six R16 sub-trees above each landed their typed module under the
standard CI bar. `R16-MODULE-HARDENING` is a follow-on quality tree that
brings those seven modules — `contract`, `protocol_graph`, `fidelity`,
`fusion`, `waveform`, `figure_region`, `cve` — to the same *direct*
unit-test signoff bar the rest of the codebase holds (the discipline the
`R6-*-HARDENING` trees applied to the mature modules).

**Why bother, when the surface is dormant?** These modules carry no live
extractor today, but they are load-bearing the moment upstream extraction
lands. A silent bug in, say, `figure_region::inferred_ticks` (an off-by-one
on tick counts) or `contract_from_temporal_rule` (a mis-mapped consequent)
would quietly corrupt the typed contracts everything downstream depends on.
Locking the behavior now, while it is small and readable, is far cheaper
than debugging it through a full pipeline later.

**How it was done — audit, then backfill only genuine gaps.** Each module
was read against its tests to find *real* untested behavior — branching,
precedence, edge cases, the serde wire contract — never padding for trivial
getters. The gaps found were closed:

- `figure_region` (the only module with **0** direct tests) → tests locking
  `inferred_ticks` precedence/derivation/saturation and the enum-tagged
  serde shape.
- `waveform` → the `capped_confidence` never-promote-to-`High` cap and the
  single-tick `ValueSpan` skip.
- `protocol_graph` → the `phase()` unknown-id `None` path.
- `fidelity` → the `evaluate_figure_conformance` trace-present Pass / Fail /
  obligation-unsupported (`NotEvaluated`, never silently `Pass`) glue.
- `contract` → the windowed-no-consequent empty-`Observe` defensive path,
  the actor-grounded `ActorMaintainsSignalStable` stability arm, and the
  `ActorSamplesSignal` / `SignalSampled` Observe-as-`Assume` arms.
- `fusion` → the merge guard-candidate union/dedup and the `apply_fusion`
  distinct-key early-exit.
- `cve` was already comprehensively covered and is recorded as such.

**How it is verified.** Every leaf ran the full `scripts/run_ci.sh` gate
(fmt / clippy-`-D` / tests-`-D` / rustdoc / mdBook). The R16 module test
count rose from 67 to 86 across the tree, and the whole lib suite stayed
green throughout.

*Authoritative tracking:* `docs/tasks/R16-MODULE-HARDENING.md`.

## CVE-PROSE-EXTRACTION — wiring the constrained extractor to the live provider

The `R16-CONSTRAINED-VERIFIED-EXTRACTION` surface above built the
fails-closed parser, the entailment verifier, the template library, and the
value-of-information selector — then left them **dormant**: nothing called
`parse_constrained_contract` on real prose, so `specforge validate` always
read `constrained: schema_rejects=0 entailment_fails=0 template_hits=0`. The
typed machinery was ready; it had no producer. `CVE-PROSE-EXTRACTION` is that
producer.

### Why this was the right next step (and why it is *wiring*, not invention)

SpecForge already ships a production LLM/VLM provider — Ollama + Qwen2.5VL,
the default for `converge`, already driving `enrich` (diagrams) and
`nlp_enrich` (prose constraints). The constrained surface didn't need a *new*
provider; it needed the *existing* one routed into
`parse_constrained_contract`. So this tree connects two things that already
existed.

### The mental model

> For each normative prose sentence, ask Qwen for one `ActorContract`-shaped
> JSON object — then trust nothing it says. Parse it through the fails-closed
> adapter, verify the sentence actually licenses it, and keep only what
> survives, as honest `Lowerable` *or* `Residual` — never fabricated.

### The `extract-contracts` command

`specforge extract-contracts <evidence-ir> [--provider ollama] [--model …]
[--max-statements N] [--dry-run]` walks the EvidenceIR's `NormativeStatement`
prose. For each candidate it prompts the provider with
`actor_contract_json_schema_summary()` plus a strict "emit JSON or `none`,
never invent" instruction, then classifies the reply through one pure,
deterministic function:

- **`none`** (the no-contract sentinel) → skipped.
- **invalid / missing-required JSON** → `parse_constrained_contract` fails
  closed → counted a `schema_reject`, with *no* contract produced.
- **valid JSON** → `apply_entailment_to_contract` against the source
  sentence: if every signal and bound the contract names appears in the
  sentence it stays `Lowerable`; otherwise the honesty doctrine reroutes it
  to `Residual{reason:"entailment fail: …"}`.

Survivors get their provenance overridden to the *actual* statement (the
model's self-reported provenance is never trusted) and a namespaced
`contract_id` (`cve:<statement_id>`), then land on a new additive
`EvidenceIR.extracted_contracts`, beside a persisted
`ConstrainedExtractionStats {candidates_seen, schema_rejects,
contracts_accepted}`.

### How the survivors reach the typed KG

`SemanticIr::build` folds `extracted_contracts` into `actor_contracts`
**before** the existing `apply_fusion` / `apply_fidelity_gates` pass — so an
extracted contract is treated exactly like a temporal-rule-projected one:
cross-modal duplicates fuse, contradictions route to `Residual`, and the
fidelity gates apply. `IntentIR` carries the result, and `validate`'s
`constrained:` block now reports a real `schema_rejects` (read from the
carried stat) alongside the `entailment_fails` / `template_hits` it already
derived from the contracts themselves.

### Why it is safe to ship before accuracy is high

The honesty doctrines exist precisely so an *imperfect* extractor is still
*safe*: a bad answer becomes a `schema_reject` (nothing), an unverifiable one
becomes a `Residual` (modelled, not lowered), and a cross-source
contradiction becomes a `Residual` at fusion. The producer **cannot** mint a
`Lowerable` contract the prose does not license. Low extraction yield shows
up as *few contracts* — never as fabrication. Raising precision/recall is
iterative tuning, measured by the fidelity gates, not a prerequisite for
shipping the wired surface.

### How it is verified

The per-response decision is a pure `classify_response`, unit-tested over all
four outcomes (skip / `schema_reject` / accepted-`Lowerable` /
accepted-entailment-`Residual`) plus Markdown-fence stripping — the logic is
proven without any network call. The transport (curl + the
`SPECFORGE_VLM_HELPER` test hook) mirrors the already-tested `nlp_enrich`. The
command was also exercised on a real protocol EvidenceIR in `--provider skip`
mode: it loads the artifact and selects its `NormativeStatement` candidates
(the load + candidate-selection path, no network). A live `--provider ollama`
run was dispatched too — but because the shared Ollama server is single-model
and was saturated by concurrent `converge` jobs, that one inference could not
get a slot; it is recorded honestly as *dispatched, server-gated* rather than
claimed as a completed run. Crucially, neither the wiring nor the decision
logic depends on that empirical call — it is confirmation, runnable any time
the server is free.

*Authoritative tracking:* `docs/tasks/CVE-PROSE-EXTRACTION.md`.
