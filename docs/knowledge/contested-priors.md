---
id: contested-priors
title: SpecForge detects contested priors (same key, conflicting values across docs) — read-only
answers:
  - "how does SpecForge detect contradicting or conflicting priors"
  - "does SpecForge revise or decay priors"
  - "what is a contested prior"
  - "where are cross-document prior contradictions surfaced"
  - "does corpus prior memory only accrete"
date: 2026-06-04
tags: [corpus-memory, priors, cross-document, residual-honesty, prior-decay]
evidence: crates/specforge/src/ir/prior_memory.rs; crates/specforge/src/commands/learn_priors.rs
reverify: grep -n "fn contested_priors" crates/specforge/src/ir/prior_memory.rs
---

`CorpusMemory` priors **accrete only** — support grows, contradictions are never noticed.
`CorpusMemory::contested_priors()` (read-only) closes that gap by detecting **cross-document
contradictions**: keys that two or more documents map, *within one protocol family*, to
**different** values (e.g. a term → `RequesterLike` vs `CompleterLike`; a phrase →
`HandshakeValidLike` vs `HandshakeReadyLike`). Covers the **ActorTaxonomy / SemanticPhrase /
TableShape** families; for each contested key it reports the competing values + their support +
the **strongest-supported value as an advisory hint** (never auto-resolved). Surfaced by the
`learn-priors` command as a `contested_priors:` line.

**Additive** — it does NOT mutate the harvest, merge, or consultation (zero behavior/fixture
change); it only adds insight. Realizes the Parisi (2019) revision-on-contradiction gap
(`PRIOR-DECAY`); *down-weighting* a contested prior during consultation and *time-based
staleness* are deliberate later steps (the latter has no clean recency ordering in
document-keyed priors). See `docs/tasks/PRIOR-DECAY.md` and `cross-document-learning.md`.
