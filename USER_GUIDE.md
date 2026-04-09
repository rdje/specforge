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
2. `docs/book/src/getting-started.md`
3. `docs/book/src/runtime-and-doctor.md`
4. `docs/book/src/commands/overview.md`
5. `docs/book/src/pipeline/overview.md`
6. `docs/book/src/quality/validation.md`
7. `docs/book/src/quality/kg-bench.md`
8. `docs/book/src/quality/corpus-memory.md`
9. `docs/book/src/reference/troubleshooting.md`

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
  - `Top datapath link producer.output_data -> consumer.input_data.`
  - `Module producer_core signal output_data is output width 8.`
  - `Module producer_core block produce: output_data = 8'3.`
- when explicit `active high` / `active low` wording is omitted, the current canonical reset slice infers active-low from `_n` / `_b` reset naming and otherwise falls back to active-high with lower automation confidence

## Planned user workflow
1. provide a source specification
2. build `SourceIR`
3. build `EvidenceIR` from text, figures, captions, charts, and other grounded evidence
4. build `SemanticIR`
5. build canonical `IntentIR`
6. lower `IntentIR` through an adapter such as `.fsm` or RTL
7. validate the result and back-annotate findings

## Planned command shape
- `specforge converge <source> --target <fsm|systemverilog|verilog|vhdl>` defaults to full Ollama-backed VLM image enrichment plus NLP Level 3 backannotation; pass `--vlm-provider skip` and/or `--nlp-provider skip` only when you intentionally want a narrower run
- `specforge ingest <source>`
- `specforge inspect <artifact-or-path>`
- `specforge evidence <source-ir>`
- `specforge semantic <evidence-ir>`
- `specforge intent <semantic-ir>`
- `specforge adapt <intent-ir> --target <fsm|systemverilog|verilog|vhdl>` (today, only `fsm` is implemented)
- `specforge validate <artifact>`
- `specforge project-validation <artifact>...`
- `specforge learn-priors <intent-ir>...`

## Expected user-visible principles
- the tool should be staged and inspectable
- intermediate IR artifacts should be preserved, not hidden
- evidence and provenance should be visible
- images, figures, captions, and charts should stay available as first-class evidence when they carry semantic value
- unresolved ambiguity should be emitted as residual decision packets instead of being hidden in ad hoc notes
- `IntentIR` should remain backend-independent
- `.fsm` is only one adapter target among several

## Current limitation
- `SourceIR`, the first real `EvidenceIR` pass, the first real `SemanticIR` pass, and the first real `IntentIR` pass are implemented
- the current `EvidenceIR` extraction logic is still heuristic and does not yet perform deeper OCR, chart extraction, or semantic lifting from visual regions
- the current `SemanticIR` extraction logic is still heuristic and conservative, so later `IntentIR` work will need refinement rather than semantic invention
- the current `IntentIR` canonicalization logic is still heuristic and conservative, so adapter work should refine backend lowering rather than treat the current pass as a complete semantic endpoint
- the first `.fsm` adapter slices are implemented and can now emit standalone renderable `?dt:name` text for explicit canonical combinational and sequential DT cases, structured renderable `?fsm:name` text for explicit canonical state-graph cases, canonical symbol-definition/reset-role lowering, selector/test-node branches, compound-update shorthand for honest canonical cases, and first-slice renderable `?top:name` text for explicit canonical composition cases, but unsupported selector/predicate shapes still remain deferred and compatibility-level `?mod:name` / `?module:name` spellings stay outside the current canonical root-kind model
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

## Where to look next
- `README.md`
- `INTENTIR_SPEC.md`
- `ROADMAP.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `RUST_CODEBASE_ANALYSIS.md`
