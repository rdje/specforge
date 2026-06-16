# KG-ISF-TRANSACTIONS: every supported protocol transaction + its signals, fully captured & ISF-ready

## Metadata

- Tree ID: `KG-ISF-TRANSACTIONS`
- Status: `active`
- Roadmap lane: `R15`/`R16` (extraction quality / design-intent capture, ISF-fidelity lens)
- Created: `2026-06-16`
- Parent context: owner directive (`2026-06-16`): *"For protocol-style chip-spec PDFs, transaction is a key
  concept. Each such PDF describes a certain number of supported transactions, each involving a set of
  signals. All these transactions and corresponding signals shall be accurately captured, modeled
  (relation/constraint/...) — everything about transactions shall be precisely, accurately and fully
  captured, modelled and ready for ISF lowering."* This is the KG-ISF-COMPLETENESS north star
  (`[[project_kg_isf_completeness]]`) applied to the **transaction layer**, and it is the natural home of
  the `KG-ISF-COMPLETENESS.2+` "behaviors/transactions" bar dimension. Memory:
  `[[project_kg_isf_transactions]]`.

## The point (why this tree exists)

**Transactions are a FIRST-CLASS concept** (owner, `2026-06-16`, reinforced): on par with actors and
signals, not a derived afterthought — because **a transaction is HOW an action between actors happens**.
It is the verb/behaviour layer over the actor↔signal graph: an actor *initiates* a transaction, which
drives/samples a **specific set of signals** through ordered *phases*, and another actor *responds*. This
holds for BOTH protocol-based PDFs (AXI *read*/*write* with burst/ordering/exclusive/response semantics,
AHB *NONSEQ/SEQ/BUSY/IDLE* transfers, APB *read/write* setup/access, I2C *write/read* with (re)start/stop+
ack) AND **platform-based PDFs** (a platform/SoC interconnect describes the transactions its components
exchange). A protocol/platform spec is, in large part, a **catalogue of supported transactions**.

For the `.isf` lowering to be faithful, the IntentIR must carry every transaction the document defines —
its initiating/responding actors, its signal set with roles, its ordered phases/steps and the handshakes
that gate them, and its governing constraints — so `FSMGen` schedules the `(transaction …)` body into the
FSM. This tree therefore connects the actor-surface completeness work (`KG-ISF-COMPLETENESS.1b.*` — who the
agents are and what they drive) to the behaviour surface (what the agents *do*, as transactions).

## Measured baseline (`2026-06-16`, read-only over the persisted IntentIR corpus + the build code)

The IntentIR ALREADY has a typed `transactions` surface (`TransactionIntent` in `ir/intent.rs`: ports,
`activation_port`, ordered `steps` of `TransactionStep`, source ids), lowered to ISF `(transaction …)` by
`IsfIr::from_intent_ir()` with FSMGen-contract-exact grammar (`ISF-TXN-GRAMMAR-FIX` / `R16-CONTRACT-IR`
already correct). **55 transactions across 15 of 35 docs.** But the surface is THIN and partly non-agnostic
— it is built fresh at the IntentIR stage (`synthesize_transactions` + `recognize_digital_patterns`,
`ir/intent.rs`) from exactly three sources:

1. **Per-channel valid/ready handshakes** — a `*VALID`/`*READY` name pair → a `{base}_handshake`
   micro-transaction (`await_all` + `sample`). AXI yields `aw_handshake`, `ar_handshake`, `w_handshake`,
   `b_handshake`, `r_handshake`, … — one per channel, but NEVER the composed read/write transaction that
   sequences them.
2. **Per-actor temporal "behavior" blobs** — temporal rules grouped by their driving actor →
   `Manager_behavior` / `Subordinate_behavior`, a bag of `when` steps (no activation, no transaction
   identity the spec would recognise).
3. **Three HARDCODED protocol blobs** — `recognize_digital_patterns` literally tests for `"HTRANS"`,
   `"HREADY"`, `"HADDR"`, `"PSEL"`, `"PENABLE"`, `"MISO"`, `"MOSI"`, `"SCLK"` and emits hardcoded
   `ahb_transfer` / `apb_transfer` / `spi_transfer` names.

### The three gaps (vs the directive)

- **G1 — composed named transactions are missing.** The document's actual supported transactions (AXI
  read = AR→R, write = AW→W→B; AHB NONSEQ/SEQ; APB read/write; I2C write/read) are NOT captured as
  named, multi-phase transactions that sequence their channels/phases. Only per-channel handshakes and
  per-actor when-blobs exist.
- **G2 — ADR-0006 breach (high priority).** The only protocol-aware transaction recognition hardcodes
  spec signal names + protocol-family transaction names (`ahb_transfer`/`apb_transfer`/`spi_transfer`),
  which is non-agnostic (fails the 101st protocol) and brittle (rots on a revision). It must be replaced
  by STRUCTURAL/universal recognition (the document's own transaction catalogue + handshake/phase
  structure), per `[[feedback_no_hardcoded_chip_spec_names]]` / `[[feedback_avoid_denylists_prefer_structural]]`.
- **G3 — signal-set linkage incomplete.** Transactions are not comprehensively tied to "the set of
  signals each involves" (the directive's explicit requirement) — the handshake form carries 2 ports;
  the behavior form is a when-blob; neither enumerates the transaction's full signal set with roles.

## Owner directive — sharpened (`2026-06-16`, multi-message reinforcement)

The owner reinforced the transaction directive across several messages, sharpening it into four
of-utmost-importance requirements (this is the authoritative requirement statement; the bar below
operationalises it):

1. **Recognition — fast and universal.** SpecForge must be able to **very quickly identify the
   transactions** in **ANY** particular chip-spec PDF (protocol **or** platform). "Very quickly" is a
   RUNTIME-capability requirement (cheap, deterministic, structural recognition that needs no heavy LLM
   and is not bound to a name list — ADR 0006), NOT a licence to rush code (the standing
   quality-over-speed-in-writing-code doctrine is unchanged). Universal: it must work on any PDF, keyed
   off the document's own structure.
2. **Boundary / membership precision.** Recognising *what is part of transaction X and what is NOT* is of
   utmost importance: every signal / phase / step must be attributed to the **right** transaction, and
   nothing from a *different* transaction may bleed in. SpecForge must be able to answer "is signal S /
   step P part of transaction X?" correctly, grounded in the document, for every transaction.
3. **Step-by-step description — thorough and accurate.** Each transaction must be described **step by
   step**, faithfully to the document (ordered phases/steps + the handshakes that gate them).
4. **Completeness — all transactions.** This holds for **all** transactions of the document, not a
   sample — protocol or platform.

**Criticality (owner, `2026-06-16`): this is a MINIMUM, not a stretch goal — "without this we can't
really move forward in our PDF→ISF tool."** Faithful transaction capture (recognise → bound → describe
step-by-step → for all transactions) is a load-bearing PREREQUISITE on the critical path of the whole
SpecForge PDF→ISF pipeline, not an optional enhancement. A protocol/platform spec is in large part a
catalogue of supported transactions; if SpecForge cannot capture them faithfully, the IntentIR cannot
lower faithfully to `.isf`, and the tool cannot meaningfully advance. `KG-ISF-TRANSACTIONS` is therefore
the near-term top-priority active tree, gating forward progress.

## The checkable "transaction-complete IntentIR" bar (per protocol/platform doc)

A document's transaction surface is ISF-complete when:
1. **Coverage (ALL transactions)** — every supported transaction the document NAMES/defines is present as
   a typed `TransactionIntent` (no spec's read/write/burst/opcode/transfer-type/etc. silently absent),
   recovered by universal structure, not a hardcoded name list (ADR 0006).
2. **Fast/universal recognition** — the transaction set is recovered by a cheap, deterministic, structural
   pass that runs on ANY chip-spec PDF (protocol or platform) and does not depend on a heavy LLM or a
   per-protocol name list. (Owner: "very quickly identify transactions in any particular chip-spec PDF.")
3. **Boundary / membership precision** — each transaction's signal set, phases, and steps are both
   COMPLETE (everything the document attributes to transaction X is present) **and EXCLUSIVE** (nothing
   from a *different* transaction bleeds in); SpecForge can correctly answer "is S / step P part of X?"
   for every transaction, grounded in the document. This is the precision counterpart to coverage.
4. **Signal set** — each transaction enumerates the set of signals it involves, with role/direction
   (address/data/control/response), grounded in the document.
5. **Phases/steps (step-by-step)** — each transaction carries its ordered phases/steps (e.g. address
   phase → data phase → response) and the handshakes that gate them, as `TransactionStep`s, described
   step by step and faithful to the document.
6. **Constraints/relations** — the obligations and timing that govern the transaction (stability,
   ordering, response rules) are linked, reusing the existing constraint/temporal/relation surfaces.
7. **ISF round-trip** — the transaction lowers to a valid `(transaction …)` that passes FSMGen
   `--strict --check --json`, and every captured transaction element appears in the `.isf` or an explicit
   residual (no silent drop).

**Quick-surface candidate (from "very quickly identify"):** a first-class way to SEE a doc's recognised
transaction set at a glance — a `validate` transaction-inventory metric + finding (mirroring the
message-field inventory), and/or a CLI surface — so an operator can quickly inspect "what transactions
does this PDF define?" Recorded as a candidate to design alongside the recognition slices, not yet built.

**Hard gate (non-negotiable):** WIRE-BASED-100 (APB/AHB/AXI/SWD) holds through every change; universal
grammar only, no name lists (ADR 0006); honest residual over fabrication. Scope per the owner: FULL,
precise, accurate, step-by-step capture of ALL transactions — but delivered measurement-first, in safe
slices, wire-docs first.

## Frontier (next slice) — phase ORDERING recovery (`.2h`); `.2g` phase recognition is DONE

The `.2e` checkpoint deferred the *choice* of ordering signal to "a dedicated measurement-first slice." **`.2f`
ran that measurement (read-only, corpus-wide) and made the choice** — see the `.2f` node + `.2f` changelog
entry below, and census report §4.4. The chosen path is a **structural transaction-phase surface** built from the
document's own `<qualifier> phase` vocabulary; the next code slice (`.2g`) recognises it.

**The `.2f` finding, in one line:** the ordered multi-phase body is NOT groundable from the *existing* typed
surfaces (the `temporal_rules` are per-signal stability/value constraints, not phase-sequencing edges; the
SemanticIR `phases` surface is section-derived — one "phase" per chapter heading, e.g.
`phase_chapter_7_clock_and_reset` — NOT the protocol's address/data/response phases; and there is no name bridge
from AXI's section-named transactions to its per-channel handshakes). **But it IS groundable from the document's
own `<qualifier> phase` structure**, which is universal and present at the EvidenceIR statement level (measured
corpus-wide: AHB `address`/`data`; APB `setup`/`access`; SWD/debug `address`/`data`/`response`/`turnaround`;
trace-bus / avalon / generic-flash / coresight `address`/`data`) — exactly parallel to the shipped
`<qualifier> transfer/transaction/operation` transaction-anchor cue, and needing the same kind of precision gate
(the raw `<word> phase` scan also catches function-word/cardinal/ordinal noise: `the`/`this`/`four`/`first`/…).

**Designed slice sequence (each measurement-first, signoff-quality, no name lists — ADR 0006):**
- **`.2g` — structural transaction-PHASE recognition (recognition + naming, recognition only). DONE
  (`2026-06-16`, measurement-first).** Landed the new typed `SemanticIr.transaction_phases` surface
  (`TransactionPhaseRecord`: `transaction_phase_id`, `phase_name`, `supporting_statement_ids`,
  `automation_confidence`), built by `build_transaction_phases` (prose scan over `context.statements`) +
  `derive_phase_name` (precision gate over the token before a `phase`/`phases` head) in `ir/semantic.rs`, plus a
  `validate <semantic-ir>` phase inventory (metric `transaction_phases` + Info finding
  `semantic_transaction_phase_inventory`, category `transactions`, emitted only when non-empty — the
  `PDF-VARIANT-DIGESTION.11` absence-is-not-an-event rule — + a `transaction_phases:` human-summary line).
  **The gate (universal English grammar, no name list — ADR 0006), tuned by a corpus-wide before/after measurement
  (census §4.5):** take the single token before `phase`/`phases`; reject a sentence/clause boundary (prev token
  ends in `.`/`:`/`;`/`!`/`?`), non-alphabetic / <3-char tokens, the `PHASE_NAME_STOPWORDS` (determiners /
  demonstratives / quantifiers / prepositions / cardinals / ordinals / position-adjectives — stronger than the
  anchor gate because prose is noisier), a transaction head noun used as a modifier (`data transfer phase` → drop
  `transfer`), and gerund-led verbs. **Measured live (rebuilt wire docs; `generated/` gitignored):** APB
  `{setup, access}`, AHB `{address, data}` (+`write`×1 honest), AXI `{data}`, SWD
  `{data, response, acknowledge, turnaround, nodata, address}` (+`write`×1) — clean recall of the protocol phases,
  determiner/cardinal/ordinal/head-noun/punctuation/boundary noise rejected. **Gates ALL GREEN:** ADR-0006 ✓;
  WIRE-BASED-100 orthogonal (additive SemanticIR field, touches no constraint/relation/temporal surface) ✓; ISF
  round-trip unchanged — `transaction_phases` lives on SemanticIR only, is NOT carried to `IntentIr.transactions`,
  so emitted `.isf` is byte-identical (verified: APB `adapt` `blocking_reasons: None`, `transaction_phases` absent
  from IntentIR) ✓; `kg-bench` 156/156 ✓; `run_ci.sh` green (fmt + warning-deny clippy/tests/rustdoc + mdBook;
  lib **1645**, +3 tests: `derive_phase_name` keep/reject, `build_transaction_phases` dedup/provenance) ✓. Book:
  `pipeline/intentir.md` gained "Recognizing a document's transaction phases". RECOGNITION ONLY — no body
  composition (that is `.2i`).
- **`.2h` — phase ORDERING recovery.** Order the recognised phases as the document orders them (section/statement
  order + same-sentence "during X … during Y" / "X phase … followed by … Y phase" / "then" cues). Measurement-first
  on whether the order is recoverable reliably + universally; honest residual where it is not.
- **`.2i` — membership-by-phase grouping + ordered body composition.** Group each named transaction's `.2c`
  signal-set membership into the ordered phases (a member signal belongs to phase P iff P's statements reference it
  — the same intersection technique as `.2c`), then compose the ordered `(transaction … <phase-1 steps> …
  <phase-N steps> (complete …))` body (drive the phase's outputs, sample/await its inputs), reusing the per-signal
  direction `.2c` already grounds. ISF round-trip + WIRE-BASED-100 hard gates. This is where bar #5 (step-by-step)
  lands; anything the document does not ground stays an honest residual, never a fabricated phase ordering.

Repo is handoff-ready: `.2g` (phase recognition) is committed and fully gated; the next slice is `.2h`
(phase ordering recovery).

## Task Tree

- ID: `KG-ISF-TRANSACTIONS` · Status: `active` · Children: `.0` (this ownership/scoping slice), `.1`+ TBD
- ID: `KG-ISF-TRANSACTIONS.0` · Status: `done` (`2026-06-16`, docs-only ownership/scoping) · Goal: own the
  directive; record the measured baseline (55 txns / 15 docs; 3 sources; 3 gaps incl. the ADR-0006 breach);
  define the checkable 5-point bar; cross-reference `KG-ISF-COMPLETENESS`. No code (own before touching).
  Memory `[[project_kg_isf_transactions]]`.
- ID: `KG-ISF-TRANSACTIONS.1` · Status: `done` (`2026-06-16`, measurement-first, read-only, docs-only;
  report `docs/research/transaction-capture-census.md`; KM card `transaction-capture-census`) · Goal: **the
  transaction-capture census + gap taxonomy.**
  **`.1` COMPLETE:** §1 ISF target model + §2 source-form census (AXI/AHB/APB/I2C/CHI) + §3 current-capture
  census + §3.5 **per-doc G1/G3 quantification** (read-only scan of all 36 persisted IntentIR
  `transactions[]`: **63 entries / 16 docs; `composed-named = 0` in EVERY doc → G1 a 100% gap**; G2 fires on
  AHB/APB **and spuriously on `readme`**; G3 anatomy — handshake 2 ports / behavior 0 ports / hardcoded
  `ahb_transfer` 3-of-~12 signals with literal widths+enum-values) + §3.6 **ontology refinement** (the
  surface is MIS-LEVELLED: 34 handshakes are really transaction *phases/steps*, 24 behavior blobs are *not
  transactions at all*, only 5 `*_transfer` attempt the right level — so G1 is also a re-levelling) + §4
  **first code slice designed & grounded** (`.2a` G2-first; structural cues confirmed present in EvidenceIR
  — section anchors `3.1 Write transfers`/`B4.2.1 Successful write operation` + signal-valued enumeration
  tables `| HTRANS[1:0] | Type | … |`). The owner's multi-message reinforcement (fast/universal recognition
  + boundary/membership precision + step-by-step + ALL transactions; **a MINIMUM/critical-path prerequisite
  for PDF→ISF**) is captured in the sharpened 7-point bar above. NO code in `.1`.
  **Census foundations (report §1–§3, gathered `2026-06-16`):** (1) **ISF target model** —
  `(transaction NAME (on TRIG) …steps… (complete PORT))` ALREADY supports composed/hierarchical
  transactions via `spawn`/`do`+`await_all` and rule-activation, so G1 is a BUILD gap not an
  ISF-expressiveness gap; (2) **source-form census** (AXI/AHB/APB/I2C/CHI) — supported transactions are
  defined by THREE recurring structural forms: enumeration tables (opcode/transfer-type/command columns),
  "transaction/transfer/operation" SECTION headings + a def-sentence, and prose+timing/flow diagrams;
  (3) **current capture** = per-channel handshakes + per-actor blobs + 3 hardcoded recognizers, none of
  which capture §2's named/composed/signal-set transactions.
- ID: `KG-ISF-TRANSACTIONS.2a` · Status: `done` (`2026-06-16`; owner-chosen "both cues"; measurement-first;
  **first code slice in this tree**) · Goal: **G2 — fast/universal STRUCTURAL transaction recognition
  (de-hardcode).** **DONE:** removed the 3 hardcoded `recognize_digital_patterns` blocks (`ir/intent.rs`
  Patterns 3/4/5: `HTRANS`/`PSEL`/`MISO` literal tests → `ahb_transfer`/`apb_transfer`/`spi_transfer` + the
  literal `tinv_ahb_addr_stable` invariant) and replaced them with a structural, universal recognizer
  (`recognize_named_transactions` + `mint_named_transaction`, `ir/intent.rs`) that mints a
  `TransactionIntent` per transaction NAMED by the document's own vocabulary.
  **"Both cues" (owner-chosen):** **Cue A (identity)** = a new SemanticIR surface `transaction_anchors`
  (`TransactionAnchorRecord`) derived in the SemanticIR builder (`build_transaction_anchors` +
  `derive_transaction_name`, `ir/semantic.rs`) from section headings — the data-flow gap the `.1` census
  flagged ("a `.2a` parsing input to confirm") was RESOLVED by measurement: section anchors live only on
  EvidenceIR, so Cue A is threaded EvidenceIR→SemanticIR→IntentIR (the recognizer runs at IntentIR from
  `SemanticIr` alone). **Cue B (corroboration)** = the already-reachable `SemanticIr.symbol_definitions`
  signal-keyed enums: a transaction whose qualifier matches an enum member keyed by a declared signal gets
  the keyed signal attached + confidence `High`. The Cue-A rule is universal English structural grammar —
  head noun ∈ {transfer,transaction,operation} (centralized in `normative_vocab::TRANSACTION_HEAD_NOUNS`),
  head-final, generic function-word/cardinal/gerund/Example discriminators, NO chip-spec name list
  (ADR 0006).
  **Measurement (read-only, corpus-wide, the slice's mandated Step 1):** the structural recognizer mints
  **212 named transactions across 42 docs** with strong precision after tuning (AHB 7 incl. `idle_transfer`
  Cue-B-corroborated→`HTRANS` High; APB `read_transfer`/`write_transfer`; AXI 14; SWD 8; CHI full
  catalogue) and `readme`/non-spec docs yield **ZERO** (the spurious `ahb_transfer`/`apb_transfer` firing is
  GONE — the concrete ADR-0006 failure mode eliminated).
  **Gates ALL GREEN:** ADR-0006 (zero hardcoded names in production `intent.rs`; new vocab is universal
  nouns only); **WIRE-BASED-100 byte-identical before/after** (constraint/relation/temporal eval over
  APB/AHB/AXI/SWD gold — `diff` empty: the transaction surface is orthogonal to the wire gold);
  ISF round-trip — `adapt --target isf` emits **0 blocking_reasons** on all wire docs and introduces
  **ZERO new** FSMGen `--strict --check --json` diagnostics (recognition-only transactions have no `steps`,
  so the `!steps.is_empty()` ISF emit filter holds them until `.2b` gives them bodies; the pre-existing
  full-doc `*_behavior` strict errors are unchanged and are exactly `.2b`'s re-levelling target);
  `kg-bench` **156/156**; `run_ci.sh` GREEN (fmt + warning-deny clippy/tests/rustdoc + mdBook); lib tests
  **+4** (`derive_transaction_name` keep/reject, `build_transaction_anchors` dedup/provenance,
  `mint_named_transaction` Cue-B corroboration). Book: `pipeline/intentir.md` gained a "How transactions are
  recognized" subsection (why-before-what, AHB example).
- ID: `KG-ISF-TRANSACTIONS.2b` · Status: `done` (`2026-06-16`; batch slice 2/3; measurement-first) · Goal:
  **G1 — composed step-by-step bodies + `*_behavior` re-levelling.** **DONE** with a measurement-driven
  refinement of the original plan (recorded below). Two changes to the IntentIR transaction synthesis
  (`ir/intent.rs`), both faithful, universal (no name list — ADR 0006), boundary-exact:
  1. **Re-levelling (the broad fix):** removed the per-actor `{actor}_behavior` synthesis from
     `synthesize_transactions` (and the two helpers it solely served — `render_temporal_predicate`,
     `temporal_consequent_to_step`). These were census-§3.6 phantoms (an actor's aggregate timed behaviour,
     not a transaction), built entirely from `temporal_rules` — which are already carried into IntentIR and
     lowered to valid `.isf` via the dedicated temporal path (`txn_temporal_*` asserts / `(rule …)` /
     residual). So the `*_behavior` "transaction" was a redundant SECOND rendering AND an `.isf`-invalid one:
     its multi-word `when` condition (`HREADY == HIGH @PreTick`) is tokenised by FSMGen's S-expression parser
     into scalar body clauses → `when body clauses must be list forms` under `--strict --check`. Removing it
     clears that pervasive strict-error class corpus-wide, loses no temporal semantics, and (since the
     `(priority … over …)` cross-product iterates the transaction list) leaves no dangling refs.
  2. **Composed body (the G1 pattern, faithfully):** `mint_named_transaction` now composes a grounded
     step-by-step body for the Cue-B-corroborated subset — a transaction named after an enumerated value of a
     declared signal is DEFINED, in the document's own terms, by driving that signal to that value
     (`idle_transfer` ⟺ `(drive HTRANS IDLE)`), so it RENDERS to `.isf` (passes the `!steps.is_empty()`
     filter). The driven value is the canonical uppercased member spelling (= the emitted enum member).
  **Measurement-driven refinement (the honest re-scoping of the original "re-level handshakes into the named
  transaction's child steps" plan):** measurement showed the section-anchor-named transactions (AXI
  `atomic_transaction`, `narrow_transfer`, …) and the per-channel handshakes (`aw/ar/w/b/r_handshake`) live
  in different naming universes with NO structural bridge in the threaded data. Mapping which handshakes are
  which named transaction's children cannot be done faithfully or universally without a name list (an
  ADR-0006 breach) and would fabricate boundary attributions — violating bar #3 (owner-elevated to "of utmost
  importance"). That composition depends on the grounded signal-set membership `.2c` owns, so it is correctly
  sequenced there (NOT a reduction in ambition — the right ordering given the grounding dependency).
  **Verification (regenerated wire/protocol docs; `generated/` is gitignored):** `*_behavior` blobs = 0 in
  every doc; **APB (`ihi0024_d`/`ihi0024_e`) strict-FAIL → strict-PASS** (`fsmgen --strict --check --json`
  `success:true`, 0 diagnostics — their only blocker was the behavior error); AHB `idle_transfer` renders
  `(transaction idle_transfer (on start) (drive HTRANS IDLE) (complete done))` and is strict-valid. Remaining
  wire-doc strict failures are PRE-EXISTING, non-transaction rule/enum-lowering issues `.2b` only unmasked
  (AHB HAUSER rule-write conflict; AXI/axi-and-ace/lti `constraint_*` assignment-action grammar; axi-stream/
  generic-flash rule-write conflicts; trace-bus `ATID` width contract; hbm2 enum-member emission) — candidate
  future slices, out of scope. **Gates:** ADR-0006 ✓; **WIRE-BASED-100 constraint+temporal F1 = 1.000** ✓
  (orthogonal — `.2b` touches only IntentIR transaction synthesis, not EvidenceIR/SemanticIR extraction);
  ISF round-trip — `*_behavior` strict-error class eliminated + 0 new transaction diagnostics ✓; `kg-bench`
  156/156 ✓; `run_ci.sh` green ✓ (lib 1641; one existing `mint_named_transaction` test updated for the
  composed body). Book `pipeline/intentir.md` updated.
- ID: `KG-ISF-TRANSACTIONS.2c` · Status: `done` (`2026-06-16`; batch slice 3/3, FINAL — bounded batch
  COMPLETE; measurement-first) · Goal: **G3 — grounded signal-set membership + boundary precision.**
  **DONE:** each named transaction now carries its grounded signal-set membership (bar #3/#4) — the declared
  signals its OWN defining section references, attached as ports with the document-grounded direction.
  **Source (`ir/semantic.rs`):** `TransactionAnchorRecord` gains `signal_set`; `build_transaction_anchors`
  (now taking the `declared_signal_names` inventory already built in `SemanticIr::build`) computes it as the
  union of the section statements' signal-shaped tokens (`StatementContext.signals`) **∩ the declared-signal
  inventory**. **Direction + ports (`ir/intent.rs`):** `recognize_named_transactions` builds a grounded
  per-signal direction map from `actor_signal_relations` (Drives → output, Reads → input, both/neither →
  in/out; exhaustive `RelationKind` match); `mint_named_transaction` attaches each member as a
  `TransactionPortRecord` with that direction, deduped against the Cue-B type-selector port; confidence keyed
  on an explicit Cue-B flag (membership never inflates a non-corroborated txn to `High`).
  **Key measurement finding:** the raw `StatementContext.signals` (from `extract_signal_tokens`)
  OVER-CAPTURES — enum VALUES (`IDLE`, `INCR4`, `NONSEQ`) and prose abbreviations (`MPMC`, `AHB5`); the
  intersection with the declared inventory is ESSENTIAL to stay faithful (claiming `IDLE` is a signal would
  breach bar #3). Universal, no name list (ADR 0006) — `declared_signal_names` is the document's own
  inventory. **Boundary precision (bar #3):** a signal is attributed to X iff X's section references it AND
  the document declares it; shared signals (AHB `HREADY` across several transfers) correctly span each
  (they genuinely participate); nothing from a *different* transaction's section bleeds in.
  **Verification (regenerated wire docs semantic→intent→adapt; `generated/` gitignored):** AHB
  `basic_transfer`→{HCLK,HRDATA,HREADY,HREADYOUT,HWDATA,HWRITE}, `burst_operation`→{HADDR,HBURST,HSIZE},
  `locked_transfer`→{HMASTLOCK,HREADY}, `idle_transfer`→{HTRANS,HREADY}(+`.2b` drive body),
  `secure_transfer`→{HNONSEC}, `waited_transfer`→{HREADYOUT}, `exclusive_transfer`→{} (honest empty — its
  single statement references no declared signal); enum values/abbreviations correctly excluded. **No
  ISF/strict change** — membership is IntentIR `TransactionIntent.ports` metadata and the ISF emitter lowers
  `steps`, not `ports`, so emitted `.isf` + FSMGen `--strict` are byte-identical to `.2b` (APB passes; AHB/AXI
  keep their pre-existing non-transaction rule errors). **Gates:** ADR-0006 ✓; **WIRE-BASED-100
  constraint+temporal F1 = 1.000** ✓ (orthogonal); ISF round-trip 0 new diagnostics ✓; `kg-bench` 156/156 ✓;
  `run_ci.sh` green ✓ (lib 1641; anchor + mint tests extended to assert membership, grounded direction, and
  the enum-value filter). **Deferred (honest, beyond this batch):** the ordered multi-phase BODY (sequencing
  the membership signals into address/data/response phases + the handshakes that gate them) — the membership
  is its prerequisite, now delivered; and finer address/data/control/response role sub-typing.
- ID: `KG-ISF-TRANSACTIONS.2d` · Status: `done` (`2026-06-16`; measurement-first; implements the measured
  design below byte-for-byte) · Goal: **quick-surface** — a `validate <intent-ir>` transaction-inventory metric +
  Info finding so an operator can "very quickly identify" a doc's recognised transactions + their signal-set
  membership at a glance (the owner's "very quickly identify" directive; mirrors the
  `evidence_message_field_inventory` precedent — `PDF-VARIANT-DIGESTION.11`).
  **DONE — what landed (`crates/specforge/src/commands/validate.rs`, intent path `fn validate_intent_ir`):**
  (1) **derived counts** (before `let mut findings`): `transactions_with_signal_set` (`!ports.is_empty()`),
  `transactions_with_steps` (`!steps.is_empty()`), `transactions_recognition_only` (`steps.is_empty()`),
  `transaction_signal_members` (Σ `ports.len()`); (2) **five metrics** after `register_records`: `transactions`,
  `transactions_with_signal_set`, `transactions_with_steps`, `transactions_recognition_only`,
  `transaction_signal_members`; (3) **Info finding** `intent_transaction_inventory` (category `transactions`),
  emitted ONLY when `!ir.transactions.is_empty()` (absence is not an event — the `.11` rule), message = count +
  bounded `take(8)` name list (+N more) + the with-signal-set/with-steps/recognition-only split, `related_ids` =
  `transaction_id`s `take(8)`; (4) **human-summary** `transactions:`/`with_signal_set:`/`with_steps:` lines in the
  printed intent summary (mirrors the message-field `containers/with_bit_range/with_byte_offset` precedent). Read-only
  observation off built IR — NO extraction change. **Test:** `validate_intent_ir_reports_transaction_inventory`
  (negative: no txns → 0 + no finding; positive: 1 corroborated {HTRANS out, HREADY in}+drive body + 1
  recognition-only → asserts all 5 metrics, the Info finding severity/category/summary/related_ids). **Live demo
  (persisted AHB `.2c` artifact):** `transactions=7` [exclusive/basic/locked/burst/waited/idle/secure], `with_signal_set=6`,
  `with_steps=1`, `recognition_only=6`, `signal_members=15` (=2+6+3+2+1+1+0) — reproduces the `.2c` membership exactly.
  **Gates:** `scripts/run_ci.sh` green (fmt + warning-deny clippy/tests/rustdoc + mdBook; lib **1642**, +1 test) ✓;
  `kg-bench` **156/156** ✓; deterministic/RAM-safe/no-LLM ✓; WIRE-BASED-100 unaffected (read-only) ✓; ADR-0006 ✓
  (no names — universal counts). Book: `pipeline/intentir.md` gained "Seeing a document's transactions at a glance".
  **Measured design (read-only, `2026-06-16`; insertion points pinned so the next session does NOT re-derive):**
  - **Surface:** the IntentIR validate path `fn validate_intent_ir` in `crates/specforge/src/commands/validate.rs`
    (transactions live on `IntentIr.transactions: Vec<TransactionIntent>`; each has `transaction_id`,
    `transaction_name`, `ports: Vec<TransactionPortRecord>` (the `.2c` grounded signal set), `steps`
    (the `.2b` composed body), `automation_confidence`).
  - **Derived counts** (compute alongside the other `let` bindings just before `let mut findings = Vec::new();`
    at ~`5999`): `transactions_with_signal_set = transactions.iter().filter(|t| !t.ports.is_empty()).count()`;
    `transactions_with_steps = …!t.steps.is_empty()…`; `transactions_recognition_only = …t.steps.is_empty()…`;
    `transaction_signal_members = transactions.iter().map(|t| t.ports.len()).sum()`.
  - **Metrics** (add into the `metrics: vec![ … ]` of the `ValidationReportRecord` built at ~`6656`, e.g. right
    after `metric("register_records", …)` at ~`6846`): `transactions`, `transactions_with_signal_set`,
    `transactions_with_steps`, `transactions_recognition_only`, `transaction_signal_members`.
  - **Finding** (append an Info finding to `findings` just before `let report = ValidationReportRecord {` at
    ~`6656`, so existing finding order is undisturbed): `intent_transaction_inventory`,
    `ValidationFindingSeverity::Info`, category `"transactions"`, emitted ONLY when `!ir.transactions.is_empty()`
    (absence is not an event — the `.11` rule); message = a one-glance summary (count + a bounded list of
    transaction names + the with-signal-set / with-steps / recognition-only split); `related_ids` =
    `transaction_id`s (bounded `take(8)`) for per-item review. Helpers: `metric()` `:261`, `finding()` `:268`,
    test helpers `metric_value()` `:10019` / `has_finding()` `:10027`.
  - **Test:** mirror `validate_evidence_ir_reports_message_field_inventory` (`:8106`); build an `IntentIr` with
    ≥1 `TransactionIntent` (either via the `IntentIr::build` pipeline like the other `validate_intent_ir_*`
    tests, or construct `TransactionIntent` directly — fields per `ir/intent.rs:863` / the `:2191` mint test)
    and assert the metric values + the Info finding presence; add a zero-transactions negative assertion (no
    finding) like the message-field test.
  - **Gates:** deterministic, RAM-safe, no LLM; `scripts/run_ci.sh` green + `kg-bench` 156/156; WIRE-BASED-100
    unaffected (read-only observation off built IR — no extraction change). Book close-rule: a user-visible
    `validate` surface → add a brief note to `pipeline/intentir.md` (or `quality/validation.md`).
- ID: `KG-ISF-TRANSACTIONS.2f` · Status: `done` (`2026-06-16`; measurement-first, read-only, docs-only — the
  "dedicated measurement-first slice" the `.2e` checkpoint called for) · Goal: **choose the ordering signal for the
  ordered multi-phase BODY** by measuring the three `.2e` candidates corpus-wide, and design the implementation path.
  **DONE — what was measured (read-only over the 36 persisted IntentIR/SemanticIR + 78 EvidenceIR artifacts; no code):**
  - **Candidate (b) `temporal_rules` ordering — REJECTED as the body's ordering signal.** Direct scan of the AHB/AXI
    `temporal_rules`: they are per-signal STABILITY / value obligations (`signal_stable`, `actor_drives_signal`,
    `signal_value`), mostly `cycle_window=none`, NOT phase-sequencing edges between a transaction's membership signals.
    AHB `burst_operation`→{HADDR,HBURST,HSIZE} is touched by exactly ONE rule (HSIZE-stable, no window); AXI's 45 rules
    are stability constraints on sideband signals. Nothing encodes "address phase → data phase → response phase". So
    temporal rules cannot order the body.
  - **The SemanticIR `phases` surface — NOT the protocol's transaction phases.** It exists (AHB: 76 records) but is
    SECTION-derived: one "phase" per chapter heading (`phase_chapter_5_subordinate_response_signaling`,
    `phase_chapter_7_clock_and_reset`), fields `phase_id`/`summary`/supporting ids — no transaction-phase name, no
    ordering, no signal grouping. Reusing it as transaction phases would mislabel chapters as phases.
  - **Candidate (c) handshake-dependency chains — still name-bridge-blocked (confirms `.2b`/`.2e`).** AXI's section-named
    transactions are all `steps=0 ports=0`; the 7 `*_handshake` micro-transactions carry no grounded "aw→w→b" precedence.
    Bridging them needs AXI channel-semantics knowledge = a name list (ADR-0006 breach) + fabricated boundaries (bar #3).
  - **Candidate (a) the document's own `<qualifier> phase` structure — CHOSEN (grounded + universal).** The cue is present
    at the EvidenceIR statement level and clean on the wire/protocol docs (AHB `address`/`data`; APB `setup`/`access`;
    SWD/debug `address`/`data`/`response`/`turnaround`; trace-bus / avalon / generic-flash / coresight `address`/`data`),
    exactly parallel to the shipped `<qualifier> transfer/transaction/operation` transaction-anchor cue. It needs the same
    precision gate (the raw `<word> phase` scan also catches `the`/`this`/`four`/`first`/… — function-word/cardinal/ordinal
    noise), and a NEW typed surface to lift it (it is not in any typed IR today, only raw statements).
  **Decision:** the ordered body is built on a new structural **transaction-phase surface**, NOT a mint-side tweak of
  existing data. Sequenced into `.2g` (phase recognition, mirrors `.2a`) → `.2h` (ordering) → `.2i` (membership-by-phase
  + ordered body composition). See the Frontier section above for the slice specs. **Gates:** read-only/docs-only — no
  code, no extraction change; WIRE-BASED-100 untouched; ADR-0006 ✓ (the chosen cue is universal grammar, no name list);
  `scripts/check_memory_architecture.sh` + the knowledge-map derive-and-diff green. Report: census §4.4; KM card
  `transaction-capture-census` refreshed (`.2f` status + 2 answer keys). `[[project_kg_isf_transactions]]`.
- ID: `KG-ISF-TRANSACTIONS.2g` · Status: `done` (`2026-06-16`; measurement-first; the first CODE slice since the
  `.2a→.2d` batch) · Goal: **structural transaction-PHASE recognition** — a new typed
  `SemanticIr.transaction_phases` (`TransactionPhaseRecord`) built by a precision-gated `<qualifier> phase`
  recogniser, + a `validate` phase inventory; recognition only (no body composition — that is `.2i`).
  **DONE — what landed:** (1) **typed surface** (`ir/semantic.rs`) — `TransactionPhaseRecord`
  (`transaction_phase_id`, `phase_name`, `supporting_statement_ids`, `automation_confidence`) +
  `SemanticIr.transaction_phases` (serde-skipped while empty ⇒ zero artifact churn). (2) **recogniser** —
  `build_transaction_phases(context)` scans `context.statements` prose for the `<qualifier> phase`/`phases` n-gram
  (the input surface STEP-1 measured: phases are in `extracted_statements`, NOT `section_anchors`), first-occurrence
  order, provenance accumulated + deduped per phase; `derive_phase_name` is the precision gate. (3) **gate (universal
  grammar, no name list — ADR 0006):** single token before the head; reject sentence/clause boundary (prev token
  ends `.`/`:`/`;`/`!`/`?`), non-alpha / <3-char, `PHASE_NAME_STOPWORDS` (determiners/demonstratives/quantifiers/
  prepositions/cardinals/ordinals/position-adjectives — stronger than the anchor gate because prose is noisier),
  transaction head noun used as a modifier (`data transfer phase` → drop `transfer`), gerund-led verbs. (4)
  **validate inventory** (`commands/validate.rs`, SemanticIR path) — metric `transaction_phases` + Info finding
  `semantic_transaction_phase_inventory` (category `transactions`, non-empty only) + `transaction_phases:`
  human-summary line.
  **Measurement (read-only corpus-wide + live rebuild; census §4.5):** wire docs recover exactly the protocol
  phases — APB `{setup, access}`, AHB `{address, data}`, AXI `{data}`, SWD
  `{data, response, acknowledge, turnaround, nodata, address}` (each + one honest `write`) — with the
  determiner/cardinal/ordinal/head-noun/punctuation/boundary noise rejected; other docs recover their real phases
  (equalization/discovery/configuration/initialization/activation/…) with low, doc-local residual noise.
  **Gates ALL GREEN:** ADR-0006 ✓; WIRE-BASED-100 orthogonal (additive SemanticIR field; no constraint/relation/
  temporal surface touched) ✓; ISF round-trip unchanged — `transaction_phases` is SemanticIR-only, never carried
  to `IntentIr.transactions`, so `.isf` is byte-identical (APB `adapt` `blocking_reasons: None`; `transaction_phases`
  absent from IntentIR) ✓; `kg-bench` 156/156 ✓; `run_ci.sh` green (lib **1645**, +3 tests) ✓. Book
  `pipeline/intentir.md` "Recognizing a document's transaction phases". Frontier → `.2h` (phase ordering recovery).
  `[[project_kg_isf_transactions]]`.

## Changelog

- `2026-06-16`: **`.0` DONE** — tree created; owns the owner's `2026-06-16` transaction directive.
  Recorded the measured baseline (typed `transactions` surface exists + lowers to ISF, but is built from
  only per-channel handshakes + per-actor temporal blobs + 3 hardcoded protocol recognizers; 55 txns /
  15 docs). Identified 3 gaps: G1 composed named transactions missing, G2 ADR-0006 hardcoding breach
  (`HTRANS`/`PSEL`/`MISO`… literals → `ahb_transfer`/`apb_transfer`/`spi_transfer`), G3 signal-set linkage
  incomplete. Defined the checkable 5-point transaction-complete bar; WIRE-BASED-100 a hard gate. Frontier
  → `.1` (census + gap taxonomy, measurement-first, read-only). Memory `project_kg_isf_transactions`.
- `2026-06-16`: **`.1` STARTED + core measurement gathered** (read-only, no code; owner reinforced
  transactions as FIRST-CLASS, spanning protocol AND platform PDFs — "how actions between actors happen",
  the verb layer over the actor↔signal graph). Captured the ISF target model (composed transactions already
  expressible via `spawn`/`do`+`await_all`), the source-form census across AXI/AHB/APB/I2C/CHI (3 recurring
  structural forms: enumeration tables / section headings+def-prose / prose+flow diagrams), and confirmed
  the current-capture gap. Report `docs/research/transaction-capture-census.md`. Tree "The point" + bar
  reframed for first-class / platform breadth / actor-action. `.1` left `in_progress` (per-doc G1/G3
  quantification + first-slice pick + KM card `transaction-capture-census` remain). No code.
- `2026-06-16`: **`.1` DONE** (read-only, docs-only) — completed the census: §3.5 per-doc G1/G3
  quantification (read-only scan of all 36 persisted IntentIR `transactions[]`: **63 entries / 16 docs;
  composed-named = 0 in EVERY doc → G1 a 100% gap**; G2 fires on AHB/APB + spuriously on `readme`; G3
  anatomy precise), §3.6 ontology refinement (the surface is MIS-LEVELLED — 34 handshakes are
  phases/steps, 24 behavior blobs are not transactions, only 5 `*_transfer` attempt the right level), and
  §4 the designed + structurally-grounded first code slice (`.2a` G2-first; cues confirmed present in
  EvidenceIR — section anchors + signal-valued enumeration tables). KM card `transaction-capture-census`
  written. Split `.2+` into `.2a` (G2 recognition/de-hardcode) → `.2b` (G1 composed step-by-step body) →
  `.2c` (G3 signal-set membership/boundary precision) + `.2d?` (quick-surface candidate). **Owner
  multi-message reinforcement captured** (sharpened 7-point bar + Criticality): recognition must be
  FAST + UNIVERSAL ("very quickly identify transactions in any chip-spec PDF"); BOUNDARY/MEMBERSHIP
  precision ("what's part of transaction X and what's not") is of utmost importance; thorough STEP-BY-STEP
  description; for ALL transactions; protocol or platform; and this is a **MINIMUM / critical-path
  prerequisite** — "without this we can't move forward in our PDF→ISF tool." `KG-ISF-TRANSACTIONS` is now
  the near-term top-priority active tree. Frontier → `.2a` (measurement-first code). No code in `.1`.
- `2026-06-16`: **`.2a` DONE** — **first code in the tree** (owner-chosen "both cues" + "bounded batch
  .2a→.2b→.2c"). G2 de-hardcode: removed the 3 hardcoded `recognize_digital_patterns` blocks
  (`HTRANS`/`PSEL`/`MISO` → `ahb_transfer`/`apb_transfer`/`spi_transfer` + the literal `tinv_ahb_addr_stable`
  invariant) and replaced them with a structural, universal recognizer. **Cue A (identity)** threaded as a
  new `SemanticIr.transaction_anchors` surface (`build_transaction_anchors`/`derive_transaction_name`,
  `ir/semantic.rs`) — resolving the `.1`-flagged data-flow gap (section anchors are EvidenceIR-only;
  threaded EvidenceIR→SemanticIR→IntentIR). **Cue B (corroboration)** via the reachable
  `symbol_definitions` signal-keyed enums (`mint_named_transaction`, `ir/intent.rs`). Head nouns centralized
  in `normative_vocab::TRANSACTION_HEAD_NOUNS`; universal grammar, no name list (ADR 0006). Measured: **212
  named transactions / 42 docs**; AHB 7 (`idle_transfer`→`HTRANS` High-corroborated), APB read/write, AXI
  14, SWD 8; `readme` **0** (spurious firing gone). Gates: ADR-0006 ✓, **WIRE-BASED-100 byte-identical
  before/after** ✓, ISF round-trip 0 blockers + 0 new strict diagnostics ✓, `kg-bench` 156/156 ✓,
  `run_ci.sh` green ✓ (+4 lib tests). Book `pipeline/intentir.md` updated. KM card `transaction-capture-census`
  refreshed. Frontier → `.2b` (G1 composed step-by-step body + re-levelling: the named transaction becomes
  the parent, today's per-channel handshakes its child steps; also fixes the pre-existing `*_behavior` ISF
  strict errors). `[[project_kg_isf_transactions]]`.
- `2026-06-16`: **`.2b` DONE** — batch slice 2/3 (measurement-first). **G1 composed step-by-step bodies +
  `*_behavior` re-levelling.** (1) Removed the per-actor `{actor}_behavior` synthesis (`synthesize_transactions`,
  `ir/intent.rs`) + its two now-orphaned helpers (`render_temporal_predicate`, `temporal_consequent_to_step`):
  census-§3.6 phantoms (an actor's aggregate timed behaviour, not a transaction) built entirely from
  `temporal_rules`, which are already lowered to valid `.isf` via the dedicated temporal path — so the
  `*_behavior` "transaction" was a redundant SECOND rendering AND an `.isf`-invalid one (its multi-word `when`
  condition `HREADY == HIGH @PreTick` tripped FSMGen's `when body clauses must be list forms`). (2)
  `mint_named_transaction` now composes a grounded `(drive signal value)` body for the Cue-B-corroborated subset
  (a transaction named after an enumerated value of a declared signal — `idle_transfer` ⟺ `HTRANS = IDLE`), so it
  RENDERS to `.isf`. **Measurement-driven refinement:** the section-named transactions and the per-channel
  handshakes have NO structural name bridge, so faithfully re-levelling handshakes into the *right* named
  transaction needs the grounded signal-set membership `.2c` owns — deferred there rather than fabricate
  boundary attributions (bar #3). **Measured:** `*_behavior` blobs now 0 corpus-wide; **APB strict-FAIL →
  strict-PASS**; AHB `idle_transfer` renders + is strict-valid; remaining wire-doc strict failures are
  PRE-EXISTING non-transaction rule/enum-lowering issues `.2b` only unmasked (documented out-of-scope residuals).
  Gates: ADR-0006 ✓, **WIRE-BASED-100 constraint+temporal F1 = 1.000** ✓ (orthogonal), ISF round-trip —
  `*_behavior` strict-error class eliminated + 0 new transaction diagnostics ✓, `kg-bench` 156/156 ✓, `run_ci.sh`
  green ✓ (lib 1641, one existing test updated). Book `pipeline/intentir.md` updated; census report §4.2 +
  KM card refreshed. Frontier → `.2c` (G3 signal-set membership + boundary precision; also unlocks the
  membership-grounded composed bodies). `[[project_kg_isf_transactions]]`.
- `2026-06-16`: **`.2c` DONE — bounded batch `.2a→.2b→.2c` COMPLETE** (slice 3/3, measurement-first). **G3
  grounded signal-set membership.** Each named transaction now carries the declared signals its own defining
  section references, as ports with the document-grounded direction. `TransactionAnchorRecord` gains
  `signal_set` (`ir/semantic.rs`); `build_transaction_anchors` computes it as the union of the section
  statements' signal-shaped tokens (`StatementContext.signals`) **∩ the declared-signal inventory**
  (`declared_signal_names`). **Key finding:** the raw token extractor over-captures enum VALUES
  (`IDLE`/`INCR4`/`NONSEQ`) + abbreviations (`MPMC`/`AHB5`) — the declared-inventory intersection is
  essential to stay faithful (claiming `IDLE` is a signal would breach bar #3). `recognize_named_transactions`
  builds a grounded direction map from `actor_signal_relations` (Drives→output, Reads→input); `mint_named_transaction`
  attaches each member as a port. **Boundary precision:** a signal is a member iff the section references it
  AND the document declares it; shared signals (AHB `HREADY`) correctly span several transfers, nothing
  foreign bleeds in. Measured (live AHB): `basic_transfer`→{HCLK,HRDATA,HREADY,HREADYOUT,HWDATA,HWRITE},
  `burst_operation`→{HADDR,HBURST,HSIZE}, `idle_transfer`→{HTRANS,HREADY}+drive body, `exclusive_transfer`→{}.
  No ISF/strict change (membership is `ports` metadata; emitter lowers `steps`). Gates: ADR-0006 ✓,
  **WIRE-BASED-100 constraint+temporal F1 = 1.000** ✓ (orthogonal), ISF round-trip 0 new diagnostics ✓,
  `kg-bench` 156/156 ✓, `run_ci.sh` green ✓ (lib 1641; anchor/mint tests extended). Book `pipeline/intentir.md`
  + census §4.3 + KM card refreshed. **Deferred beyond the batch:** the ordered multi-phase BODY (phase
  sequencing over the membership) + finer address/data/control/response role sub-typing; `.2d?` quick-surface
  validate-inventory candidate remains. `[[project_kg_isf_transactions]]`.
- `2026-06-16`: **`.2d` DONE** — quick-surface transaction inventory on the `validate <intent-ir>` path
  (`commands/validate.rs`, intent path only). Five new metrics (`transactions`, `transactions_with_signal_set`,
  `transactions_with_steps`, `transactions_recognition_only`, `transaction_signal_members`), an Info finding
  `intent_transaction_inventory` (category `transactions`, emitted only when non-empty — absence is not an event,
  the `PDF-VARIANT-DIGESTION.11` message-field rule; message = count + bounded name list + the
  signal-set/steps/recognition-only split; `related_ids` = bounded `transaction_id`s), and three human-summary
  println lines. So an operator can "very quickly identify" a doc's recognised transactions + their `.2c` grounded
  signal-set membership + `.2b` composed bodies at a glance. Pure read-only observation off built IR — no extraction
  change, WIRE-BASED-100 untouched, ADR-0006 clean (universal counts, no names). New test
  `validate_intent_ir_reports_transaction_inventory` (negative + positive). Live demo on the persisted AHB `.2c`
  artifact: `transactions=7`, `with_signal_set=6`, `with_steps=1`, `recognition_only=6`, `signal_members=15` —
  reproduces the `.2c` membership exactly. Gates: `scripts/run_ci.sh` green (lib **1642**, +1), `kg-bench` 156/156.
  Book `pipeline/intentir.md` "Seeing a document's transactions at a glance". Frontier → PNT continues on the
  deferred transaction work: ordered multi-phase transaction BODY (phase sequencing over the `.2c` membership) +
  finer address/data/control/response role sub-typing. `[[project_kg_isf_transactions]]`.
- `2026-06-16`: **`.2e` DONE** (commit `e03cd080`, docs-only) — frontier grounding for the ordered multi-phase body.
  Recorded (read-only over the persisted IntentIR) why it is the hard deferred part: AXI 14 section-named transactions
  with empty steps+ports and NO structural name bridge to the per-channel handshakes; AHB membership-without-body;
  APB 0 named. Listed the three candidate ordering signals (a phase-prose / b temporal-rule ordering / c handshake-
  dependency chains) and flagged that choosing one needs a dedicated measurement-first slice. Task-tree Frontier +
  census KM card + `KNOWLEDGE_MAP.md` + MEMORY refreshed. No code.
- `2026-06-16`: **`.2f` DONE** (measurement-first, read-only, docs-only) — ran the `.2e`-mandated measurement to CHOOSE
  the ordering signal, corpus-wide (36 IntentIR/SemanticIR + 78 EvidenceIR). **Rejected** candidate (b) — `temporal_rules`
  are per-signal stability/value constraints, not phase-sequencing edges (AHB `burst_operation` touched by 1 windowless
  rule; AXI's 45 are sideband-stability) — and confirmed the SemanticIR `phases` surface is SECTION-derived (chapters as
  "phases"), not transaction phases; candidate (c) stays name-bridge-blocked. **Chose** candidate (a): the document's own
  `<qualifier> phase` structure, present at the EvidenceIR statement level and clean on the wire docs (AHB address/data,
  APB setup/access, SWD address/data/response/turnaround, …) — universal, parallel to the shipped `<qualifier>
  transfer/transaction/operation` anchor cue, needing a precision gate + a new typed surface. Designed the slice path
  `.2g` (structural transaction-phase recognition, mirrors `.2a`) → `.2h` (ordering) → `.2i` (membership-by-phase +
  ordered body). Report census §4.4; KM card refreshed (`.2f` status + 2 answer keys); `KNOWLEDGE_MAP.md` regenerated.
  No code; WIRE-BASED-100 untouched; ADR-0006 ✓. Frontier → `.2g`. `[[project_kg_isf_transactions]]`.
- `2026-06-16`: **`.2g` STARTED** (measurement-first STEP-1 DONE, code pending — docs-only). Determined the phase
  recogniser's INPUT SURFACE: the `<qualifier> phase` vocabulary is in `extracted_statements` PROSE, **not**
  `section_anchors` (measured: AHB 0 / APB 0 phase-titled anchors of 172 / 99; SWD 1 `Data transfer phase`). So
  `build_transaction_phases` must scan `context.statements` (prose n-gram), NOT reuse the section-heading input
  `build_transaction_anchors` uses — and the precision gate must be stronger than `derive_transaction_name`'s
  (prose noise), tuned by a corpus-wide before/after measurement like `.2a`. `.2g` left `in_progress`; next = write
  the typed surface + recogniser + validate inventory + tests, then tune + gate. No code. `[[project_kg_isf_transactions]]`.
- `2026-06-16`: **`.2g` DONE** (measurement-first; first CODE since the `.2a→.2d` batch). **Structural
  transaction-PHASE recognition** — new typed `SemanticIr.transaction_phases` (`TransactionPhaseRecord`) built by
  `build_transaction_phases` (prose scan over `context.statements`) + `derive_phase_name` precision gate
  (`ir/semantic.rs`); + a `validate <semantic-ir>` phase inventory (metric `transaction_phases` + Info finding
  `semantic_transaction_phase_inventory` + human-summary line, `commands/validate.rs`). Gate = universal English
  `<qualifier> phase` grammar, no name list (ADR 0006): single token before the `phase`/`phases` head; reject
  sentence/clause boundary, non-alpha/<3-char, `PHASE_NAME_STOPWORDS` (determiners/cardinals/ordinals/
  position-adjectives), head-noun-as-modifier (`data transfer phase` → drop `transfer`), gerunds. Tuned by a
  corpus-wide before/after measurement (census §4.5). **Measured live (rebuilt wire docs):** APB `{setup, access}`,
  AHB `{address, data}`, AXI `{data}`, SWD `{data, response, acknowledge, turnaround, nodata, address}` — clean
  recall, noise rejected. Gates: ADR-0006 ✓, WIRE-BASED-100 orthogonal ✓, ISF round-trip byte-identical (phases
  SemanticIR-only, never reach IntentIR/`.isf`) ✓, `kg-bench` 156/156 ✓, `run_ci.sh` green (lib **1645**, +3
  tests) ✓. Book `pipeline/intentir.md` updated; census §4.5 + KM card refreshed. Frontier → `.2h` (phase ordering
  recovery). `[[project_kg_isf_transactions]]`.
