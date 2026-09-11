---
id: sourceir-classification-is-per-record
title: SourceIR classification is proof-verified one record at a time, so a classification rule may never read its neighbours
answers:
  - "why did a SourceIR classification change fail with 'classification is not the registered capture/proposal replay'"
  - "may a SourceIR table classifier look at the table before it"
  - "where does a rule that needs document order belong if SourceIR cannot hold it (EvidenceIR, which reads the whole SourceIR as its input)"
  - "how do I let a Continued from previous page table contribute what its first page contributes"
  - "what is continuation_inherited_table_heads and which passes use it"
  - "what does record-00000102 mean in a SourceIR proof verification failure"
  - "why can a per-table classifier not inherit a kind from a parent table"
date: 2026-09-11
status: current
tags: [sourceir, evidenceir, proof-kernel, classification, adr-0038, wire-based-100]
evidence: "crates/specforge/src/ir/source.rs (classify_source_captures, classified_table_kind); crates/specforge/src/ir/derivation.rs (per-record premise replay); crates/specforge/src/ir/evidence.rs (continuation_inherited_table_heads, synthesize_declarations_from_tables, synthesize_trapped_row_signal_declarations); docs/tasks/WIRE-BASED-100.md (.10c)"
reverify: "cargo test -p specforge-core --lib a_continuation_page_declares_through_its_chain_head continuation_fragment_inherits_captioned_head_kind"
---

# One record, one proof

ADR 0038's sealed proof kernel verifies a SourceIR field record by **replaying its own producer against
its own premises**. The replay is per record: a single `structured_tables` entry is deserialized on its
own and reclassified, then compared. `classified_table_kind` is therefore obliged to be a **pure function
of one table** — its cells, its caption, its headers — and nothing else.

That obligation is invisible until you break it. A rule that lets a `Table B1.1 Continued from previous
page` fragment inherit the kind of the table it continues is correct, generic, and measurable, and it
passes every unit test; it then fails the moment a real document carries one, with

```
SourceIR proof verification failed: SourceIR structured_tables:record-00000102
classification is not the registered capture/proposal replay
```

because the single-record replay has no neighbours to inherit from. The record index is the table's
position, so `record-00000102` is `table_0103` — the first continuation page in that document.

## Where the rule goes instead

**EvidenceIR is the first stage that legitimately holds the whole document.** It reads a complete,
proof-verified `SourceIr` and its own records are proved against that input, so a reader there may use
document order freely. SpecForge already had the mechanism: `continuation_inherited_table_heads`
(`crates/specforge/src/ir/evidence.rs`) maps each `unknown` continuation fragment to the index of its
captioned chain head, grounding the join **twice** — the fragment's caption must state the parent's table
reference, and the nearest preceding non-continuation table with that reference must carry the *exact*
same first-header-row signature. A head that is itself `unknown`, a differing header, or a missing
parent inherits nothing. `PDF-VARIANT-DIGESTION.12a` built it for the trapped-row gap-fill;
`WIRE-BASED-100.10c` wired the ordinary body-row pass to the same resolver, which is what lets a signal
table that simply ran over a page break declare its remaining rows.

The general shape, worth carrying to the next stage boundary: **a constraint the proof kernel enforces on
a producer is a statement about what that producer is allowed to know.** When a rule needs more context
than its stage's proof unit, the rule is not wrong — it is early.

Links: [[qualified-role-header-proves-no-role]], [[chain-currency-doctrine]].
