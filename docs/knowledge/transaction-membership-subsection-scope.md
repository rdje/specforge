---
id: transaction-membership-subsection-scope
title: a named transaction's .2c signal-set membership includes its section's DESCENDANT-SUBSECTION statements (dotted section-number prefix) — boundary-safe because read/write subtrees are disjoint; APB write 1->10 / read 1->7, AXI atomic 0->22
answers:
  - "why was APB write_transfer / read_transfer membership only PCLK"
  - "how does a transaction absorb signals from its subsections (3.1.1 / 3.1.2)"
  - "what do leading_section_number and is_descendant_section_number do"
  - "how is transaction membership kept boundary-precise across read vs write (bar #3)"
  - "why did AXI atomic_transaction / prefetch / writezero / writedeferrable go from 0 to a real signal set"
  - "is the descendant-subsection scope over-broad (no — 0 over-broad corpus-wide)"
  - "what does KG-ISF-TRANSACTIONS.2k add"
  - "is the .isf affected by transaction membership (no — emitter lowers steps, not ports/phase_membership)"
date: 2026-06-17
tags: [transactions, kg-isf-transactions, membership, section-hierarchy, boundary-precision, adr-0006, wire-based-100, code]
evidence: crates/specforge/src/ir/semantic.rs (build_transaction_anchors descendant union + leading_section_number + is_descendant_section_number + 2 unit tests); docs/tasks/KG-ISF-TRANSACTIONS.md (.2k); docs/book/src/pipeline/intentir.md ("Grouping a transaction's signals by phase")
reverify: "cargo test -p specforge --lib build_transaction_anchors_includes_descendant_subsection_statements descendant_section_number -> 2 pass. Live (temp evidence-root, WRITE-PATH GOTCHA): rebuild APB ihi0024_e evidence->semantic->intent with vs without the change -> write_transfer signal_set 1 (PCLK only) -> 10 (adds PADDR/PWDATA/PWRITE/PENABLE/...), read_transfer 1->7 (no PWDATA/PSTRB/PWUSER); diff the emitted .isf old-vs-new -> byte-identical; diff intent actor_signal_relations/signal_constraints/temporal_rules -> byte-identical (only transactions changed)."
---

**Landed `2026-06-17` (`KG-ISF-TRANSACTIONS.2k`, CODE).**

A transaction is NAMED from a section heading (Cue A, `[[transaction-capture-census]]`) — APB `write_transfer`
from `3.1 Write transfers`. Its `.2c` signal-set membership is the declared signals referenced by that section's
statements. But section anchors are **line-range and non-overlapping** (`build_section_anchors`, `ir/evidence.rs`):
a parent section ends one line before its first subsection begins, so the parent's `supporting_statement_ids`
carries ONLY its own intro statements. APB `3.1` had 4 intro statements (only one names a declared signal, `PCLK`)
while the signal-rich prose (`PADDR`/`PWRITE`/`PENABLE`/`PSEL`) is filed under `3.1.1 With no wait states` /
`3.1.2 With wait states` — so the transaction membership was a thin `{PCLK}`. (The signals ARE captured
document-wide; the `.2g` `transaction_phases` surface already recovers the rich `setup`/`access` sets — only the
per-transaction SCOPE was too narrow.)

**Fix (`build_transaction_anchors`, `ir/semantic.rs`):** broaden a transaction's statement scope to include every
section whose dotted number is a strict DESCENDANT of its own, then derive `signal_set` (and the record's
`supporting_statement_ids`) from that expanded set. Two universal helpers: `leading_section_number`
(`3.1 Write transfers` → `3.1`; furniture like `Chapter 10` / unnumbered → `None`) and
`is_descendant_section_number` (a strict dotted-prefix test — `3.1.1` under `3.1`, but NOT `3.10`, NOT `3.3.1`,
NOT equality). Universal document-structure grammar keyed off the section number — **no name list (ADR 0006)**;
byte-identical when the section has no subsections.

**Boundary precision (bar #3 — "of utmost importance"):** descendant subtrees are disjoint, so write (`3.1.x`)
and read (`3.3.x`) never cross — APB `read_transfer` correctly EXCLUDES write-data `PWDATA`/`PSTRB`/`PWUSER`.
Naively unioning the document-global `.2g` phase signal sets into both transactions was REJECTED for exactly this
reason (it would attribute write-only signals to the read transaction). Corpus-wide (78 docs, 260 transactions,
101 with membership): **0 over-broad** (none ≥80% of the declared inventory or == full inventory; max 77.8% = a
9-signal flash-bus doc whose `normal_operation` genuinely uses 7).

**Measured win (old-vs-new deterministic rebuilds):** APB `write_transfer` **1→10** / `read_transfer` **1→7**;
AXI moved off the `.2j`-flagged "empty" state — `atomic_transaction` **0→22**, `prefetch_transaction` **0→8**,
`writezero_transaction` **0→6**, `writedeferrable_transaction` **0→10** (distinct, each subtree-scoped to its
`A6.4`/`A8.6`/`A11.x` chapter).

**`.isf` / WIRE-BASED-100 unaffected:** membership is `TransactionIntent.ports` / `phase_membership` metadata and
the ISF emitter lowers `steps`, so the emitted `.isf` is **byte-identical** on all 4 wire docs (old-vs-new diff),
and the wire-eval surfaces (`actor_signal_relations` / `signal_constraints` / `temporal_rules` /
`conditional_rules`) are byte-identical old-vs-new — only `transactions` changed. The remaining `.2j`-recorded
candidate (AXI/SWD timing-diagram phase columns) is a VLM-tier lever, not built. `[[project_kg_isf_transactions]]`.
