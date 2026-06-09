---
id: timing-table-trapped-row-recovery
title: Timing-table data rows trapped in header_rows (row-label cell is_header=true) are recovered structurally, not skipped
answers:
  - "why does a timing_parameter table produce 0 timing_constraints when it clearly has rows (I2S table_0004, SMBus table_0012)"
  - "what is the header_rows trapped-data-row recovery in synthesize_timing_constraints"
  - "how does specforge tell a trapped data row from a genuine multi-row column header without a list or case"
  - "how were I2S timing_constraints recovered (clock period / clock HIGH / set-up / hold)"
  - "why is the nested cross-tab timing table (TRANSMITTER/RECEIVER) left an honest residual"
date: 2026-06-09
tags: [timing, table, evidence-ir, pdf-variant-digestion, i2s, smbus, adr-0006, structural]
evidence: docs/tasks/PDF-VARIANT-DIGESTION.md (.9.11); crates/specforge/src/ir/evidence.rs (synthesize_timing_constraints)
reverify: python3 -c "import json; e=json.load(open('generated/evidence_ir/um11732_v3_2022_02_17_i2s_bus_specification/evidence_ir.json')); print(len(e.get('timing_constraints',[])), [r['parameter_name'] for r in e.get('timing_constraints',[])])"
---

A `timing_parameter` table can clearly have data rows yet produce **0** `timing_constraints`. Root cause
(I2S `table_0004`, SMBus `table_0012`): Docling marks each row's **label cell** `is_header=true`, which moves
the *entire* data row into `header_rows`, leaving `body_rows` **empty**. The old
`synthesize_timing_constraints` opened with `if table.body_rows.is_empty() { continue; }`, so the whole table
was skipped — a pure structural-ingestion artifact, not a missing-name-column problem (`name_col` already
defaults to column 0).

The fix recovers the trapped rows **structurally** — no parameter-name list, no case dependence (ADR 0006,
[[feedback_avoid_denylists_prefer_structural]]). The effective data-row set is
`body_rows + header_rows[1..]` filtered to **data-shaped** rows:

```
len >= 2  &&  first cell non-empty (a row label)  &&  every value cell (skip(1)) has is_header == false
```

The discriminator is the `is_header` flag itself: a *genuine* multi-row column header (e.g. I2S `table_0005`'s
nested `TRANSMITTER / RECEIVER × LOWER / UPPER LIMIT` cross-tab) keeps its value cells `is_header=true`, so it
fails the test and is left an **honest residual** — never a fabricated parameter named "TRANSMITTER". The real
column-header row (`header_rows[0]`, blank leading cell) is skipped by `skip(1)` and by the non-empty-first-cell
guard.

Result: I2S `timing_constraints` **0 → 5** (`clock period T` 360/400/440, `clock HIGH t HC` min 110,
`clock LOW t LC` min 110, `set-up time t sr` min 60, `hold time t htr` min 0 — empty value cells stay `None`,
never invented). SMBus held at **84** (its `table_0012` shape does not match the data-row test, so it stays a
residual rather than producing garbage). The recovery is additive: tables with proper `body_rows`
(SMBus `table_0011`/`0013`) are unchanged. See also [[evidence-build-nondeterminism]].
