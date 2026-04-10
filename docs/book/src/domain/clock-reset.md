# Clock And Reset Infrastructure

Clocks and resets are special in digital systems.

They are not ordinary protocol payload signals, and `specforge` should not flatten them into the same category as request, ready, valid, data, or response lines.

## Why they are special

Clock and reset signals usually have system-level responsibilities:

- clocks define the timing reference for sequential behavior
- clock generation and distribution require careful implementation
- clock trees should avoid unsafe glitchy logic
- resets define initialization and recovery behavior
- reset trees should be treated carefully because they can affect many registers at once
- reset assertion and release timing often matter as much as the signal name

For example, an active-low reset such as `ARESETN` or `rst_n` is asserted when the signal is low, not high.
That is why `ASSERTED` must not be blindly treated as `HIGH`.
It is polarity-relative.

## What `specforge` models today

The current canonical surface models this through a `system_contract`.

That contract records:

- clock signal
- reset signal
- reset kind
- reset polarity
- reset assertion timing
- reset release timing
- reset target kind
- supporting statements
- automation confidence

In the Rust IR, this is represented by the system-contract records carried through `SemanticIR` and `IntentIR`.

The connectivity surface also distinguishes:

- `protocol`
- `system_clock`
- `system_reset`

That distinction lets validation treat `ACLK` and `ARESETN`-style signals as infrastructure connectivity instead of ordinary protocol missing-producer cases.

`SemanticIR` and `IntentIR` also carry `infrastructure_signals`.

Each infrastructure signal record captures:

- signal name
- infrastructure kind
- source status
- recovered source actor IDs and names, if grounded
- distribution status
- recovered destination actor IDs and names, if grounded
- explicit infrastructure topology records, if grounded
- supporting statements
- automation confidence

The source status is intentionally conservative.
If the current document grounds `ACLK` as a clock but does not ground a clock generator, PLL, or other source actor, the record says `unresolved_source`.
It does not invent a producer named `Clock`, `External`, or `input`.

If the current document does explicitly ground an infrastructure source, that evidence may be recovered.
For example, phrases like `clock generator drives ACLK` or `PLL generates ACLK` can make the `ACLK` infrastructure record report a recovered producer.
That is still different from treating the bare table value `Clock` as a producer actor.

The distribution status is similarly bounded.
It can say whether the clock or reset is distributed to zero, one, or multiple recovered actors.
That is useful for semantic inspection, but it is still not a physical clock-tree or reset-tree proof.

If the current document explicitly says `ACLK is distributed to the Requester and Completer`, the infrastructure record can recover those distribution targets.
Likewise, a phrase such as `reset synchronizer feeds ARESETN to the Requester` can recover both the reset infrastructure source and a recovered distribution target.
Those targets remain part of the infrastructure surface; they do not automatically become ordinary protocol actor ports.

The topology surface is narrower and more explicit.
`infrastructure_topology` can preserve bounded hints such as:

- `clock_gated_branch`
- `reset_synchronizer_stages`
- `reset_tree_targets`

For example, a current-document sentence such as `The ACLK clock gate CGATE0 feeds the Requester branch` can record a gated clock branch with component `CGATE0`.
A sentence such as `The two-stage reset synchronizer RSTSYNC0 feeds ARESETN to the Requester` can record a reset synchronizer stage count.
A sentence such as `The ARESETN reset tree targets the Requester registers and Completer registers` can record reset-tree targets.

This is intentionally not learned from generic clock/reset doctrine or prior memory.
If the current document only says that a synchronizer might exist, or gives generic advice about clock gating, the topology record should stay absent.

## Reset polarity

Single-bit control signals have polarity.
The usual case is active-high, but active-low signals are common enough that the model must treat polarity explicitly.

For resets:

- `active high` means asserted at logical high
- `active low` means asserted at logical low
- names like `rst_n` and `ARESETN` are useful cues, but explicit document evidence is stronger

This matters for temporal rules and conflict detection.

For example:

- if polarity is unknown, `ASSERTED` remains abstract
- if a signal is active-high, `ASSERTED` can refine to `HIGH`
- if a signal is active-low, `ASSERTED` can refine to `LOW`

That is why the temporal conflict model is polarity-aware rather than treating `ASSERTED` and `HIGH` as universal synonyms.

## Reset timing discipline

When a reset is modeled as asynchronous, `specforge` treats the expected discipline as:

- assertion is asynchronous to the clock
- release is synchronous to the clock
- the reset target is a dedicated reset pin

When a reset is modeled as synchronous, the expected discipline is:

- assertion is synchronous to the clock
- release is synchronous to the clock
- the reset target is the data input path

That is not a complete physical reset-tree model.
It is the current canonical intent-level contract that keeps reset semantics from being erased.

## What `specforge` refuses to do

The project should avoid unsafe shortcuts such as:

- treating `External` as a real protocol actor for clocks or resets
- treating `Tie-off` as a producer actor
- turning clock/reset infrastructure rows into ordinary producer/consumer graph facts
- assuming `ASSERTED` means `HIGH` without polarity
- hiding reset polarity conflicts behind a clean-looking artifact

This is part of the same truthfulness doctrine used elsewhere in the project:

- preserve evidence
- keep uncertainty visible
- do not fabricate a cleaner semantic model than the document supports

## Validation surface

Validation can expose clock/reset handling through:

- `has_system_contract`
- `with_resolved_polarity`
- infrastructure connectivity metrics and notes
- `infrastructure_signals`
- `infrastructure_signals_unresolved_source`
- `infrastructure_signals_shared_recovered_distribution`
- `infrastructure_topology_records`
- `infrastructure_clock_gated_branches`
- `infrastructure_reset_synchronizer_stages`
- `infrastructure_reset_tree_targets`
- signal polarity conflicts
- temporal conflicts that account for resolved polarity

When a clock or reset is intentionally classified as infrastructure, it may appear as an informational system-contract finding rather than a protocol connectivity failure.

## Current limits

The current model is not yet a full physical clock-tree or reset-tree analysis.
It preserves selected current-document topology hints, but it does not prove implementation quality.

It still does not fully model:

- clock generation cells
- complete clock gating topology
- complete reset synchronizer structure
- reset fanout and distribution quality
- physical implementation constraints

Those are future directions.
The important current boundary is that clock and reset semantics are first-class infrastructure intent, not ordinary protocol edges.
