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
- ID: `CORPUS-PATTERN-REUSE.2` · Status: `pending` (gated) · Goal: derived vendor/layout **fingerprint** +
  unsupervised clustering over the tracked corpus (structural signature + `ExtractionManifest`); measure that
  emergent clusters track real vendor/layout families WITHOUT a vendor list. Gated on: the extractor-framework
  migration maturing enough that the manifest is populated for the surfaces that matter.
- ID: `CORPUS-PATTERN-REUSE.3` · Status: `pending` (gated) · Goal: typed advisory `ExtractionProfile` in
  `CorpusMemory` keyed by cluster fingerprint; consumed via `Extractor::applies_to`; bounded + contested-gated.
- ID: `CORPUS-PATTERN-REUSE.4` · Status: `pending` (gated) · Goal: offline LLM/VLM cluster pattern miner with
  held-out precision validation before promotion; recall-gauge-measured uplift on held-out docs.

## Current frontier

`.1` design owned. The build (`.2`–`.4`) is **sequenced behind `EXTRACTOR-ARCHITECTURE`** — the run manifest
is the behavioral fingerprint this plane clusters on, so the foundation comes first. Owner may steer the
priority between (continue extractor consolidation → then this) vs. (prototype clustering now on the
structural fingerprint alone). Recommended: foundation first.
