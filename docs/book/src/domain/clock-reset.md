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
The same rule applies when a VLM timing diagram reports reset waveform values: an active-low reset observed as `asserted` and `LOW`, or as `deasserted` and `HIGH`, should stay equivalent temporal evidence, not become a contradiction.

## Document Scope

Most protocol or chip-interface PDFs are contract documents.
They are meant to let RTL designers, verification-IP authors, integration teams, and reviewers agree on the interface behavior.

For clock and reset semantics, that usually means the document can define:

- which clock and reset signals exist at the boundary
- polarity and active level
- synchronous or asynchronous reset semantics
- reset assertion and release discipline
- timing obligations that RTL and VIP can check

That is different from defining the physical clock tree or reset tree of the final chip.
Those trees are usually custom to the SoC team and depend on project-local choices such as clock generators, PLLs, reset controllers, power domains, CDC/RDC policy, DFT/scan constraints, CTS strategy, floorplan, and methodology.

So the extraction boundary is:

- recover clock/reset contract semantics from protocol PDFs
- preserve explicit current-document topology hints only when the document actually states them
- do not infer complete physical tree construction from a protocol PDF
- keep physical tree construction as an integration input or residual concern for the team building the chip

The tracked KG fixture `clock_reset_contract_scope_negative` locks this boundary.
It proves that protocol text can mention RTL/VIP contract scope and integration-owned physical clock/reset trees without creating concrete topology facts.
It also asserts that document/integration terms such as `PDF`, `RTL`, `IP`, `VIP`, `PLL`, `DFT`, and `SoC` do not become canonical interface signals.

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

For adapter lowering, the system contract can also serve as bounded shape evidence.
If a direct root or explicit module already has the clock/reset signals in its local inventory, `.fsm` lowering can recover those entries as input, 1-bit system-contract signals even when flat interface direction or width hints lag.
This is adapter-local recovery from canonical system facts, not a mutation of `IntentIR`, and it does not create undeclared clock/reset ports.
If other evidence contradicts the system-contract shape, the normal sticky conflict behavior keeps the direction or width unresolved and blocks lowering.
For example, if a local interface declaration says the clock is an output while the system contract says it is the clock, the adapter does not choose a side; it keeps the direction unresolved and asks for upstream correction.

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
The tracked KG fixture `clock_reset_topology_gold` locks the positive side of that boundary, where explicit current-document topology becomes typed infrastructure topology.
The companion `clock_reset_generic_advice_negative` fixture locks the negative side, where generic advice remains doctrine/caution and does not author topology facts.

## Reset polarity

Single-bit control signals have polarity.
The usual case is active-high, but active-low signals are common enough that the model must treat polarity explicitly.

For resets:

- `active high` means asserted at logical high
- `active low` means asserted at logical low
- names like `rst_n` and `ARESETN` are useful cues, but explicit document evidence is stronger

The same polarity rule applies to other single-bit control signals.
If the current document says `CS_N is asserted when LOW`, the assertion level is locally grounded and `ASSERTED` can refine to `LOW`.
That is different from guessing from the `_N` suffix alone.
Likewise, an unambiguous collective sentence such as `CS_N and WE_N are active LOW signals` can ground both declared controls as active-low.
Mixed low/high compound wording such as `CS_N is active LOW and ENABLE is active HIGH` can be recovered only when clause-local evidence binds every polarity phrase to exactly one known signal; detached polarity wording still stays unresolved.
If current-document sources disagree, for example prose says a reset is active-high while a signal-description table says active-low, the disagreement should stay visible as a polarity conflict rather than being hidden behind a forced active level.

This matters for temporal rules and conflict detection.

For example:

- if polarity is unknown, `ASSERTED` remains abstract
- if a signal is active-high, `ASSERTED` can refine to `HIGH`
- if a signal is active-low, `ASSERTED` can refine to `LOW`
- if a signal is active-low, `DEASSERTED` can refine to `HIGH`

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

The current model is not a physical clock-tree or reset-tree implementation engine.
It preserves contract-level semantics and selected current-document topology hints, but it does not prove implementation quality or synthesize the final chip's tree.

It still does not fully model:

- clock generation cells
- complete clock gating topology
- complete reset synchronizer structure
- reset fanout and distribution quality
- physical implementation constraints

Those are normally integration-team concerns rather than facts recoverable from AMBA-style protocol PDFs.
The important current boundary is that clock and reset semantics are first-class infrastructure intent, not ordinary protocol edges and not physical-tree signoff.
