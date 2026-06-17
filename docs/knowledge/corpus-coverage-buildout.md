---
id: corpus-coverage-buildout
title: The whole local corpus builds to IntentIR — semantic->intent needs only the persisted evidence_ir.json (not the normalized/ bundle), so 42 evidence-only docs were cheap-built (coverage 36->78 intent / 36->75 isf, 0 failures)
answers:
  - "why do only 36 of 79 ingested docs reach IntentIR / .isf"
  - "how many docs reach evidence vs semantic vs intent vs isf"
  - "can I build semantic/intent without the normalized/ bundle (yes — only evidence_ir.json is needed)"
  - "which docs need a re-ingest (Docling + source PDF) vs a cheap stage rebuild"
  - "do all corpus docs build through the pipeline without failure"
  - "why do some docs reach intent but not isf (honest block: no behavioral content to lower)"
  - "what is the corpus coverage build-out (CORPUS-COVERAGE.0)"
date: 2026-06-17
tags: [corpus-coverage, pipeline, intentir, isf, staleness, re-ingest, normalized-bundle, measured, ram-safe]
evidence: docs/tasks/CORPUS-COVERAGE.md (.0 build-out + census); generated/{evidence,semantic,intent_ir,adapters/isf}/* (coverage counts); KG-ISF-COMPLETENESS.3 (the staleness finding that motivated it)
reverify: "python3 over generated/*: count source_ir(79) vs evidence_ir(78) vs semantic_ir vs intent_ir vs adapters/isf -> after CORPUS-COVERAGE.0 = 78/78/78/75. Rebuild any evidence-only doc with ./target/release/specforge semantic generated/evidence_ir/<key>/evidence_ir.json && intent generated/semantic_ir/<key>/semantic_ir.json -> succeeds with NO normalized/ bundle present. The 3 intent-no-isf docs (risc_v_debug / den0068 BSA / ihi0069_g GIC) -> adapt prints blocking_reasons: no behavioral content."
---

**Measured `2026-06-17` (`CORPUS-COVERAGE.0`, deterministic build-out, RAM steady 77%).**

The pre-existing local corpus had only **36 of 79** ingested docs carried through to IntentIR — the other 42
sat at evidence-only. Cause is operational, not a code bug: a sweep rebuilt EvidenceIR without cascading the
downstream stages, and the per-stage commands do not auto-cascade (only `converge` rebuilds the whole chain).

**Key fact:** building `semantic`→`intent`→`adapt` needs only the already-persisted `evidence_ir.json`, NOT
the heavyweight `normalized/` page-image bundle (that bundle is only needed to rebuild EVIDENCE from source).
So the 42 evidence-only docs are **cheap-buildable** (deterministic, no LLM, no Docling, no re-ingest).

Rebuilding the 42 evidence-only + 3 stale docs (excluding the 4 gated WIRE-BASED-100 docs): **42/42 OK, 0
failures.** Coverage went **36→78 intent / 36→75 isf**; **0 stale-intent remaining**. The 3 intent-without-isf
docs (`risc_v_debug`, coresight `den0068` BSA, GIC arch `ihi0069_g`) **block honestly** ("no behavioral
content to lower" — register/architecture docs), which is the designed honest-blocking behavior, not a
failure. The staged pipeline is now validated end-to-end on the entire corpus.

The generated tree is git-ignored local cache, so a fresh clone re-derives it (run `converge`, or the staged
commands in order). The **57** docs lacking a `normalized/` bundle cannot have their EVIDENCE rebuilt without
a re-ingest (Docling + source PDF, RAM-gated, host-local source re-provisioning). See
[[relation-completeness-staleness-vs-absence]] for the staleness/honest-absence split this built on.
