---
id: llm-primary-permissive-frame-gate
title: Permissive-only frame gate is SUBJECT-SENTENCE-scoped — block-scoped modals over-kill
answers:
  - "why was HPROT[0] / HSEL / HTRANS IDLE extracted as a constraint (and how was it fixed)"
  - "how does the LLM-primary extractor handle 'It is recommended' / 'It is permitted' / 'would be' frames"
  - "what is is_permissive_only_subject_frame and why is it sentence-scoped not block-scoped"
  - "why does an incidental 'can' in a source block not drop its other constraints"
  - "what is the permission-vs-obligation gate / frame error class"
  - "why are the AHB gold negatives for statements 0561 and 0678 there"
date: 2026-06-10
tags: [extraction-quality, llm-primary, constraints, permission, normative-frames, precision]
evidence: crates/specforge/src/ir/constraint_extract_llm.rs (is_permissive_only_subject_frame); crates/specforge/test_data/llm_eval/seed_ahb.json (statements 0561/0678 negatives); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3b)
reverify: cargo test -p specforge constraint_extract_llm 2>&1 | tail -2   # → 22 passed; full re-measure = the .8/.3a redirected-copy protocol
---

A `must_*` constraint proposed from a **permissively-framed** source ("It is **recommended**
that a Manager sets HPROT[0] HIGH…", "An alternative implementation **would be** for HSEL to be
tied HIGH…") is a frame error — a recommendation/option/hypothetical, not an obligation. The
`.3b` gate drops such proposals — but THREE probed shapes dictate the design:

1. **Mandatory wins**: "It is **permitted** for a Manager to issue an Exclusive Write … In this
   case, the transfer **must** fail and HEXOKAY **must** be deasserted" — a real obligation
   behind a permissive lead-in. A must/shall in any subject-sentence keeps the proposal.
2. **Scope to the subject's own sentences** (split on `.;\n•`, same as
   `is_normative_for_subject`): the first cut checked the whole source block and the incidental
   "Although an OKAY response **can** be given in a single cycle" wrongly killed the
   ERROR-procedure records (`HRESP`×2 + `HREADYOUT`×2, AHB 15→8). Sentence-scoping restores them
   (15→12 = exactly the three frame errors). Over-kill shape is test-locked.
3. Frame vocabulary is universal normative language only (recommended/permitted/permissible/
   optional/may/can/could/would vs must/shall) — no signal names (ADR 0006). A subject-sentence
   with NEITHER frame (pure descriptive narration) is KEPT — the descriptive class is a separate
   future sub-slice (`.3c`), this gate only kills when explicitly permissive.

**Honest scoring note**: the labeled eval could NOT show this win — the eval-time
WIRE-BASED-100 filter (`is_normative_for_subject`) already masked the class from the gauge. The
real improvement is at the **canonical artifact** level: the persisted EvidenceIR that
downstream `SemanticIR`/`IntentIR` consume no longer carries the frame errors. The two new AHB
gold NEGATIVE items (statements `0561`/`0678`, `agent_drafted`) exist as regression armor: if
the eval-time filter is ever weakened, a leak of this class becomes a visible labeled FP.
Related: [[llm-primary-condition-subject-gate]], [[llm-primary-must-be-value-recall]].
