# CORPUS-PATTERN-REUSE: reuse extraction patterns across PDFs, clustered by derived vendor/layout fingerprint

## Metadata

- Tree ID: `CORPUS-PATTERN-REUSE`
- Status: `active` (learn side complete; **consume side `.3b.3` measured-STANDING / built-deferred** `2026-06-15` — no valid first opt-in extractor in the current corpus; only remaining buildable leaf is `.4` offline miner, gated on owner go)
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
  the CONSUME side (the delicate behavioral part) follows. · Children: `.3b.1`, `.3b.2`, `.3b.3`.
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
- ID: `CORPUS-PATTERN-REUSE.3b.2` · Status: `done` (`2026-06-09`) · Goal: PERSIST profiles into `CorpusMemory` —
  the typed `ExtractionProfilePriorRecord` 8th prior family (`#[serde(default)]`, schema_version bump) +
  `learn-priors` harvest (cluster the accepted input artifacts, emit one advisory profile per multi-member
  cluster) + a `_for` lookup accessor. The broad-but-mechanical schema-integration slice (18 `CorpusMemory`
  literal sites), kept separate from `.3b.1`'s new logic. Additive, no extraction-path change.
  **DONE (`2026-06-09`).** `ir/prior_memory.rs`: `ExtractionProfilePriorRecord` (cluster signature = the
  vendor-name-free lookup key, deliberately NOT `ProtocolFamily`-scoped — the signature IS the scope; members;
  `fired_extractors` union with per-member support via the persisted twin
  `ExtractionProfileExtractorSupportRecord`; `support_count` always ≥ 2) + `#[serde(default)]`
  `CorpusMemory.extraction_profile_priors` + `extraction_profile_priors_for(fingerprint)` strict
  signature-subset lookup (empty signatures skipped — they would match everything). Schema bumped 5→6 at all
  8 literal `schema_version` sites; the 9 exhaustive `CorpusMemory {` literals gained the field (the 7
  `..make_test_corpus()` spread sites need nothing — `#[serde(default)]` keeps old stores loadable);
  kg-bench `PriorMemoryPatch` gained a symmetric `extraction_profile_priors` patch field for future `.3b.3`
  fixtures. `learn-priors`: accepted artifacts' EvidenceIR fingerprints collected via new
  `load_evidence_ir_for_learning` (refactored out of `load_source_ir_for_learning`, no behavior change),
  materialized by pure `materialize_extraction_profile_priors` (multi-member clusters only — a cluster of one
  carries no cross-document pattern) at the NEW shared
  `corpus_cluster::DEFAULT_FINGERPRINT_SIMILARITY_THRESHOLD = 0.6` (also wired into the `corpus-cluster`
  CLI default via `default_value_t`, single source of truth — the families the user SEES are the families
  the store LEARNS). Live: `learn-priors` over the 10 persisted IntentIR artifacts → 10 accepted,
  `extraction_profile_priors: 2` (TileLink 1.7.1+1.8.0+I2C+HBM2+GFB support-5 family; CXS+Wishbone
  support-2 family), `fired_extractors` honestly empty (evidence predates the `.8` manifest — the recorded
  sparsity caveat), schema v6 persisted. 5 new hermetic tests (subset/partial/empty-signature lookup;
  union-with-support multi-member materialization; singleton-only → empty). Verification: see Verification
  Log. Commit: see Commit Log.
- ID: `CORPUS-PATTERN-REUSE.3b.3` · Status: `active` (design captured `2026-06-15`; split into selection +
  build) · Goal: the CONSUME side — plumb the persisted `ExtractionProfile` into `ExtractionContext`, implement
  the **activate-only** `applies_to` contract (a profile may only enable a self-disabled opt-in extractor for a
  doc whose fingerprint matches the cluster; it may NEVER disable a default-on extractor — enforced
  structurally), contested-gated via `contested_priors()`, and demonstrate the first real activated extractor
  with a measured recall uplift on a held-out cluster member. Behavioral (touches the extractor path) →
  wire-based APB/AHB/AXI/SWD must stay 100%. **The blocker was "needs a FIRST opt-in extractor (design open)";
  design now captured in Decisions `2026-06-15` + split into `.3b.3a` (select the candidate, measurement-first)
  → `.3b.3b` (build it self-disabled + plumb the activate-only contract + measure uplift).** · Children:
  `.3b.3a`, `.3b.3b`.
- ID: `CORPUS-PATTERN-REUSE.3b.3a` · Status: `done` (`2026-06-15`) · Goal: **SELECT + justify the first opt-in
  extractor, measurement-first (read-only, no extraction-path change).** Using the `corpus-cluster` profiles
  (which extractors fire per derived family) + the persisted corpus, find a candidate construction that is
  (a) measurably HELPFUL on one derived cluster, (b) too NOISY/unsafe to run default-on corpus-wide (which is
  exactly WHY it must be opt-in — justifying the activate-only machinery), and (c) bounded + deterministic +
  grounded (ADR 0006, no denylist). Strong candidate to evaluate FIRST: the `PDF-VARIANT-DIGESTION.9.10` prose
  bus-line signal lever — PARKED corpus-wide precisely because a universal version needs a forbidden denylist,
  but if scoped to ONLY the cluster where it is measurably safe (the serial-protocol / SMBus-class family) it
  becomes a textbook activate-only extractor: default-off (noisy elsewhere), activated only for the matching
  fingerprint. Output: a one-candidate decision with the measured "helps family X / noisy elsewhere" evidence,
  or an honest "no safe candidate yet" with what is missing. Acceptance: a justified candidate (or honest
  no-go) recorded with measurement; no code/extraction-path change.
  **DONE (`2026-06-15`) — MEASURED NO-GO for the leading candidate; the result SHARPENS the selection
  criterion.** Ran the candidate measurement read-only over the persisted 78-doc corpus (current
  `target/release/specforge`, post-`.3e`); no code/extraction-path change.
  - **Faithful Form-A re-derivation** (the `.9.10` grammar "the/The `<ALLCAPS-id>[#]` line" over every doc's
    `extracted_statements[].text`) reproduces the probe exactly: **fires on 5/78 docs, 0 wire-based.** Genuine
    benefit on only 2 docs — SMBus (`SMBCLK`/`SMBDAT`/`SMBSUS#`) + I2S (`WS`); NOISE on the rest — I2C
    (`VDD`/`VSS`/`DLEN` on top of real `SCL`/`SDA`), eMMC (`VDD` on top of real `CMD`/`DAT`), and a 5th the
    probe did not name — OpenCAPI-TL, where Form A FALSE-POSITIVES `AFUC2` out of cache-line prose
    ("…128-byte segments of the AFU… line").
  - **Cluster mapping at the SYSTEM threshold (0.6)** — the decisive measurement: **none of the 5 sit in a
    derived 2-wire-bus family.** SMBus, I2S, I2C, OpenCAPI-TL are each structural SINGLETONS; eMMC's only
    cluster partner is GIC (a register-heavy interrupt-controller spec — family 12), not a bus. A threshold
    sweep (0.40–0.60) confirms there is NO clean serial-bus family at any cut: at ≥0.55 the buses are
    singletons; at ≤0.50 they dissolve into 15–32-doc heterogeneous catch-alls sharing only the trivial
    ABSENCE token `serial_frame:b0` (OpenCAPI/CoreSight/HBM/… — not a bus family).
  - **Why this is a structural no-go, not a threshold artifact:** the lever's safe/noisy split is **orthogonal
    to the structural fingerprint.** SMBus (safe) and I2C (noisy) are both 2-wire buses with near-identical
    shape; the only thing separating them is whether the doc's prose says "the `VDD` line" — a LEXICAL property
    the structural fingerprint cannot see. Any cluster broad enough to carry SMBus's benefit also carries I2C's
    harm — and I2C is a MEASURED doc (declared-signal gold precision 0.600, [[feedback_scoring_rigor]]), so
    activating there REGRESSES a tracked score. The persisted `learn-priors` profile confirms it from the other
    side: I2C falls in a support-5 bus-SHAPE profile (TileLink/I2C/HBM2/GFB) while SMBus/I2S are absent from the
    harvest entirely — so the only learnable bus-ish cluster contains the regression doc and NOT the benefit docs.
  - **Conclusion:** cluster-scoping (a STRUCTURAL gate) cannot resolve the `.9.10` parking reason (a LEXICAL
    discrimination) — it inherits the exact blindness that made the supply-rail DENYLIST forbidden
    ([[feedback_avoid_denylists_prefer_structural]]). This lever genuinely needs participation-based signal
    identity (the `NLP-SHALLOW-PARSE` path, independently measured build-exhausted) — NOT the reuse plane. So
    the serial-prose parked levers (`.9.10`/`.9.8b`, lexically-discriminated by construction) are EXCLUDED as
    first-opt-in candidates. **Criterion established** (Decisions `2026-06-15`): a valid first opt-in extractor
    must be STRUCTURALLY-discriminated — safe *because of* a structural property the fingerprint captures and
    noisy only on docs lacking it — so a derived multi-member cluster cleanly separates "activate here" from
    "noisy there." `.3b.3b` (build) stays correctly gated; selection continues at `.3b.3a2`. KM card
    `corpus-reuse-serial-prose-lever-not-cluster-scopable`.
- ID: `CORPUS-PATTERN-REUSE.3b.3a2` · Status: `done` (`2026-06-15`) · Goal:
  **continue first-opt-in-extractor selection under the STRUCTURAL-discrimination criterion** (`.3b.3a` measured
  that the serial-prose levers are lexically-discriminated and so cannot be cluster-scoped). Measurement-first,
  read-only: look for an extraction that is safe *because of* a structural property a derived multi-member
  cluster's fingerprint captures (e.g. a family with a distinctive `fired:`/shape signature such as the CCIX
  message-field family — family 7) and noisy/absent elsewhere, so activation aligns with the fingerprint rather
  than a name list. Acceptance: a structurally-justified candidate (or an honest no-go) recorded with
  measurement; no code/extraction-path change. Gates `.3b.3b`.
  **DONE (`2026-06-15`) — MEASURED NO-GO for ANY first opt-in extractor in the current corpus; the activate-only
  consume mechanism has no valid first consumer yet → the consume side is measured-STANDING (built-deferred).**
  Surveyed the `Extractor` framework (`ir/extractor.rs` + the registered surfaces) read-only — the structural
  reason GENERALIZES beyond the serial-prose lever:
  - **Every production extractor self-gates LOCALLY.** `Extractor::applies_to` defaults `true`, and the contract
    is "a self-gating extractor (returns `[]` when its grammar does not match) needs no separate gate."
    Measured: **ZERO production extractors override `applies_to`** (the only `false` overrides are the two
    test-only `Toy` fixtures), and **`ExtractionContext` carries only the per-document `statements`** — there is
    no cross-document profile field today (plumbing one in is exactly what `.3b.3b` would build). So structural
    applicability is decided from the document's OWN content, per-document.
  - **The niche the activate-only mechanism serves is EMPTY in this corpus.** A cluster-profile→`applies_to`
    activation only adds value for an extractor whose applicability is (a) NOT locally determinable from the
    document's own statements yet (b) IS predictable from cross-document cluster membership. But (i) a
    STRUCTURALLY-safe extractor is locally self-testable → it is DEFAULT-ON and never needs the cluster
    mechanism (how all 8 framework surfaces already work), and (ii) the one lever whose applicability is NOT
    locally determinable — the LEXICALLY-ambiguous `.9.10` bus-line lever (`.3b.3a`) — has docs that do not
    cluster (singletons). No extraction occupies the (b)-but-not-(a) niche.
  - **Conclusion:** there is no valid first opt-in extractor to select in the current 78-doc corpus, so building
    the activate-only consume plumbing (`.3b.3b`) now would be speculative (YAGNI) and gate-risky (it touches the
    extractor path) — it stays correctly DEFERRED. The consume frontier (`.3b.3`) becomes measured-STANDING with
    a PRECISE re-open trigger: an extraction whose applicability is cross-document-predictable-but-not-locally-testable
    AND whose safe docs form a multi-member derived cluster (e.g. a future corpus carrying ≥2 SMBus-class variants
    that cluster, making the bus-line lever cluster-safe). The learn side of `.3b` stays complete; `.4` (offline
    miner) stays gated. KM card `corpus-reuse-activate-only-no-current-consumer`.
- ID: `CORPUS-PATTERN-REUSE.3b.3b` · Status: `pending` (gated — DEFERRED until a real consumer appears, per `.3b.3a2`) · Goal: **BUILD the selected opt-in
  extractor self-disabled (`applies_to` defaults `false`) + plumb the persisted `ExtractionProfile` into
  `ExtractionContext` + implement the structurally-enforced activate-only `applies_to` (enable iff the doc
  fingerprint is a subset-match of the cluster signature AND the prior is not contested) + measure recall uplift
  on a HELD-OUT cluster member.** Behavioral → wire-based APB/AHB/AXI/SWD must stay 100% (hard gate); the
  opt-in extractor emits nothing on a non-matching doc by construction (so the byte-identical guarantee on every
  current doc holds until a matching cluster is seen). Acceptance: the extractor activates only for its cluster
  (proven by a fixture pair + a held-out live doc), raises recall there without fabricating, leaves every other
  doc byte-identical, wire-based 100% intact, `kg-bench` green.
- ID: `CORPUS-PATTERN-REUSE.3c` · Status: `done` (`2026-06-09`) · Goal: the **manifest-population
  sweep** (the recorded data lever) — no code change. (1) Rebuild `evidence` for the persisted docs whose
  normalized bundles are intact but whose evidence predates the `.8` manifest (NVMe, I2C). (2) Re-ingest
  (`DOCLING_DEVICE=cpu`) the four git-tracked AMBA PDFs whose normalized bundles were reclaimed (APB `ihi0024_e`,
  AHB `ihi0033_c`, AXI `ihi0022_l`, AXI-Stream `ihi0051_b`) and rebuild their `evidence`, so every repo-backed
  doc carries an `extraction_manifest`. (3) Re-run `corpus-cluster` + `learn-priors` and record the measured
  profile enrichment (before: profiles' `fired_extractors` empty). (4) Re-measure the wire-based eval on the
  re-ingested specs (the standing 100% guarantee is only measurable after re-ingest —
  `eval-scores-persisted-evidence`); any deviation is reported honestly, never hidden. Honest scope limit: the
  ~66 evidence docs whose sources are host-local stay manifest-less until their PDFs are re-provided — partial
  by design.
  **DONE (`2026-06-09`).** All 6 docs re-ingested/rebuilt now carry real fired sets (e.g. AXI =
  `registers.field_table, fsm.transition_bound, semantic_hints.tables, semantic_hints.prose`). Also built the
  deterministic `semantic → intent → validate` chain for the 4 AMBA docs so they enter the `learn-priors`
  harvest (scores 72/62/80/67 — deterministic-only, no VLM/NLP pass, honestly lower than historical converge
  scores). **Measured enrichment:** `corpus-cluster` 78 docs → 30 clusters / 13 multi-doc (was 28/14 — fired
  tokens sharpen distinctions), non-empty family profiles 4→5; `learn-priors` 10→14 accepted, profiles 2→**3**
  — the NEW AHB+AXI-Stream profile carries a full-support behavioral union (`semantic_hints.prose (2)`,
  `semantic_hints.tables (2)`, `registers.register_map (1)`) and the bus-protocol support-5 profile gained
  I2C's `actors.prose`/`semantic_hints.prose`. **Wire-based guarantees re-verified on the FRESH evidence:**
  APB/AHB/AXI constraints+relations+temporal, I2C signals, SWD all **1.000** on the WIRE-BASED-100 filtered
  metrics; NVMe `register_field` reads 0 with `--provider skip` because that dataset is the VLM-gated
  `recover-register-bits` surface (deterministic register surface intact: 42/42 registers with fields) — not
  a regression. kg-bench 151/151 after the sweep. Verification: see Verification Log. Commit: see Commit Log.
- ID: `CORPUS-PATTERN-REUSE.4` · Status: `pending` (gated) · Goal: offline LLM/VLM cluster pattern miner with
  held-out precision validation before promotion; recall-gauge-measured uplift on held-out docs.

## Current frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `CORPUS-PATTERN-REUSE.3b.3` (consume) | `measured-STANDING` (built-deferred) | `.3b.3a` + `.3b.3a2` MEASURED that the activate-only consume mechanism has NO valid first consumer in the current 78-doc corpus: structurally-safe extractors self-gate LOCALLY (all 8 surfaces are default-on; zero `applies_to` overrides), and the one lexically-ambiguous lever (`.9.10`) has docs that don't cluster. Re-open trigger: an extraction that is cross-document-predictable-but-not-locally-testable AND whose safe docs form a multi-member cluster. |
| 1 (gated) | `CORPUS-PATTERN-REUSE.4` | `pending` (gated on owner go) | Offline LLM/VLM cluster pattern miner with held-out precision validation — the remaining buildable leaf, RAM-heavy, gated. |

`.3a` DONE (the `corpus-cluster` command), `.3b.1` DONE (typed profile + derivation + surfacing), `.3b.2`
DONE (profiles persisted into `CorpusMemory` as the 8th prior family + `learn-priors` harvest + subset-match
lookup), `.3c` DONE (the manifest-population sweep over every repo-backed doc — profiles now carry real fired
unions), **`.3b.3a` + `.3b.3a2` DONE (`2026-06-15`) — the first-opt-in-extractor SELECTION is a measured NO-GO**:
`.3b.3a` excluded the leading serial-prose lever (lexical-vs-structural), and `.3b.3a2` proved the structural
reason generalizes (self-gating ⟹ default-on; the activate-only niche is empty in this corpus) → the **consume
side `.3b.3` is measured-STANDING (built-deferred)** with a precise re-open trigger. The LEARN side of `.3b` is
complete; the only remaining buildable leaf is `.4` (offline miner, RAM-heavy, gated on owner go). Residual
sparsity (~66 host-local-source docs) shrinks only when their PDFs are re-provided — honest scope, not debt.

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
- `2026-06-15`: **First-opt-in-extractor design + selection criteria (resolves the `.3b.3` "design open"
  blocker; owner-directed "do all these").** The consume side has always needed a FIRST self-disabled opt-in
  extractor for the activate-only contract to have anything to activate. Decision: SELECT it measurement-first
  (`.3b.3a`) rather than inventing one — the right candidate is a construction that is (a) measurably helpful on
  exactly one derived cluster, (b) too noisy/unsafe to run default-on corpus-wide (this is the property that
  *justifies* opt-in — a universally-safe extractor should just be default-on), and (c) bounded + deterministic
  + grounded with NO denylist (ADR 0006, `[[feedback_avoid_denylists_prefer_structural]]`). Leading candidate:
  the **`PDF-VARIANT-DIGESTION.9.10` prose bus-line signal lever**, parked corpus-wide precisely because a
  universal version needs a forbidden denylist — but cluster-scoped to the serial-protocol/SMBus family where it
  is safe, it is a textbook activate-only extractor (default-off, fingerprint-activated). This elegantly turns a
  parked lever into the reuse-plane's first consumer. Build is `.3b.3b` (self-disabled extractor + plumb profile
  into `ExtractionContext` + structural activate-only `applies_to` + held-out uplift), wire-based 100% a hard
  gate, every non-matching doc byte-identical by construction.

- `2026-06-15`: **First opt-in extractor must be STRUCTURALLY-discriminated (criterion from the `.3b.3a`
  measured no-go).** `.3b.3a` measured the leading candidate — the parked `.9.10` prose bus-line lever — and
  found it NOT cluster-scopable: it fires on 5/78 docs (genuine benefit only on SMBus + I2S, both structural
  SINGLETONS; noise / false-positives on I2C, eMMC, OpenCAPI-TL), and no derived cluster at any threshold
  cleanly carves out a 2-wire-bus family (singletons at ≥0.55; heterogeneous absence-token catch-alls at
  ≤0.50). Root cause is general: the lever's safe/noisy discrimination is **lexical** (supply rails
  `VDD`/`VSS`), which is **orthogonal to the structural fingerprint** — so a structural gate (cluster-scoping)
  inherits the same blindness that made the denylist forbidden ([[feedback_avoid_denylists_prefer_structural]]),
  and the only learnable bus-ish cluster contains the measured-regression doc I2C (gold precision 0.600,
  [[feedback_scoring_rigor]]). **Therefore the serial-prose parked levers (`.9.10`/`.9.8b`) are excluded as
  first-opt-in candidates** — they need participation-based identity (the `NLP-SHALLOW-PARSE` path, build-exhausted),
  not the reuse plane. The reusable criterion: a valid first opt-in extractor must be safe *because of* a
  structural property the fingerprint captures (so a derived multi-member cluster separates "activate here" from
  "noisy there"); selection continues at `.3b.3a2`. This keeps the activate-only consume machinery (`.3b.3b`)
  honestly gated rather than built around a lever it cannot safely serve.
- `2026-06-15`: **The activate-only consume mechanism has NO valid first consumer in the current corpus →
  consume side measured-STANDING (built-deferred) — `.3b.3a2`.** Surveying the `Extractor` framework proved the
  `.3b.3a` no-go generalizes: structural applicability is decided per-document (`ExtractionContext` carries only
  `statements`; `applies_to` defaults `true`; ZERO production extractors override it — all 8 surfaces self-gate
  and are default-on), so a STRUCTURALLY-safe extractor never needs the cluster mechanism, and the only
  lexically-ambiguous lever (`.9.10`) has non-clustering docs. The activate-only niche — applicability
  cross-document-predictable but NOT locally testable, with the safe docs in a multi-member cluster — is empty
  here. Building `.3b.3b` now would be speculative (YAGNI) and gate-risky (touches the extractor path), so it
  stays DEFERRED with a precise re-open trigger (a future extraction meeting both niche conditions). This mirrors
  the measured-DEFER/STANDING resolutions of `NLP-SHALLOW-PARSE` (build-exhausted) and `MEMORY-BOUNDED-INGEST.5`
  ([[feedback_scoring_rigor]] — measured, not assumed). KM card `corpus-reuse-activate-only-no-current-consumer`.

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
| `2026-06-09` | `CORPUS-PATTERN-REUSE.3b.2` | `scripts/run_ci.sh` (fmt-check + clippy `-D warnings` + lib tests + rustdoc + mdBook) | green — lib **1491** passed / 0 failed |
| `2026-06-09` | `CORPUS-PATTERN-REUSE.3b.2` | `cargo run -p specforge -- kg-bench` | 151/151 pass |
| `2026-06-09` | `CORPUS-PATTERN-REUSE.3b.2` | live `learn-priors` over the 10 persisted IntentIR artifacts | 10 accepted; `extraction_profile_priors: 2` (support-5 + support-2 families, no vendor list); schema v6 persisted; `fired_extractors` honestly empty (pre-manifest evidence) |
| `2026-06-09` | `CORPUS-PATTERN-REUSE.3c` | wire-based eval on FRESH re-ingested evidence (`--provider skip`) | APB/AHB/AXI constraints+relations+temporal, I2C signals, SWD: all 1.000 (WIRE-BASED-100 filtered); NVMe register_field 0 = VLM-gated dataset, register surface intact 42/42 |
| `2026-06-09` | `CORPUS-PATTERN-REUSE.3c` | `corpus-cluster` + `learn-priors` before/after | clusters 28/14 → 30/13; non-empty family profiles 4→5; harvest 10→14 accepted, profiles 2→3; new AHB+AXI-Stream profile w/ full-support fired union |
| `2026-06-09` | `CORPUS-PATTERN-REUSE.3c` | `cargo run -p specforge -- kg-bench` after the sweep | 151/151 pass |
| `2026-06-15` | `CORPUS-PATTERN-REUSE.3b.3a` | faithful Form-A re-derivation over 78 persisted `evidence_ir` (read-only) | fires 5/78, 0 wire-based; benefit SMBus/I2S only; noise I2C(`VDD`/`VSS`/`DLEN`) / eMMC(`VDD`) / OpenCAPI-TL(`AFUC2` false-pos) |
| `2026-06-15` | `CORPUS-PATTERN-REUSE.3b.3a` | `corpus-cluster` @0.6 cluster mapping of the 5 firing docs | SMBus/I2S/I2C/OpenCAPI-TL singletons; eMMC↔GIC (register family 12) — no 2-wire-bus family exists |
| `2026-06-15` | `CORPUS-PATTERN-REUSE.3b.3a` | `corpus-cluster` threshold sweep 0.40–0.60 | no clean bus family at any cut (singletons ≥0.55; absence-token catch-alls ≤0.50) → structural no-go |
| `2026-06-15` | `CORPUS-PATTERN-REUSE.3b.3a2` | `Extractor` framework survey (`grep`/AST of `applies_to` overrides) | ZERO production `applies_to` overrides (only 2 test-only `Toy` `false` fixtures); `ExtractionContext` = per-document `statements` only → all surfaces self-gate / default-on |
| `2026-06-15` | `CORPUS-PATTERN-REUSE.3b.3a2` | logical analysis (read-only) | activate-only niche (cross-doc-predictable ∧ not-locally-testable ∧ clustered safe docs) is empty in this corpus → consume side measured-STANDING (built-deferred) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `CORPUS-PATTERN-REUSE.3a` | `CORPUS-PATTERN-REUSE.3a — corpus-cluster command` | New read-only command surfacing the `.2` clustering engine; book + README + KM in sync |
| `CORPUS-PATTERN-REUSE.3b.1` | `CORPUS-PATTERN-REUSE.3b.1 — per-cluster extraction profile` | Typed `ClusterExtractionProfile` + `derive_extraction_profiles` + command surfacing; activate-only decision recorded; book + README + KM in sync |
| `CORPUS-PATTERN-REUSE.3b.2` | `CORPUS-PATTERN-REUSE.3b.2 — persist extraction profiles into CorpusMemory` | 8th prior family (schema v6) + `learn-priors` harvest + signature-subset lookup; shared `0.6` threshold const; book + README + KM in sync |
| `CORPUS-PATTERN-REUSE.3c` | `CORPUS-PATTERN-REUSE.3c — manifest-population sweep` (this slice) | Data/no-code: 6 repo-backed docs re-ingested/rebuilt with manifests; wire-based 100% re-verified on fresh evidence; profiles 2→3 with real fired unions |
| `CORPUS-PATTERN-REUSE.3b.3a` | `CORPUS-PATTERN-REUSE.3b.3a — first opt-in extractor: measured no-go` | Read-only measurement: the serial-prose bus-line lever is not cluster-scopable (lexical-vs-structural); structural-discrimination criterion established; docs-only |
| `CORPUS-PATTERN-REUSE.3b.3a2` | `CORPUS-PATTERN-REUSE.3b.3a2 — activate-only consume: no current consumer (measured-STANDING)` | Read-only `Extractor`-framework survey: structural safety ⟹ self-gating/default-on; activate-only niche empty in this corpus → consume side built-deferred; docs-only |

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
- `2026-06-09`: Implemented + closed `.3b.2` (the LEARN-side persistence): `ExtractionProfilePriorRecord`
  8th prior family in `CorpusMemory` (schema 5→6, `#[serde(default)]` so existing stores stay loadable),
  `learn-priors` harvest via the accepted artifacts' EvidenceIR fingerprints (multi-member clusters only),
  strict signature-subset `extraction_profile_priors_for` lookup, and the shared
  `DEFAULT_FINGERPRINT_SIMILARITY_THRESHOLD` const unifying `corpus-cluster` and the harvest. Live: 10
  IntentIR artifacts → 2 persisted profiles, no vendor list, sparsity reported honestly. `run_ci.sh` green
  (lib 1491), kg-bench 151/151. Frontier advanced to `.3b.3` (gated on a first opt-in extractor) / `.4`.
- `2026-06-09`: Owned + executed `.3c` (the manifest-population sweep, data/no-code): NVMe+I2C evidence
  rebuilt; APB/AHB/AXI/AXI-Stream re-ingested (`DOCLING_DEVICE=cpu`) + evidence rebuilt + deterministic
  semantic/intent/validate chains built so they join the harvest. Measured: profiles 2→3 (new AHB+AXI-Stream
  family with full-support fired union), corpus-cluster non-empty profiles 4→5; wire-based 100% re-verified
  on the FRESH evidence (APB/AHB/AXI/SWD/I2C all 1.000 filtered; NVMe register_field 0 = VLM-gated dataset,
  not a regression); kg-bench 151/151. Remaining sparsity (~66 host-local docs) is honest scope, not debt.
- `2026-06-15`: **Resolved the `.3b.3` "design open" blocker (owner-directed "do all these").** Captured the
  first-opt-in-extractor selection criteria (helpful-on-one-cluster + noisy-default-on + bounded/grounded/no-
  denylist) + the leading candidate (the parked `PDF-VARIANT-DIGESTION.9.10` prose bus-line lever, cluster-
  scoped to its safe serial/SMBus family) in Decisions, and split `.3b.3` into `.3b.3a` (select the candidate,
  measurement-first, read-only) → `.3b.3b` (build it self-disabled + plumb the activate-only consume contract +
  measure held-out uplift, wire-based 100% a hard gate). Frontier advanced to `.3b.3a`. Docs-only ownership
  slice — no code/extraction-path change; ready for fresh-session `.3b.3a` measurement.
- `2026-06-15`: **`.3b.3a` MEASURED NO-GO (read-only) for the leading candidate; criterion sharpened.** Ran the
  candidate measurement over the persisted 78-doc corpus (no code change): a faithful re-derivation of the
  `.9.10` Form-A grammar ("the/The `<ALLCAPS-id>[#]` line") reproduces the probe (5/78 docs, 0 wire-based;
  genuine benefit only on SMBus + I2S), and the `corpus-cluster` mapping is decisive — **all 5 firing docs are
  structural singletons (SMBus/I2S/I2C/OpenCAPI-TL) or paired with a register-heavy non-bus doc (eMMC↔GIC); no
  derived 2-wire-bus family exists at any threshold (0.40–0.60 swept).** The lever's safe/noisy split is lexical
  (supply rails `VDD`/`VSS`), orthogonal to the structural fingerprint, and the only learnable bus-ish cluster
  contains the measured-regression doc I2C (gold 0.600) — so cluster-scoping can't replace the forbidden
  denylist. Closed `.3b.3a` (no-go), recorded the **structural-discrimination criterion** in Decisions, excluded
  the serial-prose levers as first-opt-in candidates, and spun `.3b.3a2` (continue the read-only search for a
  structurally-discriminated candidate); `.3b.3b` stays gated. KM card
  `corpus-reuse-serial-prose-lever-not-cluster-scopable`. Docs-only.
- `2026-06-15`: **`.3b.3a2` MEASURED NO-GO for ANY first opt-in extractor → consume side measured-STANDING.** The
  read-only `Extractor`-framework survey proved the `.3b.3a` no-go generalizes: ZERO production extractors override
  `applies_to` (only 2 test-only `Toy` `false` fixtures) and `ExtractionContext` carries only per-document
  `statements`, so structural applicability is decided LOCALLY → a structurally-safe extractor is default-on and
  never needs the cluster mechanism, while the only lexically-ambiguous lever (`.9.10`) has non-clustering docs.
  The activate-only niche (cross-document-predictable ∧ not-locally-testable ∧ clustered safe docs) is empty in
  this corpus, so `.3b.3b` stays correctly DEFERRED (building it now = YAGNI + gate-risk) with a precise re-open
  trigger. Closed `.3b.3a2`; the CORPUS-PATTERN-REUSE consume frontier (`.3b.3`) is now measured-STANDING — the
  learn side is complete and the only remaining buildable leaf is `.4` (offline miner, gated). KM card
  `corpus-reuse-activate-only-no-current-consumer`. Docs-only.
