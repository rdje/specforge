---
id: corpus-cluster-fingerprint
title: Vendor/layout clustering via a derived structural+behavioral fingerprint (corpus_cluster) — emergent families, no vendor list
answers:
  - "how does SpecForge cluster chip-spec PDFs by vendor/layout without hardcoding vendor names"
  - "what is corpus_cluster / document_fingerprint / cluster_documents / DocumentCluster"
  - "what is the per-document fingerprint made of (structural shape + extraction_manifest fired set)"
  - "do the emergent clusters actually track real vendor/layout families"
  - "why are the fired: behavioral features mostly empty in the clustering today"
date: 2026-06-09
tags: [corpus-pattern-reuse, clustering, fingerprint, vendor, extraction-manifest, adr-0006, evidence-ir]
evidence: docs/tasks/CORPUS-PATTERN-REUSE.md (.2); crates/specforge/src/ir/corpus_cluster.rs
reverify: cargo test -p specforge --lib ir::corpus_cluster
---

`CORPUS-PATTERN-REUSE.2` builds the clustering capability for the owner's vendor-pattern-reuse vision
(`[[corpus-pattern-reuse]]`): recognize that a new PDF "is like ones we've seen" so extraction patterns can be
reused — done **agnostically** (ADR 0006: no vendor name in runtime; the cluster key IS the shared structure).

Pure module `crate::ir::corpus_cluster`:
- `document_fingerprint(&EvidenceIr) -> BTreeSet<String>` — a SET of feature tokens from the document's OWN
  IR: **structural shape** = a coarse count bucket per typed surface (`shape:registers:b2`,
  `shape:protocol_states:b0`, … — `b0`/`b1`/`b2`/`b3` = 0 / 1–9 / 10–99 / 100+), available on every doc; plus
  **behavioral shape** = which extractor strategies fired (`fired:registers.field_table`, from the
  `EXTRACTOR-ARCHITECTURE.8` `extraction_manifest`).
- `fingerprint_similarity` = Jaccard `|a∩b|/|a∪b|`; `cluster_documents(&[(key, fp)], threshold)` = deterministic
  greedy agglomeration (sort by key → attach to the first cluster whose representative is ≥ threshold-similar,
  else open a new one) → `DocumentCluster { members, shared_features }` (the shared feature intersection = the
  cluster's vendor-name-free signature).

**It works on real data.** Demonstrated over all 76 persisted evidence docs (threshold 0.6 → 29 clusters, 14
multi-doc). Emergent families track reality with NO vendor list: the 3 CoreSight SoC-600 versions cluster
(identical shape), CCIX r1.0 ↔ r1.1 cluster, **7 AMBA protocol specs** (APB d/e, Trace-bus, AXI-Stream,
Generic-Flash, LTI) fall together on shared `relations:b2 + signal_constraints:b2`. **Coverage caveat:** the
`fired:` behavioral tokens were mostly absent because only ~6 docs were rebuilt since `.8`, so today's
clustering is **structural-shape driven** — yet already recovers real families; a corpus-wide re-ingest sweep
(every doc carrying an `extraction_manifest`) is the enrichment follow-up. Next (`.3`): a `corpus-cluster` CLI
command + the advisory `ExtractionProfile` consumed via `Extractor::applies_to` (bounded, contested-gated —
adjusts attention, never truth; `[[corpus-pattern-reuse]]`).
