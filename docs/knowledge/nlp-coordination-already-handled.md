---
id: nlp-coordination-already-handled
title: Coordinated drive/read objects + relative-clause distribution are already implemented in the production hand grammar; subject coordination is ≈0-prevalence → NLP-SHALLOW-PARSE.2f measured NO-GO
answers:
  - "does specforge handle coordinated drive/read objects (X drives A and B)"
  - "is 'X, which connects to Y, drives Z and W' clause distribution handled"
  - "should NLP-SHALLOW-PARSE.2f build coordination distribution"
  - "is the NLP-SHALLOW-PARSE build frontier exhausted"
  - "where is coordinated-object actor-signal relation extraction implemented"
  - "is subject coordination (Actor1 and Actor2 drive X) handled / worth building"
date: 2026-06-15
tags: [nlp-shallow-parse, actor-signal-relations, coordination, clause-distribution, measured-defer, grounding-gate, build-exhausted]
evidence: crates/specforge/src/ir/evidence.rs (active_object_contains_signal — scans the whole post-verb clause for each grounded signal token; extract_subject_phrase — relative-clause stripping; tests coordinated_active_drive_extracts_real_actor_not_payload_phrase / coordinated_active_read_extracts_all_sampled_objects, the ARCHUNKEN/RCHUNKV cases); docs/tasks/NLP-SHALLOW-PARSE.md (.2f)
reverify: "grep -roh 'drive [A-Z][A-Z0-9_]* and [A-Z][A-Z0-9_]*' generated/evidence_ir/*/evidence_ir.json (all OBJECT coordination — drive AERR and DERR / RVALID and BVALID / LAMECID and LAHWATTR — covered by active_object_contains_signal); subject coordination 'A and B drive' returns only noise (current drive of the register; deasserts valid and can drive data) → ≈0 grounded; the canonical 'which connects to … drive ARCHUNKEN and RCHUNKV' is a passing test"
---

**`NLP-SHALLOW-PARSE.2f` is a measured NO-GO as new production code (`2026-06-15`), and it exhausts the
gaps-first BUILD frontier.** The leaf ("clause split + coordination distribution") was scoped to distribute
"X, which connects to Y, drives Z and W". Measurement (the second build slice, opened measurement-first like
`.2h`) found it already done.

**1. The canonical example is already implemented AND tested.** `extract_subject_phrase` strips relative
clauses (`" which "`/`" that "`/`" who "`/`" whose "`) before choosing the subject, and `active_object_contains_signal`
scans the ENTIRE post-verb object clause for each grounded signal token — so a coordinated object list
distributes the verb across every grounded signal. The tests `coordinated_active_drive_extracts_real_actor_not_payload_phrase`,
`coordinated_active_read_extracts_all_sampled_objects`, and the `ARCHUNKEN`/`RCHUNKV` cases lock exactly
"An interconnect *which connects to components …* can drive ARCHUNKEN **and** RCHUNKV" → both objects recovered,
`interconnect` as the actor, both directions.

**2. Corpus coordinated prose is all object coordination (covered) — subject coordination is ≈0.** Over the 78
persisted `evidence_ir.json`, coordinated-drive sentences (`drive AERR and DERR`, `drive RVALID and BVALID LOW`,
`drive LAMECID and LAHWATTR outputs to 0`) are all OBJECT coordination → already handled. The only unhandled
sub-case is SUBJECT coordination ("Actor1 and Actor2 drive X", where `extract_subject_phrase` keeps only the
nearest of the two actors), and its grounded corpus prevalence is ≈0 (the only "A and B drive" hits are noise:
"current drive of the register", "deasserts valid and can drive data"). Building it would be speculative effort
against zero measured demand.

**Tree-level conclusion:** BOTH spike-flagged "gaps" — `.2h` (direction, see [[actor-signal-direction-passive-active-handled]])
and `.2f` (coordination) — already live in the mature production hand grammar. The gaps-first BUILD scope is
therefore empty; the `.3` SVO assembler collapses to a pure consolidation refactor with no recall/precision gain,
which the wire-based-100% + additive-until-proven + deterministic-byte-identical gates make high-risk/low-value
→ `deferred`. `NLP-SHALLOW-PARSE` becomes a STANDING (build-exhausted) tree. This CONFIRMS the spike's
"consolidation, not a recall multiplier" verdict at the production level: the mature hand grammar already does
what a generic shallow-parse tier would, so the bigger "digest any PDF" levers remain TABLES + the VLM arm
([[project_nlp_shallow_parse_direction]]). Honors [[feedback_scoring_rigor]] (measured, not assumed) and the
measured-DEFER precedents (MEMORY-BOUNDED-INGEST.5 / FULL-PAGE-INTENT-CAPTURE.1).
