# IntentIR

`IntentIR` is the canonical backend-independent product surface.

## What belongs in `IntentIR`

- actor responsibilities
- canonical interface inventory
- behavior records
- **named transactions** — the transactions the document itself defines (reads, writes, transfer types, operations)
- constraints
- assumptions
- system contract
- infrastructure signal source/distribution status
- carried conflicts
- carried residual decisions

This is the stage the rest of the project is trying to reach.
Everything earlier exists to make this artifact strong, inspectable, and reusable.

## How transactions are recognized

A protocol or platform specification is, in large part, a *catalogue of supported
transactions* — an AXI read or write, an AHB transfer type, an APB read/write,
an SWD operation. For the downstream `.isf` to be faithful, `IntentIR` has to carry
those transactions as the document defines them.

It would be tempting to teach the tool the names directly ("if you see `HTRANS`,
emit an `ahb_transfer`"). SpecForge deliberately does **not** do that: a hardcoded
name list is blind to the 101st protocol and rots the moment a spec is revised, and
it fires by accident on any text that merely *mentions* those names. Instead,
recognition is **structural and universal** — it reads the document's own structure,
so it works on *any* chip-spec PDF and is tied to none (this is the PDF-independence
invariant — universal grammar, no name lists, ADR 0006).

Two cues, both taken from the document itself:

- **Cue A — section headings that name a transaction.** A heading like
  `3.1 Write transfers`, `Chapter 10 Exclusive Transfers`, or
  `B4.2.1 Successful write operation` names a transaction. SpecForge strips the
  section number and furniture, keeps the `<qualifier> transfer/transaction/operation`
  phrase, and rejects sub-topics that only *talk about* a transaction
  (`Write transaction dependencies`, `Exclusive Transfer restrictions`). The
  transaction's name is the document's own words — e.g. `write_transfer`.
- **Cue B — the document's signal-keyed enumeration tables.** A table that maps a
  declared signal to its values (`HTRANS` → `IDLE`/`BUSY`/`NONSEQ`/`SEQ`) corroborates
  a recognized transaction: when a transaction's name matches one of those enumerated
  values, the keyed signal is attached and the recognition is recorded with higher
  confidence.

For example, the AHB specification yields `basic_transfer`, `burst_operation`,
`exclusive_transfer`, `idle_transfer`, `locked_transfer`, `secure_transfer`, and
`waited_transfer` — all from its own headings — and `idle_transfer` is additionally
corroborated by the `HTRANS` enumeration. A document that defines no transactions
(or a non-specification document) yields none.

At this stage the transactions are **recognized and named**; their step-by-step
bodies and full signal sets are filled in by later pipeline work, and a
recognition-only transaction is held back from the `.isf` until it has a body, so
nothing unfinished is silently lowered.

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

Then the adapter can lower that into:

- `.isf` (the single adapter target)

`.fsm` and HDL lowering (SystemVerilog/Verilog/VHDL) are out of scope — FSMGen consumes `.isf` and owns scheduling, `.fsm`, and HDL downstream.

This ordering is what keeps the project from collapsing into backend-shaped shortcuts too early.
The product boundary is supposed to be reusable implementation intent, not one specific code generator.

For `.isf`, SPECFORGE keeps a pinned FSMGEN reference checkout under `subs/fsmgen/`.
FSMGEN is the downstream consumer of `.isf` (it owns scheduling, `.fsm`, and HDL). That checkout is useful for ISF target syntax, strict-mode boundaries, bounded capability/check/semantic JSON surfaces, stable diagnostic codes, support accounting, examples, and its own live book, but it does not move the product boundary.
FSMGEN tells the adapter what `.isf` forms are real; `IntentIR` still decides what document meaning is justified.

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

## Closed task trees — how each was implemented and verified

### `R6-INTENT-HARDENING` — close mutation-testing gaps in `intent.rs` builders

Mutation testing on the `intent.rs` builder functions
(`build_intent_actors`, `build_behaviors`, `build_constraints`,
`build_assumptions`, and the `overlaps` helper) surfaced 10
missed mutants — assertions that didn't pin down the builder's
exact contract. This tree closed those gaps with regression-only
tests. Verified by cargo-mutants delta-to-zero on the targeted
symbols + `scripts/run_ci.sh`. *Authoritative tracking:*
`docs/tasks/R6-INTENT-HARDENING.md`.

### `R6-CONVERGE-HARDENING` — close zero-coverage gaps in `converge.rs`

The converge command and several command-module struct fields
were populated in production but had zero test coverage. This
tree added regression-only assertions for those fields and for
the converge command's behaviour-critical paths. Verified by
cargo-mutants delta-to-zero on the targeted symbols +
`scripts/run_ci.sh`. *Authoritative tracking:*
`docs/tasks/R6-CONVERGE-HARDENING.md`.
