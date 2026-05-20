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

### `R15-GRAPH-DIRECTION-MIGRATION` — actor-relative graph direction across the pipeline

Earlier stages already carried the actor-relative graph; the
adapter, validation, and semantic stages still consulted a flat
`direction_hint` in places. This tree replaced the remaining
`direction_hint` consumers with actor-relative graph semantics,
so the answer to "which way does this signal go?" is consistently
"ask the graph from this actor's perspective" — not "read the
flat hint and hope it was set correctly upstream." `direction_hint`
itself was kept as a compatibility surface (Non-Goal: removal);
the migration is about *who decides direction* (the graph), not
*what fields exist*. Verified by leaf-by-leaf coverage of each
migrated consumer site + the full `scripts/run_ci.sh` regression.
*Authoritative tracking:*
`docs/tasks/R15-GRAPH-DIRECTION-MIGRATION.md` (Status closed
`2026-05-20`; metadata reconciled to match the long-standing
all-leaves-complete truth + book section added per the
now-structural `BOOK-METHOD-DOC` close-rule).
