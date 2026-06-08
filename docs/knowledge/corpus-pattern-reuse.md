---
id: corpus-pattern-reuse
title: Cross-PDF pattern reuse via a DERIVED vendor/layout fingerprint — extend CorpusMemory, advisory + honest
answers:
  - "how does / will SpecForge reuse extraction patterns across different PDFs"
  - "how to exploit that same-vendor / same-brand PDFs share organization without hardcoding vendor names"
  - "what is the vendor/layout fingerprint, the ExtractionProfile, and the offline corpus pattern miner"
  - "how does pattern reuse stay agnostic (ADR 0006) and honest (no fabrication / no overfitting)"
  - "how does the EXTRACTOR-ARCHITECTURE run manifest enable cross-document clustering"
date: 2026-06-09
tags: [corpus-memory, priors, vendor, clustering, learning, extractor, adr-0006, genericity, corpus-pattern-reuse]
evidence: docs/tasks/CORPUS-PATTERN-REUSE.md (.1 design); crates/specforge/src/ir/prior_memory.rs (CorpusMemory); crates/specforge/src/ir/extractor.rs (ExtractionManifest); docs/tasks/VERB-COVERAGE-CORPUS.md (proven offline-mine pattern)
reverify: ls docs/tasks/CORPUS-PATTERN-REUSE.md && grep -n "ExtractionManifest" crates/specforge/src/ir/extractor.rs
---

Owner directive (`2026-06-09`): reuse extraction patterns across PDFs; same-vendor/brand docs tend to share
organization (likely, not certain — even within a brand). Reduce MISSES as the corpus grows. This is new
territory, cheap only because of the LLM/VLM boom — so use unconventional but grounded mechanisms.

**It is NOT greenfield.** SpecForge already has the cross-document plane: `CorpusMemory` (`learn-priors`)
harvests reusable, document-agnostic priors (table-header signatures, actor-taxonomy, semantic/temporal
phrasings) from validated `IntentIR` and feeds them back into `evidence`/`converge` to *widen interpretation*
of new docs, bounded so a prior can never inject a fact the current document doesn't ground; `corpus-kb`
compiles cross-doc synthesis pages; `PRIOR-DECAY` flags contested priors. `CORPUS-PATTERN-REUSE` *extends*
this with a vendor/layout dimension.

**The four parts of the design:** (1) a **derived structural fingerprint** per document (front-matter
doc-type vocab, heading grammar, table-header signatures, pub-ID format, layout metadata) → the cluster key,
labelled by a derived id, NEVER a hardcoded vendor name (ADR 0006). (2) The `EXTRACTOR-ARCHITECTURE`
**`ExtractionManifest` is the per-document behavioral fingerprint** (which extractors fired + produced) — so
cluster on structural + behavioral similarity; this is why the extractor-framework consolidation is the
foundation. (3) A typed advisory **`ExtractionProfile`** in `CorpusMemory` keyed by cluster fingerprint,
consumed via each `Extractor::applies_to` (the framework hook) — it prioritizes/activates extractors for a
new doc in a known cluster, then the deterministic grammars + validation ground everything. (4) The
LLM/VLM as an **offline corpus-level pattern miner**: propose a cluster's profile from N docs → validate each
proposed pattern against held-out docs (precision-gated) → promote only validated patterns — neuro-symbolic
at corpus scale, the proven `VERB-COVERAGE-CORPUS` shape.

**The invariant that keeps it safe (genericity + honesty):** a reused pattern adjusts *attention*, never
*truth*. The current document grounds every emitted fact; contested-prior detection flags a doc that deviates
from its cluster instead of forcing the cluster's expectation onto it. Recall up (fewer misses), fabrication
impossible, ANY chip-spec PDF still handled. Uplift is measured by the existing capture–recapture recall
gauges on held-out docs. Build (`CORPUS-PATTERN-REUSE.2`–`.4`) is sequenced behind the extractor framework.
