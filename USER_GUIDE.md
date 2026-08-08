# USER_GUIDE

This file is now a compatibility pointer.

The canonical user-facing documentation for `specforge` has moved to the mdBook under `docs/book/`.

## Canonical docs path

- source entry point: `docs/book/src/introduction.md`
- table of contents: `docs/book/src/SUMMARY.md`
- local build:
  - `mdbook build docs/book`
  - or `bash scripts/run_docs_ci.sh`

## Read the book in this order

1. `docs/book/src/introduction.md`
2. `docs/book/src/architecture-rationale.md`
3. `docs/book/src/getting-started.md`
4. `docs/book/src/runtime-and-doctor.md`
5. `docs/book/src/commands/overview.md`
6. `docs/book/src/pipeline/overview.md`
7. `docs/book/src/quality/validation.md`
8. `docs/book/src/quality/kg-bench.md`
9. `docs/book/src/quality/corpus-memory.md`
10. `docs/book/src/quality/corpus-kb.md`
11. `docs/book/src/reference/troubleshooting.md`

## What stays in the repo root

The root markdown docs still matter, but they are no longer the primary user-doc surface.

They now serve:

- continuity
- roadmap
- validation snapshots
- architecture notes
- session handoff

The most important ones are:

- `README.md`
- `ROADMAP.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `VALIDATION_SNAPSHOT.md`
- `DEVELOPMENT_NOTES.md`
- `CHANGES.md`
- `MEMORY.md`
- `corpus_kb/`

## Planned user workflow
1. provide a source specification
2. build `SourceIR`
3. build `EvidenceIR` from text, figures, captions, charts, and other grounded evidence
4. build `SemanticIR`
5. build canonical `IntentIR`
6. lower `IntentIR` through the `.isf` adapter (FSMGen consumes `.isf` and owns scheduling/`.fsm`/HDL downstream)
7. validate the result and back-annotate findings

## Planned command shape
- `specforge converge <source> --target isf` defaults to full Ollama-backed VLM image enrichment plus NLP Level 3 backannotation; pass `--vlm-provider skip` and/or `--nlp-provider skip` only when you intentionally want a narrower run (`.isf` is the only adapter target; `.fsm`/HDL are out of scope — FSMGen owns them downstream)
- `specforge ingest <source>`
- `specforge inspect <artifact-or-path>`
- `specforge evidence <source-ir>`
- `specforge semantic <evidence-ir>`
- `specforge intent <semantic-ir>`
- `specforge adapt <intent-ir> --target isf` (`.isf` is the only adapter target; `.fsm`/HDL are out of scope — FSMGen consumes `.isf` and owns them downstream)
- `specforge validate <artifact>`
- `specforge project-validation <artifact>...`
- `specforge learn-priors <intent-ir>...`
- `specforge corpus-kb [validation-report]... [--validation-snapshot <reviewed-snapshot>] [--kg-fixtures-root <fixture-root>]`

## Expected user-visible principles
- the tool should be staged and inspectable
- intermediate IR artifacts should be preserved, not hidden
- evidence and provenance should be visible
- images, figures, captions, and charts should stay available as first-class evidence when they carry semantic value
- unresolved ambiguity should be emitted as residual decision packets instead of being hidden in ad hoc notes
- `IntentIR` should remain backend-independent
- `.isf` is the single adapter target; FSMGen owns `.fsm`/HDL downstream

## Current limitation
- `SourceIR`, the first real `EvidenceIR` pass, the first real `SemanticIR` pass, and the first real `IntentIR` pass are implemented
- the current `EvidenceIR` extraction logic is still heuristic and does not yet perform deeper OCR, chart extraction, or semantic lifting from visual regions
- the current `SemanticIR` extraction logic is still heuristic and conservative, so later `IntentIR` work will need refinement rather than semantic invention
- the current `IntentIR` canonicalization logic is still heuristic and conservative, so adapter work should refine backend lowering rather than treat the current pass as a complete semantic endpoint
- the `.isf` adapter is implemented: `IntentIR` lowers through the typed `IsfIr` model (`IntentIR → IsfIr::from_intent_ir() → render() → .isf`), emitting valid `.isf` S-expression source when the canonical signal/behavior surface is renderable and blocking with explicit reasons otherwise; emitted `.isf` is validated against FSMGen `--strict --check --json`. The HDL surface and the prior `.fsm` adapter are being removed under `ISF-ONLY-CONSOLIDATION`
- `specforge validate <artifact>` now backannotates the latest validation report into the artifact and writes a stage-local `validation_report.json` sidecar
- `specforge project-validation <artifact>...` now turns those persisted reports into tracked markdown continuity docs, and broader adapter validation remains intentionally deferred until semantic truthfulness is stronger
- `specforge learn-priors <intent-ir>...` now turns validated canonical `IntentIR` artifacts into a local `CorpusMemory` prior store under `generated/prior_memory/corpus_memory.json`, and the current slice learns reusable actor-taxonomy, semantic-role, semantic modality-reliability, temporal-language, and table-shape priors without letting that memory author canonical facts directly
- `specforge evidence <source-ir>` and `specforge converge <source>` now consult that same local prior store by default through `--prior-memory generated/prior_memory/corpus_memory.json`; missing prior files are treated as a no-op, and the first bounded consumers now use:
  - actor-taxonomy priors to interpret explicit local actor terms already present in section headings and `Source` / `Destination` table columns, including structural KG recovery from width-only section-guided signal tables
  - semantic phrase priors to interpret locally grounded non-hardcoded role phrases that normalize to learned semantic evidence
- `EvidenceIR` now also uses table-shape priors in a bounded way: if a current structured table is still `unknown`, a matching learned header signature can advisory-recover the table kind locally, but explicit local `SourceIR` table kinds are never overridden
- `SemanticIR` now also uses semantic modality-reliability priors in a bounded way: if the current PDF already contains multiple competing locally grounded semantic candidates for one signal, a matching learned source-kind reliability prior can advisory-adjust the local arbitration margin without erasing the underlying conflict record
- the tracked benchmark harness now proves that bounded behavior on more than one table kind:
  - an `unknown` `Name | Direction | Width` table only becomes a signal-description table when the matching prior is staged
  - an `unknown` `Parameter | Min | Max | Unit` table only becomes a timing-parameter table when the matching prior is staged
- `SemanticIR` now also consults the persisted prior-memory path indirectly through the upstream `EvidenceIR`, and the first bounded temporal consumer uses temporal phrase priors only as a fallback for local timing text when the built-in cycle-window parser cannot recover the timing window directly
- `specforge corpus-kb --validation-snapshot VALIDATION_SNAPSHOT.md` refreshes the tracked validation page from the last-reviewed boundary; positional validation reports remain a mutually exclusive local exploration mode
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshes the aggregate, family, and prior-candidate projections from the complete tracked fixture set, preserving human-authored synthesis outside managed blocks and keeping the corpus KB as guidance rather than canonical truth promotion

## Where to look next
- `README.md`
- `INTENTIR_SPEC.md`
- `ROADMAP.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `RUST_CODEBASE_ANALYSIS.md`
