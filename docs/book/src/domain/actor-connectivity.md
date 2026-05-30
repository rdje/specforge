# Actor Connectivity And Graph Direction

`specforge` treats protocol direction as a graph problem first.

That means the preferred question is not only:

- is this signal textually marked `input` or `output`?

The stronger question is:

- which actor drives this signal?
- which actor reads or samples it?

This distinction matters because chip specifications often describe interfaces from multiple viewpoints.
A signal that is an output for one actor is an input for another actor.

## The core relation types

The structural actor-signal graph currently uses two primary relation kinds:

- `Drives`
- `Reads`

They mean:

- `(Requester, Drives, PSEL)` means the requester is a producer of `PSEL`
- `(Completer, Reads, PSEL)` means the completer is a consumer of `PSEL`

Those are actor-relative facts.
They are stronger than a flat direction label because they preserve who the direction is relative to.

## Actor-relative ports

`SemanticIR` converts graph relations into actor ports.

For a single actor and signal:

- `Drives` becomes actor-relative `output`
- `Reads` becomes actor-relative `input`
- both together become actor-relative `in_out`
- neither remains `unknown`

This is why `ActorPortRecord` carries:

- actor id
- actor name
- signal name
- actor-relative direction
- relation basis
- width hint
- supporting statement ids
- automation confidence

The relation basis matters because it explains why a port was classified as input, output, or bidirectional.

## Signal connectivity

Actor ports are then grouped into signal connectivity records.

A `SignalConnectivityRecord` summarizes:

- signal name
- connectivity class
- producer actors
- consumer actors
- width hint
- supporting statements
- automation confidence

For protocol signals, this gives the canonical producer/consumer view of the signal.
For infrastructure signals, the connectivity class can be `system_clock` or `system_reset` instead of ordinary `protocol`.

Infrastructure signals also have their own canonical status records.
Those records track source status and recovered distribution status separately from ordinary producer/consumer protocol ownership.
That keeps useful graph information visible without pretending a clock/reset tree is just another payload edge.

## Why graph direction beats flat direction

Flat direction hints are still useful compatibility data.
They are not the highest-trust surface.

For example, a table might say:

`Signal | Source | Width | Description`

In an AMBA-style table, the `Source` column can identify who drives the signal.
That is a structural fact, not just a string that says `output`.

Similarly, prose such as:

- `The Manager drives HTRANS`
- `HREADY is sampled by the Manager`
- `PREADY is driven by the slave`

can recover graph facts that explain direction relative to specific actors.

That is why validation reports both:

- graph-derived direction coverage
- compatibility direction hint coverage

The graph path should lead when both are available.

## Where graph facts come from

Current graph extraction can use:

- `Source` / `Driver` style table columns
- `Destination` style table columns
- active prose such as `Manager drives HTRANS`
- passive prose such as `PREADY is driven by the slave`
- sampling prose such as `HREADY is sampled by the Manager`
- section-heading actor context when locally grounded and prior-guided
- some complementary actor recovery when the current document exposes exactly one local opposite actor role

The key rule is still local grounding.
Cross-document priors can widen actor vocabulary, but they cannot invent actors or signals that are not present in the current document.

For active prose, `specforge` also treats relative clauses as dangerous actor-noise zones.
In text shaped like `An interconnect which connects to components with a mixture of chunking support can drive ARCHUNKEN`, the graph extractor should keep the head subject `interconnect`, not promote the descriptive phrase `mixture of` into a protocol actor.
If that same clause says `can drive ARCHUNKEN and RCHUNKV`, both signals should inherit the same grounded head actor.
The same rule applies to consumer prose such as `The Manager samples ARCHUNKEN and RCHUNKV`.
That distinction is small but important: the graph should get cleaner because false actors are rejected, not because unresolved real actors are hidden.

## What gets filtered

Not every noun in a source column or sentence is a real protocol actor.

`specforge` should reject non-actor labels such as:

- `Clock`
- `Reset`
- `External`
- `Tie-off`
- `input`
- payload/event nouns such as `data`, `transfer`, or `control information`
- descriptive support phrases such as `mixture of`

Those labels may still matter as local evidence, but they should not become fake protocol actors in the structural graph.

This is part of the same truthfulness rule used elsewhere:

- recover structure when the document supports it
- do not fabricate cleaner actor boundaries than the document actually provides

## Conflicts

Graph extraction can produce real conflicts.

The most important one today is multiple producer ambiguity.
If a signal appears to have more than one producer, `specforge` should surface that as a connectivity conflict instead of silently picking one.

The KG benchmark harness can assert the shape of that conflict directly with `signal_connectivity_conflicts_include`.
For example, a fixture can require that `PREADY` is a `multiple_producers` conflict involving both `Completer` and `Monitor`, not merely that some connectivity conflict count is nonzero.

That conflict can then be carried through:

- `EvidenceIR`
- `SemanticIR`
- `IntentIR`
- validation reports

This makes structural uncertainty visible to users and downstream consumers.

Those conflicts are now replay surfaces too.
When unresolved producer ambiguity survives into `SemanticIR` or `IntentIR`, validation emits `semantic_signal_connectivity_conflict_surface_rescan_guidance` so the preserved conflict ids can drive the same bounded local replay lane used for other evidence-strength gaps.

## Missing endpoints

The graph can also be incomplete without being contradictory.

For example:

- a signal may have a recovered consumer but no recovered producer
- a signal may have a recovered producer but no recovered consumer

For protocol signals, validation now treats those endpoint gaps as explicit replay targets rather than passive warnings.
The stage-specific `semantic_connectivity_missing_producer_surface_rescan_guidance` and `semantic_connectivity_missing_consumer_surface_rescan_guidance` findings tell `project-validation` to rerun the local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` lane for those same signal ids.

Infrastructure signals are intentionally different.
Clock/reset sourcing can remain a system-contract note without entering the protocol replay family, because many protocol PDFs do not name the eventual clock generator or reset controller.

## Temporal grounding

Graph facts also help temporal semantics.

If the system knows that `Requester` drives `PSEL`, a temporal rule can express an actor-grounded predicate such as:

- `ActorDrivesSignal(Requester, PSEL)`

That is more useful than only saying `PSEL is asserted`.
It ties a timing obligation back to the responsible actor.

## Adapter use

The actor-relative graph is the canonical direction surface. `SemanticIR`
and `IntentIR` carry `actor_signal_relations`, `actor_ports`, and
`signal_connectivity`, and validation scores direction coverage from that
graph first, treating any flat `direction_hint` lag as a compatibility
diagnostic rather than truth-model loss.

SpecForge's single adapter target is `.isf`. The `.isf` adapter consumes
the canonical interface and behavior surface of `IntentIR` and lowers it
through the typed `IsfIr` model. It deliberately does **not** re-derive
target-actor-relative port directions or block on missing per-signal
direction/width: the ISF IR defaults an unknown direction to `output` and
an unknown width to `1`, and FSMGen performs the cycle scheduling
downstream of `.isf`. The actor-relative graph still matters because it
makes `IntentIR` direction/connectivity honest before lowering; it is just
no longer consumed by adapter-side renderability gymnastics.

`.fsm` and HDL are out of scope — FSMGen consumes `.isf` and owns
scheduling, `.fsm`, and HDL downstream. The fail-closed sticky-conflict
rules for the canonical graph (flat vs graph disagreement, duplicate
declaration disagreement, and self-conflicting graph evidence all collapse
to unresolved rather than silently choosing a winner) continue to apply at
the `SemanticIR` / `IntentIR` layer and keep both evidence planes
inspectable in the artifacts.

## What users should inspect

When debugging direction or connectivity, inspect:

- `actor_signal_relations` in `EvidenceIR`
- `actor_ports` in `SemanticIR` / `IntentIR`
- `signal_connectivity` in `SemanticIR` / `IntentIR`
- `signal_connectivity_conflicts`
- validation metrics such as `with_graph_direction`
- findings for incomplete graph-direction coverage

If direction coverage is weak, the right question is usually:

- did the document expose actor-signal relations that the pipeline failed to recover?

not only:

- did a table contain a literal `input` or `output` string?

## Closed task trees — how each was implemented and verified

### `R15-GRAPH-DIRECTION-MIGRATION` — one source of truth for "which way does this signal go?"

If you ask SpecForge whether a signal is an input or an
output, there's exactly one correct way to find out: ask the
actor-relative graph from the perspective of the actor you
care about. That sounds tautological. It wasn't, until this
tree paid for it.

#### The user-facing guarantee

> **The actor-relative graph is THE source of truth for
> signal direction. Every pipeline stage that needs to know
> "which way does this signal go for this actor?" consults
> the graph; no stage falls back to a flat
> `direction_hint` lookup that might be stale, missing, or
> set from a different perspective.**

The flat `direction_hint` field is still on the IR — kept as
a compatibility surface for code that hasn't been migrated to
the actor-relative graph — but it's no longer the *deciding*
consumer anywhere in the adapter, validation, or semantic
stages.

#### Why this isn't trivial

Direction in a protocol is **inherently relative to a
perspective**. The same wire is an output for the manager
and an input for the subordinate. A flat "is `AWVALID` an
input or an output?" question has no correct answer in the
abstract — only "for whom?" When the IR carries a single
flat `direction_hint`, you eventually need a convention for
*whose* perspective it captures, and that convention drifts:

- some upstream stage sets it from the manager's
  perspective;
- some downstream consumer reads it from the subordinate's
  perspective (because that's the more natural framing for
  that consumer);
- the result is direction reversed for a subset of signals,
  silently, with no error path.

The actor-relative graph fixes this by making the
perspective explicit: every direction edge carries
`actor → signal → direction-from-that-actor`. The same
underlying wire shows up with `direction = Output` from one
actor's view and `direction = Input` from the other's. The
answer is always available, and always correct for the
asker.

#### What this tree did concretely

Several pipeline stages (the `.isf` adapter, validation, the
semantic stage) still consulted `direction_hint` in places
even after the actor-relative graph was canonical in
`SemanticIR` / `IntentIR`. This tree migrated each of those
sites:

- the adapter now derives direction from the actor-relative
  graph when it needs it;
- validation findings that turn on direction now consult the
  graph, so a finding about *"this signal looks like an
  output but is being driven from outside the actor"* uses
  the actor-perspective answer;
- semantic-stage internal reads were migrated through the
  same lookup.

`direction_hint` is **kept** because some external
artifacts still carry it and we don't break compatibility.
But internally, the migration is about *who decides
direction* — the graph, not the flat field.

#### What this buys you, as a SpecForge user

- **No perspective bugs.** Ask any pipeline stage about a
  signal's direction; the answer is consistent because every
  stage asks the same graph from the same actor.
- **You can read the IR by perspective.** The
  actor-relative edges in `SemanticIR` / `IntentIR` let you
  derive *"what does the manager see?"* and *"what does the
  subordinate see?"* directly, without re-inferring it from
  scratch.
- **External tooling can still consume `direction_hint`.**
  The compatibility surface stays. Removal of the field is
  out-of-scope for this tree — and would be a separate
  decision recorded under the AUDIT-DOC-RECONCILE doctrine.

*Authoritative tracking:*
`docs/tasks/R15-GRAPH-DIRECTION-MIGRATION.md` (Status
closed `2026-05-20`; metadata reconciled to match the
long-standing all-leaves-complete truth + book section
added per the now-structural `BOOK-METHOD-DOC` close-rule).

### `R14-SIGNAL-RESOLVE` — Tier-3 LLM relation extraction for the hard prose

"Where graph facts come from" (above) lists two ways an
actor→signal edge enters the knowledge graph: Tier-1 structured
tables (a `Source`/`Destination` column) and Tier-2 verb
patterns ("the Manager drives AWVALID"). Those catch the clean
cases. `R14-SIGNAL-RESOLVE` adds the third tier for the
sentences they miss.

#### The problem this fixes

Real specs phrase relations in ways no fixed pattern catches:
"the completer is responsible for returning `PRDATA`", "address
information flows from the requester onto `PADDR`". A human
reads "who drives what" instantly; the Tier-2 verb matcher sees
no `drives`/`samples` keyword and leaves the edge unrecovered —
the signal ends up with no graph direction, surfaced as
recovery debt.

#### What it does

`specforge signal-resolve <evidence-ir> [--provider ollama]`
asks Qwen, for each residual normative sentence, for one
`{actor, signal, relation:"drives"|"reads"}` object (or
`none`). It is a **new** command — it does not touch the
existing `nlp_enrich` — and it writes to the **same**
`EvidenceIR.actor_signal_relations` field Tier-1/2 populate, so
a Tier-3 edge is indistinguishable downstream from a
table-derived one: it flows through `SemanticIr::build` into
actor ports and graph-first direction the exact same way.

#### Why it can't fabricate the graph

This is the part that matters. An LLM asked "who drives what"
will happily make something up; the command refuses to trust
it:

- the **signal** must be an uppercase hardware name (`AWVALID`,
  not "the address") — prose words cannot become graph nodes;
- the **actor** must be non-empty and the **relation** exactly
  `drives` or `reads`;
- with `--grounding-signals`, the signal must be one already
  declared in the spec — a hallucinated name is dropped;
- a new edge is **deduped** against the existing graph, so
  Tier-3 never inflates counts by re-stating a Tier-1/2 edge;
- every kept edge carries provenance to the exact source
  statement.

A `none`, an unparseable reply, or any gate failure yields **no
edge** — the worst case is a *missed* relation (recovery debt
stays honestly visible), never a fabricated one.

#### How it is verified

The decision is a pure `classify_relation_response`, unit-tested
over every path: `none`→skip, malformed→skip, valid
`drives`/`reads`→accepted, lowercase/prose signal→skip, bad
relation→skip, empty actor→skip, and ungrounded-signal→skip
(with grounded→accept). The command was run end-to-end on a real
EvidenceIR in `--provider skip` mode (loads the artifact,
selects its candidates); a live `--provider ollama` run is
runnable whenever the shared Ollama server is free.

*Authoritative tracking:* `docs/tasks/R14-SIGNAL-RESOLVE.md`.
