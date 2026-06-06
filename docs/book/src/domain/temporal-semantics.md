# Temporal Semantics And Timing

Chip specifications often describe not just what signals exist, but when obligations must hold.

`specforge` models that timing surface with typed temporal rules instead of leaving all timing language as prose.

## What a temporal rule is

A temporal rule records:

- rule id
- optional clock signal
- clock edge
- antecedent predicates
- consequent predicates
- optional cycle window
- source text
- supporting statement ids
- automation confidence

This keeps timing facts inspectable.
The source prose remains visible, but the rule can also be checked and consumed as typed structure.

## Clock edge and tick phase

Temporal rules carry a clock edge:

- `rising`
- `falling`
- `unknown`

Temporal predicates carry a tick phase:

- `pre_tick`
- `post_tick`

This lets the IR say not only which condition holds, but where it sits relative to the modeled clock tick.

Temporal rules also carry an optional clock signal.
When the current sentence names a local clock explicitly, that local name should win over a document-wide default clock.

Examples:

- `on the third rising edge of HCLK` should preserve `clock_signal = HCLK`
- `on HCLK rising edge` should preserve `clock_signal = HCLK`
- `in the same ACLK cycle` should preserve `clock_signal = ACLK` and inherit the same bounded default rising-edge anchoring used for default-clock cycle timing
- if the text does not name a clock, the document default clock can still fill the gap

## Predicates

Current temporal predicates include:

- `SignalValue`
- `ActorDrivesSignal`
- `ActorMaintainsSignalStable`
- `SignalStable`
- `ActorSamplesSignal`
- `SignalSampled`
- `HandshakeComplete`

The actor-grounded predicates are important because they connect timing obligations back to structural responsibility.

For example, if the graph says `Requester` drives `PSEL`, a temporal rule can express that actor responsibility instead of only saying `PSEL` changes.

### Indexed signal families in antecedents

Specs routinely declare a per-instance signal with an index — the bus select is
declared `PSELx` (one per completer), but prose refers to it bare as `PSEL`
("PNSE must be valid **when PSEL is asserted**"). The bare name is not literally a
declared signal, so an antecedent that mentioned only `PSEL` used to be dropped —
silently weakening the recovered condition (e.g. "when PSEL, PENABLE, and PREADY
are asserted" would keep only PENABLE and PREADY).

SpecForge now resolves an un-indexed prose reference to its declared indexed family
member (`PSEL` → `PSELx`, including numeric indices like `FOO0`) using the universal
`x`/digit index convention — so the temporal rule records the **same canonical signal
identity** the catalog and the connectivity graph use, rather than a second spelling.
This is grammar (the index convention), not a hardcoded signal name, and it is purely
additive: it only fires when the bare token is not itself a declared signal, so it can
never override a real declaration or invent a signal that has no declared family.

## Cycle windows

A cycle window bounds timing.

Examples:

- same-cycle language can become `min_cycles = 0`, `max_cycles = 0`
- next-cycle language can become `min_cycles = 1`, `max_cycles = 1`
- named next-cycle language such as `next ACLK cycle` or `next HCLK rising edge` can become `min_cycles = 1`, `max_cycles = 1` too
- generic clock-edge language such as `next clock edge`, `within 2 clock edges`, or `next HCLK edge` can also become bounded `cycle_window` structure
- named bounded or ordinal edge language such as `within 2 HCLK edges` or `on the third edge of HCLK` can also become bounded `cycle_window` structure
- plural edge-of-clock language such as `within 2 edges of HCLK` can now preserve the local `clock_signal` too instead of only the window
- named diagram-style generic-edge language such as `HCLK edge T3` or `edge T3 of HCLK` can now become exact bounded windows while preserving the local `clock_signal`
- unit-first diagram-position language such as `tick T3 of HCLK` or `posedge T4 of HCLK` can now preserve the local `clock_signal` too instead of only the window
- trailing `of <clock>` shorthand-edge language such as `third posedge of HCLK` or `2 negedges of HCLK` can now preserve the local `clock_signal` too instead of only the edge and window
- next-edge language such as `next falling edge` can become `min_cycles = 1`, `max_cycles = 1`
- shorthand edge language such as `next posedge` or `next negedge` can become `min_cycles = 1`, `max_cycles = 1`, while also preserving the named clock edge as `rising` or `falling`
- bounded tick language such as `within 2 ticks` can become `max_cycles = 2`
- bounded edge language such as `after 3 falling edges` can become `min_cycles = 3`, `max_cycles = 3`
- diagram-style position language such as `tick T3` or `posedge T4` can become exact bounded windows too
- explicit later language such as `two cycles later` can become `min_cycles = 2`, `max_cycles = 2`
- ordinal edge language such as `on the third rising edge` can become `min_cycles = 3`, `max_cycles = 3`
- bounded language such as `within 2 cycles` can become `max_cycles = 2`

Cycle windows are useful because an unbounded temporal statement is weaker than one with explicit timing.

Validation can report temporal rules that still lack cycle-window grounding.
That is not always fatal, but it is an honest quality signal.

The KG benchmark harness can now assert this temporal shape directly.
Representative AXI, APB, and AHB timing fixtures lock clock/edge grounding, cycle windows, actor-grounded consequents, compound antecedents, and `HandshakeComplete` predicates at the `SemanticIR` and `IntentIR` layers instead of relying only on aggregate validation metrics.
The benchmark suite also locks the negative surface: a clock-grounded and actor-grounded temporal rule with no explicit cycle-window bound must still produce cycle-window rescan guidance, because actor and clock grounding do not imply a latency bound.
The complementary bounded fixture proves the reverse too: once a local temporal rule says `within 2 cycles`, the canonical rule keeps that max-cycle window and validation does not ask for cycle-window replay.
These checks keep the replay lanes separate: missing cycle bounds are not reported as missing actor or clock grounding when those dimensions are already known.
The actor-grounding mirror fixture keeps the other side sharp too: a bounded, clock-grounded rule that lacks actor grounding reports actor replay guidance without also reporting cycle-window or clock-grounding debt.
The clock-grounding mirror completes the split: a bounded, actor-grounded rule that lacks a clock reports clock replay guidance without also reporting cycle-window or actor-grounding debt.
That clockless fixture also checks the canonical rule itself, so the unknown edge, actor-stability predicate, signal-stability predicate, max-cycle window, and support id are preserved through both canonical layers.
The actorless fixture mirrors that exactness by preserving clock signal, edge, asserted-value predicate, max-cycle window, and support id while actor grounding remains unresolved.
The unbounded fixture completes the shape checks by preserving clock signal, edge, actor-drive predicate, asserted-value predicate, and support id while the cycle window remains unresolved.
The bounded fully grounded fixture now locks the no-replay side too: once clock, actor, and cycle-window grounding are all present, none of the temporal replay lanes should fire.

## Polarity-relative values

Temporal values such as `ASSERTED` and `DEASSERTED` are not always logic levels.
They are polarity-relative.

For an active-low reset such as `ARESETN`, `ASSERTED` means `LOW`.
For the same reset, `DEASSERTED` means `HIGH`.
For an active-high control signal, `ASSERTED` means `HIGH`.

The temporal-conflict surface therefore normalizes assertion values through resolved signal polarity when that polarity is known.
This prevents a VLM timing diagram that observes an active-low reset as both `asserted` and `LOW`, or as both `deasserted` and `HIGH`, from becoming a false conflict.

## Handshake completion as timing

Handshake recovery is not only a semantic-role problem.
It can also become temporal structure.

When a guard contains a valid-like signal and a ready-like signal, the rule can carry:

- `HandshakeComplete`

That predicate should prefer grounded semantic roles over raw signal spelling.
If role evidence is contested or alias-dependent, the temporal rule should preserve that caveat instead of pretending the handshake was directly grounded.

## Actor-grounded timing

Graph connectivity can strengthen temporal rules.

For example:

- `ActorDrivesSignal(Requester, PSEL)`
- `ActorSamplesSignal(Completer, PSEL)`
- `ActorMaintainsSignalStable(Requester, PADDR)`

These predicates are stronger than signal-only statements because they keep actor responsibility visible.

## Temporal conflicts

If two rules assert incompatible values for the same signal under the same relevant context, `specforge` should surface a temporal conflict.

Temporal conflicts preserve:

- conflict id
- clock signal
- edge
- antecedents
- cycle window
- signal name
- phase
- conflicting values
- supporting rule ids
- supporting statement ids
- automation confidence

The goal is not to hide contradictions.
The goal is to make them precise enough that the user can inspect them.

The KG benchmark harness can assert temporal conflicts directly.
That means a fixture can prove not only that a conflict exists, but also which signal, phase, clock/edge context, antecedents, values, rule ids, and statement ids make up the contradiction.

## Polarity-aware values

Temporal value comparison is polarity-aware.

`ASSERTED` and `DEASSERTED` are not universal aliases for `HIGH` and `LOW`.

For example:

- if polarity is unknown, `ASSERTED` remains assertion-level meaning
- if a signal is active-high, `ASSERTED` can compare as `HIGH`
- if a signal is active-low, `ASSERTED` can compare as `LOW`
- if a signal is active-low, `DEASSERTED` can compare as `HIGH`

This is especially important for resets such as `ARESETN` or `rst_n`, and for non-reset control signals when the current document explicitly grounds the assertion level.
For example, `CS_N is asserted when LOW` is enough local evidence to treat `ASSERTED` as `LOW` for `CS_N`; the `_N` suffix alone is not.

## Prior-guided timing

The cross-document prior store can help with timing phrase recovery.

For example, if validated documents taught that a phrase like `one beat later` indicates a one-cycle window, a new document can use that prior when the same phrase appears locally.
The KG benchmark now checks that prior-guided recovery as exact rule shape too: the recovered rule must keep local clock grounding, edge, one-cycle window, asserted-value predicate, and supporting statement id.
The paired no-prior fixture checks the negative side: the same local text still preserves clock, edge, asserted value, and support, but the cycle window stays unresolved without the learned phrase prior.
The prior-guided fixture also locks validation behavior: once the prior supplies the one-cycle window on a locally clocked rule, cycle-window and clock-grounding replay guidance should not fire.
The no-prior mirror keeps that validation lane split: the missing prior leaves cycle-window replay visible, but it must not become clock-grounding debt.

The safety rule is still strict:

- the timing phrase must appear in the current document
- the prior can recover a cycle window
- the prior cannot invent a timing rule that has no local grounding

## Standard LTL/MTL notation

A temporal rule is not a SpecForge invention — it is the shape the hardware-verification
world has used for decades. When researchers mine timing properties from traces or RTL
(Pnueli's temporal logic; tools like GoldMine and Texada), the unit they produce is a
**`G(antecedent → consequent)`** template with "holes" — *globally* (`G`, i.e. on every
clock tick), if the antecedent holds, then the consequent must follow. SpecForge's
`temporal_rules` **are** that template; they were just stored in a typed Rust shape rather
than written in the notation.

So SpecForge can render any mined rule back into that standard notation. For
*"PBUSER must be valid when PSEL, PENABLE, and PREADY are asserted"* it produces:

```text
G( PSEL==ASSERTED & PENABLE==ASSERTED & PREADY==ASSERTED -> X (drive(Completer,PBUSER) & PBUSER==VALID) )
```

Read it as: *always, if PSEL, PENABLE, and PREADY are all asserted, then on the next tick
(`X`) the Completer drives PBUSER and PBUSER is valid.* The pieces map one-to-one onto the
typed rule:

- **`G( … )`** — the rule holds on every rising (or falling) edge of the clock; the clock
  signal and edge are shown alongside the formula, since they are the domain `G` ranges over.
- **`->`** — the `antecedent → consequent` implication.
- **`X`** — *next tick* (the pre-tick guard implies the post-tick result). When the rule has a
  **cycle window**, `X` becomes the bounded *eventually* `F[min,max]` of Metric Temporal
  Logic (e.g. `F[1,2]` = "within one to two cycles"); an open upper bound prints as `∞`.
- **atoms** — `sig==VALUE`, `drive(actor,signal)`, `stable(…)`, `sample(…)`,
  `handshake(valid,ready)` — one per typed predicate. A rule with no antecedent renders as a
  plain invariant, `G( … )`.

Why bother? Because that notation is the *lingua franca* of formal verification: a property
in this form can be read by a verification engineer, checked by a model checker, or lowered
to PSL/SVA assertions. Grounding the mined rules in it makes SpecForge's timing intent
**portable and verifiable**, not just inspectable. The rendering is **derived on demand**
(`crate::ir::temporal_ltl::temporal_rule_to_ltl`) and never stored in the IR, so it adds no
weight to the artifacts; turning these formulas into checkable PSL/SVA assertions is the
next step, owned separately so it can align with the downstream FSMGen contract.

## What users should inspect

When debugging timing, inspect:

- `temporal_rules`
- `temporal_conflicts`
- `cycle_window`
- clock signal and edge
- antecedent and consequent predicates
- actor-grounded predicates
- `HandshakeComplete` predicates
- validation metrics such as `temporal_rules_with_cycle_window`, `temporal_rules_with_actor_grounding`, and `temporal_rules_with_handshake_completion`

The aim is to turn timing prose into typed, inspectable obligations without fabricating precision the source document does not justify.
