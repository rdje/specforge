---
id: one-record-per-obligation-clause
title: A statement yields one constraint record per OBLIGATION, not one per statement — and narrowing the kind's span without that would have refused records the document states
answers:
  - "why does a statement with three requirements publish three constraints"
  - "how many obligations does extract_signal_constraints read from one statement"
  - "why was the AXI write-address handshake invariant never extracted"
  - "why did HSELx must_be_asserted carry the wrong condition"
  - "what is constraint_bearing_sentences and how does it differ from constraint_bearing_sentence"
  - "how does SpecForge read a fronted condition like When asserted, X must remain asserted"
  - "why does obligation_subject_part look after the comma"
  - "why is the constraint subject fallback bounded by the obligation"
  - "why does is_post_passive_binding_only_subject need to be told which obligation it judges"
  - "does the constraint record source_text become the clause"
  - "why does AXI publish WTAG VALID and WTAG ZERO as a temporal conflict"
  - "what happened to WTAGUPDATE must_be_value UPDATED"
date: 2026-09-13
status: current
evidence: crates/specforge/src/ir/evidence.rs (constraint_bearing_sentences, obligation_subject_part, extract_signal_constraints, is_post_passive_binding_only_subject_in, mod extraction_quality_gauge_3k_3); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3k, .3k.2b, .3k.3, .3k.7-.3k.10); docs/book/src/pipeline/obligation-reading.md
reverify: "cargo test -p specforge-core --lib extraction_quality_gauge_3k_3 — eight controls; and `specforge replay-constraints --evidence-root generated/evidence_ir` for the corpus figure"
tags: [evidence-ir, constraint-extraction, genericity, extraction-quality-gauge, claim-verification]
---

A SpecForge *statement* is often bigger than a sentence — a paragraph, or a whole serialized table
row. `extract_signal_constraints` used to mint **one record per statement**: its subject, condition
and negation came from `constraint_bearing_sentence`, the FIRST clause carrying a modal, and its KIND
was classified over the whole statement. Two things follow, and both were measured on the corpus:

* **obligations 2..n were dropped.** AXI states the write-address handshake invariant in the second
  sentence of a statement whose first sentence carries no modal, and the record was never minted —
  five invariants of that shape in one specification.
* **the surviving record could be a composite.** AHB's `| HSELx a | … |` row published
  `HSELx must_be_asserted` with a kind from the row's THIRD sentence and a condition from its SECOND.

## Narrowing the span alone would have been wrong, and that is the load-bearing part

`EXTRACTION-QUALITY-GAUGE.3k.3` was originally written as "classify the kind over
`constraint_bearing_sentence(text)` instead of over `text`". Re-derivation found the trap: AHB's first
modal clause states **no kind**, so narrowing to it would return `None`, `.3k.2a` would refuse the
record, and a fact the document plainly states would be lost. The defect underneath was not the kind's
span but the record's GRANULARITY. Read per obligation, the third sentence mints the record with its
own condition and the second mints nothing.

## Three readings had to move with it

| reading | before | why it had to change |
| --- | --- | --- |
| the subject part | `text_before_condition_marker` | it cuts at the EARLIEST marker, so a FRONTED condition cuts at offset 0 and leaves nothing; `obligation_subject_part` takes the main clause after the comma |
| the subject fallback | scanned the whole STATEMENT | per clause it hands one clause's signals to another clause's kind — the AXI pronoun case below |
| `is_post_passive_binding_only_subject` | re-derived the obligation as the statement's FIRST clause | it then judges the Nth record against the 1st record's clause; the `_in` form is told which obligation it is judging |

The fallback case is the sharp one. *"When ACVALID is asserted, it must remain asserted"* names its
subject with a pronoun; a statement-wide scan supplied the previous sentence's `ACADDR`, `ACPROT` and
`ACSNOOP` and published a requirement about signals the sentence does not mention. **That fabrication
was caught by measuring the change as an ADDITION before shipping it**, which is what the leaf's node
demanded and why the first prototype was rebuilt rather than committed.

`source_text` deliberately stays the STATEMENT: it is what `supporting_statement_ids` cites and what
the replay's merge identity keys on. The clause is the span the record is READ from, not its
provenance.

## What it measured

Corpus-wide over the 74 replayable documents, **187 → 211 replayed records**, and the only persisted
records that stop reproducing are NVMe `sigcon_0005`/`0006` — the leaf's own population. On the three
rebuilt documents: AHB 13 → 13 with its condition corrected, APB-E 23 → 27, AXI-L 40 → 53. AXI-L also
**loses `WTAGUPDATE must_be_value UPDATED`**, closing the residual `.3k.2b` named and predicted
("narrowing the classifier's span fixes the value binder at the same time").

AXI-L gains one temporal conflict — `WTAG` post-tick `VALID` vs `ZERO` — and that is a RESULT. The
document states both, conditional on the `WTAGOP` enum row the obligation sits in, and that condition
lives in a value cell rather than in a `when` clause. Surfacing the ambiguity is the contract;
silently keeping one reading is not.

## Four residuals, each with its own leaf

`.3k.7` AXI `WSTRB must_be_value VALID` — a subject inside `enabled by WSTRB` that the table-row
subject exemption admits; the obvious repair was measured and costs three reproduced records.
`.3k.8` the statement path and the row path now publish APB's six `PAUSER`/`PWUSER` obligations twice,
differing only in `source_text`. `.3k.9` eMMC `PARTITION` out of `PARTITION\_ACCESS`, an
escaped-underscore tokenization fragment. `.3k.10` a fronted condition that opens the STATEMENT
carries no leading space, so its marker is never seen — and the cheap repair is wrong, because that
sentence's first comma is a list separator rather than the condition's boundary.

Related: [[constraint-record-producer-strata]], [[one-modal-vocabulary-per-constraint-record]],
[[a-relational-predicate-is-not-a-value]], [[persisted-census-measures-published-not-current]],
[[table-row-obligation-binds-to-the-token-before-its-modal]].
