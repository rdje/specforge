# Decision records (layer C of `MEMORY_ARCHITECTURE.md`)

Durable, cross-cutting facts/decisions that must survive across sessions, AI models,
and harnesses — one record per file, dated, `Context → Decision → Consequences`.
Append + dedupe + supersede (never silently rewrite). Link records from the related
task-trees under `docs/tasks/`.

This is **layer C**: facts that outlive any single unit of work and don't belong in the
resume pointer (`MEMORY.md`, layer A) or a single task-tree (layer B). History of *what
changed* lives in git (layer D), not here.

| # | Title | Date | Status | Tags |
|---|---|---|---|---|
| [0001](0001-docling-device-cpu.md) | Docling ingest runs on CPU on this stack (torch MPS lacks float64) | 2026-06-01 | accepted | ingest, environment |
| [0002](0002-llm-vlm-provider-default.md) | Production LLM/VLM provider is local Ollama + qwen2.5vl:7b | 2026-06-01 | accepted | llm, provider |
| [0003](0003-task-tree-and-commit-doctrine.md) | Task-tree ownership before any code change; strict commit workflow | 2026-06-01 | accepted | process, doctrine |
| [0004](0004-severity-never-gated-by-verbosity.md) | Severity ≥ Warning is never gated by a verbosity/trace level | 2026-06-01 | accepted | observability, invariant |
| [0005](0005-temporal-logic-ltl-mtl-not-ctl-tla.md) | Temporal behavior captured in LTL/MTL, not CTL or TLA+ (mine, don't model-check) | 2026-06-04 | accepted | temporal, ltl, formalism |
| [0006](0006-doctrine-enforcement-architecture.md) | Doctrine-enforcement architecture: every doctrine becomes a mechanically-gated check (registry/driver, E1→E4) | 2026-06-22 | accepted | process, doctrine, enforcement |
| [0006-genericity](0006-no-hardcoded-chip-spec-vocabulary.md) | No hardcoded chip-spec vocabulary (PDF-independence invariant) | 2026-06-05 | accepted | extraction, genericity, doctrine |
| [0007](0007-live-document-containment-and-data-locality.md) | Live-document containment and repository-volume data locality | 2026-08-08 | accepted | documentation, containment, locality, portability |
| [0008](0008-lossless-rolling-ledger-protocol.md) | Lossless rolling-ledger source-capsule and live-window protocol | 2026-08-08 | accepted | documentation, continuity, archive, retrieval |
| [0009](0009-bounded-knowledge-map-projection-set.md) | Knowledge Map uses a bounded landing page and generated question shards | 2026-08-08 | accepted | documentation, knowledge-map, projection, retrieval |
| [0010](0010-bounded-current-roadmap-and-exact-history.md) | Roadmap keeps current direction; an exact capsule keeps accumulated history | 2026-08-08 | accepted | documentation, roadmap, archive, current-truth |
| [0011](0011-bounded-fsmgen-feedback-channel.md) | FSMGen feedback keeps a bounded current channel over exact correspondence history | 2026-08-08 | accepted | documentation, feedback, archive, current-truth |
| [0012](0012-reviewed-validation-snapshot-boundary.md) | Tracked validation is the last reviewed projection, not ambient local artifact state | 2026-08-08 | accepted | validation, projection, review, current-truth |
| [0013](0013-git-indexed-source-pdf-registry-authority.md) | Git-indexed corpus PDFs define source-registry membership | 2026-08-08 | accepted | corpus, PDF, registry, currentness |
| [0014](0014-corpus-kb-managed-currentness-authority.md) | Tracked inputs and reviewed validation define corpus-KB currentness | 2026-08-08 | accepted | corpus-kb, validation, kg-bench, currentness |
| [0015](0015-mdbook-owns-public-product-contracts.md) | mdBook owns public product contracts; stable root documents are compatibility pointers | 2026-08-08 | accepted | documentation, mdbook, architecture, current-truth, containment |
| [0016](0016-swd-protocol-projection-and-honest-isf-boundary.md) | SWD protocol projection is lossless; ISF lowering requires complete behavioral bindings | 2026-08-09 | accepted | swd, projection, intentir, isf, residual-honesty |
| [0017](0017-partitioned-rolling-ledger-archive-route.md) | Rolling-ledger archives use a bounded landing and per-ledger authority partitions | 2026-08-09 | accepted | documentation, continuity, archive, retrieval |
| [0018](0018-terminal-task-tree-current-history-boundary.md) | A completed oversized task tree becomes a bounded closed root over an exact source capsule | 2026-08-09 | accepted | documentation, continuity, task-tree, archive, retrieval |
| [0019](0019-bounded-active-task-root-and-semantic-evidence-parts.md) | An oversized active task uses a bounded current root over semantic evidence parts and exact provenance | 2026-08-09 | accepted | documentation, continuity, task-tree, active-work, archive, retrieval |
| [0020](0020-bounded-fact-card-browse-projection.md) | Fact-card browsing uses a bounded ID landing over deterministic title parts | 2026-08-09 | accepted | documentation, knowledge-map, generated-projection, retrieval, containment |
| [0021](0021-cross-directory-fact-catalog-links-preserve-destinations.md) | Cross-directory fact rows preserve semantics and destinations, not relative link bytes | 2026-08-09 | accepted | documentation, knowledge-map, generated-projection, links, retrieval |
| [0022](0022-fact-catalog-parts-pack-below-warning.md) | Fact-card title parts pack below the existing warning milestone | 2026-08-09 | accepted | documentation, knowledge-map, generated-projection, containment, pressure |
| [0023](0023-fact-catalog-landing-fits-full-capacity.md) | The fact-card landing scaffold fits the full declared capacity | 2026-08-09 | accepted | documentation, knowledge-map, generated-projection, containment, pressure |
| [0024](0024-corpus-task-bounded-active-root-and-evidence-parts.md) | Corpus coverage keeps a bounded active root over seven evidence parts and exact provenance | 2026-08-10 | accepted | documentation, continuity, task-tree, corpus-coverage, active-work, archive, retrieval |
| [0025](0025-persisted-chain-currency-is-measured-not-assumed.md) | Persisted chain currency is measured and gated, isolation comes from replay, and normalized bundles are retained | 2026-08-10 | accepted | corpus-coverage, extraction, artifacts, doctrine-enforcement, continuity, storage |
| [0026](0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md) | Fact-plane capacity is re-derived from its binding structural join, and the landing stops scaling with cards | 2026-08-10 | decision 1 superseded by 0027 | documentation, knowledge-map, generated-projection, containment, pressure, continuity |
| [0027](0027-fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound.md) | Fact-plane capacity requires reshaping the projection, because 198 cards is already the exact maximum its shape permits | 2026-08-10 | accepted | documentation, knowledge-map, generated-projection, containment, pressure, continuity |
| [0028](0028-a-file-locator-means-one-file-and-aggregate-ceilings-must-warn.md) | A file locator means one file, so multi-file surfaces stopped being blind to their own aggregate ceilings | 2026-08-10 | accepted | documentation, containment, live-document-size, pressure, continuity |
| [0029](0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md) | Fact-plane capacity is one derived profile, and an aggregate bound with no remedy is a trap | 2026-08-11 | accepted | documentation, knowledge-map, generated-projection, containment, pressure, continuity |
| [0030](0030-a-bounded-snapshot-needs-a-declared-repeatable-rollover.md) | A bounded snapshot needs a declared repeatable rollover, not a one-time migration | 2026-08-11 | accepted | documentation, containment, live-document-size, roadmap, archive, retrieval |
| [0031](0031-a-bounded-snapshot-bounds-its-sections-not-just-its-file.md) | A bounded snapshot bounds its sections, not just its file | 2026-08-11 | accepted | documentation, containment, live-document-size, roadmap, pressure |
| [0032](0032-no-collection-may-declare-an-aggregate-below-its-own-legal-maximum.md) | No collection may declare an aggregate below its own legal maximum, and pressure must name the wall | 2026-08-11 | accepted | documentation, containment, live-document-size, pressure, continuity |

## How to add a record
1. Copy the shape of an existing record (`Date`, `Status`, `## Context / Decision /
   Consequences / Links`).
2. Use the next sequential number; add a row to the table above.
3. Link it from the task-tree(s) it relates to.
4. To change a fact, add a new record (or mark the old one `superseded by 00NN`) — do
   not rewrite history.
