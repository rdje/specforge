---
id: corpus-cluster-fingerprint
title: Vendor/layout clustering via a derived structural+behavioral fingerprint (corpus_cluster) — emergent families, no vendor list
answers:
  - "how does SpecForge cluster chip-spec PDFs by vendor/layout without hardcoding vendor names"
  - "what is corpus_cluster / document_fingerprint / cluster_documents / DocumentCluster"
  - "what is the per-document fingerprint made of (structural shape + extraction_manifest fired set)"
  - "do the emergent clusters actually track real vendor/layout families"
  - "why are the fired: behavioral features mostly empty in the clustering today"
  - "how do I see which ingested PDFs form structural families (the corpus-cluster command)"
  - "what is a ClusterExtractionProfile / derive_extraction_profiles (the per-cluster extraction profile)"
  - "why are the corpus-cluster extraction profiles mostly empty / 'none recorded yet'"
  - "what is the ExtractionProfilePriorRecord 8th prior family / extraction_profile_priors in CorpusMemory"
  - "how does learn-priors harvest extraction-profile priors (multi-member clusters only, schema v6)"
  - "how are extraction-profile priors looked up (extraction_profile_priors_for signature-subset match)"
  - "why is the extraction-profile prior family not scoped by ProtocolFamily"
  - "where is the shared 0.6 fingerprint clustering threshold defined (DEFAULT_FINGERPRINT_SIMILARITY_THRESHOLD)"
date: 2026-06-09
tags: [corpus-pattern-reuse, clustering, fingerprint, vendor, extraction-manifest, adr-0006, evidence-ir, prior-memory, learn-priors]
evidence: docs/tasks/CORPUS-PATTERN-REUSE.md (.2/.3a/.3b.1/.3b.2); crates/specforge/src/ir/corpus_cluster.rs; crates/specforge/src/commands/corpus_cluster.rs; crates/specforge/src/ir/prior_memory.rs; crates/specforge/src/commands/learn_priors.rs
reverify: cargo test -p specforge --lib corpus_cluster; cargo test -p specforge --lib extraction_profile; cargo run -p specforge -- corpus-cluster
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
(every doc carrying an `extraction_manifest`) is the enrichment follow-up.

**`.3a` shipped the user-facing surface (`2026-06-09`):** the `corpus-cluster` command
(`crate::commands::corpus_cluster`) walks `generated/evidence_ir/<doc_key>/evidence_ir.json`, fingerprints +
clusters every persisted doc with the `.2` engine, and prints the emergent families (members + shared
structural signature) plus the unique-shape singletons. It is **read-only and additive** — no IR rebuild, no
mutation, no extraction-behavior change — a *window* into corpus structure, deterministic. Live over the
persisted corpus (78 docs, threshold 0.6): 28 clusters / 14 multi-doc families, no vendor list. Args:
`--evidence-root <root>` (default `generated/evidence_ir`), `--threshold <0.0-1.0>` (default `0.6`).
**`.3b.1` added the advisory per-cluster extraction profile (`2026-06-09`):** pure
`derive_extraction_profiles(&[(key, fingerprint)], threshold) -> Vec<ClusterExtractionProfile>` in
`ir::corpus_cluster`. Per cluster it records the cluster's shared signature PLUS the UNION of `fired:` extractor
strategies across members with per-member support (`ProfileExtractorSupport { extractor_name, member_support }`)
— strictly more than the intersection signature, since it keeps a strategy that fired on only SOME members.
Surfaced in the `corpus-cluster` command as a per-family `profile (fired extractors, member support): …` line.
ADR-0006-safe (keyed by structure), read-only, zero extraction-path change. **Honestly sparse:** the `fired:`
tokens only exist on docs (re)built since `EXTRACTOR-ARCHITECTURE.8`, so most families currently print
`none recorded yet (run after a corpus re-ingest sweep)` rather than a guess — the re-ingest sweep fills them in.

**Decided honesty contract for the consume side (`.3b` Decisions `2026-06-09`): ACTIVATE-ONLY.** A consumed
profile may only *activate* an opt-in (self-disabled) extractor for a doc whose fingerprint matches the cluster
— it may NEVER deactivate a default-on extractor (every real `Extractor::applies_to` defaults `true`, so gating
it could only SUPPRESS a real extraction, which the genericity/honesty guardrails forbid). So a profile is
strictly recall-additive and fabrication-impossible. Sequencing: `.3b.2` persists profiles into `CorpusMemory`
(8th prior family + `learn-priors` harvest); `.3b.3` is the activate-only consume via `applies_to` + the first
opt-in extractor + a measured recall uplift. `.4` = an offline LLM/VLM cluster pattern miner.
`[[corpus-pattern-reuse]]`.

**`.3b.2` persisted the profiles as the 8th prior family (`2026-06-09`).** `CorpusMemory` (schema 5→6,
`#[serde(default)]` so v5 stores stay loadable) gained `extraction_profile_priors:
Vec<ExtractionProfilePriorRecord>` — per record: `cluster_signature` (sorted derived feature tokens; the
lookup key), `fired_extractors` (the persisted twin `ExtractionProfileExtractorSupportRecord` of the union
with per-member support), `support_count` (always ≥ 2; **singleton clusters are never harvested** — a cluster
of one has no cross-document pattern), `supporting_document_keys`. This family is keyed directly by the
ADR-0006-safe structural signature rather than the general `PriorScope` (layout families cross identity lines).
Harvested in `learn-priors` from the accepted artifacts' EvidenceIR fingerprints (`load_evidence_ir_for_learning`)
via pure `materialize_extraction_profile_priors`, clustered at the shared
`corpus_cluster::DEFAULT_FINGERPRINT_SIMILARITY_THRESHOLD = 0.6` — the same const that backs the
`corpus-cluster --threshold` default (`default_value_t`), so the families a user SEES are the families the
store LEARNS. Lookup: `CorpusMemory::extraction_profile_priors_for(&fingerprint)` = strict signature-subset
match (every signature token present in the document's own fingerprint; empty signatures skipped — they would
match everything). Live (10 persisted IntentIR artifacts): 2 profiles persisted (a support-5 bus-protocol
family containing both TileLink versions + I2C + HBM2 + GFB, and a support-2 CXS+Wishbone family), no vendor
list, `fired_extractors` honestly empty until the re-ingest sweep.

**`.3c` ran that sweep over every repo-backed doc (`2026-06-09`).** NVMe+I2C evidence rebuilt;
APB/AHB/AXI/AXI-Stream re-ingested from the git-tracked corpus PDFs (`DOCLING_DEVICE=cpu`) + evidence rebuilt +
deterministic semantic/intent/validate chains added so they enter the harvest. Measured: `corpus-cluster`
28/14 → 30/13 clusters/multi (fired tokens make fingerprints MORE distinctive, so some shape-only families
split); `learn-priors` 10→14 accepted, profiles 2→3 — the new AHB+AXI-Stream profile carries a FULL-support
fired union (`semantic_hints.prose (2)`, `semantic_hints.tables (2)`). Wire-based re-verified on the fresh
evidence: APB/AHB/AXI/SWD/I2C all 1.000 (WIRE-BASED-100 filtered); NVMe `register_field` 0 with
`--provider skip` is the VLM-gated `recover-register-bits` dataset (register surface intact 42/42), not a
regression. ~66 host-local-source docs stay manifest-less until their PDFs are re-provided.
