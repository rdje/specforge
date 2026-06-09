# CORPUS-PATTERN-REUSE: reuse extraction patterns across PDFs, clustered by derived vendor/layout fingerprint

## Metadata

- Tree ID: `CORPUS-PATTERN-REUSE`
- Status: `active` (design owned; build gated on the extractor-framework foundation + owner go)
- Roadmap lane: `R15` (cross-document learning plane) / `R16` (extraction) — builds on `CorpusMemory`
- Created: `2026-06-09`
- Parent: owner strategic directive (`2026-06-09`) — "we should be able to [recognize/reuse] patterns in
  different PDFs … PDFs from the same brand/vendor will tend to share the same way their PDF is organized …
  even within the same brand it is not certain, but very likely … think out of the box, find unconventional
  but elegant and efficient solutions … this is very new, enabled by the LLM/VLM boom 2–3 years ago."

## Goal

Let SpecForge recognize that a new PDF resembles ones it has seen, and reuse what worked — so extraction
**recall improves and misses shrink** as the corpus grows, especially within a vendor/brand whose documents
share an organizational style. Do it **agnostically** (ADR 0006: no vendor/brand names baked into runtime;
the cluster key is *derived* from document structure) and **honestly** (a reused pattern only changes *where
we look* in the current document — it can never inject a fact the current document does not itself ground).

## Non-goals (the genericity + honesty guardrails — non-negotiable)

- NOT vendor-name hardcoding. No "ARM"/"NXP"/"Intel" literal in runtime ([[feedback_no_hardcoded_chip_spec_names]],
  ADR 0006). The vendor/layout cluster is a *derived structural fingerprint*, labelled by a derived key.
- NOT overfitting that jeopardizes handling ANY chip-spec PDF ([[feedback_genericity_guardrail]]). A pattern
  is ADVISORY: the current document always wins; deviation from a cluster is detected, not suppressed.
- NOT fabrication. Patterns raise recall (reduce misses) only; they never create a signal/width/field/state
  the document does not ground (the standing honesty guardrail; [[project_intent_completeness_research]]).
- NOT a from-scratch learning plane — this EXTENDS the proven `CorpusMemory` (`learn-priors`) + `corpus-kb`.

## Design (`.1` — owned `2026-06-09`)

Four moving parts, each grounded in machinery that already exists or is being built:

1. **Derived vendor/layout fingerprint (the cluster key).** Compute a structural fingerprint per document
   from already-captured signals — front-matter doc-type vocabulary (`.5c`), heading grammar, table-header
   signatures (the `table-shape` priors already learn these), pub-ID format, page/layout metadata in
   `SourceIR`. Cluster documents by fingerprint similarity (unsupervised — emergent families, no vendor
   list). The cluster key labels a *scope* for priors, the way the plane already scopes per "protocol family".

2. **The `ExtractionManifest` IS the behavioral fingerprint.** `EXTRACTOR-ARCHITECTURE.2`'s run manifest —
   *which extractors fired on a doc and what each produced* — is a per-document behavioral signature. Cluster
   on (structural fingerprint + manifest similarity); the consolidation work is the substrate this needs.
   This is why the extractor-framework migration is the foundation: no manifest, no behavioral fingerprint.

3. **Per-cluster extraction profile in `CorpusMemory`.** A learned, advisory `ExtractionProfile` keyed by the
   derived cluster fingerprint: which extractors/surfaces tend to apply, which table-shape/actor/semantic
   priors are reliable for docs like this, where facts tend to live. Consumed through each `Extractor`'s
   `applies_to(cx)` (the framework already has this hook) — the profile *prioritizes/activates* extractors
   for a new doc in a known cluster, then the deterministic grammars + validation ground everything.

4. **LLM/VLM as an OFFLINE corpus-level pattern miner (the unconventional move).** Feed the model N docs from
   a cluster → it proposes the cluster's extraction profile (conventions, where facts live) → **validate each
   proposed pattern against held-out docs in the same cluster** (precision-gated) → promote only validated
   patterns into `CorpusMemory`. Neuro-symbolic at corpus scale, mirroring the proven `VERB-COVERAGE-CORPUS`
   offline mine (640 verbs from 82 PDFs, cross-model-curated, then validated). Provider is the
   production-default Ollama+Qwen ([[project_llm_provider_ollama_qwen]]); offline, never a CI dependency.

**Honesty reconciliation (the core invariant):** a reused pattern adjusts *attention*, never *truth*. The
current document grounds every emitted fact; `PRIOR-DECAY`'s contested-prior detection flags a doc that
deviates from its cluster (the "not certain even within a brand" case) instead of forcing the cluster's
expectation onto it. Recall up, fabrication impossible. Measured by the existing capture–recapture recall
gauges (did the profile reduce misses on held-out docs?).

## Grounding — what already exists to build on

- `CorpusMemory` (`learn-priors`): actor-taxonomy / semantic-phrase / modality-reliability / temporal-phrase
  / **table-shape** priors, harvested from validated `IntentIR`, consumed (advisory, bounded) by
  `evidence`/`converge`. Family-scoped already.
- `corpus-kb` (R15g): tracked cross-document synthesis pages (table/visual/state-machine/timing/infra/protocol
  families) with managed blocks — the human-readable corpus knowledge base.
- `PRIOR-DECAY`: `contested_priors()` — same-key/different-value detection within a family (the deviation gate).
- `EXTRACTOR-ARCHITECTURE`: the `Extractor`/`run_surface`/`ExtractionManifest` substrate (the fingerprint source).
- `VERB-COVERAGE-CORPUS`: the proven offline-LLM-mine → curate → validate → integrate pattern.

## Task tree

- ID: `CORPUS-PATTERN-REUSE` · Status: `active` · Children: `.1` (design/grounding — DONE) + build leaves (gated).
- ID: `CORPUS-PATTERN-REUSE.1` · Status: `done` (`2026-06-09`) · Goal: capture the owner's cross-PDF /
  vendor-clustered pattern-reuse direction as an owned design grounded in the existing prior plane + the
  extractor framework, with the genericity/honesty guardrails made explicit. Verification: docs-only; design
  recorded here + KM card `corpus-pattern-reuse`. Commit: this slice.
- ID: `CORPUS-PATTERN-REUSE.2` · Status: `in_progress` (`2026-06-09`; gate met by `EXTRACTOR-ARCHITECTURE.8` —
  the `extraction_manifest` fingerprint now exists) · Goal: derived vendor/layout **fingerprint** +
  unsupervised clustering over the tracked corpus (structural signature + `ExtractionManifest`); measure that
  emergent clusters track real vendor/layout families WITHOUT a vendor list.
  **Design (this slice):** a pure `crate::ir::corpus_cluster` module. `document_fingerprint(&EvidenceIr) ->
  BTreeSet<String>` emits ADR-0006-safe feature tokens derived from the document's own structure — coarse
  count buckets per surface (`shape:registers:b2`, `shape:protocol_states:b0`, …) PLUS which extractors fired
  (`fired:registers.field_table`, from the `.8` manifest). `fingerprint_similarity` = Jaccard of the feature
  sets; `cluster_documents(&[(key, fp)], threshold)` = deterministic greedy agglomeration (sort by key →
  attach to the first cluster whose representative is ≥ threshold-similar, else open a new one) → clusters with
  their shared feature intersection. No vendor names anywhere (the cluster key is the shared structural
  signature). The structural part works on every persisted doc; the behavioral (`fired:`) part enriches docs
  rebuilt since `.8`. Verification: hermetic tests (synthetic fingerprints → expected clusters; determinism;
  Jaccard math) + a live demonstration over the manifest-bearing docs recorded here + KM. A first-class
  `corpus-cluster` CLI command + full-corpus re-ingest sweep is the `.3`/follow-up (this slice lands the
  capability + proves the concept).
  **DONE (`2026-06-09`).** Pure `crate::ir::corpus_cluster` (`document_fingerprint` / `fingerprint_similarity`
  / `cluster_documents` + `DocumentCluster`), 4 hermetic tests (bucket math, Jaccard, similar-cluster/
  dissimilar-split, order-independent determinism); `run_ci.sh` green (lib 1457 → 1461). **Live demonstration
  over all 76 persisted evidence docs (probe replicating the exact fingerprint), threshold 0.6 → 29 clusters,
  14 multi-doc.** Emergent families track reality WITHOUT any vendor list: the 3 CoreSight SoC-600 versions
  cluster (identical shape); CCIX r1.0 ↔ r1.1 cluster; **7 AMBA protocol specs** (APB d/e, Trace-bus,
  AXI-Stream, Generic-Flash, LTI) fall together on shared `relations:b2 + signal_constraints:b2`; AHB ↔ HBM2;
  CHI ↔ I2C (actor+relation+constraint-heavy). **Honest coverage caveat:** the `fired:` behavioral features
  were mostly absent (only ~6 docs rebuilt since `.8`), so the clustering is currently **structural-shape
  driven** — yet it already recovers real families. Full behavioral enrichment needs a corpus-wide re-ingest
  sweep (so every doc carries an `extraction_manifest`). KM card `corpus-cluster-fingerprint`.
  Follow-ups (`.3`): a first-class `corpus-cluster` CLI command (run the Rust over the corpus, surfaced + book),
  the re-ingest sweep, and the advisory `ExtractionProfile` consumed via `Extractor::applies_to`.
- ID: `CORPUS-PATTERN-REUSE.3` · Status: `active` (split `2026-06-09`) · Goal: surface the `.2` clustering
  capability as a first-class command AND consume a per-cluster advisory profile in extraction. Split into the
  user-facing surfacing (`.3a`, clean/additive, zero extraction-path risk) and the behavioral advisory
  consumption (`.3b`, gated, touches the extractor path) because they are independently reviewable and carry
  very different regression risk (Splitting Rules). · Children: `.3a`, `.3b`.
- ID: `CORPUS-PATTERN-REUSE.3a` · Status: `done` (`2026-06-09`) · Goal: a first-class `corpus-cluster`
  CLI command that walks the persisted `generated/evidence_ir/<doc_key>/evidence_ir.json` corpus, computes each
  document's `document_fingerprint` (`.2`), clusters them with `cluster_documents` (`.2`), and reports the
  emergent families (members + shared structural signature) so the owner can SEE the corpus structure the
  reuse plane will exploit — additive, read-only, no vendor names, no extraction-path change.
  **DONE (`2026-06-09`).** New `crate::commands::corpus_cluster` (registered in `cli.rs`/`lib.rs`/`mod.rs`):
  `collect_corpus_documents` walks the evidence root, loads each readable EvidenceIR, derives its `.2`
  fingerprint, and records honest skips for unloadable/wrong-stage artifacts (dirs without `evidence_ir.json`
  are simply not members); `build_corpus_cluster_report` (pure) clusters + orders the families largest-first
  (deterministic tie-break by representative key); `render_report` prints the header counts, the multi-document
  families with their shared signature + members, then the unique-shape singletons. Args `--evidence-root`
  (default `generated/evidence_ir`) + `--threshold` (default `0.6`). No IR rebuild/mutation, no extraction-path
  change. **Live demonstration over the persisted corpus (78 docs, threshold 0.6): 28 clusters / 14 multi-doc
  families, 0 skipped, NO vendor list** — the 3 CoreSight SoC-600 TRM versions form a family, the 7 AMBA
  protocol specs (APB d/e, Trace-bus, AXI-Stream, Generic-Flash, LTI, OpenCAPI TL) fall together, register-heavy
  AMD-IOMMU ↔ GIC ↔ eMMC cluster. 6 hermetic tests (report ordering/determinism, render, missing-root error,
  unloadable-artifact skip, CLI defaults). `run_ci.sh` green (lib 1481) + kg-bench 151/151 + KM check green.
  Verification: see Verification Log. Commit: see Commit Log.
- ID: `CORPUS-PATTERN-REUSE.3b` · Status: `active` (owner go `2026-06-09`; split same day) · Goal: typed
  advisory `ExtractionProfile` in `CorpusMemory` keyed by cluster fingerprint, consumed via
  `Extractor::applies_to`, bounded + contested-gated. **Split (Splitting Rules) because building the consume
  path surfaced an unresolved honesty-policy choice + a missing dependency** (see Decisions `2026-06-09`
  "activate-only"): every real `Extractor::applies_to` defaults `true`, so a profile that gates `applies_to`
  could only ever turn extractors OFF = suppression, which the genericity/honesty guardrails forbid ("the
  current document always wins; deviation is detected, not suppressed"); the safe contract is the inverse
  (a profile may only ACTIVATE a self-disabled opt-in extractor, never deactivate a default-on one), and no
  opt-in extractor exists yet. So the LEARN side (schema + harvest, zero extraction-path risk) lands first and
  the CONSUME side (the delicate behavioral part) follows. · Children: `.3b.1`, `.3b.2`.
- ID: `CORPUS-PATTERN-REUSE.3b.1` · Status: `done` (`2026-06-09`) · Goal: the typed advisory
  `ClusterExtractionProfile` + a pure `derive_extraction_profiles` derivation in `ir/corpus_cluster.rs`,
  **surfaced in the `corpus-cluster` command**. A profile carries the cluster's shared structural signature
  PLUS the UNION of extractor strategies that fired across the cluster's members with per-extractor member
  support — the genuinely-new info beyond `.3a`'s intersection signature ("what tends to work for docs shaped
  like this").
  **DONE (`2026-06-09`).** `ir/corpus_cluster.rs`: `ProfileExtractorSupport { extractor_name, member_support }`
  + `ClusterExtractionProfile { cluster_signature, members, fired_extractors }` + pure
  `derive_extraction_profiles(documents, threshold)` (clusters, then per cluster unions the members' `fired:`
  tokens with per-member support, sorted by name — deterministic). The `corpus-cluster` command computes the
  profiles and prints a per-family `profile (fired extractors, member support): …` line, or
  `none recorded yet (run after a corpus re-ingest sweep)` when sparse. **Pure + deterministic + ADR-0006-safe
  (keyed by structural signature, never a vendor name); zero `CorpusMemory` churn, zero extraction-path
  change.** Live over the persisted corpus (78 docs): the union-with-support works (e.g.
  `registers.register_map (1), semantic_hints.prose (1)`); honestly only 3/14 families have non-empty profiles
  today because the `fired:` tokens are sparse (only ~6 docs rebuilt since `.8`) — reported truthfully, never
  fabricated. 5 hermetic tests (union-with-support vs intersection, sparse-manifest, determinism, + 2 render).
  `run_ci.sh` green (lib **1486**) + kg-bench 151/151 + KM green. Verification: see Verification Log.
  Commit: see Commit Log.
- ID: `CORPUS-PATTERN-REUSE.3b.2` · Status: `pending` (gated) · Goal: PERSIST profiles into `CorpusMemory` —
  the typed `ExtractionProfilePriorRecord` 8th prior family (`#[serde(default)]`, schema_version bump) +
  `learn-priors` harvest (cluster the accepted input artifacts, emit one advisory profile per multi-member
  cluster) + a `_for` lookup accessor. The broad-but-mechanical schema-integration slice (18 `CorpusMemory`
  literal sites), kept separate from `.3b.1`'s new logic. Additive, no extraction-path change.
- ID: `CORPUS-PATTERN-REUSE.3b.3` · Status: `pending` (gated) · Goal: the CONSUME side — plumb the persisted
  `ExtractionProfile` into `ExtractionContext`, implement the **activate-only** `applies_to` contract
  (a profile may only enable a self-disabled opt-in extractor for a doc whose fingerprint matches the cluster;
  it may NEVER disable a default-on extractor — enforced structurally), contested-gated via
  `contested_priors()`, and demonstrate the first real activated extractor with a measured recall uplift on a
  held-out cluster member. Behavioral (touches the extractor path) → wire-based APB/AHB/AXI/SWD must stay 100%.
- ID: `CORPUS-PATTERN-REUSE.4` · Status: `pending` (gated) · Goal: offline LLM/VLM cluster pattern miner with
  held-out precision validation before promotion; recall-gauge-measured uplift on held-out docs.

## Current frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `CORPUS-PATTERN-REUSE.3b.2` | `pending` | Persist profiles into `CorpusMemory` (8th prior family + `learn-priors` harvest) — the broad-but-mechanical schema slice. Next, after `.3b.1` (the typed profile + derivation) DONE. |
| 2 | `CORPUS-PATTERN-REUSE.3b.3` | `pending` (gated) | The CONSUME side via the **activate-only** `applies_to` contract (Decisions `2026-06-09`); behavioral → needs a first opt-in extractor to activate. Wire-based 100% specs must not regress. |
| 3 | `CORPUS-PATTERN-REUSE.4` | `pending` (gated) | Offline LLM/VLM cluster pattern miner with held-out precision validation. |

`.3a` DONE — the `corpus-cluster` command surfaces the proven `.2` engine (read-only). `.3b` owner-go'd and
split into `.3b.1` (learn/schema, in progress) + `.3b.2` (consume, gated on the activate-only contract + a
first opt-in extractor). `.4` (offline miner) stays gated.

## Decisions

- `2026-06-09`: **Profile consumption is ACTIVATE-ONLY (honesty contract for `.3b`).** A consumed
  `ExtractionProfile` may only *activate* an opt-in (self-disabled) extractor for a document whose fingerprint
  matches the learned cluster — it may **never** deactivate a default-on extractor. Rationale: every real
  `Extractor::applies_to` defaults `true`, so any profile that gated `applies_to` could only turn extractors
  OFF, i.e. SUPPRESS a real extraction — directly violating the genericity/honesty guardrails
  ([[feedback_genericity_guardrail]], [[project_intent_completeness_research]]: "the current document always
  wins; deviation from a cluster is detected, not suppressed; patterns raise recall, never create or remove a
  fact"). Activate-only makes a profile strictly recall-additive and fabrication-impossible. Consequence: the
  consume side (`.3b.2`) needs a first opt-in extractor to activate (none exists today), so the learn side
  (`.3b.1`) lands first and the consume side is its own gated leaf.
- `2026-06-09`: **Profiles are harvested, not authored.** `.3b.1` derives profiles by clustering the accepted
  `learn-priors` input artifacts' `.2` fingerprints and aggregating per cluster — the same accrete-from-
  validated-IntentIR discipline as the other 7 prior families; advisory-only, contested-aware.

`.1` design owned; `.2` clustering engine DONE (works over the persisted corpus). The build (`.3`–`.4`) was
**sequenced behind `EXTRACTOR-ARCHITECTURE`** — its run manifest is the behavioral fingerprint this plane
clusters on — and that foundation is now in place (`.8` manifest exists). `.3a` surfaces the proven engine
first (no behavior change); `.3b`/`.4` (advisory consumption + offline miner) stay gated on an owner go.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-09` | `CORPUS-PATTERN-REUSE.3a` | `scripts/run_ci.sh` (fmt-check + clippy `-D warnings` + lib tests + rustdoc + mdBook) | green — lib **1481** passed / 0 failed |
| `2026-06-09` | `CORPUS-PATTERN-REUSE.3a` | `cargo run -p specforge -- kg-bench` | 151/151 pass |
| `2026-06-09` | `CORPUS-PATTERN-REUSE.3a` | `knowledge-map/scripts/check_knowledge_map.sh` | green (facts valid, map in sync) |
| `2026-06-09` | `CORPUS-PATTERN-REUSE.3a` | live `corpus-cluster` over persisted corpus (78 docs, threshold 0.6) | 28 clusters / 14 multi-doc families / 0 skipped; emergent families track real vendor/layout shape with no vendor list |
| `2026-06-09` | `CORPUS-PATTERN-REUSE.3b.1` | `scripts/run_ci.sh` (fmt-check + clippy `-D warnings` + lib tests + rustdoc + mdBook) | green — lib **1486** passed / 0 failed |
| `2026-06-09` | `CORPUS-PATTERN-REUSE.3b.1` | `cargo run -p specforge -- kg-bench` + KM check | 151/151 pass; KM green |
| `2026-06-09` | `CORPUS-PATTERN-REUSE.3b.1` | live `corpus-cluster` profile surfacing over persisted corpus | union-with-support works (`registers.register_map (1), semantic_hints.prose (1)`); 3/14 families non-empty today (manifest sparse) — honestly reported `none recorded yet` elsewhere |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `CORPUS-PATTERN-REUSE.3a` | `CORPUS-PATTERN-REUSE.3a — corpus-cluster command` | New read-only command surfacing the `.2` clustering engine; book + README + KM in sync |
| `CORPUS-PATTERN-REUSE.3b.1` | `CORPUS-PATTERN-REUSE.3b.1 — per-cluster extraction profile` (this slice) | Typed `ClusterExtractionProfile` + `derive_extraction_profiles` + command surfacing; activate-only decision recorded; book + README + KM in sync |

## Changelog

- `2026-06-09`: Split `.3` (was a single `pending` leaf) into the container `.3` + children `.3a` (the
  user-facing `corpus-cluster` command — clean, additive, read-only) and `.3b` (the behavioral advisory
  `ExtractionProfile` consumption — gated), per the Splitting Rules (independently reviewable, very different
  regression risk). Implemented + closed `.3a` (new `corpus-cluster` command; live 78-doc demonstration;
  `run_ci.sh` green lib 1481, kg-bench 151/151, KM green). Frontier advanced to `.3b`/`.4` (gated).
- `2026-06-09`: Owner gave the go on `.3b`. Split `.3b` into container + `.3b.1` (typed profile + derivation +
  command surfacing) / `.3b.2` (persist into `CorpusMemory`) / `.3b.3` (activate-only consume), recorded the
  **activate-only** honesty contract in Decisions, and implemented + closed `.3b.1` (`ClusterExtractionProfile`
  + `derive_extraction_profiles` + per-family `profile (fired extractors…)` line; lib 1486; honestly sparse
  today). Frontier advanced to `.3b.2`.
