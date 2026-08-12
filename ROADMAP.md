# ROADMAP
## Objective
- build `specforge` as a staged Rust toolchain for extracting implementation-relevant intent from specifications into canonical `IntentIR`
- keep `.isf` as the single adapter target downstream of `IntentIR` (`.fsm`/HDL are out of scope — FSMGen consumes `.isf` and owns them downstream)
- preserve deterministic provenance, typed intermediate data, and explicit residual decisions across all stages
- treat text, layout, figures, captions, tables, and charts as first-class evidence rather than markdown decoration
- prioritize semantic truthfulness and KG quality ahead of adapter breadth until the canonical four-layer pipeline is top-notch
- make the workflow resumable and understandable through live project documentation
- apply the same identity-independent engine to any digital-chip specification: production decisions may use
  universal digital semantics and document grammar, but never document/vendor/protocol identity, named symbols,
  or corpus-specific phrases; undecidable input must become an explicit residual rather than a special case

## Canonical pipeline
- `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`

## Cross-cutting implementation doctrine
- do not try to program a general reader of English; program a compiler for digital-design intent
- define the universal digital-intent world model first:
  - actors
  - signals
  - roles
  - timing predicates
  - state transitions
  - dependencies
  - handshake events
- prefer deterministic extraction when the source surface is crisp:
  - signal tables
  - widths
  - enum rows
  - clock/reset declarations
  - polarity cues
  - section/figure/table kinds
- treat prose, tables, figures, captions, and layout as evidence for typed domain facts, not as final outputs themselves
- use convergence, backannotation, and evidence aggregation to decide what survives into canonical IR
- use AI/VLM/NLP as bounded local hypothesis generators for hard spans or images, not as an unstructured end-to-end PDF-to-intent black box
- every AI-derived hypothesis must be forced back through:
  - schema checks
  - grounding checks
  - conflict/arbitration checks
  - convergence checks
  - validation
- preserve ambiguity explicitly:
  - keep alternatives when the evidence is not yet decisive
  - surface contradictions and residual decisions
  - do not fabricate semantic certainty
- keep generated replay plans inspectable without making display strings authoritative:
  - dry-run command text remains a review-facing rendering surface
  - empty dry-run selections still render an explicit queue header
  - selected queue indices, not the whole raw plan, drive dry-run recommendation rows
  - selected recommendation order is preserved in dry-run previews
  - related replay IDs render in plan order for review
  - execution parsing stays pinned to structured command fields and explicit allowlists
- keep rescan queue selection deterministic for automated batches:
  - positive limits bound work without reordering it
  - limits larger than pending work preserve every selected pending recommendation
  - zero-limit unlimited mode still honors document scope before selecting work
- treat the mdBook under `docs/book/` as a live user-facing product surface:
  - it is not a one-time scaffold or release-time afterthought
  - it is the public-facing documentation product for `specforge`, so it should openly explain what the tool does, how it works, and why it is designed that way
  - every meaningful user-facing aspect of the project should ultimately land in the book with its own section or chapter as coverage deepens
  - it should evolve alongside user-facing commands, runtime behavior, IR semantics, validation surfaces, and cross-document learning behavior
  - meaningful user-facing changes should update the book in the same task whenever the exposed contract or workflow has changed
- keep the book and the live root docs as separate planes:
  - the book is for the world-facing, transparent explanation of the product
  - the root markdown docs are for continuity, crash recovery, live validation projection, steering, and handoff between sessions
- treat local generated artifacts as rebuildable execution state with an explicit lifecycle:
  - rerunning the same document should replace stale doc-scoped normalization bundles instead of layering fresh files over old leftovers
  - the CLI should expose first-class cleanup scopes so heavyweight bundles, per-document stage trees, or the full generated root can be reclaimed without hand-deleting directories or touching tracked docs
- keep canonical document truth local and provenance-pure:
  - each `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` pipeline run should remain grounded only in the current document
  - any future cross-document learning layer must learn reusable extraction priors rather than smuggling facts from earlier PDFs into later canonical artifacts
- roadmap progress should favor meaning-based role inference and protocol semantics over literal spelling heuristics whenever the evidence can support that shift
- production core decisions must be invariant to filename/title/vendor-like identity and alpha-renaming of every
  input-defined symbol; protocol-specific conformance knowledge belongs only in tests, fixtures, and research

## Current strategic priorities

The near-term north star is a complete, faithful `IntentIR` knowledge graph that lowers to strict-valid
`.isf`: all real agents, relations, constraints, behaviors, transactions, symbols, and storage must be
represented without fabricated certainty. [`KG-ISF-COMPLETENESS`](docs/tasks/KG-ISF-COMPLETENESS.md)
and [`KG-ISF-TRANSACTIONS`](docs/tasks/KG-ISF-TRANSACTIONS.md) own that bar;
[`WIRE-BASED-100`](docs/tasks/WIRE-BASED-100.md) is the hard wire-protocol gate.

The current sequencing is upstream-first: faithful PDF evidence must populate SourceIR through canonical
`IntentIR` before adapter expressiveness can be called the blocker. ISF/FSMGen remains the eventual executable
boundary, but SpecForge does not anticipate a language gap without a source-grounded value.
[`SPEC-TO-INTENT-ALIGNMENT`](docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md) makes that ordering measurable (ADR 0033/0034).
Its reviewed 12/12 current replay and access/provenance correction ship. Generic timing unit/provenance and
physical-applicability carriers now have clean whole-population qualification: current IntentIR is 24/2/16
TP/FP/FN with complete 29/29 provenance, and physical-link is the first supported reviewed category. The
metric controller next selects the two remaining unrelated AMD IOMMU and GIC-400 register fabrications under
`.6e`; however, the owner-mandated whole-production genericity audit found release-blocking identity, named-schema,
signal-spelling, corpus-phrase, prompt, and module-boundary coupling. Structural remediation and behavioral
invariance under `.6d.ii.b`–`.6d.ii.f` therefore precede `.6e`. A finite forbidden-vocabulary list is diagnostic,
not proof. The signoff architecture is one-way production/conformance dependencies, opaque input symbols and
identity, registered universal grammar interfaces, source-grounded residuals, and alpha-renaming/identity/
paraphrase/held-out qualification. A reproduced 400-page failure also drove resource-sized bounded activation,
fail-closed page counting, typed signal status, and exact four-stage fidelity. The task tree owns detailed
metrics, evidence, and ordering.

Five active program groups support that north star:

- extraction quality and breadth: [`EXTRACTION-QUALITY-GAUGE`](docs/tasks/EXTRACTION-QUALITY-GAUGE.md),
  [`PDF-VARIANT-DIGESTION`](docs/tasks/PDF-VARIANT-DIGESTION.md),
  [`EXTRACTION-GAP-FIX`](docs/tasks/EXTRACTION-GAP-FIX.md),
  [`CORPUS-COVERAGE`](docs/tasks/CORPUS-COVERAGE.md),
  [`CORPUS-HARDENING`](docs/tasks/CORPUS-HARDENING.md),
  [`DOC-INTENT-TAXONOMY`](docs/tasks/DOC-INTENT-TAXONOMY.md), and
  [`SPEC-TO-INTENT-ALIGNMENT`](docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md); the completed
  [`SWD-SERIAL-EXTRACTION`](docs/tasks/SWD-SERIAL-EXTRACTION.md) program supplies the serial-protocol
  29/29 signoff and exact canonical projection baseline;
- size-immune ingest: [`MEMORY-BOUNDED-INGEST`](docs/tasks/MEMORY-BOUNDED-INGEST.md) keeps large-PDF
  work bounded in RAM and repository-volume storage;
- meaning-based language extraction: [`PURE-NLP-INTENT-EXTRACTION`](docs/tasks/PURE-NLP-INTENT-EXTRACTION.md),
  [`NLP-SHALLOW-PARSE`](docs/tasks/NLP-SHALLOW-PARSE.md), and
  [`CORPUS-PATTERN-REUSE`](docs/tasks/CORPUS-PATTERN-REUSE.md);
- learning, arbitration, and corpus synthesis: the open R15c–R15g lanes remain owned by
  [`R15C-R15G-LEARNING-PLANE-BACKFILL`](docs/tasks/R15C-R15G-LEARNING-PLANE-BACKFILL.md);
- repository durability and portability: measured artifact currency is owned by
  [`CORPUS-CHAIN-CURRENCY`](docs/tasks/CORPUS-CHAIN-CURRENCY.md), and live-document containment by
  [`LIVE-DOC-STOP-RISK`](docs/tasks/LIVE-DOC-STOP-RISK.md), under the rule that a bound a surface can
  actually reach must have a remedy compliant work can take.

The complete, always-current execution/status catalog is [`docs/TASK_TREE.md`](docs/TASK_TREE.md).
This roadmap owns only high-level direction and milestone status; it does not mirror leaf frontiers,
delivery chronology, or per-leaf measurements. Direction retired by a rollover is sealed byte-exact in
the [roadmap archive](docs/archive/roadmap/INDEX.md) rather than deleted.

## Workstream status

<!-- roadmap_workstreams:start -->
| Workstream | Status | Current contract | Owning task tree |
| --- | --- | --- | --- |
| R0 | Done | Repository, workflow, continuity, CI, and bootstrap foundations are established. | [coverage/alignment](docs/tasks/ROADMAP-TASKTREE-COVERAGE.md) |
| R1 | Done | `IntentIR` is the canonical endpoint and `specforge` is the CLI/crate identity. | [foundation audit](docs/tasks/R1-R5-FOUNDATION-BACKFILL.md) |
| R2 | Done | Typed `SourceIR` ingest and promoted source artifacts are delivered. | [foundation audit](docs/tasks/R1-R5-FOUNDATION-BACKFILL.md) |
| R3 | Done | Typed, provenance-carrying `EvidenceIR` is delivered. | [foundation audit](docs/tasks/R1-R5-FOUNDATION-BACKFILL.md) |
| R4 | Done | Backend-neutral `SemanticIR` is delivered. | [foundation audit](docs/tasks/R1-R5-FOUNDATION-BACKFILL.md) |
| R5 | Done | Versioned, serializable canonical `IntentIR` is delivered. | [foundation audit](docs/tasks/R1-R5-FOUNDATION-BACKFILL.md) |
| R6 | Done | `.isf` is the only adapter boundary; `.fsm` and HDL belong to FSMGen downstream. | [ISF-only consolidation](docs/tasks/ISF-ONLY-CONSOLIDATION.md) |
| R7 | Done | Validation/backannotation hardening is delivered; canonical mutation remains separately decision-gated. | [validation](docs/tasks/R7-VALIDATION.md) |
| R8 | Done | Tier-1 structured source capture is delivered; corpus robustness continues in active breadth tasks. | [R8–R13 audit](docs/tasks/R8-R13-EXTRACTION-BACKFILL.md) |
| R9 | Mostly done | Typed evidence is delivered; completeness hardening remains incremental and measurement-led. | [R8–R13 audit](docs/tasks/R8-R13-EXTRACTION-BACKFILL.md) |
| R10 | Done | Visual enrichment reaches typed evidence and downstream semantic records. | [R8–R13 audit](docs/tasks/R8-R13-EXTRACTION-BACKFILL.md) |
| R11 | Done | NLP Level-3 enrichment and backannotation are delivered. | [R8–R13 audit](docs/tasks/R8-R13-EXTRACTION-BACKFILL.md) |
| R12 | Done | Multi-spec validation and the original quick-fix baseline are delivered. | [R8–R13 audit](docs/tasks/R8-R13-EXTRACTION-BACKFILL.md) |
| R13 | Done | Tier-2 actor-signal relation extraction is delivered and graph-preserved. | [R8–R13 audit](docs/tasks/R8-R13-EXTRACTION-BACKFILL.md) |
| R14 | Done | Tier-3 LLM relation extraction ships as the grounded `signal-resolve` command. | [signal resolve](docs/tasks/R14-SIGNAL-RESOLVE.md) |
| R15 | Done | Actor-relative graph semantics are canonical; obsolete `.fsm` direction work is retired. | [graph migration](docs/tasks/R15-GRAPH-DIRECTION-MIGRATION.md) |
| R15b | In progress | Typed tick-relative rules and ISF lowering ship; corpus-wide behavioral completeness keeps advancing. | [temporal lowering](docs/tasks/ISF-TEMPORAL-LOWERING.md) |
| R15c | In progress | Convergence accounting ships; anchored prose/figure rescan recall remains open. | [learning-plane audit](docs/tasks/R15C-R15G-LEARNING-PLANE-BACKFILL.md) |
| R15d | In progress | Typed conflict records ship; full-corpus multimodal arbitration remains open. | [learning-plane audit](docs/tasks/R15C-R15G-LEARNING-PLANE-BACKFILL.md) |
| R15e | In progress | Evaluation is an evergreen accuracy, residual-honesty, and false-positive-control lane. | [learning-plane audit](docs/tasks/R15C-R15G-LEARNING-PLANE-BACKFILL.md) |
| R15f | In progress | Typed prior memory ships; protocol scoping and guarded prior families keep advancing. | [learning-plane audit](docs/tasks/R15C-R15G-LEARNING-PLANE-BACKFILL.md) |
| R15g | In progress | Corpus KB ships; additional validation-backed managed page families remain open. | [learning-plane audit](docs/tasks/R15C-R15G-LEARNING-PLANE-BACKFILL.md) |
| R16 | Program complete | The design-intent capture program closed; forward improvements use independent owned trees. | [R16 program](docs/tasks/R16-INTENT-CAPTURE.md) |
<!-- roadmap_workstreams:end -->

## Recommended implementation order

1. Preserve `IntentIR` as the canonical product boundary and `.isf` as the sole adapter target.
2. Define and measure what each document category must contribute from every source modality through
   canonical `IntentIR`; never use emitted-file count or strict validity as the upstream recall proxy.
3. Turn measured PDF-to-IR misses into general extraction capability, and make the canonical workflow account
   for every production extractor before expanding breadth for its own sake.
4. Advance KG/IntentIR completeness and transaction fidelity under held-out, provenance, residual-honesty,
   stage-conservation, and wire-protocol gates.
5. Keep large-document ingest bounded in RAM, disk, temporary storage, and host impact.
6. Deepen deterministic and model-assisted language understanding without hardcoded document vocabulary.
7. Extend R15c–R15g learning, arbitration, evaluation, priors, and corpus synthesis only behind typed
   provenance, local grounding, and measurable no-regression gates.
8. Extend ISF/FSMGen only after a source-grounded IntentIR value demonstrates a concrete executable-language
   gap; keep `.fsm`, scheduling, and HDL downstream in FSMGen.

## History and execution

- Current task status and exact next leaves: [`docs/TASK_TREE.md`](docs/TASK_TREE.md)
- Detailed current change history: [`CHANGES.md`](CHANGES.md)
- Current delivery snapshot: [`LIVE_ACHIEVEMENT_STATUS.md`](LIVE_ACHIEVEMENT_STATUS.md)
- Crash-safe resume pointer: [`MEMORY.md`](MEMORY.md)
- Exact pre-containment roadmap, every rollover-retired bounded root, and their identity manifest:
  [`docs/archive/roadmap/INDEX.md`](docs/archive/roadmap/INDEX.md)
- File-level evolution in Git: `git log -- ROADMAP.md`
