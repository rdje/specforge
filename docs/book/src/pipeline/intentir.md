# IntentIR

`IntentIR` is the canonical backend-independent product surface.

## What belongs in `IntentIR`

- actor responsibilities
- canonical interface inventory
- behavior records
- constraints
- assumptions
- system contract
- infrastructure signal source/distribution status
- carried conflicts
- carried residual decisions

This is the stage the rest of the project is trying to reach.
Everything earlier exists to make this artifact strong, inspectable, and reusable.

## What makes it canonical

`IntentIR` is the stage where the pipeline tries to present the best stable typed intent surface that survived:

- extraction
- arbitration
- validation

It is still allowed to be incomplete.
It is not allowed to pretend uncertainty does not exist.

That second rule is important.
Canonical does not mean "perfect".
Canonical means "the best stable typed product surface the pipeline can justify honestly right now".

## Why adapters come after `IntentIR`

Because the project boundary is not "emit one backend format".

The project boundary is:

- recover honest, typed, reusable implementation intent

Then adapters can lower that into:

- `.fsm`
- `.isf` (planned)

HDL lowering (SystemVerilog/Verilog/VHDL) is out of scope — downstream toolchains own HDL generation.

This ordering is what keeps the project from collapsing into backend-shaped shortcuts too early.
The product boundary is supposed to be reusable implementation intent, not one specific code generator.

For `.fsm`, SPECFORGE keeps a pinned FSMGEN reference checkout under `subs/fsmgen/`.
That checkout is useful for target syntax, strict-mode boundaries, bounded capability/check/semantic JSON surfaces, stable diagnostic codes, support accounting, examples, generated-HDL validation, and its own live book, but it does not move the product boundary.
FSMGEN tells the adapter what target forms are real; `IntentIR` still decides what document meaning is justified.

## What a good `IntentIR` artifact looks like

- high direction and width coverage
- grounded actor-relative connectivity
- explicit system-contract handling
- explicit infrastructure-signal sourcing, distribution, and topology status
- typed timing and behavioral constraints
- low residual count
- conflicts surfaced rather than hidden

## What should already be settled by this stage

By the time the pipeline reaches `IntentIR`, the artifact should already have done the work of:

- preserving source provenance
- gathering evidence
- arbitrating competing semantic meanings
- deciding what can be promoted safely

So `IntentIR` is not where the project should first discover the meaning of the document.
It is where the promoted result is packaged as the canonical product surface.

## What is still allowed to remain imperfect

An `IntentIR` artifact can still contain:

- assumptions
- carried conflicts
- residual decisions
- incomplete coverage

That is not a bug by itself.
For `specforge`, honest incompleteness is better than fabricated certainty.

Validation may also attach negative-knowledge caution to this surface.
If a carried conflict or residual packet class matches a learned corpus-memory failure pattern, `IntentIR` validation can report `negative_knowledge_prior_matches`.
It can also report `intent_negative_knowledge_rescan_guidance` plus rescan/corroboration metrics, giving downstream loops a deterministic target list for stronger local rechecking.
That is not a canonical correction; it is a diagnostic and routing reminder that the current artifact is showing a known failure shape.

## Why users should care about this stage

If you want to understand what `specforge` currently believes about a specification, `IntentIR` is the place to look.

It is the artifact that:

- validation scores directly
- adapters consume downstream
- prior learning harvests from upstream
- project-level snapshots summarize for live baselines

That is why `IntentIR` is the main public product surface today.
