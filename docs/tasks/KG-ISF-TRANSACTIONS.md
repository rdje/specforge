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
- ID: `KG-ISF-TRANSACTIONS.2b` · Status: `pending` · Goal: **G1 — composed named transactions
  (step-by-step body + re-levelling).** Compose each recognised transaction's ordered phases/steps via ISF
  `spawn`/`do`+`await_all`, re-levelling today's per-channel handshakes into the named transaction's CHILD
  steps (bar #5 step-by-step; #3 boundary precision for steps). Measurement-first; WIRE-BASED-100 + ISF
  round-trip gates.
- ID: `KG-ISF-TRANSACTIONS.2c` · Status: `pending` · Goal: **G3 — signal-set membership + boundary
  precision.** Enumerate each transaction's full signal set with roles (address/data/control/response),
  COMPLETE and EXCLUSIVE (what is part of X and what is NOT — bar #3/#4), grounded in the document's
  channel/phase grouping. Measurement-first; WIRE-BASED-100 + ISF round-trip gates.
- ID: `KG-ISF-TRANSACTIONS.2d?` · Status: `candidate` · Goal: **quick-surface** — a `validate`
  transaction-inventory metric + finding (and/or CLI surface) so an operator can "very quickly identify"
  a doc's recognised transactions at a glance (from the owner's "very quickly identify" directive).
  Design alongside `.2a`–`.2c`; not yet scoped.

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
