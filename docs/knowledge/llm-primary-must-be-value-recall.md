---
id: llm-primary-must-be-value-recall
title: LLM-primary must_be_value recall gap CLOSED — 10/16 → 16/16 gold facts (APB/AHB/AXI)
answers:
  - "why did the LLM-primary extractor miss PBUSER / PNSE / HAUSER must_be_value VALID"
  - "what was the must_be_value recall gap and how was it closed"
  - "how does the extract-constraints-llm prompt express a validity requirement"
  - "what is the must_be_value + VALID typed convention"
  - "how does ground_constraint recover a value the model did not echo"
  - "what is the LLM-primary extractor's measured recall on APB / AHB / AXI gold"
date: 2026-06-10
tags: [extraction-quality, llm-primary, constraints, must-be-value, recall, prompt]
evidence: crates/specforge/src/ir/constraint_extract_llm.rs (extraction_prompt, parse_kind, ground_constraint, is_value_kind); crates/specforge/src/ir/evidence.rs (extract_protocol_state_value, pub(crate)); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.8)
reverify: cargo test -p specforge constraint_extract_llm 2>&1 | tail -2   # → 8 passed; full re-measure = the .8 redirected-copy protocol in docs/tasks/EXTRACTION-QUALITY-GAUGE.md
---

The `.7` clean-test recall gap (`EXTRACTION-QUALITY-GAUGE.8`, fixed `2026-06-10`) had ONE root
cause with two compounders, all probed live (qwen2.5:14b-instruct, temp 0, the exact persisted
sentences) **before** coding:

1. **The prompt could not express a validity requirement.** For "PNSE must be valid when PSEL is
   asserted" the model output `[]` — the kind menu (`must_be_asserted`…`must_be_value`) gave it no
   slot for "must be valid", so it self-filtered the whole sentence. No Rust-side backstop can fire
   on an empty proposal — the fix HAD to be in the prompt.
2. `parse_kind` silently rejected the model's natural `must_be_valid` spelling (unknown kind).
3. `parse_kind` silently rejected `must_be_value` with no echoed value (`value?`).

**The typed convention** (already the Pattern extractor's own output via
`extract_protocol_state_value`, and the gold's `label_note`): *"<signal> must be valid" =
`must_be_value` + value `VALID`*. The fix teaches the prompt that convention verbatim, accepts the
`must_be_valid`/`valid` spellings in `parse_kind`, and — when the model names a value-kind but
omits the value — recovers the value from the SOURCE sentence by reusing
`extract_protocol_state_value` (the same binder grammar: "must be <value>"; grounded, never
fabricated; unrecoverable → honest drop).

**Measured** (pre→post, redirected evidence copies, eval canonical keys, doc-level gold-fact
recall): APB **4/6 → 6/6**, AHB **2/6 → 6/6**, AXI (zero `must_be_value` gold — control)
**4/4 → 4/4**. Total **10/16 → 16/16**; every pre-fix miss was a `must_be_value VALID` fact.
FP ledger net unchanged (3→3), and all FPs are the condition/permission-as-obligation class —
that is the open `.3` leaf, not a validity artifact. Bonus: AHB's two pre-fix condition-junk FPs
(`HREADY must_be_high`, `HRESP must_not_change` read out of the HRUSER sentence) disappeared once
the model could express what the sentence actually says.

GOTCHA for re-measurement: `extract-constraints-llm` writes to the path stored INSIDE the
artifact (`artifact_layout.evidence_ir_path`) — a naive file copy still clobbers the corpus
original. Redirect `artifact_layout` in the copy first (the `.8` protocol), exactly like
`eval-extraction`'s `extract_on_copy`.
