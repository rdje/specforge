# CORPUS-COVERAGE — program and latest refreshes

- Part ID: `program-and-latest-refreshes`
- State: `legacy`

<!-- corpus-task-source-region:program-and-latest-refreshes:start -->
# CORPUS-COVERAGE: build every ingested doc through to IntentIR/.isf + keep downstream stages non-stale

## Metadata

- Tree ID: `CORPUS-COVERAGE`
- Status: `active` (`.0` build-out + `.1` stage-staleness validator done; `.2` current-binary refresh batch
  remains active with 47 completed documents and `.2.47` Cortex-A76 Optimization Guide refresh done; `.3`
  lifecycle/currentness reconciliation done `2026-08-09`)
- Roadmap lane: `R15e`/`R16` (corpus digestion — the owner's substantive gap #2)
- Created: `2026-06-17`
- Owner directive: `2026-06-17` — after the owner rejected the "buildable frontier exhausted" framing
  (SpecForge is NOT complete, `[[feedback_not_complete_attack_substantive_gaps]]`), attack three substantive
  north-star gaps; this tree owns **#2 corpus coverage** ("only 36 of 79 ingested docs reach IntentIR/.isf").

## The point

A document that stops at EvidenceIR contributes nothing to the canonical IntentIR/.isf surface the whole
tool exists to produce. The pre-existing local corpus had **only 36 of 79** ingested docs carried through to
IntentIR — the other 42 sat at evidence-only (operational: a sweep rebuilt evidence without cascading
downstream; the per-stage commands do not auto-cascade, only `converge` rebuilds the whole chain). Building
semantic→intent needs only the already-persisted `evidence_ir.json`, **not** the heavyweight `normalized/`
bundle, so the evidence-only docs are cheap-buildable with no re-ingest.

## `.0` — corpus build-out + coverage census (DONE `2026-06-17`)

Read-only census + a deterministic, RAM-safe build-out (release binary; no LLM, no Docling).

**Census (before):** of 79 ingested source docs — evidence **78**, semantic/intent/isf **36/36/36**; 42
evidence-only; 3 stale-intent (`tilelink_1_8_0` ev 40/int 0, `um10204` i2c 17/0, `wbspec` 1/0 — the
`KG-ISF-COMPLETENESS.3` staleness class); 57 of 79 lack the `normalized/` bundle.

**Build-out:** rebuilt the 42 evidence-only + 3 stale docs `semantic`→`intent`→`adapt --target isf`
(excluding the 4 WIRE-BASED-100 gold docs, which already build and are gated). **Result: 42/42 build OK, 0
failures**, RAM steady 77% throughout.

**Census (after):** evidence **78**, semantic **78**, intent **78**, isf **75**. **0 stale-intent
remaining**; the 3 stale docs recovered their relations (tilelink_1_8_0 → 40, i2c → 17, wbspec → 1). The 3
intent-without-isf docs (`risc_v_debug`, coresight `den0068` BSA, GIC arch `ihi0069_g`) **block honestly**
(`adapt` reports `no behavioral content (temporal/conditional rules, signal constraints, or control
blocks)`) — register/architecture docs with nothing behavioral to lower, not a failure. Intent relation
distribution across 78: 30 at 0 (the honest-absence register/guide class from `KG-ISF-COMPLETENESS.3`), 18 at
1–10, 17 at 11–50, 13 at 50+.

**Finding:** the staged pipeline is now validated end-to-end on the **entire** local corpus (78/78 to intent,
0 errors). The generated tree is git-ignored local cache, so the durable deliverables are this measurement +
the two follow-ups below. The 57 docs lacking a `normalized/` bundle cannot have their EVIDENCE rebuilt
without re-ingest (Docling + source PDF, RAM-gated) — a standing frontier gated on host-local source
re-provisioning (`[[feedback_source_pdfs_in_repo]]`).

**Lifecycle correction (`2026-08-09`, `.3`):** "57 normalized-missing" described the cohort when `.2`
started; it is not a permanent retention denominator. Re-ingest completion means the document's EvidenceIR and
downstream stages were refreshed with the then-current binary. A later, documented `2026-07-05` artifact sweep
intentionally reclaimed every normalized cache. After the next completed refresh, the live tree has 80 SourceIR /
2 normalized bundles / 80 EvidenceIR / 79 SemanticIR / 79 IntentIR / 79 adapters, while all five stage artifacts
for every one of the 33 completed `.2` documents remain present (165/165). Therefore `.2` progress is **33
completed refreshes**, and the remaining queue is **23 real chip-spec documents not yet refreshed by `.2`**;
neither number is inferred from
today's ephemeral normalized-directory count.

## Task Tree

- ID: `CORPUS-COVERAGE` · Status: `active` · Children: `.0` (build-out + census, done), `.1` (stage-staleness
  validator, done), `.2` (host-local re-ingest batch, active), `.3` (frontier/lifecycle currentness audit,
  done)
- ID: `CORPUS-COVERAGE.0` · Status: `done` (`2026-06-17`) · Goal: build every evidence-only doc through to
  IntentIR/.isf and census the result. Done: 36→78 intent / 36→75 isf, 0 build failures, 0 stale remaining,
  3 isf honest-blocks. Verification above.
- ID: `CORPUS-COVERAGE.1` · Status: `done` (`2026-06-17`, CODE) · Goal: a generic **stage-staleness
  detector** in `validate` so a downstream artifact silently dropping intent (the `tilelink` 39→0 relation
  class) is SURFACED, not hidden — directly serves "the KG must be COMPLETE." **DONE:** `validate
  <intent-ir>` / `<semantic-ir>` now loads the upstream artifact (via the carried `semantic_ir_path` /
  `evidence_ir_path` — `validate` already does this for graph-aware findings) and emits a `stage_staleness`
  **Warning** (`intent_stale_relations_dropped` / `semantic_stale_relations_dropped`) when the downstream
  carries 0 `actor_signal_relations` while the upstream carries some. **False-positive-free** because the
  agent-identity gates (consolidation/split/phantom-drop) NEVER empty a non-empty relation set — a 0-vs-N
  split is staleness, not gating; the I/O is paid only when the downstream is empty (a `let`-chain
  short-circuit), and is skipped when the upstream is not on disk (detached copy). Pure decision helper
  `stage_staleness_relation_finding` (+3 unit tests: 0-vs-39 fires, 39/17-vs-N silent, 0-vs-0 honest-absence
  silent). **Verified live** (release binary, temp-CWD to avoid the WRITE-PATH GOTCHA): POSITIVE fires on a
  synthetic stale tilelink (0 vs real semantic 39), NEGATIVE silent on `nvme` (0-vs-0 honest absence — the
  critical no-false-positive case) and on healthy tilelink (39). ADR-0006 (universal/structural, no name
  list). WIRE-BASED-100 unaffected by construction (validate-only additive finding; wire docs carry non-empty
  relations → silent; extraction/IR content untouched); `run_ci.sh` GREEN, lib 1660 passed (+3); `kg-bench`
  156/156. Book `quality/validation.md`; KM `[[stage-staleness-validate-detector]]`.
- ID: `CORPUS-COVERAGE.2` · Status: `active` (`2026-06-21`) · Goal: **RAM-guarded re-ingest of the
  57-document cohort identified by missing normalized bundles, using the CURRENT binary**, now that the owner has re-provisioned the host-local
  spec library. **Provisioning (owner-chosen `2026-06-21`):** instead of copying ~150 MB of PDFs into tracked
  `corpus/` (permanent git bloat), the library is reached through a **git-ignored symlink**
  `.cache/local-references/chipdoc → <owner host-local chipdoc git repo>` (`.cache/` added to `.gitignore`;
  the owner's absolute library path is therefore never recorded in any tracked file — `[[feedback_source_pdfs_in_repo]]`
  — and tracked docs cite only the repo-relative `.cache/local-references/chipdoc/...` path). The owner confirms
  chipdoc is permanent. The 22 gold/measured docs stay copied in `corpus/` for the reproducible
  WIRE-BASED-100/eval path; this symlink serves the bulk coverage re-ingest only.
  **The substantive win (not mere "reach .isf"):** all 57 already reached IntentIR via `.0`, but their
  EvidenceIR is STALE — built before the `.10a`–`.10g` register/message-field families, the `.12a`/`.12b`
  presence records, and the `.2a`–`.2m` transaction recognition landed. Re-ingest → `evidence` → `semantic` →
  `intent` → `adapt --target isf` with the current binary surfaces all that new typed intent → more complete
  KG/IntentIR → more faithful `.isf`. **Method:** PNT, one doc per slice (smallest/highest-value AMBA &
  interconnect PROTOCOL specs first — CXS/GFB/ACC/ATP/TileLink/LPI/DTI/CHI-C2C — then the register/TRM/ISA
  docs), `DOCLING_DEVICE=cpu` (`[[project_docling_mps_cpu]]`), the built-in `.4a` RAM guard active (clean
  abort at ≥85% used), Ollama kept idle, RAM+swap monitored between docs (`[[feedback_ram_ceiling_monitor]]`),
  commit per `COMMIT.md` after each doc. No fabrication / ADR-0006 unchanged (this is a re-run of existing
  deterministic extractors, not new code); WIRE-BASED-100 + register/wire golds + `kg-bench` stay green
  (orthogonal — the 4 gold docs are not re-ingested). Record per-doc before/after typed-surface deltas here.
- ID: `CORPUS-COVERAGE.2.48` · Status: `done` (`2026-08-10`, DATA/CODE/DOC) · Children: `.2.48a`
  spanned-row timing scalar-authority repair (done). Goal: re-ingest the
  43-page OpenCAPI 4.0 32 Gbps PHY Signaling Specification
  (`opencapi_4_0_32g_phy_signal_spec_1_0_16nov2020`) from the owner-authorized same-SSD library with the current
  release, then rebuild and validate EvidenceIR→SemanticIR→IntentIR→ISF. Preserve all grounded physical timing
  while refusing glossary/measurement acronyms, generic sections, or broad prose cues as interface, phase, gate,
  behavior, or enum authority. It is the smallest of the nine remaining candidates at 376 retained elements.

  Read-only authentication pins portable source
  `.cache/local-references/chipdoc/cxl/opencapi/current/OpenCAPI-4.0-32G_PHY_Signal_Spec_1.0_16NOV2020.pdf` at
  `d3eb19fc…38be`, 855025 bytes, the initial release at `0b497079…5756`, and the final child release at
  `efb57ab3…8ab4`. Source and repository share device
  `16777240`. The exact stale seven-file / 1476701-byte chain is SourceIR `6e1d7395…abd` (43 pages / 39 visuals /
  27 tables / 73 sections / 376 elements), EvidenceIR `41752734…dad` (73 anchors / 605 spans / 613 statements /
  39 visuals / 460 links / zero relations / two conditionals / zero registers / 69 timings), SemanticIR
  `47dd8220…13c0` (eight actors / three interfaces / zero ports or relations / 18 phases / 19 gates / 76
  invariants / 21 contracts / one assertion / 69 timings), and IntentIR `d0f481fe…0b03` (seven actors / three
  interfaces / 51 behaviors / 77 constraints / four assumptions / 69 timings). Adapter `bb614867…84a` emits a
  four-output / one-enum `channel.isf` (`CDR`, `DDJ`, `DL`, `DL3`) with no transactions, rules, constants, or
  storage; the stale SourceIR still records retired boot-volume provenance.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.48`

- [x] **REPRODUCE / MEASURE** — the exact stale chain is authenticated. Two guarded CPU ingests reproduce
  SourceIR `e2c0b9e5…859f`; the final 27.2-second run under the 85% abort guard restores 130 files / 34843407
  bytes at `30f70511…8232`. Row 48 pins full counts and path locality.
- [x] **ROOT CAUSE (WHY + WHERE)** — the complete stale→current delta is classified at its first shared seam.
  Source 376→318 removes 58 visual labels; Evidence 613→547 also removes four synthetic signals and four generic
  enums while links 460→510 gain current grounding. Child `.2.48a` owns timings 69→60. Retired phase/gate
  authority explains interfaces 3→0 / phases 18→0 / gates 19→0 / behaviors 51→21 and the removed stale target.
- [x] **ADDRESSED (verified)** — the current SourceIR→EvidenceIR→SemanticIR→IntentIR→adapter chain validates with
  60 grounded timings, 75 constraints, 21 behaviors, no declared interface or relation, and no emitted target.
  Two final-code downstream replays preserve all four artifact hashes byte-for-byte.
- [x] **NO REGRESSION** — final downstream hashes reproduce; focused EvidenceIR tests, nine provider-free evals,
  KG 156/156, 59/59 current emitted-ISF FSMGen strict, full CI 1,798/five ignored, mdBook, doctrines, path,
  locality, and live-size gates pass.
- [x] **GENERICITY** — no OpenCAPI/document/acronym allowlist or denylist, manual artifact edit, fabricated model,
  validator relaxation, or host-specific persisted path enters production.
- [x] **LOCKSTEP** — generated chain, row 48, task/frontier, roadmap, status, live docs, mdBook, Knowledge Map,
  and resume pointer agree before commit; delete only authenticated rollback data after durable recording.
- ID: `CORPUS-COVERAGE.2.48a` · Status: `done` (`2026-08-10`, PROBE/CODE/DATA/DOC) · Goal: repair the
  universal spanned informational-row defect exposed by parent #48 before accepting its cascade. OpenCAPI table
  5-11 has a valid eight-column timing schema and two real scalar rows, followed by one `Notes:` cell spanning all
  eight columns. Docling expands that cell into eight identical `StructuredTableCellRecord`s, each retaining
  `col_span: 8`; `synthesize_timing_constraints` indexes the expanded row as if every position were independent.
  The note therefore becomes parameter, min, typ, max, and unit simultaneously and survives `.2.47a`'s
  value-bearing gate as false `timing_table_0023_002`. This is a shared row-role/span defect, not an OpenCAPI or
  note-prefix exception.

  First census every retained timing-classified table row that carries a multi-column span or repeats one source
  value across scalar roles; distinguish genuine merged-value layouts from informational/footer rows and measure
  every current false record. Then reject only rows whose source-cell geometry cannot independently ground the
  mapped parameter and scalar columns, add paired spanned-footer/ordinary/legitimate-span regressions, and replay
  every affected real EvidenceIR consumer through the adapter. No note wording, document key, table id, vendor,
  parameter name, or expanding text denylist is permitted.

  The complete retained boundary contains 80 SourceIR documents, 105 timing-classified tables, and 608 current
  scalar records. Exact structural replay identifies 23 records whose mapped parameter or populated scalar role
  comes from a multi-column source cell: three SWP notes, seven HBM2 section/group rows, twelve eMMC note/group
  rows, and the one OpenCAPI footer. None of the 585 survivors has a span in an authority-bearing role.

  The shared EvidenceIR seam now requires the parameter cell and every populated min/typ/max cell to have
  `col_span: 1`. Blank or `-` scalar absences and optional unit/description spans remain legal. Paired tests prove
  that independently repeated values survive, a spanned optional description survives, and cloned footer or
  parameter cells do not emit. The active rebuildable OpenCAPI cascade reproduces downstream hash
  `b9418108…8951` twice and changes timings 61→60; the three non-rebuildable controls remain byte-exact across all
  twelve current stage directories instead of being misreported as complete cascades.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.48a`

- [x] **REPRODUCE / MEASURE** — pin the exact real false record and complete retained-corpus span/role census;
  identify every changed EvidenceIR consumer and positive/negative calibration case before production code.
- [x] **ROOT CAUSE (WHY + WHERE)** — prove how Docling-expanded `col_span` geometry reaches independent
  name/min/typ/max/unit indexing, and distinguish the defect from valid repeated values or ordinary scalar rows.
- [x] **ADDRESSED (verified)** — enforce source-cell geometry at the shared timing-row extraction seam; the real
  footer emits no constraint while both real table 5-11 scalar rows and all independently grounded controls hold.
- [x] **NO REGRESSION** — focused EvidenceIR tests, the active rebuildable cascade twice, exact retained-surface
  replay for all four affected documents, twelve byte-exact non-rebuildable control comparisons, nine
  provider-free evals, KG 156/156, 59/59 current emitted-ISF FSMGen strict, full CI 1,798/five ignored, mdBook,
  doctrines, path, locality, and live-size gates pass.
- [x] **GENERICITY** — production policy uses only row geometry and mapped scalar roles; no `Notes`, OpenCAPI,
  document/table/parameter identity, text denylist, or manual artifact edit.
- [x] **LOCKSTEP** — code, corpus measurement, child/parent task state, durable fact, live docs, mdBook, and resume
  pointer agree; exact controls/rollback artifacts restore or remain intentionally active before child commit.
- ID: `CORPUS-COVERAGE.2.47` · Status: `done` (`2026-08-10`, DATA/CODE/DOC) · Children: `.2.47a`
  timing-table structural-authority repair (done). Goal: re-ingest the
  46-page Cortex-A76 Software Optimization Guide
  (`pjdoc_466751330_7215_10_0_cortex_a76_software_optimization_guide`) from the owner-authorized same-SSD host
  library with the current release, then rebuild and validate EvidenceIR→SemanticIR→IntentIR→ISF without
  treating instruction mnemonics or optimization terminology as declared hardware ports. It is the smallest of
  the ten remaining real chip-spec candidates at 260 retained elements / 637434 source bytes, ahead of the
  43-page / 376-element OpenCAPI 4.0 32G PHY Signaling candidate.

  Read-only authentication pins portable source
  `.cache/local-references/chipdoc/arm/processors/cortex-a/cortex-a76/current/PJDOC-466751330-7215_10.0_Cortex_A76_Software_Optimization_Guide.pdf`
  at `8358c5ae…3a22`, release `c4072c33…a1b05`, and an exact seven-file / 2208588-byte stale chain. Source,
  repository, and generated chain share device `16777240`. The stale SourceIR has 46 pages / 71 visuals / 68
  tables / 68 sections / 260 elements and retired boot-volume provenance. EvidenceIR has 68 anchors / 971 spans
  and statements / 71 visuals / one link / zero relations / one conditional / zero registers / three timings.
  SemanticIR has four actors / 228 interfaces / zero ports or relations / 18 phases / 23 gates / 15 invariants /
  48 decompositions / three timings; IntentIR has three actors / 228 interfaces / 37 behaviors / 28 constraints /
  two assumptions / three timings. The stale adapter emits a 537-output `consumer.isf` made predominantly from
  instruction names, with zero transactions, rules, constants, enums, or storage.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.47`

- [x] **REPRODUCE / MEASURE** — the exact seven-file / 2208588-byte stale chain is preserved and verified before
  mutation. Three guarded CPU ingests reproduce 46 pages / 71 visuals / 68 tables / 68 sections / 249 elements,
  SourceIR hash `a396a074…d478`, and normalized-manifest hash `872e7dba…1bae`; two sampled runs keep 52–64% system
  memory free and the final committed-release run completes without a guard abort. The final
  normalized bundle is 168 files / 46082616 bytes, repository-relative; the authorized source stays labeled
  external and read-only on the same SSD.
- [x] **ROOT CAUSE (WHY + WHERE)** — stale instruction names entered the sparse interface surface and produced
  228 false interfaces / a 537-output adapter under retired mixed-vintage authority. Current shared signal,
  phase, and gate boundaries remove that topology. Child `.2.47a` owns and closes the distinct universal timing
  defect that initially turned instruction-performance rows into 151 value-empty constraints.
- [x] **ADDRESSED (verified)** — final EvidenceIR has 68 anchors / 960 spans and statements / 71 visuals / one
  link / one conditional and zero relations, registers, or timings. SemanticIR has four actors, zero interfaces,
  ports, relations, phases, gates, or timings, 15 invariants, and 44 decompositions. IntentIR has zero actors,
  interfaces, behaviors, or timings, 15 constraints, and two assumptions. The adapter blocks only on no declared
  signals, carries two canonicalization residuals, and emits no target instead of fabricating instruction ports.
- [x] **NO REGRESSION** — release binary `0b497079…5756` from child commit `46af2eca` reproduces downstream hash
  `b4b50237…9ade` across two complete cascades; all five stages validate. Nine provider-free eval datasets retain
  their established results, KG is 156/156, all 60 emitted ISFs pass FSMGen strict, and the child full-CI run
  passes 1,796/five ignored. mdBook, doctrines, persisted-path, locality, and live-size gates pass.
- [x] **GENERICITY** — the parent adds no producer exception. Its child uses only category provenance,
  identifier-safe header tokens, structural row roles, and typed scalar values; no ARM/Cortex/document/mnemonic
  policy appears in production.
- [x] **LOCKSTEP** — generated chain, row 47, task/frontier, roadmap, status, mdBook, ledgers, and resume pointer
  agree on 47 completed / nine remaining / 60 strict-clean emitted ISFs. The exact parent and child task bundles
  are deleted only after this durable recording commit, then frontier selection may begin for refresh #48.
- ID: `CORPUS-COVERAGE.2.47a` · Status: `done` (`2026-08-10`, PROBE/CODE/DATA/DOC) · Goal: repair the
  universal timing-table structural-authority defect exposed by #47 before accepting its cascade. Fresh SourceIR
  classifies 11 Cortex-A76 instruction-performance tables as `timing_parameter` and current EvidenceIR emits 151
  `TimingConstraintRecord`s, apparently a 3→151 recall gain. Inspection falsifies that interpretation: records such
  as `Load, immed offset` have no min/typ/max value and duplicate the operation name into `unit`.

  The failure crosses two producer seams. Docling marks the first row-label cell of data rows as a header cell;
  `classify_table_kind` flattens *all* `header_rows`, so instruction data contaminates column-header vocabulary.
  Substring matching then lets mnemonic/text fragments supply `min` and lets `instruction` supply `ns`, creating
  false timing authority. Downstream, `synthesize_timing_constraints` searches the real first header row but uses
  the same broad `contains("ns")`; `Instruction group` is therefore selected as the unit column, and the builder
  emits a record even when every min/typ/max cell is absent. This is a shared structural bug, not an ARM or
  document exception. The genuine `Exec latency` / `Execution throughput` columns do not fit the current
  min/typ/max timing schema and must remain an honest residual until a separately typed performance surface owns
  them.

  First measure the complete retained SourceIR/EvidenceIR population: distinguish real timing tables from
  contaminated instruction/optimization tables, enumerate value-empty timing records and every consumer, and
  calibrate positive/negative headers. Then constrain classification to actual column-header structure with
  token-aware unit vocabulary, require a recognized value-bearing timing schema before emitting constraints,
  add paired trapped-row/normal/body-row regressions, and replay every affected real document. No vendor,
  document, mnemonic, table id, caption phrase, or expanding denylist is permitted.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.47a`

- [x] **REPRODUCE / MEASURE** — the retained population contains 284 timing-labeled tables and 2,144 emitted
  timing records across 39 documents. Exact production-boundary replay yields 608 records; 38 documents change,
  I2S correctly holds at five, and every retained record has at least one min/typ/max value. Cortex is 151→0.
- [x] **ROOT CAUSE (WHY + WHERE)** — the embedded Docling classifier flattened data rows trapped in
  `header_rows`; raw substring matching let `SMIN` supply `min` and `Instruction group` supply `ns`. EvidenceIR
  then chose that header as `unit` and emitted name/unit-only records. The defect is structural and shared.
- [x] **IMPLEMENT / VERIFY** — classification now reads only the leading all-column-header prefix with
  identifier-safe tokens. Structural shape may validate, but never invent, timing category authority. A separate
  scalar schema requires unique min/typ/max columns and a value-bearing row; multi-variant limit tables remain
  timing-category residuals instead of being collapsed or passed to another extractor. Paired source/evidence
  tests retain normal and trapped I2S rows while rejecting instruction, identifier-substring, nested, and
  value-empty cases.
- [x] **NO REGRESSION** — all 39 affected SourceIR artifacts were replayed through the exact timing surface.
  Thirty lack intentionally reclaimed normalized Markdown, so full cascades are impossible from retained cache
  alone; the eight rebuildable real documents completed two EvidenceIR→adapter rounds with 32 stage validations
  and one reproduced combined hash (`58411566…6457a`). Seven control cascades were restored byte-exact; the
  active Cortex chain stays repaired. Nine provider-free eval datasets retain their established results, KG is
  156/156, full CI passes 1,796/five ignored, and all 60 emitted ISFs pass FSMGen strict.
- [x] **GENERICITY** — production policy contains only normalized identifier tokens, structural header/data
  roles, category provenance, and typed scalar value requirements. It contains no vendor, document, mnemonic,
  table id, caption exception, or expanding denylist.
- [x] **LOCKSTEP** — code/tests, parent/child task state, corpus measurement, durable fact, live ledgers, mdBook,
  Knowledge Map, and resume pointer record the same boundary. Parent `.2.47` resumes final signoff from this
  committed repair; its generated chain remains the sole refreshed member of the task cohort.
- ID: `CORPUS-COVERAGE.2.46` · Status: `done` (`2026-08-10`, DATA/CODE/DOC) · Children: `.2.46a`
  parenthetical `data`-head signal-authority repair (done). Goal: re-ingest the 40-page
  OpenCAPI Discovery Configuration specification (`opencapi_discovery_configuration_v201`) from the
  owner-authorized same-SSD host library with the current release, then rebuild and validate
  EvidenceIR→SemanticIR→IntentIR→ISF without treating table/glossary acronyms as declared hardware signals.
  This is the smallest of the 11 remaining real chip-spec candidates: 181 retained source elements and a
  403646-byte input. Read-only authentication pins source `bc767d6e…fe103`, release `463a79e3…b06510`, and an
  exact seven-file / 1951530-byte stale chain before any generated mutation.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.46`

- [x] **REPRODUCE / MEASURE** — the seven-file / 1951530-byte stale chain is copied and byte-verified on the
  repository volume. Three guarded CPU ingests reproduce 40 pages / 54 visuals / 49 tables / 50 sections / 172
  elements, a 139-file / 40288658-byte normalized bundle, and the same SourceIR hash. The 51-sample memory trace
  stays at 47–53% free, so the built-in 85%-used abort never trips.
- [x] **ROOT CAUSE (WHY + WHERE)** — Source 181→172 removes nine flattened labels on visual-bearing pages;
  Evidence 831→754 additionally removes 65 synthetic table/front-matter enums plus false `BDF`/`DL`/`VPD`
  declarations. The first two names converge under existing shared authority; child `.2.46a` owns and repairs
  the distinct bare-`data` authority defect behind `VPD`. Remaining downstream deltas are mixed-vintage phase,
  gate, actor, and interface projections, not a new document-specific defect.
- [x] **ADDRESSED (verified)** — final EvidenceIR retains 50 anchors / 754 spans and statements / 54 visuals /
  643 links / four conditionals / one register / six timings. SemanticIR retains four actors / 55 invariants /
  ten contracts / five assertions / 37 decompositions / six timings; IntentIR retains three actors / ten
  behaviors / 55 constraints / six assumptions / six timings. With no grounded interface signal, lowering
  blocks honestly and keeps one storage record plus the `isf_register_fields_not_lowered` residual.
- [x] **NO REGRESSION** — the final release binary (`c4072c33…a1b05`) validates all five stages and reproduces
  eight downstream hashes from the repaired committed replay; the complete 12-artifact hash set is pinned.
  Child `.2.46a` has already passed 351 EvidenceIR tests, warning-deny Clippy, nine provider-free WIRE/I2C/SWD
  datasets, KG 156/156, full CI 1,793/five ignored, and 61/61 FSMGen strict on this binary and corpus. Parent
  path/locality, mdBook, and doctrine gates pass with zero repository-owned absolute paths. The required changes
  entry crosses two mandatory thresholds; exact plan `corpus-coverage-2.46-changes-rollover-2026-08-10`
  losslessly seals 14 records into `changes-0003` and restores every live warning dimension below 80%.
- [x] **GENERICITY** — the parent changes no producer code; the child repair is shared noun/modifier grammar with
  no OpenCAPI, document, candidate-name, or expanding-token policy. Unknown tables, unenriched visuals, and
  partially structured normative prose remain visible instead of being guessed into topology.
- [x] **LOCKSTEP** — generated chain, task row/log, roadmap, live docs, mdBook, durable fact, product tally, and
  resume pointer agree on 46 done / ten remaining / 61 strict-clean emitted ISFs. The change ledger's manifest,
  index, sealed segment, live root, and task-owned plan agree; the exact 181-file / 9908950-byte task bundle is
  removed only after all verification and recording surfaces are complete.

Stale baseline (`2026-08-10`, read-only): SourceIR has 40 pages / 54 visuals / 49 tables / 50 sections / 181
elements. EvidenceIR has 50 anchors / 763 spans / 54 visuals / 636 links / 831 statements / one relation / four
conditionals / one register / six timings. SemanticIR has six actors / two interfaces / one actor port / one
relation / 16 phases / 20 gates / 57 invariants / ten contracts / five assertions / 38 decompositions / six
timings. IntentIR has six actors / two interfaces / one actor port / one relation / 42 behaviors / 58 constraints /
six assumptions / six timings. The adapter is renderable as `controller.isf` with synthetic width-one
`BDF`/`DL`/`VPD` outputs, three enums, one storage record, and no transactions or rules.

Live result (`2026-08-10`): three guarded ingests produce the same 172-element SourceIR and keep all 188
normalized project paths relative, present, and final-rooted. The nine source removals are flattened labels on
pages 1 and 13; both pages retain visual evidence. Evidence's 77 removals are exactly those nine labels, 65
synthetic `CONTENTS`/`REPRESENTATION`/`TABLE`/`VITAL` enum statements, and false `BDF`/`DL`/`VPD` signal
declarations; no statement is added. Current shared authority removes all interfaces, ports, relations, phases,
and gates while retaining ten grounded behaviors and 55 constraints. Validation classifies this self-declared
specification as under-extracted: 37 normative statements are only partially structured, 15 intent-bearing
tables remain unexplained, 54 visuals are unenriched, one register overlaps fields, and eight temporal-source
IDs have no typed temporal rule. The adapter therefore blocks honestly on no declared signals, emits no target,
and leaves exactly `adapter.json` plus its report. Two repaired cascades reproduce eight downstream hashes; the
final committed-binary run validates and pins all 12 artifact/report hashes. After #46: 80/18/80/79 stage census,
**ten real chip-spec refreshes remain**, and all 61 emitted ISFs are strict-clean.

- ID: `CORPUS-COVERAGE.2.46a` · Status: `done` (`2026-08-10`, PROBE/CODE/DATA/DOC) · Goal: repair the
  universal sparse-catalog parenthetical signal-authority gap exposed by #46 before accepting its cascade.
  `synthesize_signal_declarations_from_prose` correctly requires the immediate noun-phrase head before
  `(ACRONYM)` to be a single-wire noun, but `data` remains ambiguous: fresh #46 turns table-field phrase
  `Vital Product Data (VPD)` into synthetic `Signal VPD is width 1.`, and that declaration becomes a one-port
  SemanticIR interface and emitted `device.isf` despite zero actor ports or relations.

  The complete 80-EvidenceIR census finds exactly six current width-one declarations admitted through a `data`
  head across four documents. Four are genuine and have an adjacent wire qualifier: I2C `serial data (SDA)`,
  `serial data (USDA)`, `high-speed data (SDAH)`, and I2S `Serial Data (SD)`. Two are false: OpenCAPI `Vital
  Product Data (VPD)` and Wishbone example-memory `memory output data (DO)`; the latter reaches the corpus's
  only Wishbone adapter port even though the named Wishbone bus port in the same sentence is `DAT_O`.

  Selected repair: when and only when the immediate parenthetical head is `data`, require the adjacent modifier
  `serial` or `high-speed` (case-insensitive) before granting single-wire authority. Preserve all other current
  wire heads, the pin-appositive and definitional forms, and the four measured positive declarations. Add paired
  unit tests, replay the four affected real documents, prove `VPD`/`DO` absent and the I2C/I2S positive set held,
  then rebuild #46 through adapter. No vendor, document, signal-name, or expanding token denylist is permitted.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.46a`

- [x] **REPRODUCE / MEASURE** — the complete 80-EvidenceIR census finds six `data (ACRONYM)` width-one
  declarations across four documents: four real qualified serial wires and false `VPD`/`DO`; both false names
  reached an emitted adapter port before repair.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs:8709` shows the sparse `<8` fallback
  granted immediate-head authority to bare `data`, which can denote metadata/payload rather than a wire; the
  other retained heads are intrinsically single-wire nouns in this grammar.
- [x] **IMPLEMENT / VERIFY** — only a `data` head now requires adjacent `serial` or `high-speed`; paired tests
  reject `VPD`/`DO` and keep `SDA`/`USDA`/`SDAH`/`SD`. Real four-document replay removes both false ports,
  preserves I2C's six-signal/26-rule target and both I2S signals, and reproduces 33 validated artifact hashes.
- [x] **NO REGRESSION** — all 351 EvidenceIR tests pass (346/five ignored); warning-deny Clippy, nine provider-free
  WIRE/I2C/SWD datasets, KG 156/156, full CI 1,793/five ignored, 61/61 FSMGen strict, mdBook, six doctrines,
  2,349-artifact/357257-value persisted paths, and project-data locality pass. The required live-status append
  triggers an authenticated root-last rollover that seals 12 exact records into segment 0006 and retains 60
  warning-safe live records with zero loss.
- [x] **GENERICITY** — production code contains only normalized noun/modifier grammar; no candidate name, vendor,
  document key, section, source sentence, or expanding denylist appears in the policy.
- [x] **LOCKSTEP** — code comments/tests, parent/child task state, four real cascades, durable fact, live docs,
  mdBook, product count, and resume pointer agree; parent `.2.46` resumes final committed-binary replay/signoff.
- ID: `CORPUS-COVERAGE.2.45` · Status: `done` (`2026-08-10`, DATA/DOC) · Goal: re-ingest the
  30-page OpenCAPI 25 Gbps PHY Signaling Specification (`opencapi_25gbps_phy_signaling_spec_1_0`) from the
  owner-authorized same-SSD host library with the current release, then rebuild and validate
  EvidenceIR→SemanticIR→IntentIR→ISF without fabricating digital behavior from physical-link measurements.
  This is the smallest of the 12 remaining real chip-spec candidates: 231 retained source elements and a
  748724-byte input. Read-only authentication pins source `0e0c8afc…ece12`, release `463a79e3…b06510`, and an
  exact seven-file / 841810-byte rollback chain before any generated mutation.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.45`

- [x] **REPRODUCE / MEASURE** — the exact seven-file / 841810-byte rollback is byte-verified on the repository
  volume; guarded CPU Docling holds 30 pages / 26 visuals / 19 tables / 59 sections at 17% peak sampled memory.
- [x] **ROOT CAUSE (WHY + WHERE)** — source loss is diagram-label suppression on four visual-bearing pages;
  downstream changes are current shared signal/interface, typed-phase, and generic phase/gate authority acting on
  a mixed-vintage retained chain. No distinct shared producer defect is present.
- [x] **ADDRESSED (verified)** — the faithful 224-element / 385-statement chain retains all 70 physical timing
  constraints while removing the unsupported three-port adapter; lowering blocks on no declared signals and
  leaves only `adapter.json` plus its validation report.
- [x] **NO REGRESSION** — two validated cascades reproduce eight downstream hashes; nine WIRE/I2C/SWD datasets,
  KG 156/156, 64/64 FSMGen strict, full CI 1,791/five ignored, mdBook, six doctrines, a 2,077-artifact /
  355906-path census, and locality pass with zero repository-owned absolute paths.
- [x] **GENERICITY** — no code or OpenCAPI/vendor/document/token exception changed; all deltas come from current
  shared grammar, structure, typed evidence, and honest blocked-lowering rules.
- [x] **LOCKSTEP** — task/log, roadmap, live docs, mdBook, containment authority, resume pointer, and generated
  chain agree on 45 done / 11 remaining / 64 emitted ISFs; the exact rollback/task bundle is deleted only after
  this verified unit reaches its durable commit.

Live result (`2026-08-10`): the guarded CPU ingest holds 30 pages / 26 visuals / 19 tables / 59 sections,
peaks at 17% sampled system memory used, and produces 224 source elements with high confidence / zero residuals.
Source 231→224 removes eight diagram labels and adds one clean prose replacement; all four affected pages retain
visual evidence. Evidence 395→385 additionally removes only synthetic `Signal DL/DDJ/CDR is width 1.` statements
and reclassifies one figure caption to mixed modality while holding 12 normative statements, one conditional,
70 physical timing constraints, zero relations/declarations/signal constraints/registers, and 26 visuals. Current
shared authority removes two stale interfaces, false physical-property phase `recovery`, 12 generic phases, and
11 generic gates. Semantic actors 6 hold; final SemanticIR is zero interfaces/phases/gates / 27 invariants / four
contracts / one assertion / 42 decompositions / 70 timing constraints. Intent actors 6→4, behaviors 25→4, and
constraints 28→27 while two assumptions and all 70 timing constraints hold. The three-signal `channel.isf` is
removed; lowering blocks honestly on no declared interface signals and leaves only `adapter.json` plus its report.
Validation classifies this as the known physical-link/under-extracted-spec frontier: ten timing tables remain
typed while nine unknown tables, 26 unclassified visuals, and 12 partially structured normative statements stay
visible without fabricated topology. Two cascades plus validation reproduce eight downstream hashes; all 112
project-owned path values are relative and present; normalized retention is 91 files / 21360835 bytes. Nine
WIRE/I2C/SWD datasets, KG 156/156, and all 64 retained ISFs through real FSMGen strict pass.
- ID: `CORPUS-COVERAGE.2.44` · Status: `done` (`2026-08-10`, DATA/DOC) · Goal: re-ingest the
  23-page OpenCAPI 4.0 32 Gbps PHY Mechanical Specification
  (`opencapi_4_0_32gbps_phy_mech_spec_v10_17mar2021`) from the owner-authorized same-SSD host library with the
  current release, then rebuild and validate EvidenceIR→SemanticIR→IntentIR→ISF without fabricating
  hardware authority. This is the smallest of the 13 remaining real chip-spec candidates: 262 retained source
  elements and a 765080-byte input. Read-only authentication pins source `d15f07bb…61f67`, release
  `463a79e3…b06510`, and an exact seven-file / 653360-byte rollback chain before any generated mutation.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.44`

- [x] **REPRODUCE / MEASURE** — the exact seven-file / 653360-byte rollback is byte-verified on the repository
  volume; guarded CPU Docling holds 23 pages / 19 visuals / eight tables / 45 sections at 21% peak sampled memory.
- [x] **ROOT CAUSE (WHY + WHERE)** — source loss is diagram-label suppression on seven visual-bearing pages;
  downstream changes are the current shared ToC-enum, signal/interface, typed-phase, and generic phase/gate
  authority boundaries acting on a mixed-vintage retained chain. No new shared producer defect is present.
- [x] **ADDRESSED (verified)** — the faithful 212-element / 267-statement chain retains engineering prose and
  explicit residual visibility while removing the unsupported nine-port/one-enum adapter; lowering blocks on no
  declared signals and leaves only `adapter.json` plus its validation report.
- [x] **NO REGRESSION** — two validated cascades reproduce eight downstream hashes; nine WIRE/I2C/SWD datasets,
  KG 156/156, 65/65 FSMGen strict, full CI 1,791/five ignored, mdBook, six doctrines, 2,039-artifact / 355769-path
  census, and locality pass with zero repository-owned absolute paths.
- [x] **GENERICITY** — no code or OpenCAPI/vendor/document/token exception changed; all deltas come from current
  shared grammar, structure, typed evidence, and honest blocked-lowering rules.
- [x] **LOCKSTEP** — task/log, roadmap, live docs, mdBook, containment authority, resume pointer, and generated
  chain agree on 44 done / 12 remaining / 65 emitted ISFs; the exact rollback/task bundle is deleted only after
  this verified unit reaches its durable commit.

Live result (`2026-08-10`): the guarded CPU ingest holds 23 pages / 19 visuals / eight tables / 45 sections,
peaks at 21% sampled system memory used, and produces 212 source elements with high confidence / zero residuals.
Source 262→212 removes 52 old records and adds two clean replacements: all losses are diagram labels on seven
visual-bearing pages. Evidence 350→267 removes those labels plus 33 synthetic `CONTENTS` enum statements while
holding 21 normative statements, two conditional rules, zero relations/declarations/constraints/registers/timing,
and 19 visuals. Current shared authority removes five stale acronym/group interfaces, physical-property typed
phases `dynamic` / `static`, seven generic phases, and 13 generic gates. Semantic actors 4 and Intent actors 3
hold; final SemanticIR is zero interfaces/phases/gates / 45 invariants / three contracts / six assertions / 30
decompositions, and IntentIR is three behaviors / 45 constraints / two assumptions. The nine-signal/one-enum
`channel.isf` is removed; lowering blocks honestly on no declared interface signals and leaves only
`adapter.json` plus its report. Validation classifies this as the known physical-link/under-extracted-spec
frontier: eight unknown tables, 19 unclassified visuals, and 21 partially structured normative statements remain
visible without fabricated topology. Two cascades plus validation reproduce eight downstream hashes; all 84
project-owned page/layout/visual/caption path values are relative and present; normalized retention is 70 files /
18190643 bytes. Nine WIRE/I2C/SWD datasets, KG 156/156, and all 65 retained ISFs through real FSMGen strict pass.
<!-- corpus-task-source-region:program-and-latest-refreshes:end -->
