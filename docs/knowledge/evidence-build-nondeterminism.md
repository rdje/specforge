---
id: evidence-build-nondeterminism
title: The EvidenceIR build is content-level non-deterministic (HashSet iteration in relation extraction)
answers:
  - "why does rebuilding the same SourceIR give a different evidence_ir.json (non-determinism)"
  - "why did an EXTRACTOR-ARCHITECTURE byte-identical proof fail on SWD/ADI but pass on other docs"
  - "what causes actor_signal_relations / extracted_statements to differ run-to-run"
  - "is the EvidenceIR build reproducible / deterministic"
  - "how should a behavior-preserving evidence refactor be verified given the non-determinism"
date: 2026-06-09
tags: [determinism, reproducibility, evidence-ir, hashset, relations, eval, extractor-architecture]
evidence: docs/tasks/EVIDENCE-DETERMINISM.md (.1); crates/specforge/src/ir/evidence.rs (extract_actor_signal_relations — `for signal in known_signals: &HashSet`)
reverify: "build evidence for ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification twice; md5 the two evidence_ir.json — they differ (until EVIDENCE-DETERMINISM.2)"
---

The EvidenceIR build is **non-deterministic at the content level** (not just ordering): two runs of identical
code on the SWD/ADI `source_ir` produce **set-different** `actor_signal_relations` and `extracted_statements`
(confirmed `EXTRACTOR-ARCHITECTURE.5`, `2026-06-09` — an order-insensitive comparison still differs). Most
docs (RISC-V Debug, I2C, SWP, CAN) ARE deterministic; the non-determinism is specific to the **relation-heavy
prose** path.

**Root cause:** `extract_actor_signal_relations` iterates `known_signals: &HashSet<String>`
(`for signal in known_signals`) to drive prose relation extraction, and a first-seen-wins
`seen: HashSet<(String,String,u8)>` dedup keeps **different representatives** depending on the
non-deterministic `HashSet` iteration order. That varies the surviving relation CONTENT (and, via the
build-wide statement counter, the relation-derived synthesized statements + their ids), which cascades into
`extracted_statements` content and `fact_provenance` order.

**Why it matters:** (1) reproducibility / crash-safe handoff; (2) eval-score stability — the scorer reads
PERSISTED evidence, so a score can drift run-to-run (violates [[feedback_scoring_rigor]]); (3) it weakens the
`EXTRACTOR-ARCHITECTURE` byte-identical "behavior-preserving" proofs (a sound proof needs a deterministic
baseline). **Until the fix (`EVIDENCE-DETERMINISM.2`: deterministic iteration via sorted view / `BTreeSet`),
verify evidence refactors order-insensitively or on the deterministic docs.** The fix must be
multiset-preserving (same relations, just a stable chosen representative + order — no fabrication).
