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

Once a transaction is recognized and named, the next question is *what does it do,
step by step?* SpecForge fills that in only where the document grounds it, never by
guessing:

- **A transaction named after a signal value gets a grounded body.** When the name
  *is* an enumerated value of a declared signal — AHB's `idle_transfer` corresponds
  to driving `HTRANS` to `IDLE` (the spec's own transfer-type encoding) — the
  transaction's body is exactly that: `(drive HTRANS IDLE)`. That is the document's
  own definition of the transaction, so SpecForge can write it down with confidence,
  and the transaction now appears in the emitted `.isf`.
- **Everything else stays recognized but body-less, on purpose.** Most transactions
  (an AXI `atomic_transaction`, a `narrow_transfer`) describe a multi-phase sequence
  whose step-by-step membership the tool cannot yet attribute *exactly* from the
  document's structure. Rather than invent a plausible-but-wrong body, SpecForge
  keeps the transaction recognized in `IntentIR` and holds it back from the `.isf`
  as an honest gap, to be filled once the signal-set membership work grounds it.
  Nothing unfinished is silently lowered.

SpecForge also records **which signals belong to each transaction**. The answer comes
from the document itself: a transaction's signal set is the declared signals that its
own defining section talks about. AHB's "Basic transfers" section, for instance, yields
`{HCLK, HRDATA, HREADY, HREADYOUT, HWDATA, HWRITE}`; "Burst operation" yields
`{HADDR, HBURST, HSIZE}`. A signal belongs to a transaction *because the section that
defines that transaction references it* — so a shared signal like `HREADY` correctly
appears in several transfers (it genuinely participates in each), while signals a
transaction's section never mentions stay out. One subtlety matters for honesty here:
the raw text mentions more than signals — it also names *values* like `IDLE` or `INCR4`
and abbreviations like `MPMC`. Those are not signals, so SpecForge keeps only the names
the document actually declares as signals; the rest are dropped rather than passed off
as part of the transaction. (How those signals are then ordered into the document's
address/data/response *phases* is a further step that builds on the phase recognition
described below and on this membership.)

This stage also deliberately **does not** treat "everything an actor does over time"
as a transaction. Earlier, SpecForge minted a catch-all `Manager_behavior` /
`Subordinate_behavior` entry per actor — a bag of timed rules. That conflated two
different things: a *transaction* (a named interaction the spec defines) versus an
actor's *aggregate timing behaviour*. The timing behaviour is already captured, and
faithfully lowered, by the typed temporal-rule surface; re-emitting it as a fake
"transaction" only produced an invalid `.isf` clause. So those per-actor blobs are
gone — the timing semantics are preserved where they belong, and the transaction
list now contains only things the document actually calls transactions.

### Seeing a document's transactions at a glance

Because the owner's directive is to *very quickly identify* the transactions in any
chip-spec PDF, `specforge validate <intent-ir>` surfaces the recognized transaction set
directly. It reports a small set of counts — how many transactions were recognized, how
many carry a grounded signal set, how many carry a step-by-step body, how many are still
recognition-only, and the total number of signal members across them — and, when there is
at least one transaction, an `intent_transaction_inventory` Info finding that lists the
transaction names with that with-signal-set / with-steps / recognition-only split. The
finding's `related_ids` are the transaction ids, so a reviewer can drill into any single
one. A document with no recognized transactions emits no such finding: absence is not an
event, it is simply silence — the same honesty rule the message-field inventory follows.
This is pure observation off the already-built `IntentIR`; it changes nothing in the
extraction, so it can never inflate or distort what was recognized.

### Recognizing a document's transaction phases

A transaction is rarely a single instant — a protocol usually describes it as a short
sequence of *phases*. AHB and AXI talk about an **address phase** and a **data phase**,
APB about a **setup phase** and an **access phase**, the debug interface about
**address**, **data**, **response**, and **turnaround** phases. Knowing a document's phase
vocabulary is the groundwork for eventually describing each transaction *step by step*, in
the document's own terms.

SpecForge recognizes these phases the same way it recognizes transaction names: from the
document's own words, with no built-in protocol name list. It reads the prose for the
`<qualifier> phase` shape and keeps the single word the document places in front of
`phase` — `address`, `data`, `setup`, `access`, `response`, `turnaround`. Because prose is
noisier than a section heading, the recognizer is deliberately strict, and rejects:
grammatical filler (`the phase`, `this phase`, `each phase`); counting and ordering words
(`first phase`, `two phases`, `next phase`); a head noun used only as a modifier (`data
transfer phase` is about the *data* phase, not a "transfer" phase); and anything that
crosses a sentence boundary. What survives is a clean list of the phases the document
actually names — on the AMBA buses, exactly their address/data and setup/access phases.

This is *recognition only* at this stage: SpecForge records which phases exist and which
sentences name them, but it does not yet sequence a transaction's signals through them —
that ordered, step-by-step body is a later step built on top of this list. The recognized
phases live on the `SemanticIR`, and `specforge validate <semantic-ir>` surfaces them at a
glance: a `transaction_phases` count and, when the document names any, a
`semantic_transaction_phase_inventory` Info finding listing the phase names with
`related_ids` for review. A document that names no phases emits no finding — absence is
silence, not an event.

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
