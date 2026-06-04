---
id: temporal-eval-residual-fps-are-stale
title: The temporal-rule eval's residual false positives are a stale artifact, not a live bug
answers:
  - "why doesn't the temporal_rule eval reach precision 1.0"
  - "are the degenerate PSEL-header or WIDTH-subject temporal rules a live bug"
  - "should I fix the PSEL valid when PSEL asserted temporal rule"
  - "temporal rule eval false positives root cause"
  - "is the eval-extraction temporal precision 0.6 a real defect"
date: 2026-06-02
tags: [eval, temporal, constraints, archaeology-trap]
evidence: crates/specforge/src/ir/evidence.rs:2838; crates/specforge/src/ir/evidence.rs:5756; docs/tasks/TEMPORAL-RULE-EVAL.md
reverify: grep -n " when " crates/specforge/src/ir/evidence.rs
---

`eval-extraction <temporal seed>` scores `temporal_rule P=0.600` — its two remaining false
positives (a degenerate "PSEL valid when PSEL asserted" rule synthesized from a
list-introducer header, and a `USER_RESP_WIDTH`-as-subject rule) come from signal constraints
in a **stale** on-disk APB EvidenceIR. They are **NOT a live bug**: the current `evidence.rs`
already strips `" when "` in `text_before_condition_marker` and excludes `*_WIDTH` in
`collect_subject_signal_tokens` (the `CONSTRAINT-SUBJECT-PRECISION` fix), so a clean APB
re-ingest removes them. Do **not** chase them as a live defect — at most run an APB re-ingest
to confirm.

The temporal *recall* side (a coordinated list "A, B, and C are asserted" dropping its leading
signals) WAS a live bug and was fixed by `TEMPORAL-ANTECEDENT-RECALL` (recall 0.667→1.000).
Canonical homes: `docs/tasks/TEMPORAL-RULE-EVAL.md`, `docs/tasks/TEMPORAL-ANTECEDENT-RECALL.md`.
