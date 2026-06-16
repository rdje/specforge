# Transaction-capture census (`KG-ISF-TRANSACTIONS.1`)

Read-only measurement (`2026-06-16`), gathered before any transaction code. Captures the ISF
**target** model, how protocol/platform PDFs **define** their supported transactions (source form),
the **current** SpecForge capture, and the gap — so the first code slice is evidence-driven.
(Partial: target model + source-form census + current-capture census done; per-doc G1/G3
quantification and the first-slice design are the remaining `.1` step.)

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

## 4. Remaining `.1` step (next session) + first-slice direction

- Finish `.1`: per-doc G1/G3 quantification across the protocol corpus; decide the first code slice.
  Leading candidate ordering: **G2 first** (replace the 3 hardcoded recognizers with a structural
  enumeration-table + section-heading recognizer — removes the ADR-0006 breach and is the foundation
  the named-transaction catalogue builds on), then **G1** (compose per-phase children into named
  read/write transactions via `spawn`/`do`), then **G3** (signal-set linkage). Each measurement-first,
  WIRE-BASED-100 a hard gate, ISF round-trip (`adapt --target isf` → FSMGen `--strict --check`) verified.
- Write the KM card `transaction-capture-census` (deferred from this session's handoff).
