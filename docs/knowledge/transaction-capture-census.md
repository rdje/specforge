---
id: transaction-capture-census
title: SpecForge's IntentIR transactions[] surface is THIN and MIS-LEVELLED — 63 entries/16 of 36 docs are per-channel handshakes (phases, not transactions) + per-actor behavior blobs (not transactions) + 3 hardcoded ADR-0006-breaching recognizers; composed named transactions (AXI read=AR→R, write=AW→W→B, AHB transfer-types, APB/SWD read/write) are a 100% gap; the G2-first de-hardcode recognizer keys off section anchors + signal-valued enumeration tables already in EvidenceIR
answers:
  - "how does specforge currently capture transactions and why is it thin"
  - "what are the 3 gaps G1 G2 G3 in specforge transaction capture"
  - "why are the IntentIR transactions[] entries not real transactions (handshakes/behavior blobs)"
  - "what is the KG-ISF-TRANSACTIONS census / transaction-capture baseline"
  - "where does recognize_digital_patterns hardcode HTRANS/PSEL/MISO and why is it an ADR-0006 breach"
  - "why does the project README get spurious ahb_transfer/apb_transfer transactions"
  - "what structural cues recognize transactions universally (section anchors + enumeration tables)"
  - "how many transactions does each persisted IntentIR doc have (AXI=9, AHB=3, APB=3)"
  - "what is the owner directive on transaction recognition / membership / step-by-step / fast / minimum"
date: 2026-06-16
tags: [kg-isf-transactions, transactions, intent-ir, isf, adr-0006, measured, north-star, ir-intent, recognize-digital-patterns, baseline]
evidence: docs/research/transaction-capture-census.md (full census §1–§4); docs/tasks/KG-ISF-TRANSACTIONS.md (sharpened 7-point bar + .2a/.2b/.2c slices); crates/specforge/src/ir/intent.rs (synthesize_transactions + recognize_digital_patterns, the 3 hardcoded Patterns 3/4/5); generated/intent_ir/*/intent_ir.json (the transactions[] surface scanned)
reverify: "Read docs/research/transaction-capture-census.md (the report is the authority — do NOT re-derive §1–§3; §4.1 records the .2a outcome). POST-.2a (2026-06-16): the 3 hardcoded recognizers are GONE — grep -nE '\\\"HTRANS\\\"|\\\"PSEL\\\"|\\\"MISO\\\"|ahb_transfer|apb_transfer|spi_transfer' crates/specforge/src/ir/intent.rs finds them ONLY in the removal comment + the cfg(test) module, never production. Named recognition now lands: after rebuilding (semantic then intent), for d in generated/intent_ir/*/intent_ir.json; do jq '[.transactions[]|select(.transaction_id|startswith(\"txn_named_\"))]|length' \"$d\"; done prints AHB=7 / APB=2 / AXI=14 / SWD=8 / readme=0; jq on the AHB doc shows idle_transfer with port HTRANS at confidence high (Cue-B corroborated). POST-.2b (2026-06-16): the per-actor *_behavior synthesis is GONE — grep 'fn render_temporal_predicate\\|fn temporal_consequent_to_step\\|_behavior\\\", actor_name' crates/specforge/src/ir/intent.rs finds them ONLY in removal comments; after rebuilding a wire doc (intent then adapt), jq '[.transactions[]|select(.transaction_name|endswith(\"_behavior\"))]|length' is 0, and the Cue-B subset now carries a body — jq on the AHB doc shows idle_transfer with steps=[{drive HTRANS IDLE}]; subs/fsmgen/bin/fsmgen --strict --check --json on the APB .isf returns success:true (the *_behavior 'when body clauses must be list forms' class is cleared). POST-.2c (2026-06-16, bounded batch COMPLETE): each named transaction carries a grounded signal_set — after rebuilding a wire doc (semantic then intent), jq '.transactions[]|select(.transaction_id|startswith(\"txn_named_\"))|{n:.transaction_name, ports:[.ports[]?.port_name]}' on the AHB doc shows basic_transfer→[HCLK,HRDATA,HREADY,HREADYOUT,HWDATA,HWRITE], burst_operation→[HADDR,HBURST,HSIZE], idle_transfer→[HTRANS,HREADY]; enum values (IDLE/INCR4) are filtered (TransactionAnchorRecord.signal_set ∩ declared_signal_names in build_transaction_anchors). The ordered multi-phase BODY (phase sequencing over the membership) + finer role sub-typing remain beyond the batch."
---

**Measured `2026-06-16` (`KG-ISF-TRANSACTIONS.1`, read-only, docs-only).** The baseline census of how
SpecForge captures transactions today, why it is insufficient for faithful `.isf` lowering, and the
designed first code slice. The full authority is `docs/research/transaction-capture-census.md`; this card
is the greppable summary so the next session does not re-excavate it.

> **STATUS UPDATE — `.2a` LANDED (`2026-06-16`):** G2 is fixed. The 3 hardcoded recognizers are REMOVED
> and replaced by a structural universal recognizer (`recognize_named_transactions`/`mint_named_transaction`
> in `ir/intent.rs`, fed by a new `SemanticIr.transaction_anchors` surface built by
> `build_transaction_anchors`/`derive_transaction_name` in `ir/semantic.rs`; head nouns in
> `normative_vocab::TRANSACTION_HEAD_NOUNS`). **Cue A** (section-heading names) is threaded
> EvidenceIR→SemanticIR→IntentIR (section anchors are EvidenceIR-only); **Cue B** (`symbol_definitions`
> signal-keyed enums) corroborates. Live: **212 named transactions / 42 docs** (AHB 7, APB 2, AXI 14,
> SWD 8; `readme` 0 — spurious firing gone). So **G1's *named-recognition* gap is closed; only the
> composed multi-phase *bodies* remain** (`.2b` — recognition-only txns carry no `steps` and are held out
> of the `.isf` by the `!steps.is_empty()` emit filter until then). G3 signal-set membership = `.2c`.

> **STATUS UPDATE — `.2b` LANDED (`2026-06-16`):** two faithful changes in `ir/intent.rs`. **(1)
> Re-levelling:** the per-actor `{actor}_behavior` synthesis is REMOVED (with its two orphaned helpers
> `render_temporal_predicate`/`temporal_consequent_to_step`) — census §3.6 phantoms built entirely from
> `temporal_rules` (already lowered to valid `.isf` via the temporal path), and `.isf`-invalid (their
> multi-word `when` condition `HREADY == HIGH @PreTick` tripped FSMGen's `when body clauses must be list
> forms`). This clears that strict-error class corpus-wide with no semantic loss. **(2) Composed body:**
> `mint_named_transaction` now composes a grounded `(drive signal value)` body for the Cue-B-corroborated
> subset (a transaction named after an enumerated value of a declared signal — `idle_transfer` ⟺
> `HTRANS = IDLE`), so it RENDERS to `.isf`. **Measured re-scoping:** the section-named transactions and the
> per-channel handshakes have NO structural name bridge, so faithfully re-levelling handshakes into the
> *right* named transaction needs the grounded signal-set membership `.2c` owns — deferred there, not
> fabricated (bar #3). Live: `*_behavior` blobs now 0 corpus-wide; **APB strict-FAIL → PASS**; AHB
> `idle_transfer` renders + is strict-valid; remaining wire-doc strict fails are PRE-EXISTING non-transaction
> rule/enum issues `.2b` only unmasked. So **the composed *bodies* gap is now partly closed (the
> enum-grounded subset) — the membership-grounded bodies for the rest are `.2c`.**

> **STATUS UPDATE — `.2c` LANDED (`2026-06-16`; bounded batch `.2a→.2b→.2c` COMPLETE):** each named
> transaction now carries its grounded signal-set MEMBERSHIP (bar #3/#4). `TransactionAnchorRecord` gains
> `signal_set` (`ir/semantic.rs`); `build_transaction_anchors` computes it as the union of the section
> statements' signal-shaped tokens (`StatementContext.signals`) **∩ the declared-signal inventory**
> (`declared_signal_names`, from `interfaces[].signal_records`). The intersection is ESSENTIAL — the raw
> token extractor over-captures enum VALUES (`IDLE`/`INCR4`/`NONSEQ`) + abbreviations (`MPMC`/`AHB5`); keeping
> only declared signals makes the set faithful (a signal is a member iff the section references it AND the
> doc declares it — bar #3). `mint_named_transaction` (`ir/intent.rs`) attaches each member as a port with a
> relation-grounded direction (Drives→output/Reads→input). Live AHB: `basic_transfer`→6 signals,
> `burst_operation`→{HADDR,HBURST,HSIZE}, `idle_transfer`→{HTRANS,HREADY}. No ISF/strict change (membership is
> `ports` metadata; emitter lowers `steps`). **Deferred beyond the batch:** the ordered multi-phase BODY
> (phase sequencing over the membership) + finer address/data/control/response role sub-typing.

## What the `transactions[]` surface holds today

IntentIR HAS a typed `transactions` surface (`TransactionIntent` in `ir/intent.rs`: ports,
`activation_port`, ordered `steps`) that lowers to ISF `(transaction …)` with FSMGen-contract-exact
grammar. But it is built fresh at the IntentIR stage by `synthesize_transactions` +
`recognize_digital_patterns` from exactly three forms — **63 entries across 16 of 36 persisted docs**:

- **per-channel `*_handshake`** (34/63) — a `*VALID`/`*READY` pair → a 2-port `await_all`+`sample`
  micro-transaction (AXI `aw_handshake`/`ar_handshake`/`w_handshake`/`b_handshake`/`r_handshake`/
  `ac_handshake`/`cr_handshake`). **This is a transaction PHASE/STEP, not a transaction.**
- **per-actor `*_behavior`** (24/63) — temporal rules bagged by driving actor → `Manager_behavior`
  (0 ports, `activation_port=null`, 32 `when`-steps). **Not a transaction at all.**
- **3 hardcoded `*_transfer`** (5/63) — `recognize_digital_patterns` literally tests `HTRANS`/`HREADY`/
  `HADDR`, `PSEL`/`PENABLE`/`PREADY`, `(MISO|MOSI)&(SCLK|SCK)` → `ahb_transfer`/`apb_transfer`/
  `spi_transfer` (with hardcoded widths 2/1/32 and the literal enum values `NONSEQ`/`SEQ`). Only this
  form attempts the right level — and it is the ADR-0006-breaching one.

## The three gaps (quantified)

- **G1 — composed named transactions: a 100% gap.** `composed-named = 0` in EVERY doc. Not one document
  carries the named multi-phase transactions the spec defines (AXI read=AR→R / write=AW→W→B; AHB
  IDLE/BUSY/NONSEQ/SEQ; APB read/write; SWD read/write operation; CHI ReadClean/WriteBackFull/…).
- **G2 — ADR-0006 breach, live and demonstrably non-agnostic.** The hardcoded recognizers fire on AHB/APB
  AND **spuriously on `readme`** (a non-spec doc gets 2 `*_transfer` purely because its prose mentions
  `HTRANS`/`PSEL`). `spi_transfer` fires on no corpus doc (dead-but-brittle). Must become STRUCTURAL.
- **G3 — signal-set membership incomplete.** handshake = 2 ports; behavior = 0 ports; `ahb_transfer` = 3
  of ~12 AHB transfer signals. No form enumerates a transaction's full signal set with roles, nor bounds
  what is / is not part of transaction X.

**Ontology:** the surface is MIS-LEVELLED, not merely thin — phases (handshakes) and actor-behaviour are
mislabelled as top-level transactions, so the G1 fix is also a re-levelling (named transaction = parent;
today's handshakes = its child steps; behavior blobs re-attributed to the constraints they describe).

## First code slice (designed, measurement-first) — `.2a` G2-first

Replace the 3 hardcoded blocks with a cheap, deterministic, UNIVERSAL structural recognizer keyed off
cues already in EvidenceIR — **no new ingest, no name list (ADR 0006)**:
- **Cue A — section anchors** naming the transactions: APB `3.1 Write transfers`/`3.3 Read transfers`;
  AHB `Chapter 3 Transfers`/`3.2 Transfer types`; SWD/debug `B4.2.1 Successful write operation`/`B4.2.2
  Successful read operation`. (`readme` has none → the spurious firing self-kills.)
- **Cue B — signal-valued enumeration tables** in `extracted_statements`: AHB's `| HTRANS[1:0] | Type |
  Description |` with rows `| 0b10 | NONSEQ | … |`. (EvidenceIR has NO dedicated `enum_definitions`
  surface — recover from `extracted_statements` table text + `section_anchors`.)

Then `.2b` (G1: compose step-by-step body via ISF `spawn`/`do`) → `.2c` (G3: full signal-set membership,
COMPLETE and EXCLUSIVE). Each measurement-first; WIRE-BASED-100 (APB/AHB/AXI/SWD) + ISF round-trip
(`adapt --target isf` → FSMGen `--strict --check`) as hard gates.

## Owner directive (the requirement this serves)

`2026-06-16`, multi-message: transaction recognition is of **utmost importance** and a **MINIMUM /
critical-path prerequisite** — *"without this we can't move forward in our PDF→ISF tool."* SpecForge must
**very quickly** (fast, deterministic, structural) identify the transactions in **ANY** chip-spec PDF
(protocol or platform); precisely bound **membership** (*what is part of transaction X and what is not*);
and describe each transaction **step by step**, for **all** transactions. See
`[[project_kg_isf_transactions]]` / `[[project_kg_isf_completeness]]` and the sharpened 7-point bar in
`docs/tasks/KG-ISF-TRANSACTIONS.md`.
