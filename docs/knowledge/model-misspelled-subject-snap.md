---
id: model-misspelled-subject-snap
title: A subject absent from its own source sentence snaps to the sentence's declared token (edit distance 1, unambiguous-only)
answers:
  - "why did the LLM-primary extractor lose SYSCOREQ from a coordinated-subject sentence"
  - "what is snap_subject_to_sentence_token and when does it fire"
  - "how does SpecForge recover a subject the model misspelled"
  - "can the subject snap fabricate or rewrite a signal name"
  - "why did seed_axi_temporal fail after constraint promotion and how was it fixed"
  - "what is the model-misspelled-subject / phantom-subject defect class"
  - "why is the snap trigger absence-from-sentence and not typing failure"
date: 2026-06-10
tags: [extraction-quality, llm-primary, grounding, recall, typo, promotion]
evidence: crates/specforge/src/ir/constraint_extract_llm.rs (snap_subject_to_sentence_token, is_snap_candidate_token, within_one_edit_ignore_case, the ground_constraint_typed hook); docs/tasks/LLM-PRIMARY-PROMOTION.md (.3/.3a)
reverify: cargo test -p specforge --lib snap_ 2>&1 | tail -2
---

**Defect class** (caught by the `.3` gold-gate battery; probed live temp 0 ×2 identical): on
"SYSCOREQ and SYSCOACK must be deasserted when ARESETn is asserted." the model emits BOTH
records but misspells the first subject `SYCOREQ`. **Refined root cause (the first-cut reading
was wrong):** production entity typing DEFERS to the LLM judge for an undeclared,
non-structural token, so the typo *types as Signal and grounds* — a **phantom-name record**
that then dies silently at the SemanticIR declared-signal filter (where the AXI temporal rules
actually vanished). A snap hooked on *typing failure* therefore never fires — the first cut
shipped green unit tests and still failed the live gate (the per-item audit caught it again).

**Fix (`.3a`)** — the trigger is **absence-from-sentence**: this extractor's subjects are
quotes from the sentence, so a proposed subject with zero identifier-boundary occurrences in
its own source sentence is suspect per se. `ground_constraint_typed` then snaps it to the
document's own token iff ALL hold:

- the candidate literally appears in the sentence with a signal shape (≥4 chars, leading
  uppercase, ≤1 lowercase — `ARESETn` convention; prose words never qualify);
- the candidate types as a valid subject (Signal or declared Field) — the load-bearing
  document-grounded gate;
- it is within ONE edit (case-insensitive); equal-ignoring-case is not a typo;
- it is the ONLY such candidate (two near-twins → no snap).

A subject that occurs in its sentence is NEVER rewritten; an unsnappable absent subject keeps
the pre-fix behavior (types → grounds or drops). The corrected subject passes ALL downstream
gates under the document's spelling. No name lists (ADR 0006); the correction target is the
sentence's own text. Verified by the re-promoted AXI gate battery (see the tree's `.3a`
verification log) plus the `snap_fires_even_when_a_deferring_judge_would_accept_the_typo`
regression test locking the production shape.
Related: [[llm-primary-promotion-stage]], [[llm-primary-condition-subject-gate]].
