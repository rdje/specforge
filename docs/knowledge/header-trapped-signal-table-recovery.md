---
id: header-trapped-signal-table-recovery
title: Header-trapped SIGNAL tables — shared trapped-row rule, continuation-kind inheritance, inventory-gated gap-fill (.12a)
answers:
  - "how are signal presence matrices with signals trapped in header rows handled"
  - "how does an unknown-kind Continued from previous page table fragment get a kind"
  - "what is continuation_inherited_table_heads and what grounds the join"
  - "what is recovered_trapped_data_rows and who shares it"
  - "why did APB unexplained_intent_bearing_tables go to zero"
  - "why did AXI unexplained tables go 39 to 32 and the denominator 94 to 98"
  - "why did LTI unexplained tables go UP from 5 to 6"
  - "does the trapped-row gap-fill mint duplicate signal declarations"
  - "where do the 9 new ACE wires AWBAR AWDOMAIN AWSNOOP come from and when do they land"
  - "what is the canonical declared signal inventory key on SemanticIR"
  - "what is PDF-VARIANT-DIGESTION.12a"
date: 2026-06-11
tags: [extraction, completeness, signal-tables, continuation-fragments, pdf-variant-digestion]
evidence: crates/specforge/src/ir/evidence.rs (recovered_trapped_data_rows, continuation_inherited_table_heads, synthesize_trapped_row_signal_declarations); crates/specforge/src/ir/completeness.rs (densest_signal_name_column_tokens); docs/tasks/PDF-VARIANT-DIGESTION.md (.12 probe + .12a)
reverify: cargo test -p specforge --lib trapped -- --nocapture ; cargo test -p specforge --lib continuation ; ./target/debug/specforge validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>/dev/null | grep unexplained_intent_bearing_tables   # → 0
---

Signal-presence matrices (AXI `B2.2`/`B2.3`, ACE `Signal matrix`, LTI `C5.1`, APB `B-3`; 21 tables /
487 trapped tokens corpus-wide) often have EVERY data row Docling-trapped in `header_rows` (label
cell `is_header=true`, `body_rows` empty). `PDF-VARIANT-DIGESTION.12a` handles them three ways, all
structural (ADR 0006):

1. **One shared trapped-row rule** — `recovered_trapped_data_rows` (the `.9.11` definition: past the
   first header row, value cells all `is_header=false`, non-empty label). The timing recovery, the
   signal gap-fill, and the completeness coverage all consume THIS function — they cannot drift. A
   genuine multi-row column header (value cells `is_header=true`, e.g. LTI's `A A.b|B|C|D` sub-label
   row) is refused.
2. **Continuation-kind inheritance** — `continuation_inherited_table_heads`: an `unknown`-kind
   fragment captioned `Table <ref> Continued from previous page` inherits its head's kind only when
   the nearest preceding NON-continuation table with the same `<ref>` has a known kind AND the exact
   first-header-row signature (measured 9/9 on AXI). Consumed by the gap-fill, the unexplained-table
   accounting, and validate's `intent_bearing_table_count` denominator. SourceIR is never mutated.
3. **Inventory-gated gap-fill** — `synthesize_trapped_row_signal_declarations` runs LAST in the
   declaration seed: mints only under the body-row content rules (direction or width present) AND
   only for names absent from the declared inventory. Duplicates are NEVER re-minted — they are
   coverage-marked instead (`densest_signal_name_column_tokens` now chains trapped rows, same
   one-unknown-keeps-it-flagged strictness).

Measured (2026-06-11, canonical artifacts, validate-time — no rebuild needed because the gauge
recomputes from tables + persisted records): APB **1→0** (the `table_0018` WIRE-BASED-100.3 residual
closed), AXI **39→32 of 94→98**, ACE **39→36**, LTI **5→6** — UP, honestly: `table_0080` (a third
C5.1 fragment with cell-fused body rows carrying LTVALID/LTCREDIT/LTCTAG presence) was invisible to
the gauge and is now an accounted candidate miss. 13/13 intact bundles old-vs-new `evidence
--dry-run` byte-identical (the gap-fill mints nothing on the rebuildable corpus). The ACE +9 wires
(AWBAR w2, AWDOMAIN w2, AWSNOOP w4, CRRESP w5, CDDATA wV, 4×BROADCAST* w1 — real `Width|Source|
Default` cells) land when the host-local ACE doc is re-ingested.

Key lookups (the `.12` probe's resolved keys — do not re-derive): the evidence-level declared
inventory is `collect_known_signal_names` over `extracted_statements` ("Signal X is …"); the
CANONICAL inventory is SemanticIR `interfaces[].signal_records[].signal_name` (there is NO
`interface_signals` key). The presence-CONDITION intent (AXI 157 rows / 73 distinct property
expressions like `SUBSYSID_WIDTH > 0`; LTI 23/15) is real but un-captured — typed home = `.12b`.
See [[apb-signal-catalog-fully-extracted]].
