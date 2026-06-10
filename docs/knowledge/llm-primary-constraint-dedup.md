---
id: llm-primary-constraint-dedup
title: LLM-primary constraint dedup — provenance-merging, condition-aware (AXI 54→50 live)
answers:
  - "how are duplicate signal constraints deduplicated in the LLM-primary extractor"
  - "what is dedup_constraints / its canonical key"
  - "why do two constraints with different conditions not merge"
  - "where did the AXI AWIDUNQ / WTAGUPDATE duplicate records go"
date: 2026-06-10
tags: [extraction-quality, llm-primary, constraints, dedup, provenance]
evidence: crates/specforge/src/ir/constraint_extract_llm.rs (dedup_constraints); crates/specforge/src/commands/extract_constraints_llm.rs; docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.4)
reverify: cargo test -p specforge constraint_extract_llm 2>&1 | tail -2   # → 24 passed
---

`extract-constraints-llm` dedups its grounded set (`EXTRACTION-QUALITY-GAUGE.4`,
`2026-06-10`): key = the eval's `signal_constraint_record_key` (subject + kind incl. value +
negation) **plus the normalized condition**; the FIRST record wins (stable ids/order) and every
duplicate's `supporting_statement_ids` are merged into it — duplicate noise is removed while all
provenance survives. A different condition is a different requirement and never merges. The map
is lookup-only, so no hash-iteration order reaches the output ([[evidence-build-nondeterminism]]).

Live on AXI: 54→50 — `AWIDUNQ must_be_asserted (if present)` and `WTAGUPDATE must_be_deasserted`
were each re-stated by 3 different statements and each collapsed to one record carrying all three
ids. The command prints `Pattern → grounded → deduped (N merged)`. CHI's original `.gauge`
taxonomy measured ~30% duplicates — the mechanism is now in place for the CHI-class re-measure
once the signal-vs-field ontology (`.FIELD`) lands. Related:
[[llm-primary-condition-subject-gate]], [[llm-primary-must-be-value-recall]].
