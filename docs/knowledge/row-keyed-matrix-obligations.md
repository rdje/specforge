---
id: row-keyed-matrix-obligations
title: An obligation minted from a compatibility-matrix cell loses BOTH axes — 7 records in the whole corpus, and 5 of the 109 exposed rows bind on an axis the span cannot see
answers:
  - "how many persisted constraints were minted from a matrix row whose first cell binds a configuration (7 in the whole 78-document corpus — all LLM-primary, all in the historical LTI document, all from one cell of Table B12.2; 6 of them carry no condition at all)"
  - "has the LLM-primary path ever carried a table row key into a constraint condition (no — 7 of 7 lose it; llm_sigcon_0017 is the only one with a condition_text and it holds the sentence's own predicate, connected to Manager LRMPAM .MPAM_NS input, not the row key)"
  - "how many spans in the persisted corpus are matrix rows whose first cell binds a configuration (109 occurrences across 14 documents — 9 measured-stratum and 100 historical; 104 bind on the row axis only and 5 also bind on the column axis)"
  - "is a lost table row key a source-assembly gap or a proposal-shape defect (both, and which one depends on the table — for 104 of 109 rows the binding is inside the span and a span-local rule can reach it; for 5 the column header binds the other axis and lives in a DIFFERENT statement, so the span cannot express the scope even in principle)"
  - "why can a row-key remedy not be validated on the measured stratum (the measured stratum holds 9 key-scoped rows across 3 documents and ZERO constraints have ever been minted from them; the LLM promotion's recall universe is the distinct source_text of constraints that already exist, so EXTRACTION-QUALITY-GAUGE.3j.4.a will not visit one either)"
  - "what scopes a cell of an ARM LTI compatibility matrix (both axes — the row key binds the Manager's properties and the column header binds the Subordinate's, so an obligation in a cell holds only under the conjunction; in the persisted LTI artifact the row is statement_1027 and the header is statement_1025)"
  - "how do I re-run the row-keyed obligation census (cargo test -p specforge-core --lib row_keyed_obligation_population -- --ignored --nocapture; the crate is specforge-core because crates/specforge/src/ir/** compiles into it by #[path])"
  - "why was attaching the row key as a condition refused as a remedy (it cannot be validated on the publishable stratum, and on the 5 two-axis rows it produces an obligation that is still wrong but now carries a condition that makes it look checked; the adjudicated direction is refusal, not reconstruction)"
date: 2026-09-18
status: current
tags: [extraction-quality-gauge, llm-primary, tables, conditions, census, adjudication, method]
evidence: docs/tasks/extraction-quality-gauge/llm-path-family.md (.3j.2.c); crates/specforge/src/ir/constraint_extract_llm.rs (row_keyed_obligation_population_local_measurement, a_key_scoped_matrix_row_is_recognised_by_structure_alone); crates/specforge/src/commands/extract_constraints_llm.rs (promote_constraints); generated/evidence_ir/ihi0089_d_2025_08_amba_lti_protocol_specification/evidence_ir.json (llm_sigcon_0012-0018, statement_1025, statement_1027)
reverify: "cargo test -p specforge-core --lib row_keyed_obligation_population -- --ignored --nocapture — expect 3 measured / 11 historical documents, 9 measured / 100 historical key-scoped rows, 5 two-axis / 104 one-axis, 7 constraints minted with 6 carrying no condition"
---

A compatibility matrix states its scope in two places, and a cell's obligations hold only under both.
An extractor that reads the cell's row as one sentence gets neither for free.

## The document

`Table B12.2` of the persisted LTI artifact is a Manager-by-Subordinate compatibility matrix. Its header
(`statement_1025`) binds the **Subordinate**'s properties per column; each row's first cell binds the
**Manager**'s. `statement_1027` is one such row:

```text
| LTI_MMU = True LTI_GPC = False | Compatible. | Compatible. Subordinate LASECSID[1] input is tied LOW.
  Subordinate LAPAS[2:1] input is tied 0. Subordinate LAPM is tied LOW. … | Not compatible |
```

The whole row is one span, and seven constraints were minted from it. Every one asserts its obligation
unconditionally: `Subordinate LAPM` `must_be_value` `LOW`, always. It is true only when the Manager is
`LTI_MMU = True LTI_GPC = False` **and** the Subordinate is `LTI_MMU = True LTI_GPC = True`, which is the
column the text sits in.

## The two mechanisms are not one

For **104** of the 109 key-scoped rows in the corpus the binding is inside the span. A rule reading the
proposal's own span can reach it.

For **5** — 3 of LTI's 4 and both of DTI's — the column header binds as well, and it is a *different
statement*. No span-local rule can recover it, because the span was never shown it. The six records this
was opened from come from a two-axis row, so the example that motivated the leaf is in the class the
leaf's own diagnosis excluded.

## Why the remedy is refusal, not reconstruction

The realised defect is 7 records in one cell of one historical document. The measured stratum holds 9
key-scoped rows and has produced **zero** constraints from them, and it cannot produce one through the
LLM path either: that path's recall universe is the distinct `source_text` of constraints that already
exist (`[[measured-stratum-promotion-population]]`). So a rule that *reconstructs* the scope would carry
no publishable population, and on the two-axis rows it would replace a visibly-unconditional error with an
invisibly-incomplete one.

Refusing an unconditional obligation from a key-scoped matrix row refuses **7 records in 78 documents,
and 7 of 7 are wrong as written** — including `llm_sigcon_0017`, whose `condition_text` holds the
sentence's own predicate rather than the key. For comparison, `EXTRACTION-QUALITY-GAUGE.3j` declined to
wire five positional gates whose refusals were correct 3 times in 7.
