---
id: eval-scores-persisted-evidence
title: eval-extraction scores the PERSISTED evidence_ir.json — rebuild before trusting a baseline (it can be stale)
answers:
  - "why is the AHB eval baseline wrong or stale"
  - "does eval-extraction rebuild evidence or load the persisted file"
  - "why do FPs appear in eval that the current code does not produce"
  - "how to get a fresh eval-extraction baseline for a spec"
  - "why can't specforge evidence rebuild the evidence (normalized missing)"
  - "is the WIRE-BASED-100.5a AHB 0.364 baseline real"
date: 2026-06-07
tags: [eval, stale-artifact, wire-based-100, gotcha, re-ingest]
evidence: docs/tasks/WIRE-BASED-100.md (.5a/.5b); crates/specforge/src/commands/eval_extraction.rs
reverify: ./target/debug/specforge evidence generated/source_ir/<doc_key>/source_ir.json   # errors if normalized was reclaimed
---

`eval-extraction` loads each doc's **persisted** `generated/evidence_ir/<doc_key>/evidence_ir.json`
(then runs `nlp-enrich`/`signal-resolve` on a temp copy for the Nlp tier). The base deterministic
constraints/relations come from that persisted file — so if the file was built by OLDER code, the eval
measures **stale** extraction, not current code.

This bit `WIRE-BASED-100.5a`: the AHB constraint baseline `P=0.364` included 5 list-introducer FPs
(`HTRANS`/`HREADY`/`HRESP` as subjects of `The following signals … when <cond>`) that **current code does
not produce** (proven by an isolated `extract_signal_constraints` call AND an instrumented build — both
yield 0 records for those introducers; the APB condition-subject fix already covers AHB). The only REAL
current-code defect was the `must_not_change` double-negative (`negated=true`), fixed in `.5b`.

**Before trusting a cross-spec eval baseline, rebuild evidence:** `specforge evidence
generated/source_ir/<doc_key>/source_ir.json`. CAVEAT: that needs the `normalized/<doc_key>.md` bundle,
which `specforge clean --scope source-normalized` reclaims — if it's gone, rebuild errors and you must
**re-ingest the PDF** (`specforge ingest`, needs the source PDF + `.venv-docling` + `DOCLING_DEVICE=cpu`).
When a rebuild/re-ingest isn't possible (PDF absent), demonstrate extractor correctness with **hermetic
unit tests** over the exact gold sentences instead of trusting the stale aggregate. See
`[[docling-device-cpu]]` and `[[apb-signal-catalog-fully-extracted]]`.
