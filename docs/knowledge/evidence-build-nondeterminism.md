---
id: evidence-build-nondeterminism
title: EvidenceIR build determinism — two HashSet-iteration leaks (relations + enum name) found & FIXED (EVIDENCE-DETERMINISM.2)
answers:
  - "why does rebuilding the same SourceIR give a different evidence_ir.json (non-determinism)"
  - "why did an EXTRACTOR-ARCHITECTURE byte-identical proof fail on SWD/ADI but pass on other docs"
  - "what causes actor_signal_relations / extracted_statements to differ run-to-run"
  - "is the EvidenceIR build reproducible / deterministic"
  - "how should a behavior-preserving evidence refactor be verified given the non-determinism"
date: 2026-06-09
tags: [determinism, reproducibility, evidence-ir, hashset, relations, eval, extractor-architecture]
evidence: docs/tasks/EVIDENCE-DETERMINISM.md (.1); crates/specforge/src/ir/evidence.rs (extract_actor_signal_relations — `for signal in known_signals: &HashSet`)
reverify: "build evidence for ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification twice; md5 the two evidence_ir.json — they are now IDENTICAL (fixed by EVIDENCE-DETERMINISM.2; were content-different before)"
---

**FIXED by `EVIDENCE-DETERMINISM.2` (`2026-06-09`).** This card documents a now-resolved defect + the pattern
to avoid re-introducing it.

The EvidenceIR build WAS **non-deterministic at the content level** (not just ordering): two runs of identical
code on the SWD/ADI `source_ir` produced **set-different** `actor_signal_relations` and `extracted_statements`
(found by `EXTRACTOR-ARCHITECTURE.5` — a pure refactor's byte-identical check failed only on SWD). Most docs
(RISC-V Debug, I2C, SWP, CAN) were already deterministic; the leaks were on the **relation-heavy + enum**
paths. **Two root causes, both `HashSet` iteration leaking into output:**
1. `extract_actor_signal_relations` iterated `known_signals: &HashSet<String>` directly, so the relation
   `asr_NNNN` ids + record order (and the first-seen-wins dedup representative) depended on hash order. Fix:
   iterate a **sorted** `Vec<&String>` once before the statement loop. The relation SET (`(actor,signal,kind)`
   dedup keys) is invariant — only ids/order/attribution become stable.
2. `derive_encoding_enum_name` sorted candidate signals by `Reverse(len())` ONLY — a partial order, so
   same-length names (`TDO`/`TDI`) stayed tied and a stable sort preserved the non-deterministic HashSet
   order, making the chosen enum name (and thus `Enum <name> …` statements) non-deterministic. Fix: a
   **total** order (length desc, then name).

**Why it mattered:** (1) reproducibility / crash-safe handoff; (2) eval-score stability — the scorer reads
PERSISTED evidence, so a score could drift run-to-run (violates [[feedback_scoring_rigor]]); (3) it weakened
the `EXTRACTOR-ARCHITECTURE` byte-identical "behavior-preserving" proofs (a sound proof needs a deterministic
baseline). **Verified:** all 6 intact-bundle docs now double-run byte-identical; SWD eval still
`P=R=F1=1.000`; kg-bench 151/151; +1 regression test (build a HashSet twice → identical relation output).
**Pattern to keep:** never let `HashSet`/`HashMap` iteration order reach output — sort first or use a total
order / `BTreeSet`; the fix is multiset-preserving (same facts, stable representative + order — no fabrication).
