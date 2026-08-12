---
id: persisted-chain-currency-is-measured-not-assumed
title: Persisted chain currency is measured and gated, isolation comes from replay, and normalized bundles are retained
date: 2026-08-10
status: accepted
scope: corpus-coverage, extraction, artifacts, doctrine-enforcement, continuity, storage
evidence: docs/tasks/CORPUS-CHAIN-CURRENCY.md; docs/tasks/CORPUS-COVERAGE.md (CORPUS-COVERAGE.2.50a)
answers:
  - "what is ADR 0025"
  - "must a repair leaf rebuild every affected document or only the ones it measures"
  - "how does a repair prove its change is isolated if it also rebuilds drifted documents"
  - "should persisted chain currency be a gated doctrine or an advisory report"
  - "why does SpecForge retain normalized bundles instead of reclaiming them after a refresh"
  - "how much disk do normalized bundles cost across the corpus"
  - "why are 58 corpus documents not currency-measurable"
---

# ADR 0025: Persisted chain currency is measured and gated, isolation comes from replay, and normalized bundles are retained

## Context

`CORPUS-COVERAGE` states that completed chains are kept non-stale, and its `.1` stage-staleness lane is closed.
Nothing re-checked that property after a shared extractor changed. Completing `CORPUS-COVERAGE.2.50a` exposed the
gap: the three documents that repair touched had persisted chains the **pre-repair** binary could not reproduce, so
rebuilding them also absorbed every delta accumulated since each document's own refresh (USB 3.2: timings
102 → 74, actors 18 → 16, behaviors 2,705 → 866).

A read-only `evidence --dry-run` census then measured the standing drift: of 22 rebuildable documents, 19 were
current and three differed, all on timing surfaces — residue of the `.2.47a`/`.2.48a` timing-authority repairs,
which rebuilt only the documents their own measurement named. The remaining 58 documents have no normalized
bundle, so they cannot be replayed at all without re-ingest.

Three questions followed, and the owner delegated them to engineering judgment. Each was decided from measurement,
not preference.

## Decision

**1. Rebuild to currency; prove isolation by replay.** A leaf that changes a shared extractor must run the
exhaustive baseline-versus-change `--dry-run` replay over every rebuildable document, then rebuild every document
whose replay differs from its persisted chain — not only the documents its own change caused to move. Each
rebuilt document's delta is attributed in the leaf to either this change or a named earlier leaf.

The apparent tension between "close the drift" and "keep the evidence sharp" is false. Isolation is established by
the old-versus-new replay at fixed inputs, which is exact and independent of what the artifacts happen to hold;
withholding a rebuild does not sharpen that evidence, it only leaves the corpus wrong. The replay costs about four
seconds for the whole rebuildable population, so there is no economic reason to accept drift.

**2. Currency becomes a gated doctrine, fail-closed on a present corpus and explicitly skipped otherwise.** A
registered `CHAIN-CURRENCY` check replays every rebuildable document and fails when a persisted chain differs from
what the current code produces. It must skip — loudly, never silently — when `generated/` is absent, because a
fresh clone and a hosted CI runner have no corpus and the doctrine does not govern them. Because the full replay
costs roughly 36 seconds against a debug binary plus a cargo freshness build, it is a CI-tier doctrine under
`DOCTRINE_ENFORCEMENT.md` §4.7 rather than a pre-commit gate. It reports the unmeasurable population as a count,
so a partial measurement can never read as full coverage.

**3. Normalized bundles are retained; reclamation becomes task-owned.** Retention is what makes a document
replayable, and it is nearly free: 22 bundles occupy 1.3 GB against 3.4 TB of free space on the repository volume,
so the full 80-document corpus extrapolates to roughly 4.7 GB, about 0.14% of what is available. A refresh
therefore keeps its bundle, and `clean --scope source-normalized` becomes a deliberate, task-owned reclamation
rather than routine hygiene. The 58 missing bundles are backfilled at each document's own refresh; no bulk
re-ingest is scheduled, because re-ingesting the corpus is a roughly hour-long, RAM-sensitive, corpus-scale
mutation that needs its own owned leaf.

## Consequences

- The measured drift is real but benign to close: all 27 records the three drifted documents would lose are
  prose cells misread as timing parameters — a JTAG/SWD ACK-response table with `parameter_name: "Read"` and
  `unit: "Capture read data."`, OpenCAPI crosstalk rows whose `max_value` repeats the description, and OpenCAPI
  configuration-limit rows. Nothing real is lost and nothing is added.
- One of the three is the SWD wire gold document. Every gold-scored surface — serial frame fields, SWD
  operations, protocol states, interface edge timings, signal constraints, actor-signal relations — is
  byte-identical across the rebuild, so closing its drift carries no gold risk. This matters because
  `eval-extraction` scores persisted artifacts: "the gold is 1.000" and "the current code scores 1.000" are two
  claims, and only the gated replay makes the second one true by construction.
- Retention raises steady-state disk use by a few gigabytes and removes the reclamation step from the refresh
  routine.
- Until the 58 non-rebuildable documents are backfilled, corpus currency is a *partial* guarantee, stated as such
  by the check's own output rather than implied.
- A proof-schema migration can deliberately quarantine a persisted upstream artifact before the next stage is
  migrated. The replay that directly consumes that input is **unmeasurable**, not stale: the current binary is
  correctly refusing an inspection-only input. A still-unmigrated later stage may remain locally reproducible
  from its persisted immediate input; that does not establish end-to-end authority. The gate recognizes only the
  closed legacy/proofless compatibility diagnostic. A malformed, missing, or stale current proof remains a hard
  failure. This preserves the distinction between historical bytes that still exist, local reproducibility, and
  canonical authority the current binary can actually verify.

## Links

- `docs/tasks/CORPUS-CHAIN-CURRENCY.md` — the owning tree and its leaves.
- `docs/tasks/CORPUS-COVERAGE.md` — the refresh program whose slice transaction this policy amends.
- `DOCTRINE_ENFORCEMENT.md` §4.7 — the CI-tier allowance for a check too heavy for pre-commit.
