# Transaction-capture census (`KG-ISF-TRANSACTIONS.1`)

Read-only measurement (`2026-06-16`), gathered before any transaction code. Captures the ISF
**target** model, how protocol/platform PDFs **define** their supported transactions (source form),
the **current** SpecForge capture, the per-doc gap quantification, and the designed first code slice —
so the first code slice is evidence-driven. **`.1` COMPLETE** (`2026-06-16`): §1 target model + §2
source-form census + §3 current-capture census + §3.5 per-doc G1/G3 quantification (corpus-wide,
read-only over the 36 persisted IntentIR artifacts + their EvidenceIR cues) + §4 first-slice design.

## 1. ISF target model — what `(transaction …)` supports (FSMGen, pinned `c0b7eaa7`)

From `subs/fsmgen/docs/book/src/12-cookbook.md` (§9–§12) and `13c-drive-blocks.md`:

```lisp
(transaction NAME
  (on TRIGGER)            ; activation: an input port (or `start`)
  <body steps…>
  (complete PORT))
```

Body step kinds: `(wait N)`, `(drive sig val)`, `(await port)`, `(sample port as name)`,
`(when cond …)`, `(shift_left …)`/`(shift_right …)`, and — crucially — **composition**:
`(spawn child as inst)` + `(await_all done)` (generated child + barrier), `(do child (params …))`
(blocking call), and actor-level `(rule NAME trig (trigger TXN))` (rule-activated once per pulse).

**Key consequence:** the ISF model ALREADY supports **composed / hierarchical transactions** — a
parent transaction can `spawn`/`do` child transactions. So a composed AXI *read* (AR-request phase →
R-data phase) or *write* (AW → W → B) IS expressible as a parent that sequences per-phase children.
The gap (G1) is purely that SpecForge does not BUILD such composed transactions — not that ISF can't
represent them.

## 2. Source form — how the corpus DEFINES supported transactions (5-doc census)

| Doc | "Transactions" section(s) | Enumeration form | Named transactions | Signal-set conveyance |
|---|---|---|---|---|
| **AXI** | Ch A3 "AXI transactions"; A1.3.2 "AXI transactions and transfers" (def: *"a read transaction consists of a request transfer and one or more read data transfers"*); Ch A7 "Request Opcodes" | **Opcode tables** (A7.3/A7.4, AxSNOOP, 20+ opcodes) + attribute qualifiers (burst/size/len/domain) | read/write transaction; ReadNoSnoop/ReadOnce/WriteUnique/… opcodes | **Channels** AW/W/B/AR/R (request/data/response separated) |
| **AHB** | Ch3 "Read and write transfers"; 3.2 "Transfer types" | **Enumerated signal-field values** — Table 3-1 HTRANS 4 types (IDLE/BUSY/NONSEQ/SEQ); Table 3-4 HBURST 6 burst types | IDLE/BUSY/NONSEQ/SEQ; SINGLE/INCR/WRAP4/… | Single addr/data bus + sidebands; address phase → data phase |
| **APB** | Ch3 "Transfers"; 3.1 "Write transfers"; 3.3 "Read transfers" | **Prose + timing diagrams** (no type table) | write transfer, read transfer (by PWRITE) | Setup phase (PSEL/PADDR/PWRITE/PWDATA) → access phase (PENABLE/PREADY/PRDATA/PSLVERR) |
| **I2C** | 3.1.4 "START and STOP conditions" (*"All transactions begin with a START (S) and are terminated by a STOP (P)"*) | **Sequence/state prose** (bit-level) | START/repeated-START/STOP; read/write (R/W bit); ACK/NACK | Serial SDA/SCL; START → addr+R/W → ACK → data → ACK → STOP |
| **CHI** (platform-ish) | B2.3 "Transaction structure"; B2.3.1 "Read transactions"; .2 Write; .3 Atomic; .4–.7 Stash/Dataless/Prefetch/DVM | **Named-type table** (Table B1.2, 20+ types) + per-type subsection w/ flow diagram | ReadClean/ReadShared/WriteBackFull/AtomicStore/… | Packet-based: Request/Data/Response/Snoop; SrcID/TxnID/Opcode |

**Three recurring structural forms (the universal recognition targets, ADR 0006 — no name lists):**
1. **Enumeration tables** — an opcode / transfer-type / command column mapping an encoding to a
   symbolic transaction name + description (AXI opcode tables, AHB HTRANS/HBURST tables, CHI Table B1.2).
2. **"transaction/transfer/operation" SECTION headings** that name the transactions (read/write/…)
   and a def-sentence *"a X transaction consists of …"* (all five).
3. **Prose + timing/flow diagrams** giving the phase sequence and the per-phase signal set (APB, I2C,
   CHI flow diagrams).

A universal extractor should key off these structural cues (section headings naming transaction/transfer
+ enumeration-table column vocabulary + the def-prose pattern + handshake/phase structure), and recover
the signal set from the document's channel/phase grouping — never from a hardcoded protocol name.

## 3. Current SpecForge capture (the gap, restated with evidence)

Built fresh at IntentIR (`synthesize_transactions` + `recognize_digital_patterns`, `ir/intent.rs`):
per-channel valid/ready handshakes (`aw_handshake`…), per-actor temporal "behavior" blobs
(`Manager_behavior`), and 3 HARDCODED protocol recognizers (`HTRANS`/`PSEL`/`MISO`… → `ahb/apb/spi_transfer`).
55 txns / 15 of 35 docs. So NONE of §2's named, composed, signal-set-bearing transactions are captured
as such.

- **G1** — the composed named transactions (read=AR→R, write=AW→W→B, the opcode/transfer-type catalogue)
  are missing; only per-channel handshakes exist. ISF can already represent them (§1 spawn/do).
- **G2** — the only protocol recognition is hardcoded spec names (ADR-0006 breach) → replace with a
  structural recognizer keyed off §2's forms.
- **G3** — transactions aren't tied to their full signal set with roles (address/data/control/response).

## 3.5 Per-doc G1/G3 quantification (corpus-wide, `2026-06-16`)

Read-only scan of the `transactions` array on all 36 persisted IntentIR artifacts, classifying each
transaction by its synthesis form (`*_behavior` per-actor blob · `*_handshake` per-channel · the 3
hardcoded `*_transfer` recognizers · **`other` = a composed/named transaction**), with the max ports &
steps any transaction in the doc carries.

| Doc (key prefix) | txns | behavior | handshake | hardcoded | **composed (`other`)** | max ports | max steps |
|---|---:|---:|---:|---:|---:|---:|---:|
| `100336…gic_600` (platform) | 8 | 0 | 8 | 0 | **0** | 2 | 2 |
| `101542…mmu_700` (platform) | 2 | 0 | 2 | 0 | **0** | 2 | 2 |
| `ihi0022_h_c…axi_and_ace` | 11 | 3 | 8 | 0 | **0** | 2 | 10 |
| `ihi0022_l…axi` | 9 | 2 | 7 | 0 | **0** | 2 | 32 |
| `ihi0024_d…apb` | 3 | 2 | 0 | 1 | **0** | 3 | 8 |
| `ihi0024_e…apb_5` | 3 | 2 | 0 | 1 | **0** | 3 | 9 |
| `ihi0032…trace_bus` | 3 | 1 | 2 | 0 | **0** | 2 | 2 |
| `ihi0033…ahb_5` | 3 | 2 | 0 | 1 | **0** | 3 | 6 |
| `ihi0051…axi_stream` | 2 | 1 | 1 | 0 | **0** | 2 | 2 |
| `ihi0068…low_power` | 3 | 3 | 0 | 0 | **0** | 0 | 2 |
| `ihi0074…debug_v6` (incl. SWD) | 1 | 1 | 0 | 0 | **0** | 0 | 1 |
| `ihi0079…cxs` | 1 | 1 | 0 | 0 | **0** | 0 | 2 |
| `ihi0083…generic_flash_bus` | 2 | 2 | 0 | 0 | **0** | 0 | 7 |
| `ihi0089…lti` | 3 | 3 | 0 | 0 | **0** | 0 | 1 |
| `jesd235a…hbm2` | 1 | 1 | 0 | 0 | **0** | 0 | 2 |
| `readme` (NON-spec smoke doc) | 8 | 0 | 6 | **2** | **0** | 3 | 2 |
| **TOTAL** | **63** | **24** | **34** | **5** | **0** | — | — |

(The other 20 of 36 docs carry **zero** transactions — register/guide-class docs with no
handshake/actor/temporal surface to synthesize from; honest absence, not a measured miss.)

## 3.6 Ontology note — the surface is MIS-LEVELLED, not just thin

A subtlety worth stating plainly,
because the word "transaction" is doing double duty: the 63 entries are what the **current code** labels
as transactions, not transactions as the specs define them. Read against §2, the existing
`transactions[]` surface conflates three *different* granularities under one type and only one of them is
even attempting the right level:
- a **`*_handshake`** (34/63) is a transaction **phase / handshake step** — a `*VALID`/`*READY` gate that
  belongs *inside* a transaction body (one of its ordered steps), not standing alone *as* a transaction;
- a **`*_behavior`** blob (24/63) is an **actor's aggregate timed behaviour** (a bag of `when`-steps with
  no activation, no ports) — **not a transaction at all**, at any level;
- only a **`*_transfer`** (5/63) even *tries* to be a named transaction — and does it by hardcoding
  (G2), so it is both the only right-levelled form and the ADR-0006-breaching one.

So the defect is not only "composed transactions are missing" (G1) and "signal sets are incomplete"
(G3) — it is that **phases and actor-behaviour are mis-typed as top-level transactions**. The G1 fix is
therefore also a *re-levelling*: the real named transaction becomes the parent, and today's handshakes
become its child *steps* (ISF `spawn`/`do` + the per-channel `await`/`sample`), while the `*_behavior`
bags are re-attributed to the transactions/constraints they actually describe rather than surviving as
phantom "transactions".

**The three gaps, now quantified:**

- **G1 (composed named transactions) — a 100% gap.** `composed = 0` in **every** doc. Not one
  document carries a `read`/`write`/opcode/transfer-type **named** multi-phase transaction. The entire
  catalogue each spec defines (AXI read=AR→R / write=AW→W→B; AHB IDLE/BUSY/NONSEQ/SEQ; APB read/write;
  SWD read/write operation; CHI ReadClean/WriteBackFull/…) is absent from IntentIR.
- **G2 (ADR-0006 breach) — live, and demonstrably non-agnostic.** The 3 hardcoded recognizers fire on
  exactly the 3 docs whose literal names they test (AHB×1 `ahb_transfer`, APB×2 `apb_transfer`) — **and
  spuriously on `readme`** (2 hardcoded `*_transfer` minted purely because the project's own README/task
  prose mentions `HTRANS`/`PSEL` literals). A non-spec document getting protocol transactions is the
  ADR-0006 failure mode made concrete. The `spi_transfer` recognizer fires on **no** corpus doc (no SPI
  doc present) — dead-but-brittle code keyed off `MISO`/`MOSI`/`SCLK`.
- **G3 (signal-set linkage) — incomplete in every form.** A `*_handshake` carries only its `*VALID`/
  `*READY` pair (2 ports — never the channel payload AWADDR/AWID/AWLEN/AWSIZE/AWBURST/…); a `*_behavior`
  blob carries **0 ports** and `activation_port=null` (a 32-step `when`-bag for AXI `Manager_behavior`);
  the hardcoded `ahb_transfer` carries just `[HTRANS, HREADY, HADDR]` (3 of the ~12 AHB transfer
  signals, with **hardcoded widths 2/1/32 and the literal enum values `NONSEQ`/`SEQ`**). No form
  enumerates a transaction's full signal set with address/data/control/response roles.

## 4. First code slice — designed (`.1` DONE; first slice = `KG-ISF-TRANSACTIONS.2a`)

**Decision: G2 first** (per the census-§4 leading order). It removes the ADR-0006 breach immediately,
deletes the spurious `readme` firing, and lays the structural-recognition foundation that G1 (the
composed named catalogue) then builds the multi-phase body onto. It is also the **safest** first slice:
the transaction surface is **not** part of WIRE-BASED-100 (that gate is constraints/relations/temporal
gold), so de-hardcoding cannot regress the wire gold — while ISF round-trip stays a per-slice gate.

**Structural cues are present and grounded (measured `2026-06-16`):** the universal recognition targets
of §2 exist in the already-built EvidenceIR — no new ingest, no name list:
- **Cue A — section anchors naming the transactions.** Wire docs carry them universally:
  APB `3.1 Write transfers` / `3.3 Read transfers` / `Chapter 3 Transfers`; AHB `Chapter 3 Transfers`
  / `3.2 Transfer types`; SWD/debug `B4.2.1 Successful write operation` / `B4.2.2 Successful read
  operation`. `readme` has **none** → the recognizer self-kills the spurious firing.
- **Cue B — signal-valued enumeration tables.** AHB's `extracted_statements` carry the literal table
  `| HTRANS[1:0] | Type | Description |` with encoding rows `| 0b10 | NONSEQ | … |`, `| 0b01 | BUSY |
  … |` — the transfer-type catalogue as structured rows. (Note: EvidenceIR has **no** dedicated
  `enum_definitions` surface; the enumeration is recovered from `extracted_statements` table text +
  `section_anchors`, not a typed enum field — a `.2a` parsing input to confirm.)

**`.2a` plan (measurement-first, its own slice — NO code in `.1`):**
1. **Measure first** (read-only): across the corpus, count docs with (a) a `(read|write|…)
   (transfer|transaction|operation)` section anchor and (b) a signal-valued enumeration table in
   `extracted_statements`; quantify the recall of a structural recognizer vs the 3 hardcoded names, and
   confirm `readme`/non-spec docs yield **zero**. This sizes the precision/recall of the replacement
   before writing it.
2. **Replace** the 3 hardcoded blocks in `recognize_digital_patterns` (`ir/intent.rs`, Patterns 3/4/5,
   the `HTRANS`/`PSEL`/`MISO` literal tests) with a structural recognizer that mints a
   `TransactionIntent` per transaction **named by the document's own section-anchor / enumeration
   vocabulary** (no literal `ahb_transfer`/`apb_transfer`/`spi_transfer` strings — ADR 0006). Scope `.2a`
   to **recognition + naming**; the composed multi-phase body + full signal set are G1 (`.2b`) / G3
   (`.2c`).
3. **Gates:** ADR-0006 (zero hardcoded names, universal grammar); WIRE-BASED-100 (APB/AHB/AXI/SWD) held
   1.000 on fresh-Pattern temp-evidence-root eval; ISF round-trip (`adapt --target isf` → FSMGen
   `--strict --check --json`) on the affected docs; `readme` spurious `*_transfer` **gone** (precision
   win); `kg-bench` green; `run_ci.sh` green before declaring the slice done.

**Then G1 (`.2b`)** — compose per-phase children into named read/write transactions via ISF `spawn`/`do`
(the ISF model already supports it, §1; this is also the re-levelling of §3.6 — the named transaction
becomes the parent, today's handshakes its child steps); **then G3 (`.2c`)** — enumerate each
transaction's full signal set with roles (address/data/control/response), **COMPLETE and EXCLUSIVE** (what
is part of transaction X and what is NOT), reusing the channel/phase grouping the doc already declares.

### 4.1 `.2a` OUTCOME (`2026-06-16`, owner-chosen "both cues") — DONE

The design above was implemented as `KG-ISF-TRANSACTIONS.2a`. Two findings from the slice's own
measurement refined the §4 plan:

- **Data-flow gap RESOLVED.** The §4 cues were measured at the *EvidenceIR* vantage, but
  `recognize_digital_patterns` runs at the *IntentIR* stage from `SemanticIr` **alone** — and `SemanticIr`
  does **not** carry `section_anchors`/`extracted_statements` (those are EvidenceIR-only). So **Cue A had to
  be threaded** EvidenceIR→SemanticIR→IntentIR: a new typed `SemanticIr.transaction_anchors`
  (`TransactionAnchorRecord`), built in the SemanticIR builder where `evidence_ir.section_anchors` is in
  hand. **Cue B was already reachable** — `SemanticIr.symbol_definitions` carries the signal-keyed
  enumeration tables directly (AHB `HTRANS`→{IDLE,BUSY,NONSEQ,SEQ}), so no enum threading was needed; it is
  used as corroboration (member match → attach keyed signal + raise confidence).
- **Cue A precision needed a tight rule.** The broad §2 cue (any anchor mentioning transfer/transaction/
  operation) has high recall but low precision (it matches sub-topics like "Write transaction
  dependencies", "Exclusive Transfer restrictions"). The landed rule (`derive_transaction_name`,
  `ir/semantic.rs`) keeps only a transaction-defining noun phrase: head noun **final** ∈
  {transfer,transaction,operation} (centralized in `normative_vocab::TRANSACTION_HEAD_NOUNS`), with generic
  function-word / cardinal / gerund / `Example`-prefix discriminators (universal English grammar, no name
  list — ADR 0006).

**Measured result:** **212 named transactions across 42 docs** (AHB 7 incl. `idle_transfer` Cue-B-
corroborated→`HTRANS`; APB `read_transfer`/`write_transfer`; AXI 14; SWD 8; CHI full catalogue), `readme`
and non-spec docs **0** (the spurious hardcoded firing eliminated). Gates green: ADR-0006, WIRE-BASED-100
byte-identical before/after, ISF round-trip 0 blockers + 0 new strict diagnostics, `kg-bench` 156/156,
`run_ci.sh`. Recognition-only transactions carry no `steps`, so the ISF emitter's `!steps.is_empty()`
filter holds them out of the `.isf` until `.2b` gives them composed bodies — no fabrication, no silent
break. See `docs/tasks/KG-ISF-TRANSACTIONS.md` (`.2a` node) for the full verification log.

### 4.2 `.2b` OUTCOME (`2026-06-16`) — composed bodies + `*_behavior` re-levelling, with a measured re-scoping

`KG-ISF-TRANSACTIONS.2b` was implemented measurement-first, and the measurement **refined** the original
`.2b` plan ("re-level today's per-channel handshakes into the named transaction's child steps"). Two
findings reshaped it:

- **The `*_behavior` blobs are a redundant, `.isf`-invalid second rendering of `temporal_rules`.** Confirmed
  by reading the AHB/APB ISF round-trip directly: the single AHB strict diagnostic was
  `Transaction 'subordinate_behavior': when body clauses must be list forms` — its multi-word `when`
  condition (`HREADYOUT == HIGH @PreTick`) is tokenised by FSMGen's S-expression parser into scalar body
  clauses. The blob's content is built entirely from `temporal_rules`, which are *also* lowered (validly) as
  `txn_temporal_*` asserts / `(rule …)`. So the fix is to **drop the blob** (census §3.6 re-levelling): it
  clears the strict-error class corpus-wide and loses no temporal semantics. (The `(priority … over …)`
  cross-product iterates the transaction list, so removal leaves no dangling refs.)
- **The section-named transactions and the per-channel handshakes have NO structural name bridge.** The AXI
  named transactions are `axi_transaction`, `narrow_transfer`, `atomic_transaction`, … (section-anchor
  qualifiers); the handshakes are `aw/ar/w/b/r_handshake` (channel prefixes). Mapping "which handshakes are
  which named transaction's children" needs AXI channel-semantics knowledge — a name list, an ADR-0006
  breach — and doing it by guess would fabricate boundary attributions (bar #3, owner-elevated to "of utmost
  importance"). That mapping depends on the grounded signal-set membership `.2c` owns, so the broad composed
  body is **correctly sequenced into `.2c`**, not fabricated in `.2b`.

**What `.2b` delivered (faithful, universal, boundary-exact):** (1) removed the per-actor `{actor}_behavior`
synthesis (and its two now-orphaned helpers); (2) `mint_named_transaction` composes a grounded
`(drive signal value)` body for the Cue-B-corroborated subset — a transaction named after an enumerated
value of a declared signal is defined by driving that signal to that value (`idle_transfer` ⟺
`(drive HTRANS IDLE)`), so it RENDERS to `.isf`. **Measured (regenerated wire/protocol docs):** `*_behavior`
blobs now 0 in every doc; **APB strict-FAIL → strict-PASS** (`success:true`, 0 diagnostics — its only blocker
was the behavior error); AHB `idle_transfer` renders + is strict-valid; the remaining wire-doc strict failures
are PRE-EXISTING, non-transaction rule/enum-lowering issues `.2b` only unmasked (AHB HAUSER rule-write
conflict; AXI/axi-and-ace/lti `constraint_*` assignment-action grammar; axi-stream/generic-flash rule-write
conflicts; trace-bus `ATID` width contract; hbm2 enum-member emission — candidate future slices). Gates:
ADR-0006 ✓, WIRE-BASED-100 constraint+temporal F1 = 1.000 ✓ (orthogonal), ISF round-trip — behavior strict
class eliminated + 0 new transaction diagnostics ✓, `kg-bench` 156/156 ✓, `run_ci.sh` green ✓.

### 4.3 `.2c` OUTCOME (`2026-06-16`) — grounded signal-set membership (bounded batch COMPLETE)

`KG-ISF-TRANSACTIONS.2c` attaches each named transaction's grounded signal SET (bar #3/#4), completing the
bounded `.2a→.2b→.2c` batch. The membership is the declared signals the transaction's OWN defining section
references — computed as the union of `StatementContext.signals` over the anchor's supporting statements,
**intersected with the document's declared-signal inventory** (`declared_signal_names`, from
`interfaces[].signal_records`), stored on the new `TransactionAnchorRecord.signal_set` and attached by
`mint_named_transaction` as `TransactionPortRecord`s with a relation-grounded direction (Drives → output,
Reads → input, both/neither → in/out).

**The slice's key measurement finding:** the raw per-statement signal tokens (`extract_signal_tokens`)
OVER-CAPTURE — they include enum VALUES (`IDLE`, `INCR4`, `NONSEQ`, `WRAP8`) and prose abbreviations
(`MPMC`, `SWP`, `AHB5`) that are not interface signals. Attaching those as transaction "signals" would
breach boundary precision (bar #3) — `IDLE` is a *value* of `HTRANS`, not a signal. The intersection with the
declared-signal inventory is therefore essential and is what makes the set faithful: a signal is a member
iff the section's text references it AND the document declares it as a signal. This was caught by a direct
before/after measurement (the unfiltered set surfaced `IDLE`/`INCR4`/`MPMC`; the filtered set is clean).

**Measured (live AHB):** `basic_transfer`→{HCLK,HRDATA,HREADY,HREADYOUT,HWDATA,HWRITE},
`burst_operation`→{HADDR,HBURST,HSIZE}, `locked_transfer`→{HMASTLOCK,HREADY}, `idle_transfer`→{HTRANS,HREADY}
(plus its `.2b` `(drive HTRANS IDLE)` body), `secure_transfer`→{HNONSEC}, `waited_transfer`→{HREADYOUT},
`exclusive_transfer`→{} (honest empty — its single statement references no declared signal). Boundary-precise:
shared signals such as `HREADY` are attributed to each transfer whose section references them (they genuinely
participate); nothing from a different transaction's section bleeds in. **No ISF/strict change** — membership
is IntentIR `TransactionIntent.ports` metadata and the ISF emitter lowers `steps`, not `ports`, so emitted
`.isf` + FSMGen `--strict` are byte-identical to `.2b`. Gates: ADR-0006 ✓, WIRE-BASED-100 constraint+temporal
F1 = 1.000 ✓ (orthogonal), ISF round-trip 0 new diagnostics ✓, `kg-bench` 156/156 ✓, `run_ci.sh` green ✓.
**Deferred beyond the batch (honest):** the ordered multi-phase BODY (sequencing the membership signals into
address/data/response phases + the gating handshakes) — the membership is its prerequisite, now delivered;
and finer address/data/control/response role sub-typing.

### 4.4 `.2f` OUTCOME (`2026-06-16`) — ordering-signal choice for the multi-phase body (measurement-first)

The `.2e` checkpoint deferred *which* ordering signal grounds the body to "a dedicated measurement-first slice."
`KG-ISF-TRANSACTIONS.2f` ran that measurement corpus-wide (read-only over the 36 persisted IntentIR/SemanticIR
artifacts + 78 EvidenceIR artifacts; no code) and made the choice. The three `.2e` candidates were tested:

- **(b) `temporal_rules` ordering — REJECTED.** Direct scan of the persisted rules: they are per-signal STABILITY /
  value obligations (`signal_stable`, `actor_drives_signal`, `signal_value`), with mostly `cycle_window=none`, and
  they do NOT form phase-sequencing edges across a transaction's membership. AHB `burst_operation`→{HADDR,HBURST,
  HSIZE} is referenced by exactly one rule (HSIZE-stable, no window); AHB `basic_transfer`'s rules touch only
  HREADYOUT/HREADY (the wait/handshake signals), not an HADDR→HWDATA→HRDATA order; AXI's 45 rules are sideband
  stability constraints (`WTAGUPDATE`, `AWIDUNQ`, `AWSNOOP`, …). Nothing encodes address→data→response ordering.
- **The SemanticIR `phases` surface — NOT the protocol's transaction phases.** It exists (AHB: 76 records) but is
  SECTION-derived: one "phase" per chapter heading (`phase_chapter_5_subordinate_response_signaling`,
  `phase_chapter_7_clock_and_reset`), with fields `phase_id`/`summary`/supporting ids only — no transaction-phase
  name, no ordering, no signal grouping. It cannot be reused as transaction phases without mislabelling chapters.
- **(c) handshake-dependency chains — still name-bridge-blocked** (re-confirming `.2b`/`.2e`). AXI's 14 section-named
  transactions are all `steps=0 ports=0`; the 7 `*_handshake` micro-transactions carry no grounded precedence among
  them. Bridging "aw→w→b for a write" needs AXI channel semantics = a name list (ADR-0006 breach) + fabricated
  boundary attributions (bar #3).
- **(a) the document's own `<qualifier> phase` structure — CHOSEN (grounded + universal).** The cue is present at the
  EvidenceIR statement level and clean on the wire/protocol docs. Corpus-wide `<qualifier> phase` scan (precision-gate
  candidate qualifiers shown): AHB `address`/`data`; APB(d/e) `setup`/`access`; SWD/debug (`ihi0074`) `address`/`data`/
  `response`/`turnaround`; AXI-and-ACE `address`; trace-bus / avalon / generic-flash / coresight `address`/`data`. This
  is exactly parallel to the shipped `<qualifier> transfer/transaction/operation` transaction-anchor cue, so it can
  reuse the same recogniser machinery. It needs the same precision gate — the raw `<word> phase` scan also catches
  function-word / cardinal / ordinal noise (`the`/`this`/`four`/`first`/`second`/`and`/`for`/…) — and a NEW typed
  surface to lift it (it lives only in raw statements today, not in any typed IR).

**Decision:** the ordered multi-phase body is built on a new structural **transaction-phase surface** keyed off the
document's own `<qualifier> phase` vocabulary — NOT a mint-side reinterpretation of existing data (which would either
fabricate ordering or require a name list). Designed slice path, each measurement-first and ADR-0006-clean:
- **`.2g`** — structural transaction-PHASE recognition (recognition + naming only): a new typed
  `SemanticIr.transaction_phases` (`TransactionPhaseRecord`), recognised by a precision-gated `<qualifier> phase`
  rule; plus a `validate` phase inventory (mirrors `.2d`). No body composition yet. **STEP-1 (input surface) measured
  `2026-06-16`:** the `<qualifier> phase` vocabulary is in `extracted_statements` PROSE, **not** `section_anchors`
  (AHB 0 / APB 0 phase-titled anchors of 172 / 99; SWD 1 `Data transfer phase`) — so the recogniser scans
  `context.statements` (prose n-gram before a `phase`/`phases` head), reusing the `derive_transaction_name`
  *discriminator* idea (a precision gate) but NOT its section-heading input; the gate must reject more (function
  words / cardinals / ordinals / determiners / gerunds) and be tuned by a corpus-wide before/after measurement like
  `.2a` (clean recall of address/data/setup/access/response/turnaround on the wire docs; low noise elsewhere).
- **`.2h`** — phase ORDERING recovery (section/statement order + "during X … during Y" / "then" / "followed by"
  cues); honest residual where the order is not reliably recoverable.
- **`.2i`** — membership-by-phase grouping (a `.2c` member signal belongs to phase P iff P's statements reference it —
  the same intersection technique as `.2c`) + ordered `(transaction … <phase-1 steps> … (complete …))` composition,
  reusing the per-signal direction `.2c` grounds. ISF round-trip + WIRE-BASED-100 hard gates; bar #5 lands here.

### 4.5 `.2g` OUTCOME (`2026-06-16`) — structural transaction-PHASE recognition (measurement-first)

`KG-ISF-TRANSACTIONS.2g` implemented the `.2f`-chosen path's first slice: recognise (recognition only, no body
composition — that is `.2i`) the protocol PHASES a document NAMES in its own prose, the same way `.2a` recognised
transaction NAMES.

**Gate-tuning measurement (read-only over the 78 persisted EvidenceIR; then a live SemanticIR rebuild of the wire
docs).** A RAW `<word> phase`/`phases` scan (token before the head, edge-punctuation trimmed, lowercased) over every
`extracted_statements[].text` exposed the noise classes that a precision gate must reject:

| Class | Examples (count) |
|---|---|
| determiners / demonstratives / quantifiers | `the`(29), `any`(12), `this`(10), `these`(4), `all`(2), `another`(2) |
| cardinals | `two`(9), `three`(5), `ten`(2), `eight`(2), `four`(1) |
| ordinals | `first`(9), `second`(6), `third`(5), `fourth`(2) |
| head noun used as a modifier | `transfer`(16) ← "data **transfer** phase", `transfers`(1) |
| prepositions / conjunctions | `for`(6), `and`(5), `from`(1) |
| symbols / non-alphabetic | `-`(7), `link-up`(6), `|`(6), `pre-boot`(1), `3-1`(1) |
| position / quantity adjectives | `separate`(6), `following`(1) |
| too short | `a`(3), `of`(2), `aw`(1), `rx`(1) |
| sentence/clause boundary | `lanes`, `compliant`, `rate`, `called` (the head began a NEW sentence/clause) |

**The landed gate** (`derive_phase_name`, `ir/semantic.rs`): single token before the head; reject a sentence/clause
boundary (prev token ends in `.`/`:`/`;`/`!`/`?`), non-alphabetic / `<3`-char tokens, the `PHASE_NAME_STOPWORDS`
(determiners/demonstratives/quantifiers/prepositions/cardinals/ordinals/position-adjectives — a stronger
English-grammar stoplist than `TXN_NAME_STOPWORDS`), a transaction head noun used as a modifier
(`transaction_head_singular`), and gerund-led verbs (`len>5 && ends_with("ing")`). Universal grammar, no name list
(ADR 0006). **A plural-rejection heuristic was MEASURED-REJECTED:** `access` ends in `ss`, so an `ends_with('s')` rule
would wrongly drop a genuine APB phase.

**After the gate, on the rebuilt wire docs (`build_transaction_phases` live):**

| Doc | Recognised phases |
|---|---|
| APB (`ihi0024_e`) | `setup`, `access` |
| AHB (`ihi0033_c`) | `address`, `data` (+ `write`×1 — honest low-count) |
| AXI (`ihi0022_l`) | `data` |
| SWD (`ihi0074_a`) | `data`, `response`, `acknowledge`, `turnaround`, `nodata`, `address` (+ `write`×1) |

Clean recall of the protocol phases with the noise classes rejected; other docs recover their own real phases
(equalization/discovery/configuration/initialization/activation/startup/validation/…) with low, doc-local residual
noise. **Surfaced for the operator** via `validate <semantic-ir>`: a `transaction_phases` metric, a
`semantic_transaction_phase_inventory` Info finding (non-empty only — the `PDF-VARIANT-DIGESTION.11` rule), and a
`transaction_phases:` human-summary line.

**Gates:** ADR-0006 ✓; WIRE-BASED-100 orthogonal (additive SemanticIR field) ✓; ISF round-trip byte-identical —
`transaction_phases` is SemanticIR-only and is NOT carried to `IntentIr.transactions` (verified: APB `adapt`
`blocking_reasons: None`; field absent from the rebuilt IntentIR) ✓; `kg-bench` 156/156 ✓; `run_ci.sh` green (lib
**1645**, +3 tests) ✓. Frontier → `.2h` (phase ordering recovery).

> **Precision correction (`2026-08-10`, `CORPUS-COVERAGE.2.43a.ii`):** the wire-only tuning above established
> recall but overclaimed corpus-wide precision. An exact 82-record retained audit found 59 valid / 23 false;
> mixed-vintage current-rule replay found 101 candidates / 31 false. The one-token gate admitted passive `called
> phases`, CAN `phase error`, physical `phase tolerance/relation/modulation`, comma/table adjacency, and similar
> non-head uses. Current recognition additionally requires positive local phrase authority, preserves every
> valid record payload, and emits 70 valid records across a complete 79-EvidenceIR replay. See
> [the correction measurement](transaction-phase-qualifier-precision-measurement.md).

### 4.6 `.2h` OUTCOME (`2026-06-16`) — phase-ordering recoverability (measurement-first, docs-only)

`KG-ISF-TRANSACTIONS.2h` asked the `.2f`-style question for ordering: **is a transaction's phase ORDER reliably +
universally recoverable** from the document, so `.2i` can sequence the per-phase body? Measured read-only over the
persisted EvidenceIR statements of the wire docs; **no code, no extraction change.**

Three candidate ordering signals, scored against the protocol-correct order (APB `setup→access`, AHB
`address→data`, SWD packet `address/request → ack → data`):

| Signal | APB | AHB | AXI | SWD | Verdict |
|---|---|---|---|---|---|
| document first-occurrence order | `setup,access` ✓ | `address,data` ✓ | `data` (1 phase) | `data` before `address` ✗ | right on linear buses, WRONG on SWD |
| same-sentence sequencing-keyword cues (`then`/`followed by`/…) | none | none | none | 3, CONTRADICTORY (`ack↔data`) | too sparse / inconsistent |
| within-sentence positional precedence | none | `address→data` 5 v 2 (CONFLICTING — reverse = pipelining overlap) | none | `address/data` 1 v 1 (TIED) | absent / conflicting / tied |

**No signal is clean AND universal.** First-occurrence is right only because APB/AHB are linear; it inverts on SWD.
Positional precedence has a usable majority on AHB alone (and even there it conflicts, because "the data phase of
one transfer overlaps the address phase of the next" is a real pipelining statement), and is absent or tied
everywhere else.

**Decision (honest-residual doctrine, `[[feedback_scoring_rigor]]`): do NOT fabricate a universal phase order.** A
majority-vote heuristic would "recover" order on only 1 of 4 wire docs (AHB), off a conflicting signal, and be
silent on APB/AXI/SWD — un-demonstrable per-item and gate-risky. So phase ordering is an explicit RESIDUAL: `.2i`
composes per-phase bodies from the `.2c` membership and emits a cross-phase SEQUENCE only where the document
decisively grounds one (not the wire-doc norm), never an invented address→data→response. A richer ordering signal
— timing-diagram left-to-right phase order (VLM-tier), or a per-transaction definition sentence that enumerates the
phases in order — is recorded as a future candidate, not built here.

**Gates:** read-only/docs-only — WIRE-BASED-100 untouched; ADR-0006 ✓ (the signals measured are universal grammar,
no name list). Frontier → `.2i` (membership-by-phase grouping + per-phase body, ordering only where grounded).

### 4.7 `.2i` + `.2j` OUTCOME (`2026-06-16`) — per-phase membership grouping (CODE) + table-column cue (NO-GO)

**`.2i` (CODE, option (a), FSMGen-confirmed):** with FSMGen's `2026-06-16` answer settling the shape (NOT ordered
ISF body steps — a value-less drive is rejected, bodies are source-ordered — but **checked IntentIR metadata**),
`.2i` shipped the grounded per-phase membership grouping. `SemanticIr.TransactionPhaseRecord` gains `signal_set`
(`build_transaction_phases`, the `.2c` intersection technique); `IntentIr.TransactionIntent` gains `phase_membership`
(grouping each transaction's `.2c` membership by phase — member ∈ phase iff the phase's prose references it — reusing
the `.2c` direction); `validate <intent-ir>` reports it. Measured live: AHB faithful (`basic_transfer` →
address:{HREADY}, data:{HRDATA,HREADY,HREADYOUT,HWDATA}; HCLK/HWRITE honestly unphased; HREADY spans both), APB
minimal (access:{PCLK}), AXI & SWD honest-empty. **`.isf` byte-identical ×4 wire docs** (metadata, not `steps`);
gates all green (`kg-bench` 156/156, `run_ci.sh` lib 1645).

**`.2j` (measurement-first, docs-only, NO-GO):** tested whether mining `<qualifier> phase` from TABLE cells/headers
would move AXI/SWD off the empty grouping. Signal↔phase co-occurrence: AHB 23/36 (incl. 10/10 flattened table rows),
APB 7/7, but **AXI-l 0/4, AXI-h 1/7 (a `four-phase` noise hit), SWD 0/63.** The AHB/APB table-ROW co-occurrences are
already captured by the `.2i` statement scan (flattened rows are statements), and the only two phase-HEADER tables —
AXI-l `table_0150` (coherency-SEQUENCE: its `Phase` column is a step number 1/2/3, rows `Coherent domain access`/…,
no declared signals) and SWD `table_0057` (ACK-RESPONSE: `Operation requested | ACK received | Host response Data
phase | …`, rows `R/W/x`+`OK/WAIT/FAULT`, no signal→phase mapping) — map no declared signals to phases. **Decision:
NO-GO** (nothing to mine where present; genuinely absent on AXI/SWD — a document absence, not an extraction gap). Two
distinct real gaps recorded as future candidates: APB membership thinness (a `.2c`-breadth lever, not phase-cue) and
AXI/SWD timing-diagram phase columns (VLM-tier). The per-phase-membership surface is now as complete as the wire
docs ground.

## 5. Owner directive — sharpened (`2026-06-16`, multi-message)

The owner reinforced the requirement across several messages while this census was being finalised; the
authoritative requirement statement + the sharpened **7-point bar** live in
`docs/tasks/KG-ISF-TRANSACTIONS.md` (mirrored here so the report is self-contained):
1. **Fast + universal recognition** — SpecForge must "very quickly identify the transactions in any
   particular chip-spec PDF" (protocol OR platform): a cheap, deterministic, structural pass, no heavy
   LLM, no name list (ADR 0006). A runtime-capability requirement, not licence to rush code.
2. **Boundary / membership precision** — "what is part of transaction X and what is NOT" is of utmost
   importance: every signal/phase/step attributed to the RIGHT transaction, COMPLETE and EXCLUSIVE.
3. **Step-by-step** — each transaction described thoroughly and accurately, step by step.
4. **All transactions** — for the whole document, not a sample.

**Criticality:** this is a **MINIMUM / critical-path prerequisite** — *"without this we can't move
forward in our PDF→ISF tool."* A protocol/platform spec is in large part a catalogue of supported
transactions; without faithful capture the IntentIR cannot lower faithfully to `.isf`.
`KG-ISF-TRANSACTIONS` is therefore the near-term top-priority active tree. The `.2a` G2-first slice
serves requirement 1 directly (fast/universal/structural); `.2b`/`.2c` serve 2/3; coverage/completeness
(4) is the cross-slice acceptance bar.
