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
contradictions**: keys that two or more documents map in the one neutral global scope to
**different** values (e.g. a term → `RequesterLike` vs `CompleterLike`; a phrase →
`HandshakeValidLike` vs `HandshakeReadyLike`). Covers the **ActorTaxonomy / SemanticPhrase /
TableShape** families; for each contested key it reports the competing values + their support +
the **strongest-supported value as an advisory hint** (never auto-resolved). Surfaced by the
`learn-priors` command as a `contested_priors:` line.

**Additive** — it does NOT mutate the harvest, merge, or consultation (zero behavior/fixture
change); it only adds insight. Realizes the Parisi (2019) revision-on-contradiction gap
(`PRIOR-DECAY`).

**Consultation already abstains on a contest — the "down-weight" is a NO-BUILD.** The resolvers
`CorpusMemory::resolve_actor_taxonomy_role` / `resolve_semantic_phrase_role` collect the matching
priors' values into a `BTreeSet` and **return `None` when `len() > 1`** (a single value applies; a
contest abstains). So a contested prior is never auto-applied during extraction — exactly the
conservative behavior the advisory hint implies. Therefore PRIOR-DECAY's *detection* +
`resolve_*`'s *abstention* together complete the contest story; no further consultation
"down-weight" is needed (verified 2026-06-04). *Time-based staleness* remains out of scope (no
clean recency ordering in document-keyed priors). See `docs/tasks/PRIOR-DECAY.md`,
`cross-document-learning.md`.
