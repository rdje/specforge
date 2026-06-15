---
id: corpus-reuse-activate-only-no-current-consumer
title: The CORPUS-PATTERN-REUSE activate-only consume mechanism has no valid first opt-in extractor in the current corpus — structurally-safe extractors self-gate (default-on), so the consume side is measured-STANDING / built-deferred (.3b.3a2)
answers:
  - "is there a first opt-in extractor for the CORPUS-PATTERN-REUSE activate-only consume side"
  - "why is CORPUS-PATTERN-REUSE.3b.3b (activate-only consume) deferred / not built"
  - "do specforge extractors override applies_to / are any self-disabled opt-in"
  - "does ExtractionContext carry a cross-document cluster profile"
  - "when should the activate-only ExtractionProfile consume contract be built"
  - "what kind of extractor actually needs the cross-document cluster mechanism"
date: 2026-06-15
tags: [corpus-pattern-reuse, activate-only, opt-in-extractor, extractor-framework, applies_to, extraction-context, measured-standing, built-deferred, yagni]
evidence: crates/specforge/src/ir/extractor.rs (Extractor trait — applies_to defaults true; ExtractionContext carries only `statements`; the two `false` overrides are test-only Toy fixtures); docs/tasks/CORPUS-PATTERN-REUSE.md (.3b.3a2 / Decisions 2026-06-15)
reverify: "grep -rn 'fn applies_to' crates/specforge/src/ | grep -v 'default'  # only test-only Toy fixtures return false; ZERO production overrides. grep -n 'pub struct ExtractionContext' -A6 crates/specforge/src/ir/extractor.rs  # carries only `statements: &[ExtractedStatement]`, no cluster-profile field"
---

**`CORPUS-PATTERN-REUSE.3b.3a2` (`2026-06-15`) measured that the reuse plane's activate-only consume mechanism
has NO valid first opt-in extractor in the current 78-doc corpus — so the consume side (`.3b.3b`) is correctly
built-deferred (measured-STANDING), NOT built-and-waiting.** This generalizes the `.3b.3a` no-go (the serial-prose
bus-line lever, [[corpus-reuse-serial-prose-lever-not-cluster-scopable]]) from one lever to the whole mechanism.

**The framework facts (read-only survey of `ir/extractor.rs` + registered surfaces):**

- `Extractor::applies_to` **defaults `true`**; the contract is "a self-gating extractor (returns `[]` when its
  grammar does not match the document) needs no separate gate."
- **ZERO production extractors override `applies_to`** — the only `false` overrides are the two test-only `Toy`
  fixtures used to exercise the eligible/skip manifest path. So every one of the 8 framework surfaces self-gates
  on the document's own content and runs default-on.
- `ExtractionContext` carries **only the per-document `statements`** (`&[ExtractedStatement]`). There is no
  cross-document profile / `CorpusMemory` / cluster-fingerprint field. Plumbing one in is exactly what `.3b.3b`
  would build.

**Why this means the niche is empty.** A cluster-profile→`applies_to` activation only earns its keep for an
extractor whose applicability is **(a) NOT locally determinable** from the document's own statements yet **(b)
IS predictable from cross-document cluster membership.** But:

- a **structurally-safe** extractor is locally self-testable → it is DEFAULT-ON and never needs the cluster
  mechanism (an extractor that can test the structural property in `applies_to(cx)` from `cx.statements` should
  just be default-on — which is how all 8 surfaces already work); and
- the one lever whose applicability is **lexically** ambiguous (not locally determinable) — the `.9.10` bus-line
  lever — has docs that **do not cluster** (SMBus/I2S/I2C/OpenCAPI-TL are structural singletons).

No extraction occupies the (b)-but-not-(a) niche. Building the activate-only plumbing now would be speculative
(YAGNI) and gate-risky (it touches the extractor path, where wire-based-100% is a hard gate), so it stays
**DEFERRED** with a precise **re-open trigger**: an extraction whose applicability is
cross-document-predictable-but-not-locally-testable AND whose safe docs form a multi-member derived cluster (e.g.
a future corpus carrying ≥2 SMBus-class variants that cluster, making the bus-line lever cluster-safe). The LEARN
side of `.3b` (`.3b.1`/`.3b.2`/`.3c` — typed profile, persisted 8th prior family, manifest sweep) stays complete
and correct; only the CONSUME side is deferred. This mirrors the measured-STANDING resolutions of
[[actor-signal-direction-passive-active-handled]] (NLP-SHALLOW-PARSE build-exhausted) and `MEMORY-BOUNDED-INGEST.5`,
and honors [[feedback_scoring_rigor]] (measured, not assumed) and [[feedback_genericity_guardrail]] (don't build
machinery a real document need does not yet demand).
