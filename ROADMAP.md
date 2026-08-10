# ROADMAP
## Objective
- build `specforge` as a staged Rust toolchain for extracting implementation-relevant intent from specifications into canonical `IntentIR`
- keep `.isf` as the single adapter target downstream of `IntentIR` (`.fsm`/HDL are out of scope — FSMGen consumes `.isf` and owns them downstream)
- preserve deterministic provenance, typed intermediate data, and explicit residual decisions across all stages
- treat text, layout, figures, captions, tables, and charts as first-class evidence rather than markdown decoration
- prioritize semantic truthfulness and KG quality ahead of adapter breadth until the canonical four-layer pipeline is top-notch
- make the workflow resumable and understandable through live project documentation

## Canonical pipeline
- `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`

## Cross-cutting implementation doctrine
- do not try to program a general reader of English; program a compiler for protocol meaning
- define the protocol-world model first:
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

## Current strategic priorities

The near-term north star is a complete, faithful `IntentIR` knowledge graph that lowers to strict-valid
`.isf`: all real agents, relations, constraints, behaviors, transactions, symbols, and storage must be
represented without fabricated certainty. [`KG-ISF-COMPLETENESS`](docs/tasks/KG-ISF-COMPLETENESS.md)
and [`KG-ISF-TRANSACTIONS`](docs/tasks/KG-ISF-TRANSACTIONS.md) own that bar;
[`WIRE-BASED-100`](docs/tasks/WIRE-BASED-100.md) is the hard wire-protocol gate.

Five active program groups support that north star:

- extraction quality and breadth: [`EXTRACTION-QUALITY-GAUGE`](docs/tasks/EXTRACTION-QUALITY-GAUGE.md),
  [`PDF-VARIANT-DIGESTION`](docs/tasks/PDF-VARIANT-DIGESTION.md),
  [`EXTRACTION-GAP-FIX`](docs/tasks/EXTRACTION-GAP-FIX.md),
  [`CORPUS-COVERAGE`](docs/tasks/CORPUS-COVERAGE.md),
  [`CORPUS-HARDENING`](docs/tasks/CORPUS-HARDENING.md),
  and [`DOC-INTENT-TAXONOMY`](docs/tasks/DOC-INTENT-TAXONOMY.md); the completed
  [`SWD-SERIAL-EXTRACTION`](docs/tasks/SWD-SERIAL-EXTRACTION.md) program supplies the serial-protocol
  29/29 signoff and exact canonical projection baseline;
- size-immune ingest: [`MEMORY-BOUNDED-INGEST`](docs/tasks/MEMORY-BOUNDED-INGEST.md) keeps large-PDF
  work bounded in RAM and repository-volume storage;
- meaning-based language extraction: [`PURE-NLP-INTENT-EXTRACTION`](docs/tasks/PURE-NLP-INTENT-EXTRACTION.md),
  [`NLP-SHALLOW-PARSE`](docs/tasks/NLP-SHALLOW-PARSE.md), and
  [`CORPUS-PATTERN-REUSE`](docs/tasks/CORPUS-PATTERN-REUSE.md);
- learning, arbitration, and corpus synthesis: the open R15c–R15g lanes remain owned by
  [`R15C-R15G-LEARNING-PLANE-BACKFILL`](docs/tasks/R15C-R15G-LEARNING-PLANE-BACKFILL.md);
- repository durability and portability: the containment program is reopened only for the measured
  `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.8` derived-state contract delta; its `.8a` probe and `.8b` bounded
  four-class registry/checker plus exact Rust/gitlink authority adapters are complete. The `.8c` audit refused
  closure because three secondary copies remained hardcoded in adapter source rather than enumerated by the data
  registry. `.8d` now declares all three with neutral surface/control validation and declaration-only adapter
  reads. The `.8e` repository-wide audit confirms that repair but finds the live FSMGen hash still stored in the
  older feedback-protocol self-test renderer. `.8f` now derives the complete synthetic required-literal set from
  contract data and proves whole-array replacement; `.8g` independently confirms 17/17 markers, both authority
  groups, and no executable live FSMGen value, closing `.8`. The live-document
  contracts also forced a second status rollover during that audit; its shared archive index is now 81 lines /
  5,311 bytes and only 218 bytes before mandatory byte rollover. `.9a` has measured every reader and accepted
  ADR 0017's fixed four-route landing plus bounded per-ledger index/manifest authorities. It also found that the
  existing checker does not prove complete acyclic predecessor/successor chronology. `.9b` has now atomically
  landed the four-part authority, exact chain/index/landing enforcement, and retired-manifest residue gate,
  closing `.9` before another segment. `.10a` now measures the adoption task evidence at 2,451 lines /
  232,649 bytes, accepts ADR 0018's committed-source → exact-terminal → bounded-closed-root topology, and splits
  implementation into `.10b.i` provenance enforcement plus `.10b.ii` atomic migration. `.10b.i` has now landed
  the neutral two-state checker, 15 fail-closed cases, and an exact 2,538-line / 242,172-byte source lock while
  proving every archive path absent. `.10b.ii` has now copied that committed source byte-for-byte to an immutable
  terminal, replaced the stable task path with a 119-line closed summary behind a bounded index/manifest, and
  closed `.10` plus the entire adoption program. Its census blocks another append to active
  `PDF-VARIANT-DIGESTION`—222,616 bytes, 207 before warning. The new
  [`ACTIVE-TASK-EVIDENCE-CONTAINMENT`](docs/tasks/ACTIVE-TASK-EVIDENCE-CONTAINMENT.md) tree now owns that boundary;
  `.0` pins the unchanged source at 2,393 lines / 222,616 bytes / SHA-256 `9284dce4…a19d4` and exposes a legacy
  frontier/status contradiction. `.1.1` now closes a 13-region semantic census plus all 32 direct-path / 89
  identifier-reference inputs, generic readers, and the manual writer. It proves the nominal frontier is a
  124,201-byte activity store and cannot nominate an eligible leaf from contradictory status text. `.1.2` accepts
  ADR 0019's bounded active root, seven semantic legacy parts, exact capsule, 15-region coverage, conservative
  no-eligible-frontier reconciliation, atomic root+part writer, and fixed route/aggregate limits. `.2.1` lands
  the neutral source-locked topology contract. `.2.2` now pins clean boundary `f04db37a`, all 15 region identities,
  and 52 commit-history routes while every destination remains absent. Its 32-case checker distinguishes canonical
  route IDs from constrained full-or-tree-relative source literals, closing the mixed historical-spelling defect;
  full CI passes. `.3.1` now completes the root-last migration: the stable route is a 106-line active current root
  over a 79-line index, seven bounded semantic parts, 52 canonical routes, and the exact 2,393-line source
  capsule. A non-ASCII fixture closes byte/character boundary risk. `.3.2` reproduces the result from a clean
  clone, adds a positive root+part continuation case (35 focused cases total), and closes the tree. That audit
  also finds root-ledger pressure. The new
  [`ROOT-ROLLING-LEDGER-PRESSURE`](docs/tasks/ROOT-ROLLING-LEDGER-PRESSURE.md) tree now owns it. `.0` pins clean
  boundary `4d24b13c`, finds changes at 91.4% of line health plus warning pressure in development/status/Rust,
  and root-causes a focused-checker defect: it applies milestones to quarantine ceilings rather than generic
  health targets. `.1` now binds those authorities, adds a generic committed-boundary/dry-run/root-last/rollback
  transaction with 35 focused cases, and materializes exact 29/24/12/8-record segments. All four roots are below
  warning, every chain/index is complete, old archive members are byte-identical, and no threshold or ceiling
  widens. `.2` now reproduces commit `10d182ff` from a clean same-SSD clone, closes the ordinary uninitialized-
  gitlink prerequisite, passes six doctrines, and proves a valid next record in every grammar remains below
  warning. Exact restoration and zero residue close the tree; product work resumes at
  `CORPUS-COVERAGE.2.33d.iii`. That conditional leaf now proves no extra convergence/adapter filter is warranted:
  the `.d.ii` fixed-point regression closes every known re-entry path, and duplicating authority would risk real
  relation-grounded directions. `.2.33d.iv` then used the real USB cascade as a falsification gate: `.iv.a`
  makes generated adapter directories converge after actor/renderability changes, while the audit exposes a
  distinct authority-empty SemanticIR fallback (918 low-confidence interfaces → 556 one-bit outputs with an
  empty actor graph). `.iv.b` now closes that generic boundary: 21 affected retained documents (5,527
  interfaces / 18,397 all-low records) dry-run to zero under a typed-or-signal-led-deontic authority rule;
  declaration-free `VALID`/`READY` and declared wire surfaces remain intact, and USB blocks honestly with no
  emitted ISF. `.iv.c` now closes the USB tree after reproducing stable downstream hashes (including after the
  documented SourceIR validation backannotation) and zero
  relation/interface/port/adapter-signal/rule surfaces, passing focused/WIRE/KG/full-CI/book/doctrine/locality
  gates, and deleting only the authenticated rollback with zero residue. `CORPUS-COVERAGE.2.34a` then selected
  USB4 Inter-Domain Service and failed closed on a stale boot-volume source route. After the director supplied the
  SSD checkout, `.2.34b` completed the guarded ingest/cascade. Its `.i` recovery leaf rebuilt a coherent canonical
  SWD chain after a temporary oracle command resolved to the live artifact root: current dense-prose authority
  removes one false signal/four relations while all 29 typed SWD protocol facts remain exact. The closing USB4
  locality gate then exposed 51 page sidecars retaining absolute staging image paths. `.ii.a` now rewrites them
  against the final repository-relative destination before promotion, with traversal/symlink containment and
  last-good rollback. `.ii.b` closes the real rerun: two guarded 51-page ingests reproduce all final hashes at
  18% peak system memory used; all sidecars are final-rooted and repository-relative; the stale one-signal/
  two-rule/eight-enum model blocks honestly at zero interfaces/signals/rules and leaves only `adapter.json`.
  WIRE/I2C/SWD, KG 156/156, 69/69 current strict emits, CI/book/doctrine/locality, and exact cleanup close `.2.34`.
  `.2.35` transfers the same generic boundary to the 96-page USB4 Connection Manager Guide: 96 portable
  sidecars, 13→0 relations, 4→0 interfaces, 11→0 ports, and the stale `SB`/`USB`/`USB4` target removed. The
  methodology guide blocks honestly. `.2.36` then transfers the boundary to the 29-page CoreSight Base System
  Architecture: source/evidence structure holds, while 88 stale heuristic interfaces and its 100-signal/two-rule
  target disappear. The adapter blocks honestly; validation preserves the actual upstream cat-3 capture-recall
  gap (19 unenriched visuals / 87 partially structured normative statements / seven untyped temporal sources).
  `.2.37` transfers the boundary to the 25-page AArch64 External Debug Guide. The portable refresh removes 81
  diagram-label body records and the visual-label `host` actor while preserving the document structure, 243
  evidence statements, five conditionals, and 78 behaviors. The stale 64 heuristic interfaces and 73-signal
  target have zero relation/declaration authority; the high-confidence methodology guide blocks honestly and
  leaves only `adapter.json`. `.2.38` then falsifies the apparent topology in the 32-page Introducing CoreSight
  Debug and Trace guide. Its nine retained relations include prose fragments such as `RAM is reads APB` and
  `means drives ATB`; the six interfaces/eight ports/four connectivity edges and four-signal/two-enum adapter
  therefore have no declaration authority. Two guarded ingests remove a net 83 diagram-label records while
  preserving clean prose and structure, and the high-confidence methodology guide retains 228 statements, one
  signal constraint, four actors, 13 phases, 21 invariants, and 49 behaviors. Lowering blocks honestly with one
  unsupported temporal residual and only `adapter.json`. Two cascades reproduce all 12 hashes; all 66 remaining
  emits are strict-clean. Corpus is 38 done / 18 remaining. Preflight's copied-rollback side effect remains
  durably closed by `.2.38a`: validation follows the explicit artifact path for all five stages. `.2.39` then
  transfers the same authority boundary to the 10-page OpenCAPI 3.0 Ready Test Resources engineering note. Its
  retained synthetic `Signal DL is width 1.` came from a glossary row defining data link layer, not a signal
  declaration. Two guarded ingests preserve all 105 SourceIR elements while EvidenceIR 113→112 drops only that
  synthetic statement; the stale one-interface/one-output surface disappears. The guide blocks on no signals
  plus no behavior and leaves only `adapter.json` plus its report. Two cascades reproduce 12 hashes; all 66
  remaining emits are strict-clean. `.2.40` independently confirms the same boundary on the 13-page OpenCAPI
  3.0 Certified Test Resources engineering note. Two guarded ingests reproduce 13 pages / three visuals / two
  tables / 25 sections / 173 elements; all 13 image and 13 layout paths are repository-relative. Evidence
  173→172 drops only its sibling synthetic `Signal DL is width 1.` glossary record, removing the stale one-
  interface/one-output surface while preserving the note's actors, phases, invariants, behaviors, and constraints.
  Lowering blocks on no signals plus no behavior and leaves only `adapter.json` plus its report; all 66 remaining
  emits are strict-clean. `.2.41` is now closed on the 14-page OpenCAPI AFU Address Space Usage engineering note.
  Two guarded ingests reproduce 14 pages / nine visuals / two tables / 23 sections / 118 elements at 20% and 19%
  peak memory; every page and visual path is repository-relative. Evidence stays at 106 statements with zero
  typed hardware surfaces. Current authority removes five stale acronym-group interfaces/five synthetic actors/
  eight one-bit outputs; child `.2.41a` keeps legal source evidence but removes its false phase/gate and two legal
  invariants/behaviors. Final semantic/intent hardware and behavior surfaces are empty, so lowering blocks honestly
  and leaves only `adapter.json` plus its report. The 12-artifact set and two repaired downstream cascades are
  deterministic; WIRE/I2C/SWD, KG 156/156, full CI, 66/66 FSMGen strict, mdBook/doctrines/path/locality, and exact
  cleanup pass. `.2.42` is now closed on the 15-page OpenCAPI 3.0 Certified Definition. Two guarded ingests
  reproduce eight visuals / five tables / 31 sections / 151 elements at 18% and 33% peak sampled memory; all 46
  project-owned path references are relative and present. Source 155→151 removes exactly four flattened diagram
  labels; Evidence 172→167 additionally removes synthetic `Signal DL is width 1.` while retaining zero typed
  hardware surfaces. Current signal authority removes the stale interface/output. Child `.2.42a` separately
  measured 238 administrative/reference statements across 23 retained documents and closed the shared authority
  gap: certification requests, email review, conflict resolution, test-lab administration, and listing processes
  remain evidence but cannot become phases/gates/Intent behaviors. Final SemanticIR has two actors, zero
  interfaces/phases/gates, eight invariants, four contracts, and 14 decompositions; IntentIR retains four genuine
  compliance behaviors and eight constraints. Lowering blocks on no signals and leaves only `adapter.json` plus
  its report. Twelve hashes, two cascades, WIRE/I2C/SWD, KG 156/156, 66/66 FSMGen strict, book/doctrines/path/
  locality, exact cleanup, and zero residue pass. `.2.43` then closes the 15-page OpenCAPI 3.0 Ready Definition.
  One guarded CPU ingest peaks at 62% sampled memory and produces six visuals / three tables / 30 sections / 169
  elements with all 42 project-owned paths relative and present. Source 171→169 removes two flattened diagram
  labels; Evidence 176→173 additionally removes synthetic `Signal DL is width 1.` while retaining two typed
  conditionals and no hardware surface. Completed children retire untyped generic phase/gate projections. Final
  SemanticIR has three actors, zero interfaces/phases/gates, 21 invariants, 19 contracts, five assertions, and 17
  decompositions; IntentIR retains 19 compliance behaviors / 25 constraints / two assumptions. Lowering blocks
  only on no signals and leaves its manifest/report. Twelve hashes, two cascades, WIRE/I2C/SWD, KG 156/156, 66/66
  FSMGen strict, book/doctrines/path/locality, exact cleanup, and zero residue pass. Corpus is 43 done / 13
  remaining with stage census 80 SourceIR / 12 normalized / 80 EvidenceIR / 79 SemanticIR→IntentIR→adapter
  chains. `.2.43a.ii` then closes typed transaction-phase precision without downstream change. `.2.44` refreshes
  the 23-page OpenCAPI 32 Gbps PHY Mechanical spec: Source 262→212 and Evidence 350→267 remove visual-label and
  synthetic-ToC-enum noise; current authority removes five stale interfaces and the nine-port/one-enum `.isf`.
  Final IntentIR keeps three behaviors / 45 constraints / two assumptions; lowering blocks honestly on no
  declared signals. Two cascades reproduce; WIRE/I2C/SWD, KG 156/156, and 65/65 FSMGen strict pass. Corpus is 44
  done / 12 remaining at 80 SourceIR / 13 normalized / 80 EvidenceIR / 79 downstream chains; full CI,
  book/doctrines/path/locality pass. `.2.45` refreshes the 30-page OpenCAPI 25 Gbps PHY Signaling spec: Source
  231→224 and Evidence 395→385 remove visual-label noise plus synthetic `DL`/`DDJ`/`CDR` declarations while all 70
  physical timing constraints hold. Current authority removes two stale interfaces and the three-port `.isf`;
  final IntentIR keeps four behaviors / 27 constraints / two assumptions. Two cascades reproduce; WIRE/I2C/SWD,
  KG 156/156, 64/64 FSMGen strict, full CI, book/doctrines/path/locality pass. Corpus is 45 done / 11 remaining at
  80 SourceIR / 14 normalized / 80 EvidenceIR / 79 downstream chains. `.2.46` refreshes the 40-page OpenCAPI
  Discovery Configuration spec: Source 181→172 removes nine flattened visual labels; Evidence 831→754 further
  removes 65 synthetic table/front-matter enums and false `BDF`/`DL`/`VPD` declarations. Child `.2.46a` closes
  the distinct ambiguous bare-`data` authority defect while preserving real I2C/I2S serial signals. Current
  authority removes the stale interfaces/port/relation/phases/gates, but keeps ten behaviors / 55 constraints /
  six timings. Lowering blocks honestly on no declared signals. Validators retain the known table/visual/
  normative/temporal capture frontier; two repaired cascades reproduce eight downstream hashes and final
  committed-binary validation pins all 12 artifact/report hashes. `.2.47` refreshes the 46-page Cortex-A76
  Software Optimization Guide: Source 260→249 and Evidence 971→960 remove flattened visual labels while the
  instruction catalog no longer fabricates 228 interfaces, 18 phases, 23 gates, 37 behaviors, three timing
  constraints, or a 537-output `.isf`. Child `.2.47a` repairs the shared structural timing authority, moving the
  retained 39-document surface 2,144→608 grounded scalar records while Cortex moves 151→0 and I2S holds at five.
  Final IntentIR retains 15 constraints / two assumptions; lowering blocks honestly on no declared signals and
  keeps two canonicalization residuals. The committed release reproduces SourceIR `a396a074…d478`, normalized
  manifest `872e7dba…1bae`, and downstream `b4b50237…9ade` twice. Corpus is 47 done / nine remaining at 80
  SourceIR / 19 normalized / 80 EvidenceIR / 79 downstream chains; all 60 current emitted ISFs are strict-clean.
  `.2.48` refreshes the 43-page OpenCAPI 4.0 32 Gbps PHY Signaling specification. Two guarded ingests reproduce
  318 elements and a 130-file normalized bundle. Source 376→318 removes 58 flattened visual labels; Evidence
  613→547 additionally removes four synthetic declarations and four generic-enum statements. Child `.2.48a`
  closes the Docling-expanded footer defect across the 80-document timing surface (608→585), leaving 60 grounded
  OpenCAPI timings. Current authority removes three stale interfaces / 18 phases / 19 gates and the four-port/
  one-enum target; final IntentIR retains seven actors / 21 behaviors / 75 constraints / four assumptions. Corpus
  is 49 done / seven remaining at 80 SourceIR / 21 normalized / 80 EvidenceIR / 79 downstream chains, with 58/58
  current emitted ISFs strict-clean. `.2.49` refreshes the 45-page Generic Interrupt Controller Overview Guide:
  two guarded ingests reproduce 430 elements and a 139-file / 30,731,394-byte normalized bundle. Current source
  classification identifies the GIC-version/CPU-family table as a feature matrix rather than three timing values;
  current semantic authority removes 83 stale heuristic interfaces, 26 generic phases, 54 prose gates, and the
  unsupported `controller.isf` while preserving 87 invariants, four contracts, one assertion, four grounded intent
  behaviors, and 88 constraints. The corpus task record now has a bounded current root over eight semantic parts,
  a 50-route index/manifest, and an exact 2,308-line source capsule. The next separately owned `.2.50` leaf selects
  the 57-page / 527-element OpenCAPI Data Link Layer v2.0 specification, the smallest of seven remaining sources.
  Its first cascade removes the stale interface/target but exposes child `.2.50a`: four uppercase tokens that occur
  only after passive binding leads become false signal constraints and one temporal conflict. `.2.50a` is closed —
  both deterministic extractors now keep a passive obligation's subject only when the document names it before the
  `must/shall be|remain` lead, which deletes 26 measured false records across nine documents, rebuilds the I2C,
  USB 3.2, and WISHBONE cascades, and drops a false `SCL` rule from an emitted target while active
  `must drive`/`must have … tied` grammar and table-row context are untouched. `.2.50` then completes: two guarded
  ingests reproduce 527 elements and a 184-file / 52,570,034-byte bundle, current authority removes four
  interfaces/ports/relations, 54 phases, 71 gates, 38 contracts, and the synthetic `endpoint_dlx.isf`, and 87
  invariants / 24 behaviors / 87 constraints remain with lowering blocked on no declared signals. Corpus is 50
  done / six remaining at 80/22/80/79 stages with 57/57 emitted ISFs strict-clean. The guarded migration owned by
  [`CORPUS-TASK-EVIDENCE-CONTAINMENT`](docs/tasks/CORPUS-TASK-EVIDENCE-CONTAINMENT.md) is lossless and below all
  local warnings. Its same-SSD no-hardlink clean clone passed every doctrine, a temporary eighth active part proved
  the eligible post-migration #49 route through the composed live gate, and exact restoration left no residue.
  Containment is closed; the first post-migration product route is complete and the second is owned.
  Separately,
  [`FACT-CARD-CATALOG-CONTAINMENT`](docs/tasks/FACT-CARD-CATALOG-CONTAINMENT.md) owned the browse-plane risk at
  158 cards / 134 monolith bytes of headroom. `.0` pins the source/readers; `.1` accepts ADR 0020's stable direct-ID
  landing, count-packed title parts, derived 198-card maximum, and independent root/part/aggregate bounds.
  `.2.1.1` and ADR 0021 preserve semantic rows and resolved cards through the required `../knowledge/` rewrite;
  `.2.1.2` lands the schema-closed checker and 40 pre-migration cases. ADR 0022 tightens title parts from 64 to 56
  cards so full parts remain below warning. `.2.2` writes the 161-line landing and three parts, activates their
  generated surface, and closes the Unicode row-digest gap with a non-ASCII fixture. ADR 0023 catches the verbose
  root at 218/224 health lines for 198 cards and reduces only its scaffold: full capacity is now 201/224 with an
  exact-boundary regression. The migrated checker has no current pressure warning, the tree is closed, and the
  frontier returns to `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.2`. The
  completed `.0`–`.10`
  [`LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION`](docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md)
  implementation keeps current documentation and project data bounded on the repository volume; completed
  [`ARTIFACT-PATH-PORTABILITY`](docs/tasks/ARTIFACT-PATH-PORTABILITY.md) extends move-safe storage through every
  canonical IR/adapter stage, consumer, dormant serializable path schema, and rescan-plan field, with the present
  generated corpus migrated and fail-closed producer/data enforcement installed. The subsequent
  `SWD-SERIAL-EXTRACTION.7` closure projects every scored SWD protocol record through IntentIR, accounts for it
  at the adapter and convergence boundaries, and promotes a fresh portable canonical chain.

The complete, always-current execution/status catalog is [`docs/TASK_TREE.md`](docs/TASK_TREE.md).
This roadmap owns only high-level direction and milestone status; it does not mirror leaf frontiers or
delivery chronology.

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
2. Advance the KG/IntentIR completeness and transaction-fidelity north star under the wire-protocol
   signoff gates.
3. Turn measured corpus misses into general extraction capability while expanding PDF/protocol breadth.
4. Keep large-document ingest bounded in RAM, disk, temporary storage, and host impact.
5. Deepen deterministic and model-assisted language understanding without hardcoded document vocabulary.
6. Extend R15c–R15g learning, arbitration, evaluation, priors, and corpus synthesis only behind typed
   provenance, local grounding, and measurable no-regression gates.
7. Keep `.fsm`, scheduling, and HDL downstream in FSMGen; do not recreate those concerns in SpecForge.

## History and execution

- Current task status and exact next leaves: [`docs/TASK_TREE.md`](docs/TASK_TREE.md)
- Detailed current change history: [`CHANGES.md`](CHANGES.md)
- Current delivery snapshot: [`LIVE_ACHIEVEMENT_STATUS.md`](LIVE_ACHIEVEMENT_STATUS.md)
- Crash-safe resume pointer: [`MEMORY.md`](MEMORY.md)
- Exact pre-containment roadmap and its identity manifest:
  [`docs/archive/roadmap/INDEX.md`](docs/archive/roadmap/INDEX.md)
- File-level evolution in Git: `git log -- ROADMAP.md`
