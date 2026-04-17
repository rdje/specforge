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

## Temporal grounding

Graph facts also help temporal semantics.

If the system knows that `Requester` drives `PSEL`, a temporal rule can express an actor-grounded predicate such as:

- `ActorDrivesSignal(Requester, PSEL)`

That is more useful than only saying `PSEL is asserted`.
It ties a timing obligation back to the responsible actor.

## Adapter use

Adapters should consume this graph when they need a target-actor-relative direction.

The first bounded consumers are `.fsm` lowering paths.

The explicit-module/top-composition path has the cleanest target actor context.
When an explicit child module already has a local signal and width but the flat module-local `direction_hint` is missing or stale, the adapter can overlay matching `IntentIR.actor_ports` for that module actor before renderability analysis.

Standalone direct roots are stricter.
They do not carry a module name that says which actor the target is relative to, so the adapter first looks for one actor that graph-drives every render-critical assignment or init target already present in the direct local inventory.
That allows normal external actors to coexist in the same graph: an environment may drive an input signal, and a monitor may read output signals, without making the direct root ambiguous when one target actor clearly owns all produced outputs.
If no such output-target actor exists, the older one-actor direct context gate remains the fallback.
Unrelated graph-only actor ports are ignored by this direct-root context gate and are not added to the standalone inventory.
If the graph mixes multiple possible target-output actors, or if no actor owns the required direct output targets, the adapter leaves the missing flat directions unresolved and blocks rather than guessing.
If repeated evidence for the same actor and signal disagrees, the collapsed direction or width remains unresolved; later duplicate hints cannot resurrect a value after conflict.
That same guarded direct-root path now has regression coverage for sequential system contracts too: graph-backed actor ports can satisfy the clock/reset input directions needed for `(+system ...)` when flat direct-interface hints lag.

Explicit top composition has one more bounded recovery path.
If a top boundary port has width but no flat direction, explicit link topology can recover the boundary role: a top endpoint used as a link source is a top input, and a top endpoint used as a link target is a top output.
That path is composition-topology recovery, not actor-graph inference, and conflicting or unresolved top boundary directions still block.

This is still conservative.
Graph `input` and `output` directions can fill the module-local port role, but `in_out` and `unknown` are not turned into fake `.fsm` directions.
Conflicts between flat hints and graph evidence still collapse to unresolved state and block lowering instead of silently choosing a winner.

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
