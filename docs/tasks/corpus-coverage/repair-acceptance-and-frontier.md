# CORPUS-COVERAGE — repair acceptance and frontier

- Part ID: `repair-acceptance-and-frontier`
- State: `legacy`

<!-- corpus-task-source-region:repair-acceptance-and-frontier:start -->
### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.34b.ii.a`

- [x] **REPRODUCE / MEASURE** — fresh USB4 ingest reproducibly leaves 51/51 page-sidecar image paths absolute and
  rooted at the renamed staging directory; the canonical chain was restored exactly before implementation.
- [x] **ROOT CAUSE (WHY + WHERE)** — the helper serializes runtime staging paths into per-page JSON; only the
  document-level metadata and in-memory summary had post-backend relocation, so directory rename preserved the
  stale JSON strings.
- [x] **ADDRESSED (verified)** — every saved-image sidecar is cross-checked with its summary record, contained
  below staging, and serialized repository-relative against the final destination before the staged swap; the
  no-image case remains `null`.
- [x] **NO REGRESSION** — four focused page tests, three existing backend-metadata tests, two full stub-ingest
  materialization tests, formatting, and warning-deny Clippy pass; malformed metadata preserves last-good data.
- [x] **GENERICITY** — the repair keys only on the typed page-artifact/sidecar contract and repository path
  containment; no USB4/document/vendor/page-count exception or generated-output edit exists.
- [x] **LOCKSTEP** — source comments, task result, durable fact, live docs, mdBook locality contract, and
  `MEMORY.md` agree; `.ii.b` owns the real USB4 rerun and complete corpus gates.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.34b.i`

- [x] **REPRODUCE / MEASURE** — authenticated the pre-recovery SWD downstream snapshot and measured the exact
  22→19 actor, 25→21 relation, 22→19 port, and 12→11 adapter-signal deltas.
- [x] **ROOT CAUSE (WHY + WHERE)** — the temporary command's artifact-root resolution selected the canonical
  repository cache; the content delta is the current generic dense-prose authority gate removing `LEVEL` and
  three phrase-shaped actors, not protocol-fact loss.
- [x] **ADDRESSED (verified)** — validated the promoted EvidenceIR and rebuilt SemanticIR, IntentIR, and adapter
  in order; two full replays reproduced all eight stage/report/adapter hashes exactly.
- [x] **NO REGRESSION** — all 29 SWD derivation facts, base relation 1/1, known base constraint miss 0/1,
  FSMGen strict, focused dense-prose tests, provider-free WIRE datasets, and KG 156/156 pass.
- [x] **GENERICITY** — no SWD token, document key, manual generated-artifact edit, relaxed validator, or
  alternate extractor entered the recovery; the promoted change follows current generic authority behavior.
- [x] **LOCKSTEP** — task result, SWD durable fact, live docs, mdBook, and `MEMORY.md` record the coherent current
  chain and hand off only the USB4 closing signoff to `.2.34b.ii`.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.34`

- [x] **REPRODUCE / MEASURE** — authenticate the source PDF and exact five-stage before bundle, run guarded CPU
  ingest plus the deterministic cascade, and record source/stage hashes, typed counts, deltas, and peak RAM.
- [x] **ROOT CAUSE (WHY + WHERE)** — classify every material delta against current generic extractors, including
  whether the retained `USB4` port is the repaired parenthetical bus-acronym authority class or remains grounded.
- [x] **ADDRESSED (verified)** — promote only complete successful stages; reconcile obsolete adapter outputs;
  validate the final artifacts and run FSMGen strict only when the adapter is honestly renderable.
- [x] **NO REGRESSION** — focused transfer checks, WIRE/I2C/SWD, KG 156/156, mdBook, doctrines, path/locality,
  rollback comparison/deletion, and broader Rust gates when any product code changes all pass.
- [x] **GENERICITY** — no USB4/vendor/document-key/token exception, manual generated-output edit, relaxed
  validator, LLM/VLM inference, or fabricated target model enters the slice.
- [x] **LOCKSTEP** — task row/changelog, corpus counts/frontier, live docs, durable fact if a new causal finding is
  established, mdBook behavior, and `MEMORY.md` agree; delete only the authenticated rollback after green gates.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.34a`

- [x] **REPRODUCE / MEASURE** — `realpath` plus filesystem identity proves the live host-library route is on the
  boot volume; SSD census finds no checkout/PDF; source hash is `ab337460…396`, boot checkout size is 935 MiB.
- [x] **ROOT CAUSE (WHY + WHERE)** — the repository-local symlink target still names the pre-move boot-volume
  checkout; this conflicts with the director's SSD-only project statement, not with SpecForge path resolution.
- [x] **ADDRESSED (verified)** — fail closed before `ingest`; no pipeline command, copy, link rewrite, or generated
  artifact mutation occurred. `.2.34b` owns execution after the source route is explicitly resolved.
- [x] **NO REGRESSION** — retained Source/Evidence/Semantic/Intent/adapter hashes and output files remain unchanged;
  `.2.34a` is a read-only path/provenance probe and the prior `.iv.c` product state remains intact.
- [x] **GENERICITY** — the stop depends only on volume identity, tracked locality doctrine, and explicit director
  state; it does not depend on USB4 contents, vendor vocabulary, or an extraction result.
- [x] **LOCKSTEP** — task split, dated durable fact, and `MEMORY.md` preserve the exact blocker and two safe
  resolutions: provide the SSD checkout route, or explicitly authorize one read-only copy to project-local input.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.33d.i`

- [x] **REPRODUCE / MEASURE** — preserved the exact 45-relation USB before signature and measured 97 weak
  declaration phrases, 17 bus-acronym width-one captures, and 32 port/pin-only classifications across all
  retained artifacts; `docs/research/dense-prose-signal-authority-measurement.md` carries the recheckable census.
- [x] **ROOT CAUSE (WHY + WHERE)** — located the three first unsupported promotions in formal-declaration
  parsing, parenthetical single-wire heads, and shared table authority; proved direction synthesis only
  amplifies them; verified nine maximum-tie docs and lexicographically-last live tie behavior.
- [x] **ADDRESSED (verified)** — `.d.ii` owns the three bounded catalog repairs and exact focused regressions;
  `.d.iii` is measured unnecessary subject to direct convergence + real-USB proof; current USB artifacts retain
  the exact reproducible before signature for `.d.iv`.
- [x] **NO REGRESSION** — the design explicitly retains I2C/I2S/SWD/SWP prose forms, genuine/rotated signal
  inventories, SWJ/connector pins, and requires focused fixtures plus WIRE-BASED-100 1.000, FSMGen strict,
  `kg-bench` 156/156, full CI, mdBook, doctrine, and locality gates in the code/signoff leaves.
- [x] **GENERICITY** — the selected rules use declaration predicates, single-wire category, and table identity
  structure; no USB/vendor/document key, candidate signal token, or expanding denylist enters the design.
- [x] **LOCKSTEP** — task, durable research/fact card, live ledgers/status, mdBook repair plan, and `MEMORY.md`
  agree; no production code or generated artifact changed in the probe.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.33d.ii`

- [x] **REPRODUCE / MEASURE** — `signal_declaration_catalog_utf8_boundary`,
  `bus_parenthetical_is_not_synthesized_as_a_single_wire`, `signal_table_authority_requires_inventory_structure`,
  and `weak_dense_prose_names_cannot_reenter_through_relations` reproduce all weak shapes and before-signature
  consumers while asserting the corrected result.
- [x] **ROOT CAUSE (WHY + WHERE)** — those tests directly exercise the declaration predicate, single-wire noun,
  and shared table-authority seams; `select_initiator_actor_picks_the_net_producer` pins the independent equal-key
  tie contract rather than treating the final adapter symptom as cause.
- [x] **ADDRESSED (verified)** — both declaration catalogs share the formal parser; the parenthetical and table
  authorities reject their weak families; `AT`/`USB`/`ENHANCED`/`NO` remain absent through catalog, relation,
  direction, and provenance convergence; initiator selection is behavior-identical and explicit.
- [x] **NO REGRESSION** — formal/Unicode/width/`inout`, I2C/I2S/SWD/SWP, rotated inventories, SWJ/headerless
  connectors, and field/status negatives pass. Full library is 1,779/1,779 with five ignored; APB/AHB/AXI WIRE
  constraint/relation/temporal and I2C declarations remain 1.000, fresh SWD is 1.000 at 1/1 and 11/11·4/4·13/13·1/1;
  KG 156/156, warning-deny Clippy, full CI, mdBook, doctrines, rolling-ledger retrieval, and locality pass.
- [x] **GENERICITY** — production rules depend only on declaration predicates, single-wire noun category, and
  compact table identity structure; no USB/vendor/document key, candidate signal token, or denylist is added.
- [x] **LOCKSTEP** — code comments, durable fact/research conclusions, task frontier, live docs, mdBook, and
  `MEMORY.md` describe the verified behavior and `.d.iii`/`.d.iv` handoff. The forced rationale-ledger rollover
  seals 27 exact records and the archive index warning is recorded below its 90% rollover point.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.33d.iii`

- [x] **REPRODUCE / MEASURE** — `git diff --exit-code 5c95a041..HEAD` proves both affected Rust files unchanged;
  five focused declaration/table/parenthetical/fixed-point/initiator tests pass individually.
- [x] **ROOT CAUSE (WHY + WHERE)** — `actor_signal_relation_surface` receives only cataloged names or relations
  from the same table-authority gate; `synthesize_directions_from_relations` copies those existing names. It
  amplifies prior authority but cannot originate a rejected name.
- [x] **ADDRESSED (verified)** — `.d.ii`'s combined EvidenceIR build fixture exercises all three weak USB shapes
  and proves `AT`/`USB`/`ENHANCED`/`NO` absent from known signals, relations, direction declarations, and table
  provenance. The conditional downstream backstop is therefore closed without code.
- [x] **NO REGRESSION** — no product/generated artifact changes; focused tests, exact code diff, doctrines, mdBook,
  full CI, and locality pass. `.d.iv` retains the independent real-cascade acceptance gate.
- [x] **GENERICITY** — refusing a duplicate adapter/convergence deny-filter preserves legitimate signals whose
  only direction evidence is a grounded driver relation and keeps authority at universal grammar/structure seams.
- [x] **LOCKSTEP** — task, research, durable fact, roadmap/status/resume, and mdBook agree that `.d.iii` is
  measured unnecessary and `.d.iv` is the active real USB proof.

### Parent exit criteria — `CORPUS-COVERAGE.2.33d.iv`

- [x] **REPRODUCE / MEASURE** — preserve the exact before cascade and show the repaired rebuild's stage hashes,
  counts, selected actor, false-name absence, and obsolete `.isf` residue before changing the writer.
- [x] **ROOT CAUSE (WHY + WHERE)** — prove whether the false model survives in current IR/output or only as an
  unreconciled sibling left by `AdapterArtifact::write_to_disk`.
- [x] **ADDRESSED (verified)** — a successful renderable write leaves exactly its current `.isf`; a successful
  blocked write leaves none; unrelated files survive; the real USB rerun removes the obsolete false-model file.
- [x] **NO REGRESSION** — USB is intentionally blocked, so FSMGen is not applicable; strict adapter regressions,
  WIRE/KG/full-CI/rustdoc/mdBook/doctrine/path/locality gates all pass.
- [x] **GENERICITY** — reconciliation is limited to obsolete regular/symlink `.isf` siblings in the resolved
  document artifact root; it contains no document key, actor name, signal name, or vendor exception.
- [x] **LOCKSTEP** — generated artifacts, task/result record, durable fact, live docs, mdBook, and resume pointer
  describe the same current cascade; verified rollback data and temporary test artifacts leave zero residue.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.33d.iv.a`

- [x] **REPRODUCE / MEASURE** — repaired USB adapt writes `channel.isf` while the prior false-actor `.isf` remains.
- [x] **ROOT CAUSE (WHY + WHERE)** — `AdapterArtifact::write_to_disk` writes only the current named paths and never
  reconciles obsolete generated siblings; the old file is residue, not current adapter content.
- [x] **ADDRESSED (verified)** — successful renderable and blocked writes reconcile regular/symlink `.isf` files
  while preserving unrelated files and directories; real USB rerun leaves only `adapter.json` + `channel.isf`.
- [x] **NO REGRESSION** — eight focused adapter tests, warning-deny Clippy, full CI (1,780 pass / five ignored),
  rustdoc, doctrines, mdBook examples/build, and locality pass;
  current emitted ISF text is unchanged by the reconciliation helper.
- [x] **GENERICITY** — reconciliation uses only extension, current manifest path, and leaf file type; no corpus fact.
- [x] **LOCKSTEP** — task, code analysis, change/development/status ledgers, Knowledge Map fact, mdBook, and memory
  agree that `.iv.a` closes stale-output convergence while `.iv.b` owns the newly proven semantic blocker.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.33d.iv.b`

- [x] **REPRODUCE / MEASURE** — recorded the exact retained-corpus authority-empty interface and downstream adapter
  impact, including the real USB 918-interface/2,940-record/556-output case and any relation-only exception.
- [x] **ROOT CAUSE (WHY + WHERE)** — proved the empty authoritative-name branch in
  `retain_authoritative_interface_candidate_signals` is the first unsupported promotion and distinguish it from
  formal declarations, system-contract infrastructure, and the already-repaired EvidenceIR signal loop.
- [x] **ADDRESSED (verified)** — heuristic grouping requires formal/system-contract authority or a signal-led
  deontic behavior statement; declaration-free `VALID`/`READY` remains, while real USB has zero interfaces and a
  blocked adapter whose successful write removes the obsolete `channel.isf`.
- [x] **NO REGRESSION** — four focused authority/behavior tests; real USB stages; APB/AHB/AXI/SWD byte-equivalent
  interfaces; WIRE/I2C/SWD 1.000; KG 156/156; warning-deny Clippy; full CI 1,783/five ignored; rustdoc, mdBook,
  all six doctrines, path, and locality pass. FSMGen is not applicable to intentionally blocked USB; its strict
  adapter regressions pass in the full suite.
- [x] **GENERICITY** — the repair depends on typed authority or generic statement position/deontic/action
  structure; no document key, vendor, signal-name list, denylist, or adapter-side symptom filter enters production.
- [x] **LOCKSTEP** — task/result record, code analysis, durable causal fact, corpus measurement, live docs, mdBook,
  generated USB artifacts, and `MEMORY.md` report the same verified behavior; `.iv.c` completed final signoff and
  verified-rollback deletion with zero residue.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.33d.iv.c`

- [x] **REPRODUCE / MEASURE** — authenticated 9 files / 38,981,655 bytes and release `946766cb…488`; rebuilt the
  preserved chain with stable Evidence/Semantic/Intent/adapter hashes `87c01ab…efc` / `16736201…6b3` /
  `564eba40…7af` / `28d664c0…8f2` before and after SourceIR validation backannotation
  `e87f5003…a13`→`38cbaa82…a2b`, and recorded every typed stage count.
- [x] **ROOT CAUSE (WHY + WHERE)** — `validate`/`jq` prove the final artifacts contain neither the original EvidenceIR
  catalog/relation loop nor the SemanticIR authority-empty loop, and distinguish intentionally residual source
  words and promotion-only constraint subjects from declared/emitted hardware names.
- [x] **ADDRESSED (verified)** — current USB has zero actor relations, interfaces, interface records, adapter
  signals/rules, phantom/stale `.isf` files, and emitted target; the blocked manifest leaves exactly `adapter.json`.
- [x] **NO REGRESSION** — focused authority/adapter tests, all nine WIRE/I2C/SWD datasets, KG 156/156,
  warning-deny Clippy, full CI (1,783 pass / five ignored), rustdoc, mdBook, all six doctrines, path checks, final
  locality, and rollback-residue census pass.
- [x] **GENERICITY** — final acceptance depends only on current universal extraction/writer behavior and exact
  artifact evidence; no USB/vendor/name exception, manual output edit, relaxed validator, or inferred model enters.
- [x] **LOCKSTEP** — closed `.iv`/`.d`/`.2.33` and aligned the corpus row, parent exit criteria, durable facts,
  roadmap/status/live docs, mdBook, and `MEMORY.md`; delete only the authenticated rollback after every product
  acceptance gate was green; its exact path and task-id residue are absent.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.33a`

- [x] **REPRODUCE / MEASURE** — captured PDF/source hashes and sizes, 548/507/283/6,584 prior structure,
  80/1/79/78 stage census, current binary identity, 81–82% free RAM, and a byte-identical SSD rollback.
- [x] **ROOT CAUSE (WHY + WHERE)** — proved the symlink route's fail-closed portability behavior separately
  from the EvidenceIR panic at `collect_known_signal_names`; its sibling direction collector repeats the bug.
- [x] **ADDRESSED (verified)** — `.2.33b` owns the shared code repair and `.2.33c` owns cascade resumption;
  the failed evidence command wrote no partial downstream artifact.
- [x] **NO REGRESSION** — SourceIR hash stayed unchanged after the rejected symlink launch; the authorized
  external retry promoted atomically with zero residuals; prior SourceIR remains recoverable from the exact copy.
- [x] **GENERICITY** — root cause is UTF-8 byte/character boundary handling, not USB wording or a token denylist.
- [x] **LOCKSTEP** — task, live ledgers, durable fact, and resume pointer name the blocker and exact next leaf.
  The resulting architecture-ledger update also performs its mandatory lossless rollover: four aged-out records
  retain exact content/order in segment 0001 (with canonical terminal-newline normalization), and the live view
  returns to 60 records.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.33b`

- [x] **REPRODUCE / MEASURE** — real pre-fix USB EvidenceIR deterministically panicked at the catalog seam; the
  direct regression preserves that exact multi-byte-prefix shape and passes on the fix.
- [x] **ROOT CAUSE (WHY + WHERE)** — both collectors shared one duplicated `idx - 2..idx` byte-slice defect.
- [x] **ADDRESSED (verified)** — one helper checks the prefix at a valid match boundary and both collectors use it.
- [x] **NO REGRESSION** — bullet and non-ASCII sentence-boundary tests, focused EvidenceIR tests, formatting,
  warning-deny Clippy, complete CI, and USB evidence retry pass.
- [x] **GENERICITY** — no document key, vendor, bullet codepoint, or signal name appears in production logic.
- [x] **LOCKSTEP** — code analysis, live docs, task, fact, and memory agree; no public CLI/schema contract changed,
  so the existing mdBook EvidenceIR contract remains current without a new behavior section. The triggered
  changelog rollover seals 29 whole aged-out records with exact content/order and leaves the declared 85+2 view.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.33c`

- [x] **REPRODUCE / MEASURE** — captured final Source/Evidence/Semantic/Intent/adapter identities, typed counts,
  the prior→fresh SourceIR element delta, and real FSMGen result.
- [x] **ROOT CAUSE (WHY + WHERE)** — classified SourceIR 6,584→5,830 at the visual-page seam and traced the USB
  false-signal loop through `docling_backend.rs:classify_table_kind`, `evidence.rs` declaration/relation/direction
  convergence, and `isf_ir.rs:select_initiator_actor`, distinguishing semantic block from cascade failure.
- [x] **ADDRESSED (verified)** — completed and validated every downstream stage; real FSMGen strict reports
  success/zero diagnostics, corpus row/counts are current, exact rollback is gone, and `.2.33d` owns the universal
  semantic-trust repair rather than misclassifying the syntactically valid adapter as faithful.
- [x] **NO REGRESSION** — unrelated artifacts are preserved; focused stage validation, FSMGen strict, and
  `kg-bench` 156/156 pass; doctrine/book/locality gates complete the commit workflow.
- [x] **GENERICITY** — used current universal extractors only; no USB/vendor fact, document-specific rule,
  LLM/VLM inference, relaxed validator, or hard-coded output was introduced.
- [x] **LOCKSTEP** — parent/child task status, `.2` row/count/frontier, live docs, mdBook corpus/trust truth, new
  Knowledge Map fact `dense-prose-false-signal-loop-reaches-isf`, and `MEMORY.md` agree before commit.

- ID: `CORPUS-COVERAGE.3` · Status: `done` (`2026-08-09`, DOC/AUDIT) · Goal: reconcile the `.2`
  frontier with the later artifact-cleanup lifecycle after a live census found only the freshly promoted SWD
  normalized bundle, while `.2` still claimed 32 restored bundles and only 24 normalized-missing documents.
  Root-cause the discrepancy from durable evidence, distinguish completed current-binary EvidenceIR refreshes
  from currently retained normalized/rebuildable bundles, correct every live surface that uses the stale count,
  add a durable Knowledge Map fact, and add a mechanical currentness check if the count is meant to stay live.
  Do not re-ingest or delete any artifact in this audit leaf.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.3`

- [x] **REPRODUCE / MEASURE** — live census is 80 SourceIR / 1 normalized / 79 EvidenceIR / 78
  SemanticIR/IntentIR/adapters; the 32 logged refresh keys retain all five stages (160/160 files).
- [x] **ROOT CAUSE (WHY + WHERE)** — commit `70534fe0` records that the `2026-07-05` sweep intentionally
  reclaimed every normalized bundle; retained stage membership proves cleanup did not erase refreshed facts.
- [x] **ADDRESSED (verified)** — task frontier and live status distinguish completed current-binary refreshes
  from current cache retention; `corpus-refresh-completion-vs-normalized-retention` owns the causal fact.
- [x] **NO REGRESSION** — no generated artifact changed; Knowledge Map/fact catalog, mdBook doctests/build, and
  project-data locality pass. The complete doctrine driver is the pre-commit gate.
- [x] **GENERICITY** — accounting is stage/retention based, with no document/vendor allowlist or workstation
  path in the rule. No new currentness gate forces intentional caches to remain.
- [x] **LOCKSTEP** — roadmap and task index required no status/count change; task, live ledgers, mdBook,
  Knowledge Map, book aggregate authority, and resume pointer agree on the corrected frontier.
- Frontier: task evidence reaches 277636/278528 bytes after `.2.48`; from its clean commit, open the smallest
  owning containment leaf and partition this canonical tree before selecting refresh #49.
  Historical `.2` phase context follows: re-ingest the 57-document cohort from the
  `.cache/local-references/chipdoc` symlink, register/TRM/ISA phase, one doc per slice (**39 refreshes done after #39;
  17 real chip-spec docs remain unrefreshed by `.2`** — see the `.2` log table below for #29–#39: #29/#31 CHI-C2C marquee message-field refreshes,
  #30 RISC-V AIA restoration, #32 JEDEC HBM-gen1 `jesd235` which the re-ingest revealed is a **6-page legal-exhibit cover, not
  the real standard** → honest source-driven thin yield. The remaining tail is overwhelmingly thin/degraded — OpenCAPI×13
  PHY/mech/TL + USB3.2/USB4 guides + CoreSight/debug guides — so the high-value substantive work is now shifting to the
  surfaced upstream extraction-precision levers, not more bundle-restorations. The most recent register-class refresh, #28
  JEDEC HBM2 — a MIXED refresh that **CONFIRMS the #27 phantom explosion is PROSE-SPECIFIC**: HBM2 (register/timing-table
  DRAM spec) CONSOLIDATED actors 52→38 like the AMBA/CoreSight class, with transactions 0→3, BUT its `.isf` strict-FAILS on a
  generic-`TABLE` mega-enum with binary values emitted as bare decimal tokens → NEW spun-out lever F. #27 eMMC was the FIRST
  descriptive-prose spec — genuine transactions 0→6 BUT a phantom-actor explosion 20→153 → lever E; #26 Intel VT-d CORRECTED
  the #22 "already current" survey; #25 completed the CoreSight SoC-600 cluster).
  **Measured-corrected expectation (`2026-06-23`, #22–#23, refined #26/#27/#28):** a one-pass survey of
  the remaining docs' persisted evidence shows the register-shaped surfaces are already present (CoreSight SoC-600
  ×3 carry 597/631/833 `register_records`, AMD-IOMMU `48882` 217 `message_field_records`, Intel VT-d 103 regs) — the
  `.10`/message-field families predate them — so the **marquee table-family jump phase is over**. BUT "already
  current-binary-equivalent" is NOT uniform: a doc's stale evidence sits at whatever binary last rebuilt it, so
  re-ingest can still deliver a real KG refresh when that binary predated a LATER gate — #22 Cortex-A76 was fully
  current (byte-identical) but #23 CoreSight SoC-600 predated the `.1a`/`.1b` agent-identity consolidation + the
  section-heading transaction recognizer and got a genuine actors 60→47 / relations 64→41 / interfaces 20→8
  consolidation + transactions 0→2 (the #17 Avalon class). So the remaining tail is a MIX of pure confirmation
  (#21/#22) and consolidation/recognition refreshes (#17/#23) + bundle restoration. Next candidates (by descending
  `.isf` richness): JEDEC eMMC/HBM (`JESD84-B50`/`JESD235*`), RISC-V AIA (`1_0_2025_03_12_…advanced_interrupt`),
  the USB4/USB3.2 specs, then the thin OpenCAPI×13 PHY/mechanical/TL tail (honest-absence / Lever-D confirmation;
  the project `readme` non-spec doc is correctly skipped — it yields zero protocol surfaces). A genuinely pre-`.10`-stale
  doc with a marquee jump remains possible but is now the exception; **#26 VT-d shows even a register-current doc can
  carry a real message-field + transaction refresh, so the one-pass register survey under-counts the refresh value.**

<!-- corpus-task-source-region:repair-acceptance-and-frontier:end -->
