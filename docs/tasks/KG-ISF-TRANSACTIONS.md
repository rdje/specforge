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

A protocol spec is, in large part, a catalogue of **supported transactions** — AXI *read* / *write* (with
their burst, ordering, exclusive, and response semantics), AHB *NONSEQ/SEQ/BUSY/IDLE* transfers, APB
*read/write* (setup/access), I2C *write/read with (re)start/stop+ack*, etc. Each transaction is a named,
multi-phase sequence over a **specific set of signals**. For the `.isf` lowering to be faithful, the
IntentIR must carry every such transaction the document defines, with its signals, phases/steps, ordering,
and constraints — `FSMGen` then schedules the `(transaction …)` body into the FSM.

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

## The checkable "transaction-complete IntentIR" bar (per protocol doc)

A document's transaction surface is ISF-complete when:
1. **Coverage** — every supported transaction the document NAMES/defines is present as a typed
   `TransactionIntent` (no spec's read/write/burst/etc. silently absent), recovered by universal structure,
   not a hardcoded name list (ADR 0006).
2. **Signal set** — each transaction enumerates the set of signals it involves, with role/direction
   (address/data/control/response), grounded in the document.
3. **Phases/steps** — each transaction carries its ordered phases/steps (e.g. address phase → data phase →
   response) and the handshakes that gate them, as `TransactionStep`s.
4. **Constraints/relations** — the obligations and timing that govern the transaction (stability, ordering,
   response rules) are linked, reusing the existing constraint/temporal/relation surfaces.
5. **ISF round-trip** — the transaction lowers to a valid `(transaction …)` that passes FSMGen
   `--strict --check --json`, and every captured transaction element appears in the `.isf` or an explicit
   residual (no silent drop).

**Hard gate (non-negotiable):** WIRE-BASED-100 (APB/AHB/AXI/SWD) holds through every change; universal
grammar only, no name lists (ADR 0006); honest residual over fabrication. Scope per the owner: FULL,
precise, accurate capture — but delivered measurement-first, in safe slices, wire-docs first.

## Task Tree

- ID: `KG-ISF-TRANSACTIONS` · Status: `active` · Children: `.0` (this ownership/scoping slice), `.1`+ TBD
- ID: `KG-ISF-TRANSACTIONS.0` · Status: `done` (`2026-06-16`, docs-only ownership/scoping) · Goal: own the
  directive; record the measured baseline (55 txns / 15 docs; 3 sources; 3 gaps incl. the ADR-0006 breach);
  define the checkable 5-point bar; cross-reference `KG-ISF-COMPLETENESS`. No code (own before touching).
  Memory `[[project_kg_isf_transactions]]`.
- ID: `KG-ISF-TRANSACTIONS.1` · Status: `pending` (measurement-first, read-only — NEXT) · Goal: **the
  transaction-capture census + gap taxonomy.** Over the protocol corpus (wire docs first): (a) how does
  each document DEFINE its supported transactions (a transactions table/section? prose? a state/sequence
  diagram?) — characterise the structural form, ADR-0006-style; (b) for each, the named transaction set +
  the signals each involves; (c) compare to the current `transactions` surface to quantify G1/G3 per doc;
  (d) scope the G2 ADR-0006 remediation (replace the hardcoded AHB/APB/SPI recognizers with a structural
  recognizer keyed off the document's own transaction catalogue + handshake/phase structure). Output a
  research report + KM card + the first code slice's design. NO code in `.1`.
- ID: `KG-ISF-TRANSACTIONS.2+` · Status: `pending` · Goal: the code slices — structural transaction
  recovery (G1), ADR-0006 remediation (G2), signal-set linkage (G3) — each measurement-first, WIRE-BASED-100
  a hard gate, ISF round-trip verified.

## Changelog

- `2026-06-16`: **`.0` DONE** — tree created; owns the owner's `2026-06-16` transaction directive.
  Recorded the measured baseline (typed `transactions` surface exists + lowers to ISF, but is built from
  only per-channel handshakes + per-actor temporal blobs + 3 hardcoded protocol recognizers; 55 txns /
  15 docs). Identified 3 gaps: G1 composed named transactions missing, G2 ADR-0006 hardcoding breach
  (`HTRANS`/`PSEL`/`MISO`… literals → `ahb_transfer`/`apb_transfer`/`spi_transfer`), G3 signal-set linkage
  incomplete. Defined the checkable 5-point transaction-complete bar; WIRE-BASED-100 a hard gate. Frontier
  → `.1` (census + gap taxonomy, measurement-first, read-only). Memory `project_kg_isf_transactions`.
