---
id: signal-presence-matrix-capture
title: Signal-presence matrices — the literal presence-CONDITION typed surface (.12b)
answers:
  - "what is signal_presence_records and what does a SignalPresenceRecord hold"
  - "how are signal presence matrices captured into typed records"
  - "are presence codes like Y N O C OC ever interpreted"
  - "what is the presence-matrix structural gate and how many tables fire it"
  - "how are rotated version matrices remapped for presence capture"
  - "how does the fused two-label column ACE5-Lite ACE5-LiteACP split"
  - "when does presence capture refuse a row vs the whole table"
  - "do signal presence records mint signals or declarations"
  - "why did AXI unexplained tables go 32 to 31 and ACE 36 to 35 and LTI 6 to 4"
  - "why does ACE table_0275 stay flagged after presence capture"
  - "what is capture_signal_presence_rows and who shares it"
  - "what is PDF-VARIANT-DIGESTION.12b"
date: 2026-06-11
tags: [extraction, signal-presence, configuration-intent, signal-tables, completeness, pdf-variant-digestion]
evidence: crates/specforge/src/ir/evidence.rs (SignalPresenceRecord, capture_signal_presence_rows, signal_presence_surface); crates/specforge/src/ir/completeness.rs (signal_presence_capture_covers); crates/specforge/test_data/kg_quality/signal_presence_matrix_gold + signal_presence_malformed_refusal_negative; docs/tasks/PDF-VARIANT-DIGESTION.md (.12b)
reverify: cargo test -p specforge --lib presence -- --nocapture ; ./target/release/specforge validate generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json 2>/dev/null | grep unexplained_intent_bearing_tables   # → 31
---

Signal-presence MATRICES state per-variant signal existence (`AWSUBSYSID` is `O` in AXI5, `N` in
ACE5-LiteACP) plus property-conditioned presence (`SUBSYSID_WIDTH > 0`). `.12b` gives them a typed
home: `EvidenceIr.signal_presence_records` — `{presence_id, signal_name (literal case; `Ax*`
generics never expanded), presence_condition: Option<verbatim expr> (`-` → None), variant_presence:
[{variant_label, code}], table_id}`. The 1–2-uppercase-letter codes stay LITERAL strings, never
interpreted — every document defines its own legend in nearby prose (ADR 0006: no code vocabulary in
runtime).

ONE shared pure function — `capture_signal_presence_rows(table)` — is consumed by BOTH the
registered extractor (`signal_presence.matrix_table`, via `run_surface` with a CONTENT key so a
page-break re-listed row dedups first-wins) and the completeness coverage
(`signal_presence_capture_covers`, validate-time so promoted artifacts close without rebuild). Its
measured rules:

1. **Gate** — signal-worded FIRST header cell + ≥2 all-code columns (cells only code/`-`/empty, ≥1
   real code) over identifier-led body+trapped rows (case-soft ≥60%-uppercase identifiers). Fires on
   **36 tables / 8 docs** corpus-wide — the gate census's 33/7 PLUS AXI-Stream `0015–0017` (genuine
   rotated version matrices the census sweep missed; content verified), MINUS LTI `table_0080`
   (its garble leaves one clean code column; never fires — honest residual either way).
2. **Rotation** — the `.5h` content remap: when another column carries more distinct identifier
   labels than the header-designated one, every header column remaps by the same offset (ATB/AHB/APB
   put the signal name LAST).
3. **Integrity (split-spill)** — ≥2 distinct identifier labels in a column on rows whose name cell
   is NOT an identifier ⇒ whole-table refusal (APB `table_0018`: even its cleanly-parsing rows are
   refused — no single rotation explains the grid, so no attribution is trustworthy).
4. **Fused-pair columns** — a multi-token variant header (`ACE5-Lite ACE5-LiteACP`) splits pairwise
   with its cells (`OC N`) ONLY when EVERY identifier-led row carries exactly matching code counts.
   ACE `table_0275` is the reason for all-or-nothing: its header fuses one pair, its cells fuse a
   DIFFERENT one — lenient splitting would shift codes a column over; instead all 21 rows refuse.
5. **Per-row** — every variant cell must parse (single code, clean pair-split, or `-` → no entry);
   one fused/empty cell refuses the whole ROW. Condition column = header says `presence`/`property`;
   declaration vocabulary columns (width/default/source/…) belong to the `.12a` gap-fill — presence
   records NEVER mint signals (locked by the gold fixture).

Measured live (2026-06-11): records on exactly 4 corpus docs — AXI 306 rows (157 conditioned / 73
distinct exprs — the design census EXACT), APB 20, AHB 40, AXI-Stream 22; 12/12 rebuildable bundles
otherwise byte-identical. Canonical accounting (no rebuild): AXI 32→31 (A13.3 generic family closes),
ACE 36→35 (`0271` closes; `0275` garble stays), LTI 6→4 (0078/0079 close; 0080 stays). All
wire-based golds re-measure 1.000; APB standing gauge 5/21 unchanged. Coverage is STRICT: one
refused row keeps the matrix flagged.
