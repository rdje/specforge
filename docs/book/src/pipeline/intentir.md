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
- structured registers, including distinct register/field access and direct table provenance
- exact protocol observations: serial-frame fields, operation branches, protocol states, and interface-edge
  timings
- carried conflicts
- carried residual decisions

This is the stage the rest of the project is trying to reach.
Everything earlier exists to make this artifact strong, inspectable, and reusable.

## Canonical authority and proof

Current IntentIR is schema 2. Its JSON shape, provenance strings, or stored hashes do not authorize it. Canonical
load, serialization, persistence, validation backannotation, NLI demotion, and adapter construction first verify
the complete cumulative SemanticIR proof, independently rebuild IntentIR with the current registered
implementation, replay any closed mutations, and compare every public field exactly.

The ledger preserves the verified cumulative SemanticIR ledger as an exact ordered prefix, then appends root and
per-record claims for all 49 public fields across nine rule families. Every IntentIR claim depends on an exact
registered replay over the entire upstream claim graph. Thirty fields are byte-for-byte SemanticIR carries: the
actor/interface graph except for the filtered `actors` collection, plus the semantic collections except for
`actor_contracts`. Every root and record on those exact-carry surfaces also cites a direct SemanticIR claim with
identical conclusion bytes.

The remaining fields are typed projections, not weaker unnamed carries. `actors` is filtered to remove
unsupported inferred phantoms. Identities, summaries, behaviors, constraints, assumptions, transactions, actor
relations, temporal invariants, and residuals are deterministically synthesized. `actor_contracts` has its own
projection family because the optional NLI gate may conservatively remove a contract. That gate is demotion-only:
it may preserve contract order, remove contracts, and append one matching `nli_unentailed_...` residual per
removal; it cannot add or reorder contracts, rewrite prior residuals, or touch another field. Validation may
change only `validation_reports`. Both mutations must replay the independently rebuilt predecessor and extend
proof before canonical serialization or writing succeeds. NLI authorization is transactional: a failed replay
returns the original in-memory IntentIR unchanged.

Schemas older than 2 are inspection-only and cannot feed an adapter. A current proofless artifact, a stale
ruleset, a field edit with a recomputed digest, an unauthorized mutation, or a future schema fails closed.
Repository-owned paths are resolved and normalized before verification, so moving the repository does not alter
otherwise identical authority. The retained population has 24 proof-current IntentIR chains whose migration
changed no pre-existing public field; 54 historical chains remain explicitly unmeasurable behind legacy
SemanticIR. No synthetic proof is assigned to them.

`IntentIR.register_records` is a lossless clone of the canonical SemanticIR register surface. Optional
register-level access stays separate from optional field-level access, and structured-table support remains in
`supporting_table_ids` alongside the independent statement-provenance list. The product stage does not fill an
absent access mode or replace missing provenance with a guessed source.

`IntentIR.timing_constraints` is lossless in the same way. Units recovered from an explicit table cell or the
closed caption-wide unit grammar remain source spelling, and direct structured authority remains in
`supporting_table_ids`. IntentIR does not default a missing unit, reinterpret it, or substitute a table id into
`supporting_statement_ids`.

The timing record also carries its executable-intent disposition unchanged. A `non_applicable` decibel-domain
record is still inspectable in IntentIR—with its original values, provenance, reason, first failing boundary,
and replay route—but it is not a canonical digital timing fact and cannot have created a SemanticIR temporal
rule. Canonical and legacy records continue to omit the field. This lets downstream consumers distinguish
physical channel evidence from executable intent without re-parsing parameter names or units.

Legacy section-derived `SemanticIR.phases` do not become IntentIR behavior or actor responsibilities. The old
projection turned each synthetic section summary into a behavior assigned to every retained actor and used phase
overlap to preserve pure-inferred actors. Corpus measurement found 2,238 such behaviors and 48 such actors with
no executable adapter value. Populated old artifacts remain loadable, but current IntentIR treats the collection
as audit-only compatibility data. Typed `SemanticIR.transaction_phases`, named transactions, contracts, and
constraints remain independent.

Legacy generic `SemanticIR.gates` receive the same compatibility treatment. Their historical producer copied a
whole cue-bearing sentence without parsed condition, effect, actor role, or action; IntentIR then assigned each
sentence to every retained actor and made an interface-coupled constraint when a related interface happened to
be mentioned. Across the retained corpus that projection created 21,206 deduplicated behaviors / 642,401 actor
assignments and 5,974 constraints / 366,087 interface assignments, with zero rendered-ISF, renderability,
lowering-status, or executable-count value. Populated old gate records still load and round-trip, but current
IntentIR ignores them. Source evidence, technical invariant/constraint projection, and independently typed
`conditional_rules` / `temporal_rules` remain active; no real conditional meaning depends on the legacy copy.

## How protocol observations cross the product boundary

`IntentIR` carries the four typed protocol-observation collections from `SemanticIR` unchanged:
`serial_frame_fields`, `protocol_operations`, `protocol_states`, and `interface_edge_timings`. The projection preserves
record order, identifiers, optional values, and supporting-statement provenance. It does not sort, filter,
reinterpret, or complete the observations. Empty collections are omitted when serialized and default to empty
when older artifacts are loaded.

These records are canonical product data, but they are not automatically executable behavior. A named protocol
state without transitions, guards, initial-state identity, or encoding is still incomplete; frame, operation,
and edge records likewise do not necessarily provide every wire, value, activation, or storage binding an ISF
step requires. The adapter must account for that incompleteness explicitly rather than inventing the missing
meaning. `specforge validate <intent-ir>` reports one count for each collection so a reviewer can verify the
three-stage projection before inspecting adapter disposition.

For example, if the source states that `ALPHA[3:0]` belongs to the `lilac` phase and travels “from the
orchid to the quartz,” the carrier preserves those exact opaque strings and the width `4`. Renaming all three
source terms changes only those strings—not the record shape, admission decision, or downstream disposition.
If the source merely mentions a bit range without binding it to a frame/packet or named phase, no frame record
is manufactured.

`IntentIR` initially carries `SemanticIR.actor_contracts` unchanged, including verified figure-provenance
contracts and their residual dispositions. It does not reinterpret a timing trace, restore a lane rejected by
semantic grounding, or upgrade verifier disagreement. When explicitly requested, the typed NLI gate may only
demote a source sentence that does not entail its rendered contract claim; the removed contract becomes a
matching residual rather than disappearing. This makes the EvidenceIR → SemanticIR boundary the only place a
typed figure becomes canonical temporal intent, while IntentIR remains a faithful, conservatively filterable
product projection.

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

Signal-name fragments never create a transaction. In particular, request/acknowledge, valid/ready, full/empty,
and similar-looking identifier pairs carry no built-in behavioral template. They can participate in a
transaction only after current-document structure and typed semantic evidence establish the role. Alpha-renaming
the document's declarations and occurrences therefore renames symbols without changing transaction admission.

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

A transaction's section usually has **subsections**, and that is where the signal-rich
detail tends to live. APB's "Write transfers" section, for example, is mostly an
introduction — its own paragraphs barely name a signal beyond the clock — while the
signals that actually make up a write (`PADDR`, `PWDATA`, `PWRITE`, `PENABLE`, …) are
described one level down, under "With no wait states" and "With wait states". A
transaction genuinely *spans* those subsections, so SpecForge scopes its signal set to
the whole subtree: the defining section **and** every section whose number nests under it
(`3.1.1`, `3.1.2`, … under `3.1`). This is decided purely by the document's own section
numbering — no chip-specific list — and it keeps each transaction's boundary exact,
because nested numbers do not overlap: a write transaction (`3.1.x`) and a read
transaction (`3.3.x`) sit in disjoint subtrees, so a read never picks up write-only
signals like `PWDATA`. The effect is concrete — APB's `write_transfer` grows from a lone
`PCLK` to its full ten-signal set (and `read_transfer` to its seven, correctly *without*
the write-data signals), and AXI transactions whose detail lives in subsections, such as
`atomic_transaction`, recover a real signal set instead of an empty one. None of this
changes the emitted `.isf` — the signal set is recorded as `IntentIR` metadata, and the
adapter lowers a transaction's *steps*, not its membership.

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
noisier than a section heading, the recognizer is deliberately strict. It rejects:
grammatical filler (`the phase`, `this phase`, `each phase`); counting and ordering words
(`first phase`, `two phases`, `next phase`); a head noun used only as a modifier (`data
transfer phase` is about the *data* phase, not a "transfer" phase); and anything that
crosses a sentence or comma boundary.

A valid qualifier must also have positive phrase authority somewhere in the document. A phase heading,
`Phase 2`, `During the address phase`, `the Setup phase ... occurs`, or `the transaction has an address phase`
supplies that authority. A bare compound or accidental adjacency does not: `edge phase errors`, `dynamic phase
tolerance`, `jitter, phase noise`, `Lanes, Phase 2`, and `transfers are called phases` are rejected. Once one
occurrence authorizes a name, all of its grammatical mentions remain supporting evidence; a signal-bearing
`address phase signal` row is therefore preserved rather than forced to repeat the authority syntax. The rule is
English phrase structure only, with no built-in protocol or phase-name list.

This is *recognition only* at this stage: SpecForge records which phases exist and which
sentences name them, but it does not yet sequence a transaction's signals through them —
that ordered, step-by-step body is a later step built on top of this list. The recognized
phases live on the `SemanticIR`, and `specforge validate <semantic-ir>` surfaces them at a
glance: a `transaction_phases` count and, when the document names any, a
`semantic_transaction_phase_inventory` Info finding listing the phase names with
`related_ids` for review. A document that names no phases emits no finding — absence is
silence, not an event.

### Grouping a transaction's signals by phase

Once SpecForge knows a transaction's signal set *and* the document's phases, it can answer a
sharper question: *which of this transaction's signals belong to its address phase, and
which to its data phase?* The answer comes, again, from the document's own words — a signal
belongs to a phase when the prose that names that phase references it ("`HNONSEC` is an
address phase signal", "the following signals must be valid during the data phase"). So an
AHB basic transfer groups as `address={HREADY}`, `data={HRDATA, HREADY, HREADYOUT, HWDATA}`.
A few honest details follow directly from grounding it this way: a signal that genuinely
spans phases (AHB's `HREADY`) appears under each — that is faithful, not a bug; and a member
the document never ties to a named phase (a clock, a write-direction flag) simply stays in
the transaction's overall signal set without a phase — an honest "unphased" residual rather
than a guess. Where a document's phase prose never names a declared signal (as on AXI and
the debug interface), the grouping is honestly empty.

Two things this grouping deliberately does **not** do, and why. It does not invent an
*order* between the phases — whether the address phase strictly precedes the data phase is
something most documents do not state unambiguously (and some, like the debug interface,
describe in an order opposite the wire order), so SpecForge treats cross-phase ordering as
an explicit residual rather than fabricating one. And it does not become part of the emitted
`.isf`: an `.isf` transaction body is an *ordered* sequence of behavioural steps, and the
downstream consumer (FSMGen) confirmed that a value-less, unordered membership set is the
wrong thing to write into that body — so the grouping is carried as **checked metadata** on
the `IntentIR`, leaving the emitted `.isf` byte-for-byte unchanged. `specforge validate
<intent-ir>` surfaces it: a `transactions_with_phase_membership` count, a
`transaction_phase_groups` total, and — when any transaction carries a grouping — an
`intent_transaction_phase_membership` Info finding that lists each transaction with its
per-phase signal split (and notes plainly that it is metadata, not lowered to `.isf`).

### Grouping a transaction's signals by channel

On the AMBA AXI/ACE family (and CHI), a transaction's "phases" are really its **channels**:
a write moves through the *write request*, *write data*, and *write response* channels, a
read through the *read request* and *read data* channels. These documents do not usually
describe those phases in `<qualifier> phase` prose — so the phase grouping above is honestly
empty for them — but they *do* declare each channel's exact signal set in a clearly captioned
table: `Table B1.1: Write request channel signals`, `Table B1.2: Write data channel signals`,
and so on. SpecForge reads that caption directly. The universal cue is the phrase **`<role>
channel signals`**: it strips the leading table number (handling both the `B1.1` and the
older `A2-2` dash form so a stray digit never leaks in) and keeps the words the document
places before *"channel signals"* as the channel's name, verbatim — `write request`, `read
data`, `snoop response`. A channel table that the PDF splits across pages (its tail captioned
only `B1.1 Continued from previous page`) is stitched back to its head by the table number,
so no signal is stranded. Because every declared signal already carries provenance back to
the table that declared it, joining the two gives each signal its channel — and grouping a
transaction's signal set by channel then falls out for free. An AXI `atomic_transaction`, for
instance, groups as `write request × 12`, `write data × 3`, `read data × 4`, `write response
× 3` — a faithful, multi-channel picture of a read-modify-write operation.

This is the deterministic, structured-first counterpart of the phase grouping: it needs no
LLM, runs on the document's own captions, and (like every surface here) carries no list of
protocol names (ADR 0006). It is deliberately **boundary-precise**: a signal earns a channel
only when *every* channel-captioned table that declares it agrees on one role. Where a
document describes the same channel from several viewpoints with inconsistent wording — the
older AXI+ACE spec lists the same write-address signals under both a `Write address channel
signals` table and per-interface `Manager / Memory Subordinate interface write channel
signals` tables — the disagreeing signals are left **unchannelled** (an honest residual)
rather than forced into a guessed channel. A document with no `<role> channel signals`
captions at all (APB, AHB, the serial debug interface) simply produces nothing here. The
channel role is the document's own caption wording; it is deliberately *not* re-interpreted
into the abstract `address`/`data`/`response` phase names (that mapping is family-specific and
would risk inventing a phase the document never named for a signal). Like the phase grouping,
it is carried as **checked metadata** on the `IntentIR`, never lowered into the emitted
`.isf` (which stays byte-for-byte unchanged), and `specforge validate <intent-ir>` surfaces it:
a `transactions_with_channel_membership` count, a `transaction_channel_groups` total, and —
when any transaction carries a grouping — an `intent_transaction_channel_membership` Info
finding listing each transaction with its per-channel signal split.

## How the actor surface stays faithful

`IntentIR` lists the **actors** (agents) a specification defines — the Manager, the
Subordinate, the interconnect, the arbiter — together with the signals each one drives and
samples. For that list to be trustworthy it has to contain *real* agents and *only* real
agents: a downstream `.isf` should not sprout a phantom module for a word the document
merely happened to use in a sentence.

A specification's prose makes that surprisingly easy to get wrong. The same English that
names a real agent ("the Subordinate drives HRESP") also throws off look-alikes:

- a sentence *fragment* mistaken for a subject — `For components…`, `is recommended…`, `Then it…`;
- a real agent with a trailing word stuck to it — `Subordinate extends`, `decoder also`;
- two agents joined by *and* — `the Subordinate and decoder read HADDR`;
- a real agent named as its *interface* — `Subordinate interface`, `Transmitter interface` — written separately from the bare agent;
- a generic role word the document only *mentions* — `controller`, `agent`, `producer` — that is never actually wired to any signal.

`specforge` cleans each of these **structurally** — by the *shape* of the language, never by a
list of chip-specific names, so the rules work on any specification (see ADR 0006):

- **fragments are rejected** before they can become an actor;
- **trailing words are stripped** so the relation re-attaches to the genuine agent;
- **"X and Y" subjects are split** so both agents get connected;
- **"X interface" is folded onto "X"** — but *only* when `X` is already a connected agent in this same document, so `Subordinate interface` merges onto the real `Subordinate` while a distinct named block such as the GIC `CPU interface` (whose `CPU` is never an agent on its own) is left exactly as it is, never conflated with a generic `CPU`;
- **mentioned-but-unwired generic role words are dropped** — if the only thing the document ever says about a `controller` is that the word appeared in a sentence (no signal it drives, no behaviour it owns, no phase or obligation that refers to it), it is generic vocabulary, not a protocol agent, so it does not earn an actor entry.

That last step is deliberately **conservative**. An agent the document genuinely discusses —
one that participates in a named phase, or that a stated obligation refers to — is **kept**
even when the pipeline has not yet wired it to a specific signal, because the goal is a
*complete* picture of the agents (with honest gaps) rather than an aggressively pruned one.
Only the pure "the word was mentioned and nothing else" phantoms are removed. The payoff is
an actor list — and therefore an `.isf` — that reflects the agents the specification actually
designs, with no junk modules and none silently invented.

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

## Portable lineage paths

The public product does not persist the workstation that created it. `IntentIR.input_artifact` and its artifact
layout serialize as repository-relative paths, while a loaded or newly built Rust value exposes current-root
absolute paths for direct use. For example, the persisted lineage pointer is:

```text
generated/semantic_ir/<document_key>/semantic_ir.json
```

Stage identity is rejected before these typed paths are resolved. A legacy absolute pointer is accepted only
through the common unique-target, in-repository rebase; missing, ambiguous, traversing, or escaping paths fail
closed. Writing or reserializing the loaded artifact returns to the relative form.

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
