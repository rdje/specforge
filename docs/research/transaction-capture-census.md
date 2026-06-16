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
