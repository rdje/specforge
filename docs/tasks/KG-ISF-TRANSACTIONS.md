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

## Frontier — `.2i` DONE (metadata-only per-phase membership grouping); `.2j` table-column phase cue MEASURED = NO-GO; `.2k` DONE (descendant-subsection membership scope); `.2l` DONE (`2026-06-17`, measurement-first — the VLM-tier candidate is RESOLVED: AXI per-signal phase membership is recoverable DETERMINISTICALLY from the document's caption-named `B1.x … channel signals` tables — `provenance` already maps signal→channel, channel→phase clean — so the VLM lever is superseded; SWD is honest degenerate absence — 2-wire serial, bit-field time-phases on shared SWDIO; the bounded VLM probe confirmed redundant/fabrication-prone + RAM-expensive (7B→13 GB→host 87% used)). **`.2m` DONE (`2026-06-17`, measurement-first, CODE — GO): a DETERMINISTIC AXI-family channel-membership lever** — groups a recognized transaction's `.2c`/`.2k` membership by its document-grounded channel, recovered from the universal `<role> channel signals` table-caption cue (RAM-safe, structured-first; the VLM lever superseded by `.2l`). Channel membership is a DISTINCT typed dimension (verbatim document role, ambiguity-gated, continuation-chained) that fills the `.2i` AXI-empty phase grouping (live: AXI `with_channel_membership: 5 / 10 groups`, `atomic_transaction` write request×12/write data×3/read data×4/write response×3); metadata only, `.isf` BYTE-IDENTICAL (proven via `git stash` baseline diff), WIRE-BASED-100 orthogonal, `kg-bench` 156/156, `run_ci.sh` GREEN (lib 1664). The VLM-tier transaction frontier is exhausted; the transaction tree's deterministic levers are now built out (recognition `.2a` → bodies `.2b` → membership `.2c`/`.2k` → quick-surface `.2d` → phases `.2g`–`.2j` → channels `.2m`)

**`.2n` DONE (`2026-06-17`, measurement-first, read-only, docs-only — NO-GO): the transaction ISF BODY is faithfully complete.** A FRESH probe-first cycle on the current FSMGen pin `030f8c273` (the `2026-06-17` triage's named "substantive buildable transaction lever") closed the one unbuilt candidate — a value-free `(sample …)` membership body. **Empirical (`--strict --check --json`):** a pure `(on start (sample S as s)) (complete done)` body PASSES strict (probe A; multiple samples in one entry state also pass, probe D; `(sample …)` is NOT direction-gated, probe B), while an in-body `(drive NAME 0)` with no top-level named drive FAILS `drive 'NAME' not defined` (probe C — an in-body drive is a CALL to a top-level named drive, which is exactly why `.2b`'s enum-selector drive works). **But FSMGen-accepted ≠ faithful:** `13b-transactions.md` (`(on …)` Entry/Idle State, L133–163) shows a sample inside `(on …)` fires AT THE ENTRY TRANSITION ("Cycle N: `port && can_accept` → samples captured") and needs an activation guard `port` — so a membership-derived `(on start (sample HREADY))` would assert an un-grounded entry-cycle capture + a `start` guard the document never states (`mint_named_transaction` sets `activation_port: None`), and could MISrepresent a wait-for-ready read as a one-shot entry sample. That is the fabrication the honest-residual doctrine + FSMGen's `2026-06-16` answer forbid ("emit body steps only for facts whose value AND ordering are grounded; keep membership as metadata"). **Decision:** the body lever is exhausted — the only body-lowerable grounded fact (the enum-selector drive) already ships (`.2b`); the membership a sample body would carry already lives faithfully as IntentIR metadata (`.2c` ports / `.2i` phase / `.2m` channel), the home FSMGen chose. Reconfirms the `.2i` parking with FRESH evidence on `030f8c273` (prior probe was `8c39827f`). Gates: read-only/docs-only — WIRE-BASED-100 untouched ✓, ADR-0006 ✓ (universal grammar, no name list). KM card `transaction-body-emission-faithfully-complete`. **Frontier:** the transaction tree's faithful deterministic+body levers are now ALL built/measured; the remaining cross-`.isf` membership carriage awaits FSMGen's future checked phase-group metadata surface (FSMGen-owned, not shipped). The next SUBSTANTIVE north-star fidelity lever sits OUTSIDE this tree — interface signal DIRECTION emission (`KG-ISF-COMPLETENESS.2`, the ~98%-defaulted-`output` gap), which is owner-design-gated (the single-flat-module actor-perspective decision).

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
- **`.2h` — phase ORDERING recovery. DONE (`2026-06-16`, measurement-first, docs-only — a `.2f`-style "is it
  groundable?" slice).** Measured corpus-wide (read-only over the persisted EvidenceIR) whether a transaction's
  phase ORDER is reliably + universally recoverable from statement prose. **Finding: it is NOT.** Three candidate
  signals, none clean+universal: (1) **document first-occurrence order** — right for the LINEAR buses (APB
  `setup`→`access`, AHB `address`→`data` match the protocol truth) but WRONG for SWD (`data` appears before
  `address`, opposite the packet order); (2) **same-sentence sequencing-keyword cues** (`then`/`followed by`/
  `before`/…) — sparse: APB/AXI have ZERO, SWD has only 3 and they CONTRADICT (`acknowledge→data` and
  `data→acknowledge` both fire); (3) **within-sentence positional precedence** (phases co-listed in one sentence)
  — absent for APB/AXI, majority-but-CONFLICTING for AHB (`address→data` 5 vs 2, the reverse being legit
  pipelining overlap "the data phase of one transfer overlaps the address phase of the next"), and TIED for SWD
  (`address/data` 1 v 1). **Decision (honest-residual doctrine): do NOT fabricate a universal phase order.** A
  majority-vote heuristic would recover only AHB (1 of 4 wire docs) off a conflicting signal — gate-risky and
  un-demonstrable per the scoring-rigor doctrine, and silent on APB/AXI/SWD. So phase ordering stays an explicit
  RESIDUAL: `.2i` composes per-phase bodies from the `.2c` membership but emits a cross-phase sequence ONLY where
  the document decisively grounds one (not the wire-doc norm), never an invented address→data→response order. A
  richer ordering signal (timing-diagram left-to-right phase order, VLM-tier; or a per-transaction definition
  sentence that enumerates phases in order) is recorded as a future candidate, not built. Gates: read-only/docs-only
  — no code, WIRE-BASED-100 untouched, ADR-0006 ✓. Census §4.6; KM card refreshed.
- **`.2i` — membership-by-phase grouping + per-phase body composition (re-scoped by `.2h`).** Group each named
  transaction's `.2c` signal-set membership into its recognised `.2g` phases (a member signal belongs to phase P
  iff P's statements reference it — the same intersection technique as `.2c`), then compose a per-phase body
  (drive the phase's outputs, sample/await its inputs) reusing the per-signal direction `.2c` already grounds.
  **Per `.2h`: the cross-phase SEQUENCE is emitted only where the document decisively grounds it; otherwise the
  phases are composed without a claimed order (an honest residual), never a fabricated address→data→response.**
  ISF round-trip + WIRE-BASED-100 hard gates. This is where bar #5 (step-by-step) lands as far as the document
  grounds it.

  **UPDATE (`2026-06-16`) — `.2i` measurement done, body-emission PARKED pending FSMGEN.** The grouping
  measurement (read-only Rule-A over the 4 wire docs) found it groundable cleanly only on AHB (1 of 4), noisily,
  and empty on APB/AXI/SWD; and an empirical ISF-grammar probe (`fsmgen --strict --check --json` @ `8c39827f`)
  showed the body is totally ordered, a value-less `(drive SIG)` is rejected (every drive needs a concrete value),
  and `(drive INPUT)` is rejected (drives exist only for outputs — `(sample …)` is the input form). So lowering the
  grounded membership into an ISF body would force inventing per-output VALUES + a cross-phase ORDER the document
  does not ground. Per `[[feedback_isf_no_hacks]]` this is raised to FSMGEN (`docs/FSMGEN_FEEDBACK.md`, `2026-06-16`
  entry); owner steer = wait for FSMGEN. The grounded per-phase membership grouping (IntentIR metadata, `.isf`
  byte-identical) is the safe FSMGEN-independent fallback; the table-column phase cue is the next recognition lever.

Repo is handoff-ready: `.2g` (phase recognition) + `.2h` (ordering measurement/decision) are committed and gated;
`.2i` (membership-by-phase grouping + per-phase body) has its measurement + FSMGEN question done and its
body-emission PARKED pending FSMGEN's answer + owner steer (the metadata-only grouping is the safe fallback).

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
- ID: `KG-ISF-TRANSACTIONS.2h` · Status: `done` (`2026-06-16`; measurement-first, read-only, docs-only — a
  `.2f`-style "is it groundable?" slice) · Goal: **determine whether a transaction's phase ORDER is reliably +
  universally recoverable**, and decide the path for `.2i`. **DONE — measured corpus-wide (read-only over the
  persisted EvidenceIR statements; no code, no extraction change) three candidate ordering signals, none
  clean+universal:** (1) **document first-occurrence order** — correct for the LINEAR buses (APB `setup`→`access`,
  AHB `address`→`data` match the protocol truth) but WRONG for SWD (`data` precedes `address` in prose, opposite the
  packet order); (2) **same-sentence sequencing-keyword cues** — sparse (APB/AXI 0) and CONTRADICTORY where present
  (SWD `acknowledge→data` and `data→acknowledge` both fire); (3) **within-sentence positional precedence** — absent
  for APB/AXI, majority-but-CONFLICTING for AHB (`address→data` 5 vs 2, the reverse being legit pipelining overlap),
  TIED for SWD (`address/data` 1 v 1). **Decision (honest-residual doctrine): do NOT fabricate a universal phase
  order.** A majority-vote heuristic recovers only AHB (1 of 4 wire docs) off a conflicting signal — gate-risky,
  un-demonstrable per the scoring-rigor doctrine, silent on APB/AXI/SWD — so phase ordering stays an explicit
  residual. `.2i` re-scoped: compose per-phase bodies from the `.2c` membership, emit a cross-phase SEQUENCE only
  where the document decisively grounds one, never an invented order. Richer signal (timing-diagram left-to-right
  phase order, VLM-tier; or per-transaction phase-enumerating definition sentence) recorded as a future candidate.
  **Gates:** read-only/docs-only — WIRE-BASED-100 untouched ✓, ADR-0006 ✓ (no name list; the signals measured are
  universal grammar). Census §4.6; KM card refreshed. Frontier → `.2i`. `[[project_kg_isf_transactions]]`.
- ID: `KG-ISF-TRANSACTIONS.2i` · Status: `done` (`2026-06-16`; option (a), FSMGen-confirmed — grounded per-phase
  membership grouping as IntentIR metadata; measurement-first; CODE) · Goal: **membership-by-phase grouping**
  (group each named transaction's `.2c` signal-set membership into its recognised `.2g` phases; per FSMGen's
  `2026-06-16` answer + `.2h`, carry it as checked metadata, NOT ordered ISF steps; no fabricated value or order).
  **IMPLEMENTED (option a):** (1) `SemanticIr.TransactionPhaseRecord` gains `signal_set` (`ir/semantic.rs`) —
  `build_transaction_phases` now takes `declared_signal_names` and computes each phase's grounded signal set the
  same way the `.2c` anchor does (union of the phase-naming statements' `StatementContext.signals` ∩ declared).
  (2) `IntentIr.TransactionIntent` gains `phase_membership: Vec<TransactionPhaseMembership>` (`ir/intent.rs`) —
  `recognize_named_transactions` threads the phases (signal-set-bearing only) into `mint_named_transaction`, which
  groups each transaction's `anchor.signal_set` by each phase (member ∈ phase iff the phase's prose references it),
  reusing the `.2c` per-signal direction; a member spanning phases appears under each (faithful); a member in no
  phase stays in `ports` only (honest "unphased" residual). NOT lowered to `.isf` (the emitter lowers `steps`, not
  this metadata). (3) `validate <intent-ir>` surface (`commands/validate.rs`): metrics
  `transactions_with_phase_membership` + `transaction_phase_groups`, an `intent_transaction_phase_membership` Info
  finding (non-empty-only, the `.11` honesty rule; message carries the per-phase split + the "not lowered to .isf"
  caveat) + a human-summary line.
  **Measured live (rebuilt wire docs):** AHB grouping is faithful + non-trivial — `basic_transfer` →
  address:{HREADY}, data:{HRDATA,HREADY,HREADYOUT,HWDATA} (HCLK/HWRITE honestly unphased; HREADY spans both);
  `burst_operation` → address:{HADDR}; `idle_transfer` → address:{HREADY,HTRANS}, data:{HREADY}. APB minimal
  (read/write_transfer → access:{PCLK}, the thin membership). AXI & SWD honest-empty (phase prose names no declared
  signal). **`.isf` BYTE-IDENTICAL on all 4 wire docs** (verified `diff` old-vs-new — metadata, not `steps`).
  **Gates ALL GREEN:** ADR-0006 ✓ (universal grammar, no name list); WIRE-BASED-100 orthogonal (additive IntentIR
  metadata; no constraint/relation/temporal surface touched) ✓; ISF round-trip byte-identical ✓; `kg-bench`
  156/156 ✓; `run_ci.sh` green (lib **1645**; 4 existing transaction tests extended to assert the grouping +
  round-trip) ✓. Book `pipeline/intentir.md` "Grouping a transaction's signals by phase". The cross-phase ORDER and
  the per-signal VALUE remain honest residuals (`.2h`); the future ISF checked phase-group metadata surface
  (FSMGen-owned, `FSMGEN-REFRESH-INTEGRATE-3`) is the eventual cross-`.isf` carry path; the **table-column phase
  cue** (`| … | Write data phase |` cells, would move AXI/SWD off zero) is the next recognition lever (`.2j`).
  `[[project_kg_isf_transactions]]` / `[[feedback_isf_no_hacks]]`.
  **Design measurement (read-only, pre-implementation — the basis for option (a)):**
  - **Grouping measurement** (read-only Rule-A over the 4 wire docs — group each transaction's `.2c` membership by
    each phase's document-global signal set): groundable **cleanly only on AHB (1 of 4)** and noisily there
    (`basic_transfer` → data:{HRDATA,HWDATA,HREADYOUT,HREADY}, address:{HREADY}; HCLK/HWRITE ungrouped; HREADY
    multi-phase); **empty on APB (membership thin), AXI & SWD (the `<qualifier> phase` prose names no declared
    signal).** So even the *unordered* grouping is AHB-only — a `.2h`-style BOUNDED finding for the body.
  - **ISF transaction-body grammar, empirically probed** (`subs/fsmgen/bin/fsmgen --strict --check --json` @ pin
    `8c39827f`): the body is **totally ordered** (`13b`: "links states in order … what you write is what you get",
    one clause ≈ one cycle); a **value-less `(drive SIG)` is REJECTED** (*"missing actual for 'val'"*) — every
    drive needs a concrete value; **`(sample INPUT as name)` is value-free-ACCEPTED** but **`(drive INPUT)` is
    REJECTED** ("not defined" — drives exist only for outputs); same-cycle concurrency exists only via a multi-pair
    drive block, which collides with SpecForge's per-output top-level named drives
    (`isf_priority_mixed_timing_conflict`). **Conclusion:** lowering the grounded membership into an ISF body would
    force inventing per-output **VALUES** + a cross-phase **ORDER** the document does not ground.
  - **FSMGEN question raised** (per `[[feedback_isf_no_hacks]]`, no hack/no fabrication): `docs/FSMGEN_FEEDBACK.md`
    `2026-06-16` entry asks for value-less output participation / an unordered-or-partial-order body / phase-group
    metadata / ordering-as-constraint. Owner to forward; **owner steer = wait for FSMGEN.**
  **RESOLVED by FSMGen's answer (`FSMGEN-REFRESH-INTEGRATE-3`, pin `030f8c273`):** the two formerly-parked
  body-emission decisions are settled NO by FSMGen — (1) no same-cycle concurrent-drive block (a value-less drive is
  rejected; drives are value-bearing behaviour); (2) value-less participation stays a residual / IntentIR metadata,
  never a body step. So option (a) (metadata-only grouping) is the implemented + FSMGen-blessed answer, exactly as
  above. The table-column phase cue (`| … | Write data phase |` cells) is the next recognition lever (`.2j`).
- ID: `KG-ISF-TRANSACTIONS.2j` · Status: `done` (`2026-06-16`; measurement-first, read-only, docs-only — a
  `.2h`-style "is the cue there?" slice; **NO-GO**) · Goal: **table-column phase cue** — test the `.2i` frontier
  hypothesis that mining `<qualifier> phase` from TABLE cells/headers (not just prose) would move AXI/SWD off the
  empty per-phase grouping. **Measured (read-only over the persisted EvidenceIR statements + SourceIR
  `structured_tables` of the 4 wire docs):**
  - **Signal↔phase co-occurrence (the cue's prerequisite):** AHB 23/36 phase statements co-occur with a declared
    signal (incl. 10/10 flattened table rows like `| HWDATACHK | HWDATA | … | Write data phase |`); APB 7/7. **AXI-l
    0/4, AXI-h 1/7 (a `four-phase` noise hit), SWD 0/63.** So the docs where `.2i` is empty have essentially ZERO
    signal↔phase association in prose OR flattened table rows.
  - **The flattened-row cue is ALREADY captured.** Every AHB/APB signal↔phase table co-occurrence is a flattened
    statement the `.2i` `build_transaction_phases` scan already mines (that is why AHB's data-phase set is rich).
    There is no un-mined table-ROW cue.
  - **Phase-HEADER tables (the steel-man — a `Phase` column the prose scan would miss):** across all 4 docs, header
    cells containing `phase`/`setup`/`access` = essentially none; the only two "phase"-header tables are AXI-l
    `table_0150` (a coherency-SEQUENCE table — its `Phase` column is a step number 1/2/3, rows `Coherent domain
    access`/…, NO declared signals) and SWD `table_0057` (an ACK-RESPONSE table — `Operation requested | ACK
    received | Host response Data phase | …`, rows `R/W/x` + `OK/WAIT/FAULT`, NO signal→phase mapping). Neither maps
    declared signals to phases; mining them would fabricate.
  **Decision (honest-residual + scoring-rigor doctrine): NO-GO — do NOT build a table-column phase cue.** It would
  find nothing new on AHB/APB (already captured) and nothing at all on AXI/SWD (the cue is genuinely absent —
  AXI barely mentions phases; SWD mentions them abundantly but always abstractly / by packet, never tied to a
  declared wire). AXI/SWD per-signal phase membership is a **document absence**, not an extraction gap. **Two
  distinct real gaps surfaced (recorded, not in scope here):** (a) APB's phase prose DOES reference signals
  (PADDR/PWDATA/PWRITE/PENABLE/PREADY) but `.2i` groups only `access:{PCLK}` because the recognised APB transactions'
  `.2c` MEMBERSHIP is thin — a `.2c`-membership-breadth lever, not a phase-cue one; (b) AXI/SWD phase order/columns
  live in TIMING DIAGRAMS — a VLM-tier lever. Both future candidates. **Gates:** read-only/docs-only — no code,
  WIRE-BASED-100 untouched ✓, ADR-0006 ✓ (the cue measured is universal grammar, no name list).
  `[[project_kg_isf_transactions]]` / `[[feedback_scoring_rigor]]`.
- ID: `KG-ISF-TRANSACTIONS.2k` · Status: `done` (`2026-06-17`; measurement-first; CODE — GO) · Goal: **broaden a
  named transaction's `.2c` signal-set membership to the statements of its section's DESCENDANT SUBSECTIONS** (the
  first of the two `.2j`-recorded future candidates: the **APB `.2c`-membership-breadth lever**,
  deterministic/RAM-safe). A transaction defined by a parent section (APB `3.1 Write transfers`) now absorbs the
  signal-rich prose the PDF files under its subsections (`3.1.1 With no wait states` / `3.1.2 With wait states`),
  WITHOUT crossing a transaction boundary (read=`3.3.x` and write=`3.1.x` are disjoint section subtrees → bar #3
  exclusivity held). **Root cause (measured, read-only):** section anchors are line-range and non-overlapping, so a
  parent section's `supporting_statement_ids` carries ONLY its own intro statements — APB `write_transfer` saw
  `{PCLK}` (4 intro statements; only "…sampled at the rising edge of PCLK" names a declared signal), while
  `PSEL`/`PENABLE`/`PADDR`/`PWRITE` sit under the wait-state subsections. The signals ARE captured document-wide
  (the `.2g` `transaction_phases` surface already recovers the rich `setup`/`access` sets) — only the
  per-transaction `.2c` SCOPE was too narrow. (Naively unioning the document-global phase sets into both
  transactions was REJECTED — it would attribute write-only `PWDATA`/`PWRITE` to `read_transfer`, breaching bar #3;
  the subsection-scope broadening is boundary-safe BECAUSE the subtrees are disjoint.)
  **IMPLEMENTED (`ir/semantic.rs`, surgical — one function):** two universal helpers `leading_section_number`
  (the dotted number of a heading, `3.1 Write transfers`→`3.1`, furniture/unnumbered→`None`) +
  `is_descendant_section_number` (a strict dotted-prefix test — `3.1.1` under `3.1`, NOT `3.10`/`3.3.1`/equality);
  `build_transaction_anchors` now unions, into a transaction's statement scope, every section whose number is a
  strict descendant of its own, then derives `signal_set` (+ the record's `supporting_statement_ids`) from the
  expanded set. Universal document-structure grammar keyed off the section number — **no name list (ADR 0006)**;
  byte-identical when the section has no subsections.
  **Measured live (old-binary vs new-binary deterministic rebuilds; `generated/` gitignored):** APB
  `write_transfer` **1→10** signals (`PADDR,PAUSER,PCLK,PENABLE,PPROT,PREADY,PSTRB,PWDATA,PWRITE,PWUSER`;
  phase-grouped **1→11**), `read_transfer` **1→7** (correctly EXCLUDES write-data `PWDATA`/`PSTRB`/`PWUSER` —
  boundary precision); AXI off the `.2j`-flagged "empty" state — `atomic_transaction` **0→22**, `prefetch_transaction`
  **0→8**, `writezero_transaction` **0→6**, `writedeferrable_transaction` **0→10** (each scoped to its own
  `A6.4`/`A8.6`/`A11.x` subtree, distinct memberships, max pairwise Jaccard 0.56 with shared write-channel control
  genuinely shared); AHB/SWD per-signal counts unchanged (no regression; provenance widened only). **Corpus-wide
  boundary-safety scan (78 docs rebuilt semantic→intent):** 101 of 260 transactions carry membership, **0 over-broad**
  (none ≥80 % of the declared inventory or == full inventory; max 77.8 % = a 9-signal flash-bus doc whose
  `normal_operation` genuinely uses 7).
  **Gates ALL GREEN:** ADR-0006 ✓ (section-number grammar, no name list); **`.isf` BYTE-IDENTICAL on all 4 wire docs**
  (old-vs-new deterministic `diff`) ✓; **WIRE-BASED-100 provably orthogonal** — `actor_signal_relations`/
  `signal_constraints`/`temporal_rules`/`conditional_rules` byte-identical old-vs-new on all 4 wire docs, only
  `transactions` changed (membership is `ports`/`phase_membership` metadata; the emitter lowers `steps`) ✓;
  `kg-bench` **156/156** ✓; `run_ci.sh` GREEN (fmt + warning-deny clippy/tests/rustdoc + mdBook; lib **1662**, +2
  tests: descendant-inclusion + boundary precision, and the section-number helpers) ✓. Book `pipeline/intentir.md`
  "Grouping a transaction's signals by phase" extended with the subsection-scope rule. The remaining `.2j`-recorded
  candidate (AXI/SWD timing-diagram phase columns, VLM-tier) stays a future lever. KM card
  `transaction-membership-subsection-scope`. `[[project_kg_isf_transactions]]` / `[[feedback_scoring_rigor]]` /
  `[[feedback_no_hardcoded_chip_spec_names]]`.
- ID: `KG-ISF-TRANSACTIONS.2l` · Status: `done` (`2026-06-17`; measurement-first, read-only + bounded VLM probe;
  docs-only; owner-chosen FRESH SESSION) · Goal: **is AXI/SWD per-signal phase membership recoverable, and by
  which lever?** — close out the last `.2j`-recorded candidate (AXI/SWD timing-diagram phase columns, VLM-tier)
  the honest way: MEASURE before any code, weigh the cheaper deterministic cue against the VLM, decide GO/NO-GO
  per the scoring-rigor doctrine (demonstrate per-item, never assume).
  **Q1 (AXI) → GO, but DETERMINISTIC (the VLM lever is superseded).** AXI's transaction phases are its CHANNELS,
  and the document's OWN caption-named tables declare exactly the signals in each channel — measured live over the
  persisted AXI EvidenceIR: `table_signal_declaration_provenance` (411 entries) maps each signal to the channel
  table that declared it: `B1.1 Write request channel signals`→26 `AW*` (write **address** phase), `B1.2 Write
  data`→15 `W*` (**data**), `B1.3 Write response`→14 `B*` (**response**), `B1.4 Read request`→26 `AR*`, `B1.5 Read
  data`→18 `R*`, `B1.6/B1.7`→snoop. So per-signal phase membership for AXI is recoverable WITHOUT a VLM, via the
  caption cue `<role> channel signals` (role→phase: request→address, data→data, response→response) — the
  document's own vocabulary, universal grammar, no name list (ADR 0006). The VLM is unnecessary + RAM-expensive.
  **Q2 (SWD) → honest DEGENERATE absence (no lever recovers it).** SWD is a 2-wire serial protocol
  (`SWCLK`+`SWDIO`); its packet phases (request/ACK/data/turnaround) are TIME segments of BIT-FIELDS on the single
  shared `SWDIO` wire — confirmed VISUALLY by Figure B4-1 (`Start│APnDP│RnW│A│Parity│Stop│Park │Trn│ ACK │Trn│
  WDATA[0:31] │Parity`, "Wire driven by: Host│Target│Host"). The phase names label bit-fields (`Start`/`ACK`/
  `WDATA`), NOT declared signals; SWD's recognised phases all carry `signal_set=[]` and its transactions
  `ports=[]`. Per-SIGNAL phase membership is structurally degenerate — nothing to recover by deterministic OR
  VLM lever; forcing it = fabrication.
  **Q3 (VLM) → evidence-based NO-GO for a VLM-membership lever.** Read the actual crops (ground truth): AXI
  `timing_diagram`s are generic handshake waveforms (`picture_0008` VALID/READY), credit-timing examples
  (`picture_0013` ACLK/ARESETn/CRDT/VALID), or handshake-DEPENDENCY graphs (`picture_0011` `AW*`/`W*`→`B*`) — NONE
  carries a per-declared-signal phase-column cue beyond the deterministic channel mapping. Bounded probe
  (`qwen2.5vl:7b`, 2 crops, temp 0): on `picture_0011` the VLM was internally CONTRADICTORY ("does not depict
  phases" then listed address/write/response phases) and merely re-stated the deterministic channel mapping while
  MISSING the figure's real content (the ordering arrows); on `picture_0013` it correctly said "no phases" but
  hallucinated signal semantics (called both `CRDT` and `VALID` "write data valid"). So a VLM-membership lever is
  redundant-at-best / fabrication-prone-at-worst. **RAM:** the 7B VLM loaded as **13 GB** (image context + KV
  cache, 100% Metal) and pushed the host to **87% used** — across the 85% autonomous-kill threshold; `ollama stop`
  issued immediately, recovered to 73% free (validates `[[feedback_ram_ceiling_monitor]]`; a further argument for
  the deterministic cue). The only VLM-UNIQUE signal in the diagrams is phase ORDER (`picture_0011` arrows
  AW/W→B) = the `.2h` residual, already FSMGen-blessed as "don't fabricate order".
  **DECISION:** the "AXI/SWD timing-diagram phase columns (VLM-tier)" candidate is RESOLVED — **replaced by a
  DETERMINISTIC AXI-family channel-membership lever** (`.2m` candidate, RAM-safe, structured-first/best-wins);
  SWD per-signal membership + the VLM-membership lever are **NO-GO honest-absence**. The VLM-tier frontier on the
  transaction tree is exhausted. **Gates:** read-only/docs-only — WIRE-BASED-100 untouched ✓, ADR-0006 ✓ (the
  channel-caption cue is the document's own universal vocabulary, no name list). KM card
  `transaction-phase-membership-vlm-vs-channel`. `[[project_kg_isf_transactions]]` / `[[feedback_scoring_rigor]]` /
  `[[feedback_multi_strategy_best_wins]]` / `[[feedback_ram_ceiling_monitor]]` / `[[project_llm_provider_ollama_qwen]]`.
- ID: `KG-ISF-TRANSACTIONS.2m` · Status: `done` (`2026-06-17`; measurement-first; CODE — GO) · Goal:
  **DETERMINISTIC AXI-family channel-membership lever** — group a recognized transaction's `.2c`/`.2k`
  signal-set membership by its document-grounded CHANNEL, recovered from the universal `<role> channel
  signals` table-caption cue (the `.2l` Q1 finding, RAM-safe / structured-first, the VLM lever superseded).
  Fills the `.2i` AXI-empty phase grouping deterministically — channel membership is a DISTINCT typed
  dimension, complementary to the `.2g`/`.2i` prose-derived phases.
  **Measurement (read-only, the slice's mandated Step 1 — over the persisted SourceIR captions +
  EvidenceIR provenance + a current-binary `intent --dry-run` rebuild; `generated/` gitignored):**
  - **The cue is real + clean on the 2025 AXI doc.** `table_signal_declaration_provenance` (signal →
    table_id) joined with the SourceIR table `caption_text` of the form `<table-num>: <role> channel
    signals` (`B1.1: Write request channel signals` → `write request`, …, + the `A2.3: Credited channel
    signals` variant) yields **154 signals → 8 channel roles, ZERO ambiguous** (write/read request/data/
    response, snoop request/response, credited). Continuation fragments (`B1.1 Continued from previous
    page`, role-stripped) are chained by the caption's own table NUMBER to their head's role (recovers the
    24 AW*/19 AR* signals the fragment strands — write-request rises 26→50). Current-code per-transaction
    grouping: `atomic_transaction` 22 ports → {write request:12, write response:3, read data:4, write
    data:3} (MULTI-channel, 0 unmapped), `prefetch`/`writedeferrable` multi-channel, the 7 per-channel
    handshakes each map to their channel; **12 of 21 transactions gain grouping, 3 multi-channel**.
  - **Genericity / boundary precision — the 2021 AXI+ACE doc proves the gate is needed.** Its captions are
    dash-numbered (`A2-2`, `F2-2`) and the SAME channel role recurs across base/ACE/appendix variants AND
    per-interface default-value mega-tables (`Manager interface write` / `Memory Subordinate interface
    write`), so the same signal (AWADDR) carries 3 different role strings → **31 of 105 ambiguous**. The
    **ambiguity gate** (a signal gets a channel iff every channel-captioned table that declares it agrees
    on ONE role; conflicting → drop, honest residual) keeps 74 clean and drops the 31 (they surface as
    `unmapped`, never a fabricated channel). Correct table-number grammar (`[A-Za-z]*\d+(?:[.\-]\d+)*`)
    handles both the `B1.1` colon and `A2-2` dash forms.
  - **Absent (no fabrication) on every non-AXI-family doc** — AXI-Stream / APB / AHB / SWD have no `<role>
    channel signals` captions → 0 roles, 0 signals mapped, empty surface (byte-identical artifacts).
  **DESIGN (locked, Option A — channel membership as its OWN dimension, verbatim document role; the
  request→address/data→data/response→response interpretation is DELIBERATELY NOT applied — it is
  AXI-family semantic knowledge, not universal, would duplicate/conflict with the `.2g` prose phases, and
  risks fabricating a phase the document did not name for that signal; ADR 0006 — the cue AND the role
  string are the document's own caption words, no SpecForge name list):**
  1. **EvidenceIR** (`ir/evidence.rs`, the only stage with BOTH provenance and captions): new
     `SignalChannelMembershipRecord {signal_name, channel_role, table_ids}` +
     `EvidenceIr.signal_channel_memberships` (serde-skip-if-empty); pure `build_signal_channel_memberships`
     (universal caption grammar `derive_channel_role` + continuation-number chaining + the ambiguity gate),
     called in `EvidenceIr::build`.
  2. **SemanticIR** (`ir/semantic.rs`): carry `signal_channel_memberships` forward (clone from
     `evidence_ir`), so IntentIR can read it — mirrors the `transaction_anchors` data flow.
  3. **IntentIR** (`ir/intent.rs`): `TransactionChannelMembership {channel_role, ports}` +
     `TransactionIntent.channel_membership` (serde-skip-if-empty); `mint_named_transaction` groups the
     `.2c`/`.2k` membership (`anchor.signal_set` ports) by channel role (unmapped members stay in `ports`
     only — honest residual). Metadata only, NOT lowered to ISF (mirrors `.2i` `phase_membership`; the
     emitter lowers `steps`).
  4. **validate** (`commands/validate.rs`, intent path): metrics `transactions_with_channel_membership` +
     `transaction_channel_groups`, an `intent_transaction_channel_membership` Info finding (category
     `transactions`, non-empty-only — the `.11` rule), a human-summary line.
  **IMPLEMENTED exactly as designed** — `SignalChannelMembershipRecord` + `build_signal_channel_memberships`
  (`derive_channel_role` / `derive_continuation_table_number` / `split_leading_table_number`) on EvidenceIR,
  carried on SemanticIR, grouped in `mint_named_transaction` onto `TransactionIntent.channel_membership`, +
  the `validate` channel inventory.
  **VERIFICATION (all GREEN):** (1) **`run_ci.sh` GREEN** — fmt + warning-deny clippy (one `collapsible_if`
  let-chain fixed) + warning-deny tests (**lib 1664**, +2: `derive_channel_role_parses_universal_caption_grammar`
  keep/reject for the colon/dash/continuation/non-channel/bare-`channel` forms, and
  `build_signal_channel_memberships_chains_continuation_and_gates_ambiguity`; plus the
  `mint_named_transaction` + `transaction_intent_round_trips_through_json` + `validate_intent_ir_reports_transaction_inventory`
  tests extended for channels) + rustdoc + mdBook + memory-arch + knowledge-map. (2) **`kg-bench` 156/156**
  (no fixture has channel captions → empty surface → unchanged). (3) **WIRE-BASED-100 PROVABLY ORTHOGONAL**
  — a `git stash` HEAD-before-`.2m` vs HEAD-with-`.2m` AXI EvidenceIR rebuild diff shows the **ONLY** changed
  field is the new `signal_channel_memberships` (154 records); `actor_signal_relations` /`signal_constraints`/
  `conditional_rules`/`signal_polarities`/`extracted_statements` BYTE-IDENTICAL; and the **emitted `.isf` is
  byte-identical** (the emitter `isf_ir.rs` lowers `tx.steps` only — empirically confirmed by adapting the AXI
  intent with vs without `channel_membership`: identical `isf` source). (4) **ADR-0006** — universal `<role>
  channel signals` caption grammar + section-number grammar, no name list. **Live demo (new-binary
  evidence→semantic→intent→validate, `generated/` gitignored):** AXI evidence carries 154 channel memberships;
  `validate` reports `with_channel_membership: 5 (10 channel group(s))` with the `intent_transaction_channel_membership`
  Info finding — `atomic_transaction [read data×4/write data×3/write request×12/write response×3]`,
  `prefetch_transaction [read data×1/write request×7]`, `writedeferrable [write request×9/write response×1]`,
  `writezero [write request×6]`, `narrow_transfer [read data×3]`; AXI's `phase_membership` is 0 (its prose
  names no phase signals) so the deterministic channel grouping FILLS that gap exactly as intended. Book
  `pipeline/intentir.md` "Grouping a transaction's signals by channel". KM card
  `transaction-channel-membership`. The tree stays `active`. `[[project_kg_isf_transactions]]` /
  `[[feedback_scoring_rigor]]` / `[[feedback_no_hardcoded_chip_spec_names]]` / `[[feedback_multi_strategy_best_wins]]`.
- ID: `KG-ISF-TRANSACTIONS.2n` · Status: `done` (`2026-06-17`; measurement-first, read-only, docs-only — NO-GO; a
  FRESH probe-first cycle on FSMGen pin `030f8c273`) · Goal: **is there a buildable ordered multi-phase transaction
  BODY lever beyond the `.2b` enum-selector drive?** — the `2026-06-17` resume-pointer triage named the transaction
  body "the substantive buildable critical-path lever" and asked for a fresh probe-first cycle; this slice ran it and
  decided GO/NO-GO honestly. **Owned the leaf before any probe; no code.**
  **Empirical probe (`subs/fsmgen/bin/fsmgen --strict --check --json`, pin `030f8c273`; the doctrine: verify on the
  binary, not the book):** four minimal `.isf` probes —
  - **A** — `(transaction read_transfer (on start (sample HREADY as r)) (complete done))` over a declared `(input HREADY …)` → **`success:true`, 0 diagnostics.** A value-free `(sample …)` body IS strict-valid.
  - **D** — three samples in one `(on start …)` state → **`success:true`, 0 diagnostics.** Multiple samples in the entry state are order-free *among themselves* (they piggyback, no extra cycle).
  - **B** — `(sample HTRANS as t)` where `HTRANS` is an interface `(output …)` → **`success:true`.** FSMGen does NOT gate `(sample …)` on signal direction.
  - **C** — in-body `(drive HTRANS 0)` with NO top-level named drive → **`success:false`: `drive 'HTRANS' not defined`.** An in-body `(drive NAME …)` is a CALL to a top-level named drive defined elsewhere; `.2b` works because the emitter also emits the top-level drive block.
  **Finding (FSMGen-accepted ≠ faithful):** `13b-transactions.md` (`(on port ...)` Entry/Idle State, L133–163) is
  explicit that a sample inside `(on …)` fires **at the entry transition** — "Cycle N: `port && can_accept` →
  samples captured" (a D-input capture in the idle→active cycle) — and `(on …)` needs an activation guard `port`.
  For a recognition-only transaction SpecForge has grounded only WHICH signals participate, NOT that they are
  captured once at the entry cycle, and `mint_named_transaction` sets `activation_port: None`. So a
  membership-derived `(on start (sample HREADY))` would assert an un-grounded entry-cycle capture + a `start` guard
  the document never states, and could MISrepresent a wait-for-ready read as a one-shot entry sample — the
  fabrication the honest-residual doctrine (`[[feedback_isf_no_hacks]]`) + FSMGen's `2026-06-16` answer forbid
  ("emit body `drive`/`sample` steps ONLY for facts whose value AND ordering are grounded; keep membership as
  metadata"). **Decision: NO-GO — the transaction body is faithfully complete.** The only body-lowerable grounded
  fact (the enum-selector `(drive)`) already ships (`.2b`); the membership a sample body would carry already lives
  faithfully as IntentIR metadata (`.2c` ports / `.2i` phase / `.2m` channel), the home FSMGen explicitly chose;
  the cross-`.isf` membership carriage awaits FSMGen's future checked phase-group metadata surface (FSMGen-owned,
  not shipped). Reconfirms the `.2i` parking with FRESH evidence on `030f8c273` (prior probe was `8c39827f`).
  **Gates:** read-only/docs-only — WIRE-BASED-100 untouched ✓, ADR-0006 ✓ (universal grammar, no name list);
  `scripts/check_memory_architecture.sh` + knowledge-map derive-and-diff green. KM card
  `transaction-body-emission-faithfully-complete`. **Frontier:** the transaction tree's faithful deterministic +
  body levers are now all built/measured; the next SUBSTANTIVE north-star fidelity lever is OUTSIDE this tree —
  interface signal DIRECTION emission (`KG-ISF-COMPLETENESS.2`, ~98% defaulted `output`), which is owner-design-gated
  (single-flat-module actor-perspective decision). `[[project_kg_isf_transactions]]` / `[[project_kg_isf_completeness]]`.

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
- `2026-06-16`: **`.2h` DONE** (measurement-first, read-only, docs-only — a `.2f`-style "is it groundable?" slice).
  **Measured corpus-wide whether a transaction's phase ORDER is reliably + universally recoverable from statement
  prose — it is NOT.** (1) document first-occurrence order is correct for the LINEAR buses (APB `setup`→`access`,
  AHB `address`→`data`) but WRONG for SWD (`data` before `address`); (2) same-sentence sequencing-keyword cues are
  sparse (APB/AXI 0) and contradictory where present (SWD `acknowledge↔data` both directions); (3) within-sentence
  positional precedence is absent (APB/AXI), majority-but-conflicting for AHB (`address→data` 5 v 2, reverse =
  pipelining overlap), tied for SWD. **Decision: do NOT fabricate a universal order** (honest-residual doctrine; a
  majority-vote heuristic would recover only AHB off a conflicting signal — gate-risky + un-demonstrable). `.2i`
  re-scoped: compose per-phase bodies from the `.2c` membership, emit a cross-phase sequence ONLY where the document
  decisively grounds one, never invented; richer signal (timing-diagram order, VLM-tier) recorded as a future
  candidate. Gates: read-only/docs-only — WIRE-BASED-100 untouched ✓, ADR-0006 ✓. Census §4.6; KM card refreshed.
  Frontier → `.2i` (membership-by-phase grouping + per-phase body, ordering only where grounded).
  `[[project_kg_isf_transactions]]`.
- `2026-06-16`: **`.2i` design measurement + FSMGEN question raised; body-emission PARKED pending FSMGEN**
  (measurement-first, read-only, docs-only). **Ran the `.2i`-mandated "how many members group cleanly into a
  phase" measurement** (read-only Rule-A grouping over the 4 wire docs: group each transaction's `.2c` membership
  by each `.2g` phase's document-global signal set). Finding: groundable **cleanly only on AHB (1 of 4)** and
  noisily there (HCLK/HWRITE ungrouped, HREADY multi-phase); **empty on APB (thin membership), AXI & SWD (the
  phase prose names no declared signal)**. **Empirically probed the ISF transaction-body grammar**
  (`subs/fsmgen/bin/fsmgen --strict --check --json` @ pin `8c39827f`): a transaction body is TOTALLY ORDERED
  (`13b`: "links states in order … what you write is what you get"); a value-less `(drive SIG)` is REJECTED
  (*"missing actual for 'val'"*) — every drive needs a concrete value; `(sample INPUT as name)` is
  value-free-ACCEPTED but `(drive INPUT)` is REJECTED (drives exist only for outputs); same-cycle concurrency
  exists only via a multi-pair drive block and collides with SpecForge's per-output top-level named drives. **So
  lowering the grounded membership into an ISF body would force inventing per-output VALUES + a cross-phase ORDER
  the document does not ground.** Per the no-hacks doctrine (`[[feedback_isf_no_hacks]]`) this is raised to FSMGEN
  as a question/feature request — value-less output participation / unordered-or-partial-order body / phase-group
  metadata / ordering-as-constraint — in the `2026-06-16` entry of `docs/FSMGEN_FEEDBACK.md` (owner to forward).
  **Owner steer (`2026-06-16`): wait for FSMGEN** before deciding the two open body-emission questions
  (same-cycle concurrent-drive block now vs `.2j`; value-less drive vs pure residual). `.2i` body-emission is
  PARKED; the grounded per-phase membership grouping (metadata, `.isf` byte-identical) remains the safe fallback.
  Gates: read-only/docs-only — WIRE-BASED-100 untouched ✓, ADR-0006 ✓; `scripts/check_memory_architecture.sh` +
  knowledge-map derive-and-diff green. KM card `transaction-capture-census` refreshed. `[[project_kg_isf_transactions]]`.
- `2026-06-16`: **FSMGEN ANSWERED → `FSMGEN-REFRESH-INTEGRATE-3` (pin `8c39827f → 030f8c273`, +9).** FSMGen
  answered the phase-membership question (`ISF-SPECFORGE-PHASE-MEMBERSHIP-RESPONSE.1`/`.2`): don't fabricate value
  or order — keep value-less participation + unordered membership as IntentIR metadata/residual (NOT body
  drives/steps); checked transaction phase-group metadata is the right FUTURE ISF surface (its own FSMGen tree, not
  shipped); `.isf` stays the source of truth. **Confirms `.2i` option (a).** Verified on the new binary (emitted
  APB `.isf` strict `success:true`; 6 canaries + `run_ci.sh` `1645/0`). See `FSMGEN-REFRESH-INTEGRATE-3.md`.
- `2026-06-16`: **`.2i` DONE — CODE (option (a), FSMGen-confirmed): grounded per-phase membership grouping as
  IntentIR metadata.** (1) `SemanticIr.TransactionPhaseRecord` gains `signal_set` (`ir/semantic.rs`;
  `build_transaction_phases` now takes `declared_signal_names`, computes the phase's signal set the `.2c` way).
  (2) `IntentIr.TransactionIntent` gains `phase_membership` (`ir/intent.rs`); `recognize_named_transactions`/
  `mint_named_transaction` group each transaction's `.2c` membership by each phase (member ∈ phase iff the phase's
  prose references it), reusing the `.2c` direction; multi-phase members appear under each (faithful), unphased
  members stay in `ports` (honest residual). Metadata only — NOT lowered to `.isf` (FSMGen `2026-06-16`). (3)
  `validate <intent-ir>` reports `transactions_with_phase_membership` + `transaction_phase_groups` + an
  `intent_transaction_phase_membership` Info finding + a human-summary line. **Measured live (rebuilt wire docs):**
  AHB faithful + non-trivial (`basic_transfer` → address:{HREADY}, data:{HRDATA,HREADY,HREADYOUT,HWDATA}; HCLK/
  HWRITE honestly unphased; HREADY spans both), APB minimal (access:{PCLK}), AXI & SWD honest-empty. **`.isf`
  BYTE-IDENTICAL on all 4 wire docs** (`diff` old-vs-new; metadata, not `steps`). **Gates ALL GREEN:** ADR-0006 ✓,
  WIRE-BASED-100 orthogonal ✓, ISF round-trip byte-identical ✓, `kg-bench` 156/156 ✓, `run_ci.sh` green (lib
  **1645**; 4 transaction tests extended) ✓. Book `pipeline/intentir.md` "Grouping a transaction's signals by
  phase". Cross-phase ORDER + per-signal VALUE stay honest residuals (`.2h`); future ISF phase-group metadata
  surface (FSMGen-owned) = the cross-`.isf` carry path; table-column phase cue = `.2j` lever.
  `[[project_kg_isf_transactions]]` / `[[feedback_isf_no_hacks]]`.
- `2026-06-16`: **`.2j` DONE — measurement-first, read-only, docs-only; NO-GO (table-column phase cue).** Tested the
  `.2i` frontier hypothesis (mine `<qualifier> phase` from TABLE cells/headers to move AXI/SWD off the empty
  grouping). Measured over the persisted EvidenceIR statements + SourceIR `structured_tables`: signal↔phase
  co-occurrence is AHB 23/36 (incl. 10/10 flattened table rows) + APB 7/7, but **AXI-l 0/4, AXI-h 1/7 (noise), SWD
  0/63** — and the AHB/APB table-row co-occurrences are **already captured** by the `.2i` statement scan, while the
  only two phase-HEADER tables (AXI-l `table_0150` coherency-sequence; SWD `table_0057` ACK-response) map NO declared
  signals to phases. **Decision: NO-GO — no table cue to build** (already mined where present; genuinely absent on
  AXI/SWD — a document absence, not an extraction gap). Two distinct real gaps recorded as future candidates: APB
  membership thinness (a `.2c` lever, not phase-cue) and AXI/SWD timing-diagram phase columns (VLM-tier). Gates:
  read-only/docs-only — WIRE-BASED-100 untouched ✓, ADR-0006 ✓. The per-phase-membership surface is now as complete
  as the wire docs ground. `[[project_kg_isf_transactions]]` / `[[feedback_scoring_rigor]]`.
- `2026-06-17`: **`.2k` DONE — CODE (GO): descendant-subsection membership scope** (the first `.2j`-recorded future
  candidate, the APB `.2c`-membership-breadth lever; measurement-first). `build_transaction_anchors` (`ir/semantic.rs`)
  now broadens a named transaction's statement scope to include every section whose dotted number is a strict
  DESCENDANT of its own (new helpers `leading_section_number` + `is_descendant_section_number`), then derives the
  `signal_set` + `supporting_statement_ids` from that expanded set — so a transaction defined by a parent section
  absorbs the signal-rich prose the PDF files under its subsections, boundary-safe because read/write subtrees are
  disjoint (bar #3). Universal section-number grammar, no name list (ADR 0006); byte-identical when no subsections.
  **Measured (old-vs-new deterministic rebuilds):** APB `write_transfer` **1→10** / `read_transfer` **1→7** (read
  correctly excludes write-data `PWDATA`/`PSTRB`/`PWUSER`); AXI off the `.2j` "empty" state — `atomic_transaction`
  **0→22**, `prefetch` **0→8**, `writezero` **0→6**, `writedeferrable` **0→10** (distinct, subtree-scoped). Corpus
  scan (78 docs): 101/260 transactions carry membership, **0 over-broad**. Gates: ADR-0006 ✓; **`.isf` BYTE-IDENTICAL**
  on all 4 wire docs ✓; **WIRE-BASED-100 provably orthogonal** (relations/constraints/temporal/conditional
  byte-identical old-vs-new; only `transactions` changed) ✓; `kg-bench` 156/156 ✓; `run_ci.sh` GREEN (lib **1662**,
  +2 tests) ✓. Book `pipeline/intentir.md` updated; KM card `transaction-membership-subsection-scope`. Remaining
  `.2j` candidate (AXI/SWD timing-diagram phase columns, VLM-tier) stays a future lever — tree stays `active`.
  `[[project_kg_isf_transactions]]` / `[[feedback_scoring_rigor]]`.
- `2026-06-17`: **`.2l` DONE — measurement-first, read-only + bounded VLM probe, docs-only.** Closed the last
  `.2j`-recorded transaction candidate ("AXI/SWD timing-diagram phase columns, VLM-tier"). **Q1 (AXI) GO but
  DETERMINISTIC** (VLM superseded): AXI phases = channels; `table_signal_declaration_provenance` (411) maps
  each signal to its caption-named `B1.x … channel signals` table (B1.1=26 AW* address / B1.2=15 W* data /
  B1.3=14 B* response / B1.4 AR* / B1.5 R* / B1.6/B1.7 snoop) → per-signal phase membership recoverable with NO
  VLM via the universal `<role> channel signals` caption cue (ADR 0006). **Q2 (SWD) honest degenerate absence:**
  2-wire serial; packet phases are bit-field time-segments on the shared `SWDIO` wire (Fig B4-1); recognized
  phases `signal_set=[]`, transactions `ports=[]`. **Q3 (VLM) evidence-based NO-GO:** crops carry no per-signal
  phase-column cue; the bounded `qwen2.5vl:7b` probe was contradictory/redundant/hallucinated (`CRDT`/`VALID`)
  and RAM-expensive (7B → 13 GB → host 87% used, across the 85% kill line; `ollama stop`, recovered). Decision:
  replaced by a deterministic AXI-family channel-membership lever (`.2m`); SWD + VLM-membership are NO-GO
  honest-absence; the VLM-tier transaction frontier is exhausted. Gates: read-only/docs-only — WIRE-BASED-100
  untouched ✓, ADR-0006 ✓. KM card `transaction-phase-membership-vlm-vs-channel`. Frontier → `.2m`.
  `[[project_kg_isf_transactions]]` / `[[feedback_scoring_rigor]]` / `[[feedback_ram_ceiling_monitor]]`.
- `2026-06-17`: **`.2m` DONE — CODE (GO): deterministic AXI-family channel-membership lever** (measurement-first).
  Owned the leaf before any code edit. **Measured** the `.2l` Q1 deterministic channel cue read-only (persisted
  SourceIR captions + EvidenceIR provenance + a current-binary dry-run rebuild): the `<role> channel signals`
  caption cue is clean on the 2025 AXI doc (154 signals → 8 roles, 0 ambiguous; continuation fragments chained by
  caption table-number; `atomic_transaction` 22 ports → {write request:12, write response:3, read data:4, write
  data:3} multi-channel), boundary-gated on the 2021 AXI+ACE doc (dash-numbered captions, the same role recurring
  across base/ACE/appendix + per-interface mega-tables → 31/105 ambiguous, dropped by the **ambiguity gate** to
  honest `unmapped`), and absent on AXI-Stream/APB/AHB/SWD (no fabrication). **Built** (Option A — channel
  membership as its OWN typed dimension, verbatim document role; NO request→address interpretation): EvidenceIR
  `SignalChannelMembershipRecord` + `build_signal_channel_memberships` (`derive_channel_role` universal caption
  grammar + continuation-number chaining + ambiguity gate) → carried on SemanticIR → IntentIR
  `TransactionIntent.channel_membership` grouped in `mint_named_transaction` + a `validate` channel inventory
  (`transactions_with_channel_membership`/`transaction_channel_groups` metrics + `intent_transaction_channel_membership`
  Info finding + human-summary line). Metadata only, not lowered to `.isf`. **Gates ALL GREEN:** ADR-0006 ✓;
  **WIRE-BASED-100 PROVABLY ORTHOGONAL** — `git stash` baseline-vs-`.2m` AXI evidence diff shows ONLY
  `signal_channel_memberships` (154) changed, all wire-gold surfaces byte-identical, and the emitted `.isf` is
  byte-identical (emitter lowers `tx.steps` only — confirmed by adapting with vs without the metadata) ✓;
  `kg-bench` **156/156** ✓; `run_ci.sh` GREEN (fmt + warning-deny clippy/tests/rustdoc + mdBook; lib **1664**,
  +2; one `collapsible_if` let-chain fixed) ✓. Live: AXI `with_channel_membership: 5 (10 channel groups)`,
  filling its empty `phase_membership` deterministically. Book `pipeline/intentir.md` "Grouping a transaction's
  signals by channel"; KM card `transaction-channel-membership`. Tree stays `active`.
  `[[project_kg_isf_transactions]]` / `[[feedback_scoring_rigor]]` / `[[feedback_multi_strategy_best_wins]]`.
- `2026-06-17`: **`.2n` DONE — measurement-first, read-only, docs-only; NO-GO (the transaction ISF body is
  faithfully complete).** Ran the `2026-06-17`-triage-named "buildable transaction body lever" as a FRESH probe-first
  cycle on FSMGen pin `030f8c273`. Four `--strict --check --json` probes: a value-free `(on start (sample S as s))`
  body PASSES (A), as do multiple same-state samples (D), and `(sample …)` is NOT direction-gated (B); an in-body
  `(drive NAME 0)` with no top-level named drive FAILS `drive 'NAME' not defined` (C — an in-body drive is a CALL to
  a top-level named drive, why `.2b` works). **FSMGen-accepted ≠ faithful:** `13b-transactions.md` (L133–163) shows
  a sample inside `(on …)` fires at the entry transition ("Cycle N: `port && can_accept` → samples captured") and
  needs an activation guard — so a membership-derived sample body asserts an un-grounded entry-cycle capture + guard
  the document never states (and could misrepresent wait-for-ready as a one-shot sample), the fabrication FSMGen's
  `2026-06-16` answer + the honest-residual doctrine forbid. **NO-GO:** the only body-lowerable grounded fact (the
  enum-selector drive) already ships (`.2b`); the membership a sample body would carry already lives faithfully as
  IntentIR metadata (`.2c`/`.2i`/`.2m`). Reconfirms the `.2i` parking with fresh `030f8c273` evidence (prior probe
  `8c39827f`). Gates: read-only/docs-only — WIRE-BASED-100 untouched ✓, ADR-0006 ✓. KM card
  `transaction-body-emission-faithfully-complete`. Next substantive north-star lever is OUTSIDE this tree —
  interface signal DIRECTION emission (`KG-ISF-COMPLETENESS.2`), owner-design-gated. `[[project_kg_isf_transactions]]`.
