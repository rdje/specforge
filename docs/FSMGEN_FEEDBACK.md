# FSMGEN Feedback From SPECFORGE

## Scope update (2026-05-18) — SPECFORGE emits only `.isf`

SPECFORGE's single adapter target is now `.isf`. SPECFORGE no longer emits
`.fsm` itself and never emitted HDL; FSMGEN consumes `.isf` and owns
scheduling, `.fsm`, and HDL downstream. The relationship is therefore
strictly `SPECFORGE IntentIR → .isf → FSMGEN`.

The `.fsm`-language-feature suggestions below are retained as historical
context and may still be useful to FSMGEN, but the **active, load-bearing
asks for SPECFORGE are the ISF-facing ones**: strict-mode `.isf` acceptance,
the capability manifest, stable diagnostic codes, JSON check, normalized
semantic JSON, and reset/clock/contract metadata as they apply to `.isf`.
Wherever this document says "`.fsm` adapter", read it as historical; the
current adapter is `.isf`.

## Clarity request (2026-05-29) — actor-local `(types)` ↔ `(enums)` same-name relationship

> **RESOLVED `2026-05-29`** — FSMGen answered in upstream commit `c0b7eaa7`
> (`ISF-ENUM-TYPE-RELATIONSHIP-CLARITY.2`), in
> `subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md` (§ Actor-Local
> `(types)`↔`(enums)` Relationship Clarity), locked by
> `subs/fsmgen/t/1378-isf-enum-type-relationship.t`. Answer: `(enums (NAME …))`
> is **not** a `(type NAME)` alias; to use an enum name as a width-bearing
> type, **co-declare** `(types (type NAME (bits k)))` (accepted, required,
> not a redeclaration conflict; `k = ceil(log2(member_count))` is an accepted
> choice — the width is not cross-validated); unreferenced
> `(types)`/`(enums)`/`(constants)` are valid. This **validates SpecForge's
> existing build** (it already emits both with that `k`) and corrects the
> earlier "enums-standalone" reading. Ingested by pinning `subs/fsmgen` to
> `c0b7eaa7` (`ISF-SYMBOL-SURFACE-EMIT.1`); emission is `ISF-SYMBOL-SURFACE-EMIT.2`.

Reviewing the ISF type/enum/aggregate surface at pin `88a7af9c`
(`ISF_PUBLIC_INTERFACE_CONTRACT.md` actor-local-declarations section ~L1792–1844,
`docs/book/src/13j-type-enum-aggregate.md`) for a SPECFORGE `.isf`
symbol-emission feature, one rule is not stated explicitly and would help
downstream emitters lower a recovered symbol inventory deterministically:

- **Does `(enums (NAME (M0 0) (M1 1)))` by itself establish a type named
  `NAME`** — i.e. is `NAME` then usable as `(type NAME)` on a width-bearing
  interface port / transaction port / storage var? `13j` always declares
  enums standalone (no co-declared `(type NAME)`), and the contract says
  enums are "preserved as `+enums`", but neither states whether the enum
  name is also a scalar type alias.
- **Is co-declaring `(types (type NAME (bits k)))` AND `(enums (NAME ...))`
  for the same `NAME`** a redeclaration conflict, a harmless redundancy, or
  required? The contract's fail-closed list (unknown aliases /
  `(width)`+`(type)` conflicts / aggregate-outside-storage / …) and the book
  do not address same-name `(type)`+`(enums)`, and no duplicate/redeclaration
  rule is stated for actor-local declarations.
- (Secondary) It would also help to state explicitly that actor-local
  `(types)` / `(enums)` / `(constants)` declarations need **not** be
  referenced to be contract-valid — currently only derivable from the
  fail-closed list's silence on unreferenced declarations.

Why it matters to SPECFORGE: when SPECFORGE recovers an enum-like symbol it
derives both a member list and a backing bit-width
(`ceil(log2(member_count))`). Without a stated rule an emitter cannot know
whether to emit `(enums (NAME ...))` alone or also a backing
`(type NAME (bits k))` without risking a redeclaration. SPECFORGE's current
reading (**pending FSMGEN confirmation**) is *enums-standalone*: emit
`(enums (NAME ...))` only, reserving `(types (type NAME ...))` for non-enum
scalar aliases. A one-line statement in
`ISF_PUBLIC_INTERFACE_CONTRACT.md` / `13j` confirming the type↔enum name
relationship + the duplicate-declaration rule would resolve it.

This is a **documentation-clarity request, not a bug report**: no SPECFORGE
`.isf` is broken (SPECFORGE does not yet emit the symbol surface — that work
is deliberately gated on this clarity, per the residual-honesty doctrine and
the "parser-acceptance ≠ support" principle).

## Answer (2026-06-04) — `min > 1` window confirmation: integer-literal bounds; SPECFORGE guarantees `MIN >= 1`

Answering FSMGEN's gating question in `subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`
("2026-06-04 (follow-up)", the `min > 1` windows slice): *"are SPECFORGE's mined `cycle_window`
bounds always integer literals with `MIN >= 1`, or can `MIN` be `0`?"*

- **Always integer literals.** `cycle_window = { min_cycles: Option<u32>, max_cycles: Option<u32> }`
  (`crates/specforge/src/ir/semantic.rs`) — both bounds are concrete non-negative integers parsed
  from cycle counts (`parse_cycle_count_value`); there is **no symbolic / parameter form**.
  (`None` means the bound was not stated in the source.)
- **`MIN = 0` exists in the mined model, but SPECFORGE never emits it as a `|-> ##` consequent.**
  Two sources of a zero lower bound, both handled on SPECFORGE's side:
  1. the degenerate **same-cycle `[0,0]`** window ("same/this/current cycle") — already routed to
     a **residual** (your `(within 0)` rejection: a same-cycle obligation is not a
     bounded-eventually); never emitted;
  2. a literal **`0`-to-`N` range** (`min=0, max=N`) — semantically "from the anchor, eventually
     within `N`", i.e. your **`(monitor (within S N))`** anchored `F[0,N]` form, not a
     `|-> ##[0:N]` consequent.
- **SPECFORGE's emission commitment.** For the antecedent→consequent bounded form SPECFORGE will
  emit `(assert (=> <ante> (within <cons> MIN MAX)))` **only with `1 <= MIN <= MAX`**. A `0`
  lower bound is resolved on SPECFORGE's side — `[0,0]` → residual, `[0,N]` → `(monitor (within S
  N))`.

**So: lock `(within B MIN MAX)` to `1 <= MIN <= MAX`.** SPECFORGE will not emit `(within B 0
MAX)`, so redirecting/rejecting `MIN = 0` (your instinct) is correct and matches SPECFORGE's
residual-honesty. The `min > 1` slice is unblocked from SPECFORGE's side on that contract. (The
`(stable …)` half is already shipped; SPECFORGE will re-pin past `6700fbb4` and migrate its
stability obligations off residuals as a separate owned integration once the `min > 1` slice
lands too.)

## Suggestion (2026-06-04) — first-class LTL/MTL temporal properties in ISF

This extends §4 ("Temporal And Stability Contracts") with a concrete, named ask and a
proposed ISF shape, at SPECFORGE's request.

**Context.** SPECFORGE mines temporal behavior as typed `temporal_rules` that are, by
construction, **linear-temporal-logic** properties: each is the `G(antecedent → consequent)`
template (Pnueli, FOCS 1977) — the same shape the specification-mining literature (GoldMine,
Texada) uses — with the "next tick" as LTL `X` and a bounded `cycle_window` as the **Metric
Temporal Logic** bounded-eventually `F[min,max]`. SPECFORGE already renders these to standard
LTL/MTL (`crate::ir::temporal_ltl`) and chose LTL/MTL deliberately over CTL/TLA+ (SPECFORGE
ADR-0005: *mine, don't model-check*). ISF already carries the special case
`(contract <n> (eventually <signal> (within <N>)))` — that is exactly an MTL bounded-eventually
`F[0,N]` with an empty antecedent and a single `value` consequent.

**Ask.** Would FSMGEN consider generalizing that to a **first-class LTL/MTL temporal-property
construct in ISF** — the full `G(antecedent → [X | F[min,max]] consequent)` template — so
SPECFORGE can lower its mined temporal rules *directly into ISF* instead of (a) flattening
them to residual decisions or (b) emitting a separate SVA artifact outside the
`IntentIR → .isf → FSMGEN` path? If ISF expresses the general template, the spec→checkable-
property loop closes inside the existing handoff, and FSMGEN owns whether the property lowers
to a generated assertion, stays checked metadata, or both (consistent with §4's "even if
FSMGEN initially treats these as metadata").

**Proposed ISF shape (semantic shape only — exact syntax is FSMGEN's choice).** Generalize the
existing `eventually` contract to:

```text
(temporal-rule <name>
  (clock <clk> (edge rising|falling))           ; the tick domain; G ranges over these edges
  (antecedent <pred> ...)                       ; conjunction; empty ⇒ a plain invariant
  (consequent
    (window <min> <max>)                        ; optional MTL bound; absent ⇒ next cycle (X)
    <pred> ...))                                 ; conjunction
;; <pred> ::= (value  <signal> <VALUE>)         ; VALUE ∈ HIGH|LOW|ASSERTED|DEASSERTED|VALID|<sym>
;;          | (stable <signal>)                 ; $stable-like across the tick
;;          | (handshake <valid> <ready>)       ; both asserted at the edge
```

This maps **1:1** onto SPECFORGE's `TemporalRuleRecord` (clock + edge, antecedent predicates,
consequent predicates, cycle window) and onto the LTL/MTL form
`G( <ante> -> X|F[min,max] <cons> )`. The current `(eventually s (within N))` is the
sub-case `(temporal-rule … (antecedent) (consequent (window 0 N) (value s VALID)))`. FSMGEN
may, of course, keep the flat `eventually` alias and add the general form alongside it. Drive-
and sample-predicates SPECFORGE also mines (`actor drives`, `is sampled`) are intentionally
*not* in the proposed grammar — they are not value-over-time properties and SPECFORGE would
keep them as IntentIR metadata, not ISF temporal properties.

**Status / non-bug.** This is a **feature suggestion**, not a bug report (no SPECFORGE `.isf`
is broken; SPECFORGE does not emit temporal rules into ISF today). The alternative SPECFORGE
is weighing — a SPECFORGE-side `.isf`/IntentIR → PSL/SVA export — is logged as the deferred
tree `TEMPORAL-RULE-SVA-RENDER`; **if** ISF gains native LTL/MTL the SVA export may be
unnecessary. The decision between the two paths is open on SPECFORGE's side; this suggestion
records the FSMGEN-native option and a concrete shape so FSMGEN can weigh in.

> **ANSWERED + ACTED ON `2026-06-04`.** FSMGEN responded
> (`subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`, "2026-06-04: First-Class LTL/MTL Temporal
> Properties — Already Generalized In The Verification Family"): **yes, already shipped** as a
> generalization — the `(contract … (eventually …))` clause was removed and replaced by the
> compositional `(assert/assume/cover …)` verification family (decisions `0008`/`0009`), which
> expresses the full `G(ante → X/F[min,max] cons)` template and is strictly more general.
> SPECFORGE re-pinned `subs/fsmgen` to `43b29f5c` and migrated its bounded-eventually emission
> `(contract … (eventually s (within N)))` → `(assert (monitor (within s N)))`
> (`FSMGEN-ASSERT-MIGRATE`; empirically strict-valid). The SpecForge-side SVA export
> (`TEMPORAL-RULE-SVA-RENDER`) is **retired as superseded**. FSMGEN flagged two narrower spots —
> arbitrary `min > 1` windows and a `(stable …)` predicate; SPECFORGE *does* mine both, so those
> stay SPECFORGE residuals until FSMGEN adds the primitives (being requested separately).

## Question / feature request (2026-06-16) — lowering a transaction's phase membership without fabricating drive VALUES or step ORDER

> **ANSWERED `2026-06-16`** — FSMGEN responded in `subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`
> (§ "2026-06-16: Transaction Phase Membership Without Fabricated Values Or Order", commits
> `ISF-SPECFORGE-PHASE-MEMBERSHIP-RESPONSE.1`/`.2`; SPECFORGE re-pinned `subs/fsmgen`
> `8c39827f → 030f8c273` under `FSMGEN-REFRESH-INTEGRATE-3`). **Short answer: do not fabricate either
> value or order; no immediate FSMGen code change is needed; the right *future* feature is checked
> transaction phase-group metadata in ISF, under its own FSMGen tree.** Per question: **(1) value-less
> output participation → NO** — `(drive S)` is value-bearing behavior, a bare/default drive would be
> the same fabrication; keep it in SPECFORGE IntentIR metadata/residual, emit a `drive` only when value
> is grounded too. **(2) unordered/partial-order body → NO** — ISF bodies are intentionally
> source-ordered, the wrong container for unordered membership; keep it outside the body. **(3)
> phase-group metadata → YES, agreed as the future ISF shape** — checked metadata first (txn name /
> authored phase names / member signals per phase / actor-relative role/direction / optional
> provenance), with a hard negative rule (no implied values/schedule/HDL/SV-UVM-VHDL output until those
> have their own contracts) and real validation; distinct from the now-shipped actor-level `(observe …)`
> metadata; needs its own FSMGen tree (not shipped). **(4) ordering — confirmed division:** body =
> grounded behavior+order, verification family = grounded temporal obligations, future phase-group
> metadata = membership/phase/role facts with no behavior. **`.isf` stays the source of truth** (no
> `.val`). **SPECFORGE action:** this validates the honest-residual stance and confirms
> `KG-ISF-TRANSACTIONS.2i` option (a) — ship the grounded per-phase membership grouping as IntentIR
> metadata (not ordered ISF steps), value/order as honest residual, `.isf` byte-identical; the
> cross-`.isf` phase-group metadata surface is recorded as the future FSMGen-owned carry path.

**What SPECFORGE is trying to do.** SPECFORGE is building first-class transaction
capture (its `KG-ISF-TRANSACTIONS` work). For each transaction a chip-spec PDF defines,
it recovers, grounded in the document: (a) the **set of signals** that participate, (b)
the document's own named **phases** (address / data / setup / access / response /
turnaround / …), and (c) each signal's **direction** relative to the actor (driven vs
sampled). It wants to lower that into an ISF `(transaction …)` body so FSMGEN can schedule
the protocol behaviour — this is the "step-by-step" bar of our transaction-completeness
goal. We hit a **representational** question and would rather raise it than fabricate.

**The concern (our honest-residual doctrine).** SPECFORGE will not emit intent the source
does not ground. Lowering the *grounded* membership into an ISF transaction body appears to
force us to *invent* two facts the document usually does **not** state — per-signal drive
**values** and a total **order**. We would rather ask FSMGEN how it wants this represented
than fabricate behaviour or silently drop the grounded facts.

**What we observe — corpus measurement (the motivation).** Across the four AMBA wire specs
we use as gold, a transaction's phase structure is only partially groundable from prose:

| Doc | named phases | per-transaction signal→phase grouping |
| --- | --- | --- |
| AHB | address, data | only non-trivial case — e.g. a basic transfer groups data:{HRDATA,HWDATA,HREADYOUT,HREADY}, address:{HREADY}; HCLK/HWRITE ungrouped; HREADY spans both phases |
| APB | setup, access | recognised transactions' membership is thin → grouping trivial |
| AXI | data | the phase's prose names no declared signal → empty grouping |
| SWD | 7 phases | all phase prose names no declared signal → empty grouping |

So we can often ground *which signals participate and in which phase*, but **not a total
order across phases** — e.g. SWD prose names phases in the order opposite the packet order,
and AHB within-sentence precedence conflicts because the data phase of one transfer
legitimately overlaps the address phase of the next. We therefore treat cross-phase order
as an explicit residual rather than guess it.

**What we observe — ISF grammar (read + empirically probed at pin `8c39827f`,
`subs/fsmgen/bin/fsmgen --strict --check --json`).**

1. **A transaction body is a total order.** `13b-transactions.md`: "the scheduler links
   states **in order** … what you write is what you get", one clause ≈ one cycle. So body
   steps `(drive A)` then `(drive B)` assert A-before-B.
2. **Same-cycle concurrency exists only inside one multi-pair drive block**
   (`13c-drive-blocks.md`: "For concurrent execution, put actions in one drive").
3. **Every `(drive …)` requires a concrete value.** Probed: `(drive PNSE)` →
   `success:false`, *"drive 'PNSE' missing actual for 'val'"*; `(drive PNSE 1)` →
   `success:true`. There is **no value-less** "this output is driven / participates this
   cycle, value not grounded" form.
4. **Inputs are value-free, but only via `(sample …)`.** `(sample LEVEL as cap)` →
   `success:true`; `(drive LEVEL)` → `success:false` *"not defined"* (drives exist only
   for outputs — direction matters).
5. **Interaction with SPECFORGE's current emission (noted for awareness, ours to
   reconcile):** SPECFORGE already emits a top-level named drive per output
   (`(drive (X val)(X val))`); adding a transaction-body drive for that same output raises
   `isf_priority_mixed_timing_conflict on X`, so we could not get a clean strict pass for
   the same-cycle block form *within* SPECFORGE's current emission shape.

**The issue, precisely.** To put a transaction's grounded membership into an ISF body,
SPECFORGE would have to invent (i) a **value** for every participating *output* — we ground
the participating signal + its phase + its direction, but usually **not** the value it is
driven to (only enum-selector transactions such as AHB `idle_transfer` ⟺ `HTRANS=IDLE`
carry a grounded value), and (ii) an **order** among phases/signals (the body is totally
ordered; the source rarely grounds that order). Inputs are fine (`sample` is value-free);
outputs are the blocker.

**Questions / requests to FSMGEN.**

1. **Value-less output participation.** Would FSMGEN consider a way to express "output `S`
   participates / is driven in this transaction, value not grounded by the source" — a bare
   `(drive S)` accepted as a participation marker, a don't-care actual, or a dedicated
   `(touches S)` / `(participates S …)` clause? Today the mandatory `val` actual forces
   SPECFORGE to either fabricate a value or omit a genuinely-participating output.
2. **Unordered / partial-order body.** Is there (or would FSMGEN consider) a construct for a
   set of behavioural facts whose order the author does **not** assert — beyond the
   single-cycle multi-pair drive block — so a transaction's phase membership can lower
   without claiming a total cross-phase order? We are explicitly **not** asking FSMGEN to
   hardcode protocol phase orders; we are asking whether "unordered / partial order" is
   expressible.
3. **Phase-group metadata.** Mirroring §3 ("Interface Or Channel Grouping") below: would
   FSMGEN want a first-class, protocol-neutral **phase-group** annotation on a transaction
   (which member signals belong to the address/data/response/… phase, with roles), carried
   as **checked metadata** that need not affect HDL or impose body order? That lets SPECFORGE
   lower the part it *can* ground (membership + phase + direction) faithfully while leaving
   order/value as honest residuals.
4. **Ordering-as-constraint vs ordering-as-body.** Our reading is that a genuinely-grounded
   ordering is more faithfully a *scheduling / temporal constraint* (which FSMGEN owns) than
   an imperative step sequence. Is that the intended division — i.e. when SPECFORGE *does*
   ground a phase order, should it express it through the verification / `(assert …)` family
   rather than body step order?

**Status — question, not a bug.** No SPECFORGE `.isf` is broken: SPECFORGE currently keeps
these transactions recognition-only (held out of `.isf` by its `!steps.is_empty()` filter)
or emits only the value-grounded enum-selector body, and carries phase membership as
IntentIR metadata. Per our no-hacks doctrine (`[[feedback_isf_no_hacks]]`) we are raising
the representational question rather than fabricating values/order or hacking around the
grammar, and SPECFORGE will keep the membership as honest metadata/residual until FSMGEN
weighs in. All ISF facts above were verified empirically on the pinned `8c39827f` binary,
not inferred from the book alone.

## Feature request (2026-06-22) — declarative field-structured storage (named bit-fields in a register / packed structure layout)

**Context.** A chip-spec PDF's register / CSR programming model is, in large part, its **bit-field map**:
each register is a fixed-width word partitioned into named fields, each field carrying a bit range, an
access type (RO/RW/W1C/WARL/…), an optional reset value, an optional enumeration, and a description.
SPECFORGE recovers this in full — `RegisterFieldRecord { field_name, bits_high, bits_low, bit_width,
access_type, reset_value, description, enumerated_values }` (`crates/specforge/src/ir/source.rs:414`) — and
carries it unchanged into the canonical IntentIR (`IntentIr.register_records`, `ir/intent.rs:75`).

But the current ISF `(storage …)` grammar declares **opaque, width-only** scalar state only —
`(var NAME (width N) [(reset V)])` / `(variable …)` / `(bank …)`
(`ISF_DOWNSTREAM_INTEGRATION_SPEC.md` §8). There is no construct to declare a register's named bit-fields,
so SPECFORGE's emitter lowers each register to a single opaque `(var <reg> (width N))` and **drops the entire
field substructure**. Measured corpus-wide (`DOC-INTENT-TAXONOMY.2`,
`docs/research/document-intent-isf-completeness.md`): **12,638 register bit-fields across 32 documents reach
`.isf` zero times** — the single largest measurable intent-loss in our corpus (heaviest on platform/system-IP
TRMs, e.g. CoreSight SoC-600: 833 registers / 2,978 fields → storage 833, fields 0). The same missing
abstraction blocks the message/packet **structure** layouts (NVMe 216, AMD-IOMMU 217, CHI/DTI/CCIX flit
fields — another 1,220 fields).

We verified empirically on the pinned `030f8c273` binary that this is a representational gap, not a misuse:
the shipped field **operations** (`(set-field NAME (bits HI LO) V)`, `(when-field …)`, `(extract WORD as
FIELD…)`, `(assemble …)`, `13k` matrix) are runtime read-modify-write on an opaque register, **not** a static
field-map declaration — and emitting them to represent a documented static layout would fabricate runtime
behaviour the spec never states (forbidden by our honest-residual doctrine). The feature backlog (`14-…`)
defers aggregate records/arrays and does not list named scalar bit-fields. So there is no faithful
emitter-only path; per `[[feedback_isf_no_hacks]]` we raise the abstraction rather than hack the emitter.

**Ask.** Would FSMGEN consider a **declarative field-structured storage** construct in ISF — a storage
variable that may carry an optional named-field partition, each field with a bit range and optional
access/reset/enum? A concrete strawman shape (FSMGEN to decide the real syntax):

```lisp
(storage
  (var control (width 8)
    (fields
      (field mode   (bits 7 5) (access rw) (reset 0))
      (field prio   (bits 4 2) (access rw))
      (field enable (bits 0 0) (access rw) (reset 1) (enum (OFF 0) (ON 1))))))
```

Properties that would make it lowerable for us (and fail-closed, matching ISF's existing discipline): fields
must tile within `(width N)` without overlap; a field's `(reset …)` composes into the register reset we
already emit (`ISF-REGISTER-RESET-EMIT`); `access`/`enum`/`description` may be metadata-only if not
schedule-relevant; omitting `(fields …)` is byte-identical to today's opaque `(var …)`. The construct would
ideally generalize to the packet/structure layout family (our message-field structures, Gap B) so a single
abstraction serves both the register field-map and the flit/descriptor layout. This is adjacent to the
"memory banks / single/dual-port memory" abstractions FSMGEN noted (`2026-06-22`) for its new
verification-oriented SV/UVM + VHDL path, and would let that path emit field-accurate register/structure
models.

**Why it matters to SPECFORGE.** It is the highest-leverage single lever on our ISF-completeness scorecard
(touches categories 1–4): it turns the largest silent intent-loss in the corpus into faithful synthesis.
Until ISF carries it, SPECFORGE keeps the field map as honest IntentIR metadata + an adapter residual (it is
never lost from the IntentIR), and does not fabricate a structure. Empirically grounded on pin `030f8c273`;
re-verified before any emitter build once FSMGEN weighs in. Full design:
`docs/research/register-bit-field-isf-lowering-design.md` (`DOC-INTENT-TAXONOMY.4a`).

## Purpose

This file is SPECFORGE's tracked feedback for FSMGEN.
It exists so FSMGEN can read one stable document and decide which ideas, if any, belong in FSMGEN itself.

SPECFORGE uses FSMGEN as the reference implementation and documentation surface for the downstream `.fsm` adapter.
SPECFORGE does not expect FSMGEN to solve PDF extraction, `IntentIR` recovery, or chip-spec semantic arbitration.
The goal is narrower and cooperative: help `.fsm` become a natural, precise lowering format for SPECFORGE's canonical `IntentIR`, while staying aligned with FSMGEN's own active direction.

This feedback is therefore not only about validation tooling.
It is also about language features and orientation that would let FSMGEN represent more of the typed hardware intent that SPECFORGE recovers.

## What SPECFORGE Is

SPECFORGE is a Rust toolchain for recovering typed implementation intent from chip-design specifications, especially PDFs.

Its core pipeline is:

```text
SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters
```

The central product is `IntentIR`.
That artifact is meant to be backend-independent, provenance-aware, and honest about uncertainty.
It captures the strongest design intent SPECFORGE can justify from source evidence:

- actors and responsibilities
- interfaces and signal inventory
- actor-relative port/connectivity facts
- clock/reset/system contracts
- reset polarity and timing semantics
- control/state behavior
- typed temporal and stability rules
- assumptions, residual decisions, and conflicts

SPECFORGE is not trying to make `.fsm` the only product boundary.
Instead, `.fsm` is one downstream adapter target, alongside future SystemVerilog, Verilog, and VHDL targets.

The reason SPECFORGE cares deeply about FSMGEN is that `.fsm` can become the most natural high-level lowering format for recovered control intent.
If FSMGEN evolves `.fsm` in ways that align with the typed facts above, SPECFORGE can emit `.fsm` that preserves more real source intent instead of flattening it into comments, lossy HDL, or blocked adapter residuals.

So this feedback is written from the perspective of a tool that wants to lower honest `IntentIR` into a strong `.fsm` language, not from the perspective of a tool asking FSMGEN to contort itself around arbitrary output text.

Last SPECFORGE submodule sync reviewed:

- FSMGEN previous baseline: `9bfb9a20` (history: `955f2bb` → `32aa318` → `9bfb9a20`)
- FSMGEN refreshed baseline: `88a7af9c` (`FSMGEN-REFRESH-INTEGRATE.1`, `2026-05-29`; +637 commits over `9bfb9a20`)
- FSMGEN LATEST refreshed baseline: **`030f8c273`** (`FSMGEN-REFRESH-INTEGRATE-3.1`, `2026-06-16`; +9 commits over `8c39827f`, carrying FSMGen's answer to SpecForge's `2026-06-16` transaction-phase-membership question — `ISF-SPECFORGE-PHASE-MEMBERSHIP-RESPONSE.1`/`.2` — plus the shipped actor-level `(observe …)` verification metadata). **No contract change affects SpecForge's emitted `.isf`** — the 6 `*_passes_fsmgen_strict_validation` canaries + full `run_ci.sh` (`1645/0/2`) pass on the new binary, and the emitted APB `.isf` re-validates `success:true`/0-diagnostics. The phase-membership answer required no FSMGen code change; it confirms SpecForge's honest-residual `KG-ISF-TRANSACTIONS.2i` plan (metadata-only grouping). The prior baseline below remains for history.
- FSMGEN prior refreshed baseline: `8c39827f` (`FSMGEN-REFRESH-INTEGRATE-2.1`, `2026-06-16`; +300 commits over the intervening `d31b0b91`, itself reached via `88a7af9c → c0b7eaa7 → 43b29f5c → 92d7036b → d31b0b91`). **No contract change affects SpecForge's emitted `.isf`** — the 7 `*_passes_fsmgen_strict_validation` canaries + full `run_ci.sh` pass on the new binary, and every form SpecForge emits is still `shipped` in the `13k` matrix. The +300 commits are FSMGen-internal (compositional control-flow acceptance widening, ATL scheduling diagnostics, IAL2 feature-completeness, backend-language portability, a semantic-introspection MCP server). **Do we need NEW ISF features? — NO, not now:** the refreshed surface is sufficient for everything SpecForge extracts today and for the deferred composed multi-phase transaction body (already expressible via `spawn`/`do`+`await_all`/`(stage)`/`(repeat)` — a SpecForge BUILD gap, not an ISF gap, per the `KG-ISF-TRANSACTIONS.1` census). Per `[[feedback_isf_no_hacks]]` there is no missing abstraction being hacked around, so no FR is warranted. NEW adopt candidate (grounded, → proposed tree `ISF-REGISTER-RESET-EMIT`): register/CSR reset values `(storage (var N (width W) (reset V)))` — SpecForge extracts register-field `reset_value` + has a `(storage)` emit surface not yet fed from it. Conditional FUTURE ISF candidates (raise ONLY after SpecForge extracts the intent AND an empirical `--strict --check` probe shows ISF can't carry it): ID-based out-of-order multiple-outstanding-transaction correlation; first-class address/data/response phase-group typing. See the `FSMGEN-REFRESH-INTEGRATE-2.2` assessment in `DEVELOPMENT_NOTES.md`.
- notable reviewed surfaces (ISF, at `88a7af9c`): the ISF public-interface contract (`ISF_PUBLIC_INTERFACE_CONTRACT.md`), downstream-integration spec (`ISF_DOWNSTREAM_INTEGRATION_SPEC.md`), feature-support matrix (`13k`), and lowering reference (`13h`) — see the `FSMGEN-REFRESH-INTEGRATE.2` feature-adoption assessment in `DEVELOPMENT_NOTES.md`. Key contract point: SpecForge's nested `(contract … (eventually s (within N)))` and `(stage … (input)(output))` are now supported **compatibility aliases** (flat `within N` + `ready`/`valid` preferred); SpecForge's emission stays strict-valid. Earlier-reviewed machine surfaces (`--capability-manifest`, `--check --json`, `FSMGEN_*` diagnostic codes, `--emit-semantic-json`, support/report contracts, `--verify-hdl`) and the live mdBook at `subs/fsmgen/docs/book/` remain.

## FSMGEN Response Received

FSMGEN responded in its own tracked document:

- FSMGEN submodule path: `subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`
- initial response commit observed by SPECFORGE: `7475f07` (`Docs: track SPECFORGE feedback response`)
- latest response baseline reviewed by SPECFORGE: **`030f8c273`** (`origin/main` tip; the `2026-06-16` "Transaction Phase Membership Without Fabricated Values Or Order" response read under `FSMGEN-REFRESH-INTEGRATE-3.1`, `2026-06-16`). That entry **answers SpecForge's `2026-06-16` phase-membership question** (see the ANSWERED callout above): don't fabricate value or order, keep membership as IntentIR metadata/residual, checked phase-group metadata is the future ISF shape (its own FSMGen tree, not shipped). No SpecForge-directed ask is now open. (Prior reviewed baseline: `8c39827f`, `FSMGEN-REFRESH-INTEGRATE-2.2`, `2026-06-16`; before that `88a7af9c`, `FSMGEN-REFRESH-INTEGRATE.2`, `2026-05-29`.)

SPECFORGE's planning interpretation is:

- FSMGEN accepts the shared high-level direction: `.fsm` should remain precise, strict mode is the canonical future-facing surface, compatibility syntax must stay labeled as compatibility residue, and machine-readable contracts should complement the mdBook.
- FSMGEN accepts the near-term integration sequence: capability manifest, stable diagnostic codes, check-only JSON diagnostics, normalized semantic JSON export, and first-class reset/clock contract metadata.
- FSMGEN accepts actor-relative ports, interface/channel grouping, semantic signal roles, temporal/stability contracts, assumptions/residual/provenance metadata, contract-aware composition, and a possible canonical direct-module root as directionally valuable longer-term language features.
- FSMGEN explicitly does not want unchecked annotations. Future language additions should be parsed, validated, represented in normalized semantics, documented in the mdBook, support-accounted by fixtures, and either lowered honestly to HDL or preserved honestly as checked metadata.
- SPECFORGE should target strict-mode canonical `.fsm`, treat compatibility syntax as adapter-blocked unless FSMGEN explicitly marks a compatibility lane safe for generated output, and consult FSMGEN's mdBook plus the bounded machine-readable capability/check/semantic/support-accounting surfaces that are now present in the pinned submodule.
- SPECFORGE can keep FSMGEN as a pinned downstream dependency/reference, but FSMGEN does not need a reciprocal SPECFORGE dependency unless a concrete cross-project conformance workflow later justifies it.

## Core Adapter Stance

The `.fsm` adapter in SPECFORGE should emit target text only when canonical facts are explicit enough to map into real FSMGEN-supported syntax.

When the canonical facts are incomplete, contradictory, or too target-specific to justify, SPECFORGE should produce a blocked adapter artifact with residual decisions instead of inventing `.fsm` text.

That means FSMGEN can help SPECFORGE most by making target-language truth machine-checkable:

- what syntax is canonical
- what syntax is compatibility-only residue
- what root kinds are supported
- what expression/control/reset/composition forms are accepted
- what diagnostics mean
- what normalized target semantics FSMGEN recovered from a file

It can also help by making `.fsm` expressive enough to carry common `IntentIR` facts directly, instead of forcing SPECFORGE to choose between lossy lowering and blocked output.

## What Is Already Helpful In Current FSMGEN

- The live mdBook gives SPECFORGE a progressive human-facing map for `.fsm` syntax and support boundaries.
- Strict-mode and support-accounting work help distinguish canonical language from tolerated legacy or compatibility forms.
- Typed failure diagnostics are the right direction for adapter validation and automated residual mapping.
- The expanding aggregate, package, type, parameter, and structural actual support points toward richer future `.fsm` lowering once SPECFORGE's own semantic facts are strong enough.
- Composition/toplink typing gives useful reference shapes for explicit top-root lowering.
- The emerging forward IR split in FSMGEN is valuable as a reference for keeping authored-source intent, lowered RTL, and structural connectivity separate.

## IntentIR-Aligned `.fsm` Feature Suggestions

These suggestions are about the `.fsm` language itself.
Exact syntax is FSMGEN's choice.
The important point is the semantic shape.

### 1. First-Class System Contract

SPECFORGE's `IntentIR` carries clock/reset meaning as hardware intent, not merely as ordinary signals.

A natural `.fsm` lowering target would be able to express:

- clock signal identity
- reset signal identity
- reset polarity
- synchronous versus asynchronous reset behavior
- asynchronous assertion and synchronous release intent
- reset target registers or state elements
- reset/source/distribution caveats where representable

This would make `.fsm` a much better target for real chip-spec intent because resets and clocks are special hardware infrastructure, not just inputs named `clk` and `rst_n`.

If FSMGEN intentionally keeps some of those facts out of generated HDL, it would still be useful to preserve them as checked metadata or normalized contract data.

### 2. Actor-Relative Port Semantics

SPECFORGE often knows that a signal is an input or output only relative to a specific actor:

- Manager drives `AWVALID`
- Subordinate drives `AWREADY`
- Requester drives `PSEL`
- Completer drives `PREADY`
- controller reads `DATA_IN` and drives `DATA_OUT`

Flat port direction is still needed for generated HDL, but `IntentIR` also benefits from preserving actor responsibility.

Potential `.fsm` support:

- optional actor/role annotations on ports
- explicit producer/consumer metadata
- module-local target-actor declaration for standalone roots
- normalized export of actor-relative port facts

This would reduce impedance between SPECFORGE's graph-first actor model and FSMGEN's emitted module boundary.

### 3. Interface Or Channel Grouping

Chip specs often describe related signals as protocol channels rather than isolated scalar ports.

IntentIR can capture that shape:

- ready/valid pairs
- address channels
- data channels
- response channels
- setup/access phases
- sideband groups
- payload plus qualifier/control relationships

A natural `.fsm` target would allow optional grouping metadata for signals that belong to the same logical interface or channel.

This does not require FSMGEN to hardcode AXI/APB/AHB.
The more general feature would be protocol-neutral grouping with semantic roles such as:

- valid-like
- ready-like
- select-like
- enable-like
- address
- data
- response
- sideband
- last/terminal marker

FSMGEN could choose whether these groups affect HDL, generate assertions, or remain normalized metadata.

### 4. Temporal And Stability Contracts

SPECFORGE increasingly recovers temporal rules from specs:

- signal must remain stable while a wait condition holds
- transfer completes on a ready/valid handshake
- a response follows an accepted request after a bounded cycle window
- certain fields remain stable through a transaction phase

Today SPECFORGE should not invent `.fsm` behavior if the target language cannot express the temporal fact.
Longer term, `.fsm` would be a more natural lowering target if it could carry optional checked temporal contracts.

Useful contract families:

- stable-while predicates
- handshake-complete predicates
- next-cycle or bounded-cycle obligations
- actor-grounded drive/stability obligations
- named phase conditions
- optional assertion-generation hooks

Even if FSMGEN initially treats these as metadata or optional generated assertions, preserving them would keep `.fsm` closer to the captured `IntentIR`.

### 5. Semantic Signal Roles

IntentIR may know that a signal is not merely `input wire`.
It may know that the signal is:

- a clock
- a reset
- a valid qualifier
- a ready qualifier
- a payload
- a select
- an enable
- an error/response
- a state/control signal

If `.fsm` can carry those semantic roles, SPECFORGE can lower richer intent without encoding meaning only in names or comments.

FSMGEN could use the roles for:

- stricter diagnostics
- better generated comments
- optional assertion generation
- support-accounted examples
- normalized AST/IR export

### 6. Assumptions, Residuals, And Provenance Metadata

SPECFORGE will often lower a partially known design.
When it does, it should preserve why the target is safe enough or why something was intentionally omitted.

Useful `.fsm` metadata would include:

- assumptions
- residual decisions
- source provenance IDs
- confidence/caveat markers
- unsupported-source-intent notes

This should not pollute normal hand-authored `.fsm`.
It could live behind a generated-metadata section, structured comments, or a strict machine-readable annotation surface.

The value is round-trip honesty: generated `.fsm` remains inspectable, and downstream tools can see what SPECFORGE knew versus what it could actually express.

### 7. Explicit Direct-Module Root Shape

SPECFORGE currently keeps compatibility-level `?mod:name` / `?module:name` outside its canonical root-kind model until it has a backend-neutral direct-module distinction.

If FSMGEN wants direct-module roots to become a canonical language feature rather than compatibility residue, SPECFORGE would benefit from a documented strict-mode root shape for them.

The useful contract would define:

- how a direct module differs from `?dt`, `?fsm`, and `?top`
- what control/body forms it may contain
- how ports/system/reset/init sections behave
- whether it can carry actor/channel/temporal metadata
- how it normalizes in exported AST/IR

That would give SPECFORGE a safe future lowering lane for `IntentIR` cases that are module-like but not naturally a pure decision tree or explicit state graph.

### 8. Contract-Aware Composition

FSMGEN's current composition/toplink direction is already useful.
SPECFORGE would benefit if composition could also preserve contract facts across child boundaries:

- child actor roles
- top/child channel grouping
- reset/clock distribution
- explicit width/type compatibility
- link provenance
- interface-level direction and role consistency

This would let SPECFORGE lower more of `IntentIR` topologies without flattening away why the links are semantically correct.

## Requested Support And Tooling Features

These suggestions are about making the language contract executable for tool-to-tool integration.
As of the `32aa318` submodule sync, FSMGEN has first bounded implementations for the capability manifest, JSON check diagnostics, stable diagnostic-code registry, normalized semantic JSON export, generated-SystemVerilog validation, and several public support/report contract owners. SPECFORGE should treat those as regression-backed first slices to consume carefully, not as permission to infer target-language semantics outside FSMGEN's published contract.

### 1. Machine-Readable Capability Manifest

Please consider publishing a versioned capability manifest, preferably generated from the same support-accounting source that drives tests and docs.

Useful fields would include:

- FSMGEN version or commit hash
- supported root kinds
- strict-mode canonical syntax families
- compatibility-only syntax families
- supported assignment forms
- supported reset/system forms
- supported expression families
- supported aggregate/type/package features
- supported composition/toplink forms
- unsupported or intentionally blocked forms
- links to mdBook chapters or test fixtures

SPECFORGE would use this manifest to gate adapter renderability before emitting `.fsm`.

### 2. JSON Check And Diagnostic Mode

Please consider a stable check-only command such as:

```bash
fsmgen --strict --check --json path/to/file.fsm
```

The ideal output would avoid HDL generation and return structured diagnostics:

- stable diagnostic code
- severity
- source span
- root/module/context
- concise reason
- strict-vs-compatibility classification
- suggested migration when available

SPECFORGE would use this to validate emitted `.fsm` artifacts and map failures back into adapter residual packets.

### 3. Normalized AST Or IR Export

Please consider a parse/normalize/export command such as:

```bash
fsmgen --strict --emit-normalized-json path/to/file.fsm
```

The goal is not to expose every internal detail.
The useful surface would be a stable target-language semantic projection:

- root kind and name
- system/reset declarations
- ports/signals/types/packages
- state graph or decision-tree control
- assignments and guards
- composition children and links
- normalized expression forms
- compatibility residue, if any

SPECFORGE could then compare generated `.fsm` against FSMGEN's recovered normalized semantics instead of relying only on text snapshots or HDL shape.

### 4. Stable Diagnostic Codes

Typed diagnostics become much more useful if codes are stable across wording changes.

For example, a machine-readable code like `FSMGEN_STRICT_INFIX_ASSIGNMENT` is easier for SPECFORGE to consume than prose that may improve over time.

The exact names are FSMGEN's choice.
The key request is stable identity plus structured fields.

### 5. Reset And Clock Metadata

SPECFORGE cares a lot about clock/reset truth:

- clock source/distribution
- reset source/distribution
- reset polarity
- asynchronous assertion
- synchronous release
- reset target registers/flops
- absence of unsafe glue logic on reset/clock trees

FSMGEN does not need to become a CDC/RDC tool, but richer reset/clock metadata or diagnostics would make `.fsm` adapter validation much safer.

Useful surfaces could include:

- explicit reset polarity metadata where the language can represent it
- diagnostics when reset syntax implies less than the source intent requires
- normalized system-contract JSON for clock/reset declarations
- clear mdBook guidance on what `.fsm` can and cannot express about reset polarity and release semantics

This overlaps with the language-feature request above.
The language request is about making reset/clock facts expressible.
The tooling request is about making those facts visible and checkable.

### 6. Adapter-Facing Example Corpus

Please consider maintaining a small canonical corpus specifically for tools that emit `.fsm`.

Each case could include:

- input `.fsm`
- expected normalized AST/IR JSON
- expected strict/check result
- expected HDL-shape snippets where relevant
- short mdBook cross-reference

Useful families:

- standalone combinational decision tree
- standalone sequential decision tree
- explicit FSM root
- explicit top composition
- reset/system declarations
- aggregate/type/package use
- selector/test-node control
- compound update
- blocked strict-mode examples

This would give SPECFORGE a high-quality target conformance suite.

### 7. Keep Strict Mode First

The most useful orientation for SPECFORGE is not broader acceptance at all costs.
It is precise acceptance.

Please keep pushing canonical behavior through strict-mode support accounting, positive fixtures, negative fixtures, and mdBook documentation.

Compatibility syntax can remain useful, but it should stay labeled as compatibility residue so adapter authors do not accidentally target it as the future language.

### 8. Preserve The Public Book As A Live Contract

The FSMGEN mdBook is especially valuable if it stays in sync with shipped behavior.

SPECFORGE will treat the book as a public human-facing language contract, while treating machine-readable manifests/check output as executable contracts.

The best long-term shape is both:

- book chapters for readers
- machine-readable support metadata for tools

## Priority From SPECFORGE's Side

If FSMGEN wants an order of attack, the most leverage for SPECFORGE would be:

- first-class reset/clock contract metadata
- continued stabilization and widening of the strict-mode capability manifest
- continued stabilization and widening of JSON check diagnostics with stable codes
- continued stabilization and widening of normalized semantic JSON export
- actor-relative port and semantic-role annotations
- temporal/stability contract metadata
- adapter-facing examples

The first four already have useful first slices in the pinned FSMGEN baseline, and widening them from regression-backed support-accounting truth would make the current adapter safer.
The later ones make `.fsm` a more natural target for future `IntentIR` richness.

## How SPECFORGE Would Use These Features

SPECFORGE would not use FSMGEN features to mutate canonical `IntentIR`.

Instead, it would use them downstream:

- capability manifest decides whether an `IntentIR` shape is renderable as `.fsm`
- IntentIR-aligned `.fsm` features decide whether richer canonical facts can be preserved instead of becoming adapter residuals
- JSON check validates emitted `.fsm`
- normalized AST/IR export verifies target semantics after parsing
- diagnostic codes map FSMGEN failures into SPECFORGE adapter residual decisions
- actor/channel/reset/temporal metadata keeps generated `.fsm` closer to source intent
- example corpus becomes adapter conformance coverage
- mdBook remains the human reference for why the adapter accepts or blocks a case

## Non-Goals

This feedback is not asking FSMGEN to:

- parse chip PDF specifications
- infer hardware intent from ambiguous prose
- become SPECFORGE's canonical IR
- accept unsafe compatibility syntax just to make adapter output easier
- hide target-language limitations behind permissive parsing

SPECFORGE's side of the bargain is to keep its adapter honest.
FSMGEN's most useful side of the bargain is to keep `.fsm` behavior precise, documented, and machine-checkable.

## Tracked finding (2026-05-18) — `ISF_DOWNSTREAM_INTEGRATION_SPEC.md` §11.8 doc-vs-strict mismatches

While implementing `ISF-TEMPORAL-LOWERING.2.1`, SPECFORGE verified §11.8
constructs against the pinned `subs/fsmgen/bin/fsmgen --strict --check
--json`. Two precise mismatches between the handoff doc and the shipped
strict checker:

1. **`(contract name (eventually signal within N))`** — §11.8 prints the
   flat form `eventually signal within N`, but `--strict --check`
   rejects it: *"contract '<n>' supports only '(eventually signal
   (within cycles))'"*. The accepted shape is the **nested**
   `(eventually <signal> (within <N>))`. SPECFORGE now emits the nested
   form; the doc prose should be corrected to match the strict grammar.
2. **`(stage phase (ready r) (valid v))`** — §11.8 presents this as the
   shipped `ready_valid_barrier`, but `--strict --check` rejects it:
   *"stage '<p>' has unsupported subclause 'ready'"*. Because the
   handoff doc explicitly lists it as supported yet strict rejects it
   (the documented escalation bar), SPECFORGE does **not** emit
   `(stage …)`; ISF temporal `HandshakeComplete` obligations are
   preserved as explicit SPECFORGE residual decisions instead, and the
   `(stage …)` source shape is reported here for FSMGEN to either fix in
   the checker or correct in the spec.

SPECFORGE has not patched the submodule (per the standing rule); this is
a forward bug report. The `(contract … (eventually s (within N)))` form
is confirmed strict-valid and is what SPECFORGE emits.

## Filed issue bundles (2026-05-18) — official `DOWNSTREAM_ISSUE_REPORTING.md` protocol

The two tracked findings above are now filed as **reproducible issue
bundles** built with `subs/fsmgen/bin/fsmgen-issue-bundle` per
`subs/fsmgen/docs/DOWNSTREAM_ISSUE_REPORTING.md`. **Where FSMGEN can see
each report:** the bundles are committed in the SPECFORGE repository and
pushed to `origin/main`; this file (`docs/FSMGEN_FEEDBACK.md`) is the
stable SPECFORGE↔FSMGEN channel that points to them, so FSMGEN reads one
document and finds the reproductions.

| Finding | Bundle id | Path in SPECFORGE repo | Reproduce |
| --- | --- | --- | --- |
| F1 — §11.8 flat `(eventually s within N)` strict-rejected; nested `(within N)` required | `sf-isf-contract-eventually-flat` | `docs/fsmgen-issues/sf-isf-contract-eventually-flat/` | from a FSMGEN checkout: `cd <fsmgen-root> && bash <path>/commands.sh` |
| F2 — §11.8 `(stage p (ready r)(valid v))` strict-rejected "unsupported subclause 'ready'" despite documented `ready_valid_barrier` | `sf-isf-stage-ready-valid` | `docs/fsmgen-issues/sf-isf-stage-ready-valid/` | from a FSMGEN checkout: `cd <fsmgen-root> && bash <path>/commands.sh` |

Each bundle contains `README.md` (protocol §1 summary), `commands.sh`
(reproduces from the FSMGEN repo root using only bundled files),
`env.txt`, `sources/fsmgen-input/` (the failing `.isf`),
`observed/` (captured exit/stdout/stderr/JSON; FSMGEN HEAD
`effe591dff8487c6b1095be013540fe2aef129f8`), and `expected/`
(`baseline-good.isf` — a strict-passing counterpart that differs by
exactly one line, plus its captured `success:true` JSON). The bundles
were generated with `--bundle-dir` pointing into the SPECFORGE tree so
the pinned `subs/fsmgen/` submodule working tree was never modified.

### Count is two, not three (evidence-backed)

A third candidate — `(within 0)` strict-rejected (positive cycles
required) — is **not** an FSMGEN bug. The spec never documents
`(within 0)` as supported; rejecting a zero-cycle eventually-window is
defensible strictness. The actual defect was SPECFORGE-side (it was
about to emit `(within 0)` for `max_cycles == 0`); SPECFORGE caught it
by picky self-verification and guarded it in `ISF-TEMPORAL-LOWERING.2.2`
(zero-window → SPECFORGE residual decision). Per
`DOWNSTREAM_ISSUE_REPORTING.md` §9 this is FSMGEN following its public
contract → a downstream bug, already fixed. No bundle is filed for it.

Observed across both filed bundles: the strict rejection exits `255`
with **empty stdout even though `--json` was requested** — the failure
is not expressible through the documented JSON check surface. This is
recorded in each bundle's README as a secondary observation for
FSMGEN's triage.

## RESOLVED upstream (2026-05-18) — pin `effe591d → 9bfb9a20`

FSMGEN addressed both filed findings. Upstream `origin/main` reproduced
SPECFORGE's two minimized bundles and shipped fixes; SPECFORGE bumped
the `subs/fsmgen` pin to `9bfb9a20` (`FSMGEN-SUBMODULE-BUMP`) and
**empirically verified each fix on the new binary** (not trusted from
commit subjects):

| Finding | FSMGEN fix commit | Verified on `9bfb9a20` |
| --- | --- | --- |
| F1 — flat `(eventually s within N)` strict-rejected | `610cb26e STAGE-CONTRACT-BUGS.1: accept flat eventual contracts` | F1 bundle input now `success:true`, `diagnostic_count:0` |
| F2 — `(stage … (ready)(valid))` strict-rejected "unsupported subclause 'ready'" | `d4d6dfab STAGE-CONTRACT-BUGS.2: accept ready-valid stages` | isolated `(transaction … (stage s (ready r)(valid v)) …)` now `success:true` |
| Secondary — strict reject exits 255 with no JSON despite `--json` | `9bfb9a20 STAGE-CONTRACT-BUGS.3: emit ISF check JSON failures` | F1/F2 now emit structured check JSON instead of exit-255/empty-stdout |

FSMGEN tracked the work in its own
`ISF-SPECFORGE-REPORTED-STAGE-CONTRACT-BUGS` tree (commit `a60cc1ab`).

Honest caveat on the F2 *bundle artifact*: after the fix the F2 bundle
input still returns `success:false`, but with a NEW, correct diagnostic
`isf_priority_mixed_timing_conflict on ADDRESS`. The reported stage bug
is fixed; the bundle's minimized repro happened to inject the stage onto
a corpus-derived signal that an existing rule already drives, so now
that stages are processed FSMGEN correctly flags that artifact's own
self-conflict. This is correct FSMGEN behavior on a self-conflicting
minimization artifact, not a remaining bug — recorded so the paper trail
is precise rather than over-claiming a clean bundle pass.

Forward consequence (NOT yet acted on): `(stage …)` is now an accepted
construct, so SPECFORGE's `HandshakeComplete` temporal_rules — currently
preserved as residual decisions because `(stage …)` was rejected
(`ISF-TEMPORAL-LOWERING` `.2.1`/`.2.3` decision #2) — could now lower to
`(stage <p> (ready <r>)(valid <v>))`. That is a deliberate behavior
change requiring its own verification and task-tree ownership; it is
scoped as a proposed follow-up tree, not auto-enabled.
