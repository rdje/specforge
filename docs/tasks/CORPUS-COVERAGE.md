# CORPUS-COVERAGE: build every ingested doc through to IntentIR/.isf + keep downstream stages non-stale

## Metadata

- Tree ID: `CORPUS-COVERAGE`
- Status: `active` (`.0` build-out + `.1` stage-staleness validator done; `.2` current-binary refresh batch
  remains active with 37 completed documents and `.2.38` Introducing CoreSight refresh owned next; `.3`
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
- ID: `CORPUS-COVERAGE.2.33` · Status: `done` (`2026-08-09`, DATA/CODE/DOC) · Children: `.2.33a`
  probe (done), `.2.33b` UTF-8 boundary fix (done), `.2.33c` cascade/signoff (done), `.2.33d`
  dense-prose adapter trust repair (done). Goal/result: re-ingest USB 3.2
  (`usb_3_2_revision_1_0_2017_09`) from the owner-authorized external PDF resolved through the repository host-library route with the current release
  binary and CPU-only Docling, then build/validate EvidenceIR→SemanticIR→IntentIR→ISF. The completed chain retains
  548 pages / 507 visuals / 283 tables / 5,830 source elements and blocks honestly at the adapter after two
  universal signal-authority repairs. No `.isf` is emitted; every typed stage remains canonical and the two exact
  same-volume rollback sets were deleted only after their owning gates passed.
- ID: `CORPUS-COVERAGE.2.33a` · Status: `done` (`2026-08-09`, PROBE/DOC) · Goal: reproduce and root-cause
  the first current USB 3.2 cascade blocker before changing code. The repository-relative symlink path fails
  closed before Docling because the portability boundary forbids repository paths that escape through symlinks;
  retrying with the resolved, explicitly external PDF is the correct authorized-input route and completed a
  guarded CPU ingest (548 pages / 507 visual assets / 283 tables / 5,830 elements / zero residuals, 81–82% RAM
  free). Evidence construction then panics on `- U+F0B7 signal integrity ...`: both signal-declaration catalog
  collectors use the byte offset returned by `match_indices("signal ")` but inspect the preceding characters with
  `lowered[idx - 2..idx]`; `idx - 2` lands inside the three-byte private-use bullet. No EvidenceIR was written,
  the promoted SourceIR is intact, and the 4.9 MiB same-volume pre-ingest rollback remains byte-verified.
- ID: `CORPUS-COVERAGE.2.33b` · Status: `done` (`2026-08-09`, CODE/DOC) · Goal/result: replaced the duplicated
  unsafe preceding-byte slice with one valid-prefix sentence-boundary helper shared by both declaration catalogs.
  A direct two-catalog regression proves multi-byte-marker prose neither panics nor declares `INTEGRITY`, while a
  real `PREADY` input declaration after non-ASCII prose remains visible. Rebuilt release `1710e5…` constructs and
  validates USB EvidenceIR from the promoted SourceIR without re-ingest: 918 anchors / 8,267 spans / 507 visuals /
  22,029 links / 8,412 statements. Focused EvidenceIR passes 252 tests / five ignored; full CI passes 1,776 tests /
  five ignored plus all doctrines, Clippy, rustdoc, and mdBook.
- ID: `CORPUS-COVERAGE.2.33c` · Status: `done` (`2026-08-09`, DATA/DOC) · Result: completed and validated the
  already-promoted USB SourceIR→EvidenceIR→SemanticIR→IntentIR→adapter chain without a second Docling ingest.
  SourceIR 6,584→5,830 elements is visual-label suppression (all 53 loss pages have visuals; changed-page short
  fragments ≤10 chars 543→3; pages/visuals/tables hold 548/507/283), not prose loss. FSMGen accepts the emitted
  four-port/29-rule ISF with zero diagnostics, but the model is semantically untrustworthy: four weak false-signal
  seeds reinforce through relation-derived directions and a phantom request actor wins initiator selection. The
  exact one-file/4.9 MiB rollback was removed only after comparison; residue is zero. The repair is handed to
  `.2.33d` rather than treating strict syntax as faithful output.
- ID: `CORPUS-COVERAGE.2.33d` · Status: `done` (`2026-08-09`, PROBE/CODE/DOC) · Children: `.d.i`
  universal boundary probe (done), `.d.ii` grounded signal-authority repair (done), `.d.iii`
  convergence/adapter trust backstop (done), `.d.iv` real-cascade closure (done: `.iv.a` stale-output
  convergence, `.iv.b` authority-empty semantic-interface repair, `.iv.c` final signoff).
  Goal: repair the
  universal dense-prose false-signal feedback path before USB 3.2 is accepted as a trustworthy ISF result.
  Measurement must preserve all four independently observed seeds: an ordinary `signal at ...` clause admitted
  `AT`, a parenthetical non-wire bus acronym admitted `USB`, a VBUS requirements matrix misclassified as a signal
  table admitted `ENHANCED`/`NO`, and relation-derived directions promoted those candidates into formal signals.
  The repair must be grammar/structure-based (no USB tokens, vendor/doc keys, or expanding denylist), prevent a
  phantom prose actor from winning `select_initiator_actor`, distinguish semantic trust from FSMGen syntax, retain
  the known WIRE/FSMGen/KG corpus wins, and add a real USB regression plus the focused and broad gates warranted by
  every changed extraction seam.

- ID: `CORPUS-COVERAGE.2.33d.i` · Status: `done` (`2026-08-09`, PROBE/DOC) · Goal: measure the
  four USB false-signal seed families across all retained SourceIR/EvidenceIR/IntentIR artifacts before changing
  code; locate the first source-grounded authority boundary each lacks; quantify every candidate production and
  consumer path; verify initiator tie behavior; and choose the smallest universal grammar/structure repair that
  removes false hardware without deleting real WIRE/serial signals. **Done:** the 80-EvidenceIR census found
  3,104 formal and 97 weak sentence-start `signal <word>` matches (38 weak doc/token pairs; 15 relation-active /
  120 relations; 11 direction-promoted), plus 17 `Bus (ACRONYM)` width-one declarations across six docs
  (14 relation-active / 135 relations / 12 promotions). Of 32 port/pin-only table classifications, 11 pass the
  current authority gate and yield 32 raw row candidates. Runtime direction synthesis is closed over those
  upstream catalogs and does not independently invent names, so `.d.iii` is measured unnecessary if direct
  convergence and real-USB regressions prove closure. Selected `.d.ii`: require a formal declaration predicate,
  remove `bus` from the single-wire head grammar, and require compact inventory structure for ordinary
  port/pin tables. Preserve I2C/I2S/SWD/SWP, genuine/rotated signal tables, and connector pins. Nine of 79
  IntentIR docs have maximum producer ties; live behavior chooses the lexicographically last equal maximum, so
  `.d.ii` also owns a comment correction and tie test without changing selection behavior. Durable report:
  `docs/research/dense-prose-signal-authority-measurement.md`.
- ID: `CORPUS-COVERAGE.2.33d.ii` · Status: `done` (`2026-08-09`, CODE/DOC) · Goal: implement only the
  `.d.i`-selected universal signal-authority repair: require a formal predicate in the known-signal catalog,
  remove `bus` from the parenthetical width-one grammar, and require compact identity structure before ordinary
  port/pin-only tables receive signal-inventory authority. Add exact regressions for every rejected USB seam,
  direct convergence absence for all four names, and positive preservation for formal declarations,
  I2C/I2S/SWD/SWP prose signals, genuine/rotated inventories, SWJ routing, and headerless connectors. Preserve
  current initiator selection while correcting its tie-order comment and pinning equal-key behavior in a test.
  No document/vendor/token denylist and no downstream adapter heuristic are in scope. **Done:** both catalogs
  share the canonical predicate parser (including the broad-gate-preserved `inout` arm); `bus` is excluded from
  the width-one fallback; and the table gate requires explicit signal caption, compact identity header, or a
  headerless connector-pin diagram. Direct convergence rejects all four weak USB names across declaration,
  relation, direction, and provenance surfaces. Initiator selection remains lexicographically last on exact
  ties. Focused serial/table suites, full library 1,779/1,779, WIRE golds plus a fresh SWD rebuild, KG 156/156,
  Clippy, full CI, mdBook, doctrines, and locality pass. Frontier after commit → `.d.iii`.
- ID: `CORPUS-COVERAGE.2.33d.iii` · Status: `done` (`2026-08-09`, MEASURE/DOC; no code) · Result: closed the
  conditional backstop as measured unnecessary. The affected EvidenceIR/ISF files are byte-identical to `.d.ii`
  commit `5c95a041`; runtime relations consume only the formal/table signal authorities, and direction synthesis
  copies only an existing relation name. The combined fixed-point regression rejects `AT`/`USB`/`ENHANCED`/`NO`
  across catalog, relation, direction, and provenance. Five focused authority/tie tests pass; a second downstream
  filter would duplicate policy and risk false negatives. Commit:
  `CORPUS-COVERAGE.2.33d.iii — close redundant convergence backstop`.
- ID: `CORPUS-COVERAGE.2.33d.iv` · Status: `done` (`2026-08-09`, CODE/DATA/DOC) · Goal/result: rebuild the real
  USB EvidenceIR→adapter chain with the repaired release binary, prove the false ports/actor/rules no longer
  emit, and make a successful adapter write converge the document directory by removing obsolete sibling `.isf`
  files while preserving unrelated files. Run FSMGen/WIRE/KG/full-CI/path/locality gates, align the mdBook and
  durable facts, and close `.2.33d`/`.2.33`. **Live finding:** the repaired rebuild emitted `channel.isf`, but the
  prior `setportfeature_port_over_current.isf` remained because `AdapterArtifact::write_to_disk` only overwrites
  named current outputs; this is the previously measured corpus-wide stale-cruft mechanism, now reproduced at the
  exact semantic-trust boundary rather than accepted as manual cache hygiene. Writer reconciliation removes the
  stale file, and `.iv.b` closes the distinct authority-empty fallback: heuristic grouping requires a
  formal/system-contract signal or a signal-led deontic behavior statement. The real USB rebuild now has zero interfaces/signals/rules, blocks honestly,
  and leaves only `adapter.json`; `.iv.c` repeats the complete cascade/gates and removes the authenticated
  rollback with zero residue.
- ID: `CORPUS-COVERAGE.2.33d.iv.a` · Status: `done` (`2026-08-09`, CODE/DOC) · Result: successful adapter
  writes now retain the manifest-selected `.isf`, remove obsolete regular/symlink `.isf` siblings, preserve
  unrelated files/directories, and remove the last emitted target on a renderable→blocked transition. The real
  USB rerun converges to `adapter.json` + `channel.isf`; no extraction or rendered-source policy changed.
- ID: `CORPUS-COVERAGE.2.33d.iv.b` · Status: `done` (`2026-08-09`, PROBE/CODE/DOC) · Result: 21 retained
  authority-empty documents carried 5,527 interfaces / 18,397 all-low records and fed 4,060 adapter signals;
  19 were marked renderable. The sole relation-only case was DTI `TBU reads DOWNSTREAM`, ordinary direction
  prose rather than a declared wire. The empty-set allow-all is replaced by a signal-led deontic behavior
  grammar, preserving declaration-free `VALID`/`READY` while all 21 affected documents still dry-run to zero
  interfaces. APB/AHB/AXI/SWD surfaces are byte-equivalent. Real USB is blocked at zero signals/rules and its
  successful write removes `channel.isf`. Four focused authority/behavior tests, WIRE 1.000, KG 156/156, and
  full CI (1,783 pass / five ignored) are green. No document/token denylist or adapter symptom filter was added.
- ID: `CORPUS-COVERAGE.2.33d.iv.c` · Status: `done` (`2026-08-09`, DATA/DOC) · Result: authenticated the
  nine-file / 38,981,655-byte same-volume rollback, then reproduced the exact current USB chain with release
  `946766cb…488`: EvidenceIR/SemanticIR/IntentIR/adapter stayed `87c01ab…efc` / `16736201…6b3` /
  `564eba40…7af` / `28d664c0…8f2`. Required validation backannotated SourceIR from `e87f5003…a13` to
  `38cbaa82…a2b`; a final cascade reproduced the same four downstream hashes. EvidenceIR has 8,267 statements, zero
  actor relations, ten signal constraints, 542 conditional rules, and 102 timing constraints. SemanticIR and
  IntentIR have zero interfaces, interface records, actor ports, and relations; the adapter is blocked on no
  declared interface signals with zero signals/transactions/rules, one storage record, and 15 residuals. Only
  `adapter.json` remains. Focused tests, all nine WIRE datasets, KG 156/156, warnings-denied full CI (1,783 pass /
  five ignored), rustdoc, mdBook, six doctrines, path/locality, and residue gates pass. The authenticated rollback
  was then deleted at its exact path and a task-id census found no residue. `.iv`/`.d`/`.2.33` are closed.
- ID: `CORPUS-COVERAGE.2.34` · Status: `done` (`2026-08-09`, DATA/DOC) · Children: `.2.34a`
  source-route/locality probe (done), `.2.34b` guarded ingest/cascade (done). Goal: current-binary,
  CPU-only guarded re-ingest of `usb4_inter_domain_service_specification_v2_0_2025_11` from the owner-authorized
  host-library route, followed by the complete EvidenceIR→SemanticIR→IntentIR→adapter cascade. The retained
  pre-refresh chain is a high-value transfer check for #33's universal repair: SourceIR has 615 elements / 82
  visuals / 49 tables; EvidenceIR has 1,260 statements / 1 actor relation / 0 signal constraints / 27 conditional
  rules; IntentIR has 11 actors / 1 interface / 1 relation / 197 behaviors / 287 constraints / 0 transactions /
  6 registers. Its renderable `channel.isf` carries one `USB4` signal, two rules, eight enums, and six storage
  records. Preserve an exact same-volume rollback before ingest, authenticate the promoted source/cascade, and
  determine whether current grounded authority removes that bus-acronym port without a document exception.
- ID: `CORPUS-COVERAGE.2.34a` · Status: `done` (`2026-08-09`, PROBE/DOC; no generated mutation) · Result:
  authenticated the retained five-stage baseline and selected source, then stopped before ingest on a locality
  contradiction. `.cache/local-references/chipdoc` still resolves to the Git checkout under the user boot volume;
  filesystem identities differ from the SSD repository, while the director stated that every Git project moved
  to the 4T SSD and only shared Rust stores remain on boot. A bounded census found neither a `chipdoc` checkout nor
  the exact USB4 PDF in the external SSD project directory; the boot checkout is 935 MiB and the selected PDF is SHA-256
  `ab337460641c1f012a78c63dcf8cf182bf90deb29a7a841726973be79dcd7396`. Retained
  Source/Evidence/Semantic/Intent/adapter/ISF hashes are `a83108e3…ded` / `0d0c0659…9e4` / `4deb2481…ebd` /
  `3ca429f3…ac4` / `c363529c…80b` / `e9837e21…e4b`; the pre-existing Evidence validation report is
  `cec7b91f…151`. No pipeline command, copy, symlink rewrite, or generated-artifact mutation occurred. `.2.34b`
  requires the intended SSD checkout path or explicit authorization for a one-time read-only source copy into
  repository-local project data.
- ID: `CORPUS-COVERAGE.2.34b` · Status: `done` (`2026-08-09`, DATA/DOC) · Children: `.2.34b.i`
  SWD signoff-side-effect recovery (done), `.2.34b.ii` USB4 closing signoff (done). Goal: after the
  source route is resolved, preserve the exact baseline and execute the `.2.34` guarded ingest/cascade acceptance
  contract without cross-volume project data or an undocumented boot-volume dependency.
- ID: `CORPUS-COVERAGE.2.34b.i` · Status: `done` (`2026-08-09`, DATA/DOC; recovery) · Result: the attempted
  temporary fresh-SWD oracle rebuild resolved its artifact root to the canonical repository cache, so the
  untouched downstream chain was preserved before validation and a coherent EvidenceIR→adapter cascade was
  rebuilt from the promoted evidence. Current dense-prose authority removes false actor fragments `Class x`,
  `DbgSwEnable flag`, and `system`, the synthetic `LEVEL` signal, and four associated relations: actors 22→19,
  relations 25→21, ports 22→19, and adapter signals 12→11. All 29 serial facts remain exact at
  11 frame / 4 operation / 13 state / 1 interface-edge records; SWD derivation is 1.000 on all four surfaces,
  the base relation remains 1/1, and the known constraint-only `CSYSPWRUPACK` Pattern miss remains 0/1. The
  final Evidence/Semantic/Intent/adapter hashes are `e125616e…df9` / `0a9d75b5…a82` / `dd32a0a7…2d5` /
  `5dbf105c…b5b`; two complete cascade replays reproduced them byte-for-byte. FSMGen strict is clean,
  `kg-bench` is 156/156, focused dense-prose tests and the provider-free WIRE battery pass, and only the exact
  authenticated recovery snapshot is eligible for deletion. No production code changed. Commit:
  `CORPUS-COVERAGE.2.34b.i — recover coherent canonical SWD cascade`.
- ID: `CORPUS-COVERAGE.2.34b.ii` · Status: `done` (`2026-08-09`, CODE/DATA/DOC) · Children: `.ii.a`
  page-sidecar path portability (done), `.ii.b` USB4 closing signoff (done). Goal: finish USB4 rollback
  comparison, all acceptance gates, documentation, and cleanup after `.2.34b.i` restores a coherent corpus state.
- ID: `CORPUS-COVERAGE.2.34b.ii.a` · Status: `done` (`2026-08-09`, CODE) · Result: the fresh USB4 ingest exposed
  51 `normalized/pages/page-*.json` sidecars whose `rendered_image.path` values retained absolute
  `normalized.staging` paths. `DoclingBackendSummary::relocate_paths` repairs only the summary returned in memory,
  while the existing staged rewrite covers only the document-level `.meta.json`; neither touched page sidecars
  already written by the helper. `materialize_pdf` now rewrites every page sidecar before deleting the last-good
  bundle: saved-image paths must agree with the summary, remain canonically below staging with no traversal or
  symlink escape, and persist against the final `normalized/` destination; intentionally unpersisted images keep
  `null`. Any missing/malformed/mismatched/escaping record fails with `InvalidBackendOutput` and removes only
  staging. Four focused page tests, three document-metadata tests, two stub-ingest integration tests, formatting,
  and warning-deny Clippy pass. Commit: `CORPUS-COVERAGE.2.34b.ii.a — normalize Docling page-sidecar paths`.
- ID: `CORPUS-COVERAGE.2.34b.ii.b` · Status: `done` (`2026-08-09`, DATA/DOC) · Result: rebuilt release and ran
  two guarded CPU ingests from the director-supplied same-SSD checkout; the monitored replay completed at 82%
  minimum system memory free (18% peak used). Both produced 51 pages / 82 visuals / 49 tables / 603 elements.
  All 51 page sidecars now name the final repository-relative PNG, with zero staging/repository-absolute residue;
  compared with the preserved defective run, exactly those 51 JSON files differ and deleting
  `rendered_image.path` makes every pair equal. SourceIR plus its validation report reproduce hashes
  `eddccfb7…eb0` / `b78e76a2…f96`; every downstream artifact is byte-identical to the preserved fresh chain:
  EvidenceIR/report `111f4dc3…398` / `5bc59e80…253`, SemanticIR/report `422f70f5…a1f` /
  `df3bb7ae…8c7`, IntentIR/report `144cd981…2a5` / `9ae4c87d…ce4`, adapter `1b16b30b…ded`.
  The stale one-signal/two-rule USB4 model closes generically: relations/interfaces/ports/connectivity all 1→0,
  adapter signals 1→0, rules 2→0, enums 8→0, storage holds at six, and lowering blocks on
  `no signals declared in interface`; exactly `adapter.json` remains. Two `USB4` conditional consequents retain
  source evidence without becoming interface authority. All nine provider-free WIRE/I2C/SWD datasets are 1.000
  at their declared gates (including the documented promotion-only SWD constraint miss), KG is 156/156, all 69
  remaining emitted ISFs are strict-clean, full CI/book/doctrine/locality pass, and only then were the exact
  seven-file rollback and 198-file defective-chain evidence deleted; the exact task-id residue census is zero. Commit:
  `CORPUS-COVERAGE.2.34b.ii.b — sign off portable USB4 refresh`.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.34b.ii.b`

- [x] **REPRODUCE / MEASURE** — authenticated the seven-file baseline, source PDF `ab337460…396`, 198-file
  defective fresh chain, two guarded 51-page ingests, nine final artifact hashes, typed deltas, and 18% peak
  system memory use.
- [x] **ROOT CAUSE (WHY + WHERE)** — the real rerun confirms `.ii.a` was the only locality delta: all 51 sidecar
  pairs are content-equal after deleting `rendered_image.path`; every downstream IR/report/adapter is byte-equal.
- [x] **ADDRESSED (verified)** — 51/51 page image paths are repository-relative and final-rooted with zero
  staging/absolute residue; USB4 now blocks honestly at zero interfaces/signals/rules and leaves no `.isf`.
- [x] **NO REGRESSION** — both cascades reproduce every hash; nine WIRE/I2C/SWD datasets hold their 1.000 gates,
  KG is 156/156, 69/69 emitted ISFs pass FSMGen strict, and full CI/book/doctrine/locality gates pass.
- [x] **GENERICITY** — the result uses only current generic extraction, authority, path, and output-convergence
  rules; no USB4/vendor/document/token exception, generated-output edit, relaxed validator, or inference ran.
- [x] **LOCKSTEP** — generated artifacts, task/count/frontier, live docs, two current/superseded fact cards,
  mdBook, and `MEMORY.md` agree; authenticated task evidence is deleted only after every final gate is green.

- ID: `CORPUS-COVERAGE.2.35` · Status: `done` (`2026-08-09`, DATA/DOC) · Result: authenticated the exact
  seven-file retained chain and same-volume source PDF (`09f44419…d42`), then ran a guarded CPU ingest from the
  resolved caller-authorized SSD input. The 96-page / 46-visual / 23-table result has 1,313 elements, zero
  residuals, 96/96 final repository-relative page-sidecar paths, and 28% peak system memory used. The 1,509→1,313
  SourceIR delta is exactly body text 639→443: 197 fragments disappear across 12 visual pages, one clean body
  record is gained, and list/caption/footnote/header plus page/visual/table/section counts hold. Evidence spans
  1,617→1,421, statements 1,680→1,421, links 524→619, constraints 2→6, conditionals 19 held, relations 13→0,
  and protocol states 0→2. Semantic/Intent actors 21→12; interfaces 4→0, ports 11→0, relations 13→0, and
  connectivity 3→0; Intent transactions 0→2 are recognition-only. Four exact `USB4` EvidenceIR references
  survive without interface authority. The stale three-signal/three-rule/one-enum `device_also.isf` is removed;
  the adapter blocks on `no signals declared in interface` with four residuals and exactly `adapter.json`.
  All nine final artifact hashes reproduce across two downstream cascades. Nine WIRE/I2C/SWD gates, KG 156/156,
  68/68 current emitted ISFs strict, mdBook, doctrine, and locality gates pass; only then were the exact rollback
  and first-run replay deleted, with zero task-id residue.
  Commit: `CORPUS-COVERAGE.2.35 — sign off USB4 Connection Manager refresh`.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.35`

- [x] **REPRODUCE / MEASURE** — source, seven-file baseline, release binary, same-volume rollback, 96-page ingest,
  28% peak use, nine final hashes, typed deltas, and byte-identical downstream replay are authenticated.
- [x] **ROOT CAUSE (WHY + WHERE)** — the SourceIR delta is isolated to current visual-text cleanup, while generic
  relation/declaration authority removes stale `SB`/`USB`/`USB4` interfaces but retains grounded source facts.
- [x] **ADDRESSED (verified)** — only complete validated stages are promoted; 96/96 page paths are final
  repository-relative with no staging/absolute residue; converge obsolete adapter targets; run FSMGen strict only
  if the resulting adapter is honestly renderable.
- [x] **NO REGRESSION** — the replay is byte-identical; nine provider-free WIRE/I2C/SWD gates hold, KG is 156/156,
  all 68 current emits pass FSMGen strict, and mdBook/doctrine/path/locality gates pass; no Rust code changed.
- [x] **GENERICITY** — no USB4/vendor/document/token exception, manual generated-output edit, relaxed validator,
  fabricated model, or LLM/VLM inference enters the slice.
- [x] **LOCKSTEP** — generated artifacts, #35 row/count/frontier, live docs, durable fact, mdBook,
  and `MEMORY.md` agree; delete only authenticated rollback/task evidence after every final gate is green.

- ID: `CORPUS-COVERAGE.2.36` · Status: `done` (`2026-08-09`, DATA/DOC) · Result: authenticated the exact
  seven-file retained chain, source PDF `c4a5f342…66d`, release binary, and byte-identical same-volume rollback,
  then ran a guarded CPU ingest from the resolved caller-authorized SSD input. The 29-page / 19-visual / 12-table /
  404-element SourceIR has zero residuals, 29/29 final repository-relative page paths, and only 17% peak system
  memory use. Its structure is unchanged from the retained SourceIR; the material provenance change replaces the
  obsolete boot-volume input with the SSD source. Evidence counts also hold exactly at 55 anchors / 464 spans /
  19 visuals / 15 links / 464 statements / seven conditional rules, with zero relations or declared signals.
  Current generic authority therefore removes 88 stale heuristic SemanticIR/IntentIR interfaces while preserving
  eight semantic actors, five intent actors, 46 behaviors, 106 invariants, and seven conditional rules. Intent
  constraints become 124→106 and the stale residual disappears. The 100-signal/two-rule `agent.isf` was an
  authority-empty fabrication: lowering now blocks on `no signals declared in interface`, removes the obsolete
  target, and leaves exactly `adapter.json` with zero residuals. Validation also reports the self-declared
  architecture as under-extracted rather than a true guide: 19 visual assets remain unenriched, 87 normative
  statements remain partially structured, and seven temporal-source rules lack typed grounding. That is the
  already-owned cat-3 capture-recall frontier, not evidence to fabricate a target or change code in this refresh.
  Two complete deterministic cascades reproduce all nine final hashes. Nine provider-free WIRE/I2C/SWD gates,
  KG 156/156, all 67 current emitted ISFs through FSMGen strict, mdBook, doctrine, persisted-path, and locality
  gates pass; only then were the exact rollback/replay sets deleted with zero task-id residue.
  Commit: `CORPUS-COVERAGE.2.36 — sign off CoreSight base-system refresh`.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.36`

- [x] **REPRODUCE / MEASURE** — authenticate the source PDF, complete retained five-stage chain, stale 88-interface/
  100-signal adapter surfaces, release binary, and same-volume rollback; record guarded-ingest counts, peak memory,
  final hashes, and typed before→after deltas.
- [x] **ROOT CAUSE (WHY + WHERE)** — classify every material delta against current generic extraction, cat-3
  topology-capture, and authority-empty interface rules rather than treating strict syntax as semantic trust.
- [x] **ADDRESSED (verified)** — promote only complete validated stages; require every page path final and
  repository-relative; converge obsolete adapter targets; run FSMGen strict only if honestly renderable.
- [x] **NO REGRESSION** — reproduce the deterministic cascade, hold nine provider-free WIRE/I2C/SWD gates and KG
  156/156, sweep all current emitted ISFs through FSMGen strict, and pass mdBook/doctrine/path/locality gates plus
  broader Rust CI if any product code changes.
- [x] **GENERICITY** — no CoreSight/vendor/document/token exception, manual generated-output edit, relaxed
  validator, fabricated topology, or LLM/VLM inference enters the slice.
- [x] **LOCKSTEP** — generated artifacts, #36 row/count/frontier, live docs, durable fact when warranted, mdBook,
  and `MEMORY.md` agree; delete only authenticated rollback/task evidence after every final gate is green.

- ID: `CORPUS-COVERAGE.2.37` · Status: `done` (`2026-08-09`, DATA/DOC) · Result: authenticated the exact
  seven-file retained chain, source PDF `303504a8…379`, release binary `6e8add7b…7ec`, and byte-identical
  same-volume rollback before running a guarded CPU ingest from the caller-authorized SSD input. The current
  SourceIR has 25 pages / 11 visuals / two tables / 36 sections / 266 elements and zero residuals; all 25 page
  sidecars store final repository-relative image paths, and peak sampled system memory use was 19%. The 347→266
  element delta removes exactly 81 `body_text` records on seven visual-bearing pages while captions, list items,
  section headers, pages, visuals, and tables hold, so it is diagram-label cleanup rather than structural loss.
  Evidence retains 36 anchors / 243 spans / 11 visuals / seven links / 243 statements, including five
  conditional and two normative facts, with zero relations or signal declarations. Validation correctly
  classifies this self-declared document as a high-confidence methodology guide rather than an under-extracted
  architecture. Current generic authority removes the stale 64 heuristic interfaces and 73-signal `agent.isf`,
  along with one visual-label `host` actor; five actors, 22 phases, 54 invariants, 56 gates, two assertions, 28
  decomposition candidates, five conditionals, 78 behaviors, and one assumption remain. Lowering blocks on
  `no signals declared in interface`, removes the obsolete target, and leaves exactly `adapter.json` with zero
  residuals. Two complete cascades reproduce all nine final hashes. Nine provider-free WIRE/I2C/SWD gates, KG
  156/156, all 66 current emitted ISFs through FSMGen strict, persisted-path and locality gates pass; no product
  code changed.
  Commit: `CORPUS-COVERAGE.2.37 — sign off AArch64 External Debug refresh`.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.37`

- [x] **REPRODUCE / MEASURE** — authenticate the source PDF, retained five-stage chain, stale 64-interface/
  73-signal adapter surfaces, release binary, and same-volume rollback; record guarded-ingest counts, peak memory,
  final hashes, and typed before→after deltas.
- [x] **ROOT CAUSE (WHY + WHERE)** — classify every material delta against current generic extraction and
  signal/interface authority rather than treating strict syntax or document vocabulary as semantic trust.
- [x] **ADDRESSED (verified)** — promote only complete validated stages; require every page path final and
  repository-relative; converge obsolete adapter targets; run FSMGen strict only if honestly renderable.
- [x] **NO REGRESSION** — reproduce the deterministic cascade, hold nine provider-free WIRE/I2C/SWD gates and KG
  156/156, sweep all current emitted ISFs through FSMGen strict, and pass mdBook/doctrine/path/locality gates plus
  broader Rust CI if any product code changes.
- [x] **GENERICITY** — no AArch64/Arm/guide/document/token exception, manual generated-output edit, relaxed
  validator, fabricated model, or LLM/VLM inference enters the slice.
- [x] **LOCKSTEP** — generated artifacts, #37 row/count/frontier, live docs, durable fact when warranted, mdBook,
  and `MEMORY.md` agree; delete only authenticated rollback/task evidence after every final gate is green.

- ID: `CORPUS-COVERAGE.2.38` · Status: `in_progress` (`2026-08-09`, DATA/DOC) · Goal: run a guarded
  current-binary refresh of `102520_0101_01_2025_09_15_introducing_coresight_debug_and_trace` from the
  caller-authorized same-SSD source and complete the deterministic EvidenceIR→SemanticIR→IntentIR→adapter
  cascade. This 32-page / 25-visual / four-table / 333-element guide is the next-smallest ARM candidate and a
  deliberately different authority test: retained EvidenceIR carries nine actor-signal relations, SemanticIR /
  IntentIR carry six interfaces, eight ports, and four connectivity edges, while the four-signal adapter is
  already blocked for lack of behavioral content. The retained six-file chain, source hash `08a37c35…162`, and
  release binary were authenticated before mutation. `.2.38a` records that copied validation backannotated both
  canonical and rollback JSON after those hashes/counts were captured, so do not claim the original bytes remain
  recoverable: retain the complete side-effect snapshot and regenerate from the authenticated source. Use CPU
  Docling with the 85%-used RAM abort and no LLM/VLM inference; preserve any relation/topology surface satisfying
  current generic authority, remove only unsupported material, and accept any output grounded by the refreshed
  pipeline. If another product defect appears, preserve exact evidence and add a bounded child leaf before code.

### Closure criteria (pending) — `CORPUS-COVERAGE.2.38`

These become the earned acceptance checklist when the DATA/DOC parent closes; while the bounded CODE child
`.2.38a` is landing, they remain explicit pending criteria rather than an unchecked enforced checklist that
would be mistaken for the child commit's owning acceptance section.

- **REPRODUCE / MEASURE** — authenticate the source PDF, original six hashes/counts, nine relations, six
  interfaces/eight ports/four connectivity edges, blocked four-signal adapter, release binary, and preserved
  validator-side-effect set; record guarded-ingest counts, peak memory, final hashes, and typed deltas.
- **ROOT CAUSE (WHY + WHERE)** — classify every material delta against current generic relation, topology,
  signal/interface, and behavioral authority rather than treating guide vocabulary or strict syntax as trust.
- **ADDRESSED (verified)** — promote only complete validated stages; require every page path final and
  repository-relative; converge obsolete adapter targets; run FSMGen strict only if honestly renderable.
- **NO REGRESSION** — reproduce the deterministic cascade, hold nine provider-free WIRE/I2C/SWD gates and KG
  156/156, sweep all current emitted ISFs through FSMGen strict, and pass mdBook/doctrine/path/locality gates plus
  broader Rust CI if any product code changes.
- **GENERICITY** — no CoreSight/Arm/guide/document/token exception, manual generated-output edit, relaxed
  validator, fabricated topology/behavior, or LLM/VLM inference enters the slice.
- **LOCKSTEP** — generated artifacts, #38 row/count/frontier, live docs, durable fact when warranted, mdBook,
  and `MEMORY.md` agree; delete only authenticated rollback/task evidence after every final gate is green.

- ID: `CORPUS-COVERAGE.2.38a` · Status: `done` (`2026-08-09`, CODE/DATA/DOC) · Result: reproduced and repaired
  copied-artifact validation backannotation before resuming `.2.38`. The original six retained hashes were
  captured before the side effect: SourceIR `5b374be5…6fb`; EvidenceIR/report `2af7b31e…dd9` /
  `41b2ab7f…f79`; SemanticIR `6dbbc56b…672`; IntentIR `02657b54…e06`; adapter `6b2303fa…175`.
  `persist_*_validation` loaded the explicit copy but called each stage's normal `write_to_disk()`, which follows
  the embedded canonical artifact layout. Therefore validating the copied SourceIR→IntentIR chain backannotated
  the canonical JSON and materialized canonical `normalized/page_artifacts.json` and `visual_assets.json`, while
  the adjacent report and console correctly named the copy. Because both canonical and rollback JSON were
  backannotated before detection, the original bytes cannot honestly be called recoverable; their hashes/counts
  remain preserved, and the complete side-effect state is retained until #38 regenerates from authenticated
  source `08a37c35…162`.

  The new path-generic `write_backannotated_artifact` resolves the caller's explicit repository-owned artifact
  path and writes `to_pretty_json()` there; the report remains its adjacent `validation_report.json`. It does not
  call a stage producer or materialize embedded-layout siblings. One all-stage regression builds canonical
  SourceIR, EvidenceIR, SemanticIR, IntentIR, and adapter artifacts, validates copied JSON, requires each copy to
  gain one report plus its adjacent sidecar, and snapshots the complete canonical output tree byte-for-byte after
  every stage. Existing canonical SourceIR and IntentIR backannotation tests remain green. A real release-binary
  replay over five copied #38 artifacts likewise leaves the complete canonical chain byte-identical to the
  preserved side-effect set. Formatting, warning-deny Clippy, 1,788 tests / five ignored, rustdoc, mdBook,
  doctrines, and project-data locality pass. The required live product record reached the 72-record rollover
  threshold; an exact dry-run/apply plan seals the 12 oldest live duplicates as status segment `0005`, updates the
  reciprocal chain, and leaves a 60-record warning-safe root with zero transaction residue. Parent `.2.38` now
  owns authenticated-source regeneration and the guarded refresh; validator metadata cannot contaminate its
  delta again.
  Commit: `CORPUS-COVERAGE.2.38a — contain validation backannotation paths`.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.38a`

- [x] **REPRODUCE / MEASURE** — preserve the original six hashes/counts and current side-effect chain; a hermetic
  copied-artifact test must reproduce canonical mutation and source normalized-summary creation before the fix.
- [x] **ROOT CAUSE (WHY + WHERE)** — prove each `persist_*_validation` ignores its `artifact_path` for the IR write
  by calling embedded-layout `write_to_disk()`, while the sidecar helper correctly uses the explicit path.
- [x] **ADDRESSED (verified)** — write each backannotated JSON to the resolved explicit artifact path and its
  report to that path's sibling; canonical artifacts and unrelated embedded-layout siblings remain byte-exact.
- [x] **NO REGRESSION** — canonical-path validation still backannotates normally; focused tests cover SourceIR,
  EvidenceIR, SemanticIR, IntentIR, and adapter artifacts; formatting, warning-deny Clippy, full CI, mdBook,
  doctrines, persisted paths, and project-data locality pass.
- [x] **GENERICITY** — the repair is stage-generic and path/containment based, with no document, stage-key,
  workstation, or corpus exception and no relaxed validation semantics.
- [x] **LOCKSTEP** — source comment, task result, durable fact, live docs, mdBook command contract, and
  `MEMORY.md` explain that `validate <artifact>` mutates only that artifact and its adjacent report.

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
- Frontier: `CORPUS-COVERAGE.2.38a` — complete the verified repair's commit workflow; immediately afterward,
  parent `.2.38` regenerates the canonical chain from the authenticated same-SSD source and resumes the guarded
  ingest/cascade without classifying validator metadata as a document refresh delta.
  Historical `.2` phase context follows: re-ingest the 57-document cohort from the
  `.cache/local-references/chipdoc` symlink, register/TRM/ISA phase, one doc per slice (**37 refreshes done after #37;
  19 real chip-spec docs remain unrefreshed by `.2`** — see the `.2` log table below for #29–#37: #29/#31 CHI-C2C marquee message-field refreshes,
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

## `.2` re-ingest log (per doc — current-binary refresh; generated/ is git-ignored, so this table is the durable trace)

Columns: pages · key new typed surfaces the refresh added (vs the STALE pre-`.10`/`.12`/`.2` evidence) · `.isf` render + FSMGen `--strict --check` diagnostics.

`RESTORED` in a row records the postcondition at that slice's completion. It is historical, not a claim that the
rebuildable cache is retained forever: the `2026-07-05` cleanup later reclaimed all normalized bundles while
leaving the refreshed SourceIR→adapter stage chains intact.

| # | doc (key) | pages | refreshed surfaces (after) | `.isf` | fsmgen `--strict` |
|---|---|---|---|---|---|
| 1 | `ihi0079` AMBA CXS | 54 | message_field_records 0→1; transactions →2 (recognition now fires); 11 signal ports emitted (CXSDATA/CXSVALID/CXSCRDGNT/…); stale evidence had message/temporal/presence fields ABSENT | renderable (`transmitter.isf`, 11 ports) | **0 diagnostics** ✓ |
| 2 | `ihi0083` AMBA GFB | 45 | transactions →5 (recognition now fires); relations 16 (held); signal_constraints 14→7 (current stricter declared-subject gate drops ungrounded); no msg/reg fields (GFB has none — honest) | renderable (`device.isf`, 7 ports) | **0 diagnostics** ✓ |
| 3 | `ihi0076` ACC | 90 | register_records 2→4 (the `.10g` section-heading register-field family fires — ACC is one of its 5 docs); rich `.isf` 133 ports (vs stale 2-register); 0 txns/relations (register/debug-channel arch — honest) | renderable (`agent.isf`, 133 ports) | **0 diagnostics** ✓ |
| 4 | `ihi0082` ATP | 84 | transactions →5 (recognition fires); relations 11→9 — the CLEAN deterministic re-ingest drops the garbled-VLM-fragment actors the `KG-ISF-COMPLETENESS.4` census flagged (`"RREADY is RBR"`), so ATP no longer needs the owner-gated VLM lever just to get a clean baseline; reg 4 held | renderable (`rate_parameter.isf`, 1 port) | **0 diagnostics** ✓ |
| 5 | `tilelink_1_7_1` TileLink 1.7.1 | 107 | relations 39→33 (current agent-identity consolidation gates `.1a`/`.1b` — which postdate the stale evidence — fold fragment actors); renderable per-channel `.isf` (`channel_b`); transactions 0 (TileLink's Get/Put op vocabulary not yet in the section-heading recognizer — honest) | renderable (`channel_b.isf`, 5 ports) | **0 diagnostics** ✓ |
| 6 | `tilelink_1_8_0` TileLink 1.8.0 | 111 | relations 40→34 (same `.1a`/`.1b` consolidation as 1.7.1); transactions 0 (op vocabulary — honest); per-channel `.isf` | renderable (`channel_b.isf`, 6 ports) | **0 diagnostics** ✓ |
| 7 | `ihi0068` AMBA LPI | 66 | transactions →1 (recognition fires); relations 9→8; constraints 17 held | renderable (`controller.isf`, 5 ports) | **✅ 0 diagnostics** (was 1 — `isf_conflicting_rule_writes` on `PREQ`/`PACCEPT`, an UNCONDITIONAL rule_action vs a guarded temporal constraint; **RESOLVED `2026-06-23` by `KG-ISF-COMPLETENESS.2a.v` Lever C** — the conflicting rules are now residualized via `drop_unconditional_overlap_conflicts`, the same cross-guard fix that also cleared the AHB `HAUSER`/AXI wire-gold conflicts) |
| 8 | `ihi0088` AMBA DTI | 146 | **`message_field_records` 0→159 / 17 containers** (the `.10f` section-heading message-field family fires — the marquee re-ingest win; totally ABSENT in stale evidence); transactions →5; constraints 30→16 (stricter gate) | renderable (`channel.isf`, 294 ports) | **✅ 0 diagnostics (re-verified `2026-06-23`)** — this row's `1 ERROR` (`ATST` RHS `2'b1` width-2 vs LHS width-1) is **STALE**: `ATST` now emits `1'd1` and DTI passes FSMGen `--strict --check` clean (the width-alignment was delivered by `ISF-VALUE-WIDTH-EMIT.2`; the entry was never re-verified). Lever A = already resolved. |

| 9 | `ihi0098_b` AMBA CHI-C2C | 291 | **`message_field_records` 0→210 / 21 containers** (`.10b`/`.10d` families); register_records 76→81 (`.10g`); transactions →3; relations 4→0 (honest — coherency/packet protocol carries intent in fields/registers, not wire relations, per `KG-ISF-COMPLETENESS.3`) | renderable (`agent.isf`, 109 ports) | **0 diagnostics** ✓ (no width-mismatch → DTI's `ATST` bug is value-specific, not universal) |

| 10 | `ddi0461` CoreSight TMC | 116 | **register_records 2→30** (the `.10c` `bits\|name\|description` family fires — exactly the documented gain); relations 53→50 (`.1a`/`.1b` consolidation); 30-port `.isf` | renderable (`master.isf`, 30 ports) | **0 diagnostics** ✓ |
| 11 | `101130` CoreSight SDC-600 | 75 | **register_records 0→5** (`.10c`); transactions →6; rel 0 (honest register TRM) | renderable (`agent.isf`, 112 ports) | **0 diagnostics** ✓ |
| 12 | `ihi0029` CoreSight arch | 280 | register_records 21→26 (+5 via `.10g`, matches the census); rel 7→6 | renderable (`special_type.isf`, 9 ports) | **0 diagnostics** ✓ |
| 13 | `101542` MMU-700 TRM | 256 | **register_records 13→63 (+50, `.10c`)**; transactions →10; constr 11→7; 334-port `.isf` | renderable (`agent.isf`, 334 ports) | **0 diagnostics** ✓ |
| 14 | `100336` GIC-600 TRM | 216 | **register_records 15→33 (+18, `.10c`)**; transactions →10; relations 108→101 | renderable, but strict-FAILS | **1 ERROR** — **module-name not HDL-sanitized**: emitted `?fsm:redistributor→_distributor…` (arrow from a prose-fragment initiator actor); FSMGen requires `[A-Za-z_]\w*` → malformed name breaks the WHOLE `.isf`. 2nd emitter bug → spun-out lever (B). **RESOLVED `2026-06-21` by `KG-ISF-COMPLETENESS.2a.iii`** — module name now HDL-sanitized → GIC-600 re-checks FSMGen `success=true`, **0 diagnostics** |
| 15 | `ihi0069` GIC arch | **930** | **register_records 17→90 (+73, `.10g`)** — biggest `.10g` gain, matches census; msg 0→2; rel 23 held. **930-page doc ingested with RAM steady ~77% free** (validates `MEMORY-BOUNDED-INGEST` adaptive batch + `.4a` guard) | renderable (`following_pseudocode.isf`, 6 ports) | **0 diagnostics** ✓ (fragment-actor name but HDL-valid → no break) |
| 16 | `ihi0070` SMMU arch | 717 | **register_records 1→89 (+88, `.10g`)** — matches census; transactions →5; rel 6→2 | renderable (`agent.isf`, 1 port) | **0 diagnostics** ✓ |
| 17 | `683091` Avalon Interface Spec | 63 | relations 126→111 / actors 64→50 — the current `.1a`/`.1b`/`.1b.iv` agent-identity consolidation gates (which POSTDATE Avalon's stale Jun-7 evidence) fold fragment/phantom actors → cleaner KG; register_records 8 held; transactions 5 held; **no message-field/presence surfaces (Avalon is a prose/diagram interface spec — honest absence; the `.10b`/`.10f`/`.12` table families don't fire)** | renderable (`source.isf`, 26 signals / 8 storage / 7 enums / 1 txn body) | **0 diagnostics** ✓ |
| 18 | `wbspec_b4` Wishbone B4 | 128 | **wire-signal inventory comes out EMPTY** (only 1 `signal_description` of 25 Docling tables, 22 `unknown` → 0 usable signal_records → relations 1→0 — the lone stale fragment dropped by the `.1a` gate, NOT a regression: the stale build also had ~1); IntentIR still carries **215 free-text constraints + 2 transactions + 8 actors** (the intent is captured as obligations/behaviors, just not as the typed wire surface). Surfaces a real **signal-inventory recall gap**: Wishbone documents its signals in the `SIGNAL_O()`/`SIGNAL_I()` suffix-notation + prose signal-list style that the current signal-table/prose extractors don't recognize → an upstream `PDF-VARIANT-DIGESTION` signal-recall lever (kin to the parked `.9.10` prose-bus-line lever), NOT an emitter lever and NOT fixed in-slice | renderable but THIN (`arbiter.isf`, 1 signal) | **0 diagnostics** ✓ (strict-clean) |
| 19 | `opencapi_4_0_transactionlayer_arch` OpenCAPI 4.0 TL Arch | 240 | **typed wire/message surfaces empty — honest doc-style recognition gap, REGRESSION RULED OUT** (verified on the current binary: DTI 159 msg-fields / MMU-700 63 registers / AHB 66 relations all still hold): of **246** fresh Docling tables, **0 `signal_description`** (219 `unknown` / 20 encoding / 7 feature_matrix) → relations 17→0, signal_constraints 8→0, 0 message fields. OpenCAPI TL is a packet/command-layer spec whose command/field tables don't match the AMBA `Signal\|Direction\|Width\|Description` shape; IntentIR still captures **797 free-text constraints + 1 transaction + 12 actors**. The 219 `unknown` tables are the recall opportunity → Lever D family (table-recognition for non-AMBA styles), spun-out, not fixed in-slice | renderable but THIN (`channel.isf`, 1 signal) | **0 diagnostics** ✓ (strict-clean) |
| 20 | `ddi0471` GIC-400 TRM | 57 | **healthy refresh (register/TRM-phase #1, NOT thin like #18/#19):** 5 `signal_description` (of 25 Docling tables) → 16-signal AXI-slave-interface `.isf` with **5 transactions + 4 storage (reset) + 6 relations** held. register_records 3→4 only (modest — GIC-400's registers largely sit in its 14 `unknown`-classified tables, a different/older table style than GIC-600's `.10c`-shaped 15→33; a minor Lever-D recall opportunity, not the marquee `.10c` gain) | renderable (`a_4_axi_slave_interface.isf`, 16 signals / 5 txns / 4 storage) | **0 diagnostics** ✓ |
| 21 | `1_0_1_2026_02_22_risc_v_iommu_architecture_specification` RISC-V IOMMU arch | 108 | **register/arch refresh — evidence was ALREADY current-binary-equivalent (NOT pre-`.10` stale):** register_records 33 held / 147 fields, signal_constraints 6→8, conditional_rules 47 — the `.10c`/`.10g` register families predate this doc's Jun-15 stale evidence (which was rebuilt from the retained `source_ir.json` before the `normalized/` bundle was cleaned), so the deterministic surfaces are byte-near-identical (source_ir 13-byte diff) and the IntentIR is unchanged (12 actors / 628 constraints / 608 behaviors / 426 ifaces). Re-ingest's value here = **normalized bundle RESTORED** (was missing) + current-binary cascade confirmation + a surfaced **Lever-D structure-table recall opportunity**: of **83** structured tables, the IOMMU's in-memory device-context / process-directory / command-queue **STRUCTURE** tables don't match the `.10b`/`.10d`/`.10e` two-column bit-position families → `message_field_records` honestly absent (not regression). 0 relations / 0 interfaces (memory-mapped register arch — intent lives in registers, `KG-ISF-COMPLETENESS.3`); 0 transactions (RISC-V command vocabulary not in the section-heading recognizer — honest, like TileLink) | renderable (`agent.isf`, 175 signals / 33 storage(reset) / 5 enums / 45 rules) | **0 diagnostics** ✓ (strict-clean) |
| 22 | `100798_0401` Cortex-A76 TRM | 620 | **register/CPU-core-TRM refresh — evidence was ALREADY current-binary-equivalent (NOT pre-`.10` stale, #21 class):** register_records 42 held / conditional_rules 26 held / 0 message-fields / 0 relations / 0 interfaces / 0 transactions in evidence; IntentIR unchanged (9 actors / 7 interfaces / 707 behaviors / 393 constraints / 0 relations). The `.10c`/`.10g` register families predate this doc's stale evidence (rebuilt from the retained `source_ir.json` before the `normalized/` bundle was reclaimed), so the deterministic cascade is byte-identical and re-ingest's value = **normalized bundle RESTORED** (was missing; 620 pages / 476 visual / 0 residuals, RAM steady 73–83% free) + current-binary confirmation. Honest absences: 0 relations/interfaces in evidence (CPU-core TRM — register/behavioral intent, not wire relations, `KG-ISF-COMPLETENESS.3`); 0 message-fields (no packet/structure tables); the 42 registers' bit-fields are largely UNLOCATED → honest `.isf` residuals (`isf_register_fields_not_lowered` 469 bit-fields + `isf_storage_reset_not_lowered` 17 field-resets — no fabrication, the DOC-INTENT-TAXONOMY.4a.ii located-fields-only rule). `validate` no stage-staleness (semantic+intent relations both 0 → 0-vs-0 honest absence, the `.1` detector correctly silent); score 24/100 INCOMPLETE (honest for a register/behavior TRM lacking wire direction/width/clock/reset grounding — graph direction 0%, semantic role 0%) | renderable (`agent.isf`, 189 signals / 42 storage(reset) / 13 enums) | **0 diagnostics** ✓ (strict-clean) |
| 23 | `100806_0701` CoreSight SoC-600 0701 TRM | 842 | **GENUINE refresh (NOT byte-identical like #22 — the stale evidence predated the `.1a`/`.1b` agent-identity gates AND the section-heading transaction recognizer):** evidence register_records 833→828 (fresh-Docling table-boundary variance, minor), **actor_signal_relations 64→41 / actors 60→47 / interfaces 20→8** (the current `.1a`/`.1b`/`.1b.iv` consolidation gates fold the fragment/phantom actors the older stale evidence carried — a cleaner KG, like #17 Avalon), **transactions 0→2** (the section-heading recognizer now fires — NEW typed surface absent from stale evidence); conditional_rules 74 held; 0 message-fields (no packet/structure tables — honest absence). `validate` no stage-staleness (semantic+intent relations both 41 → non-zero, silent), score 53/100 ADEQUATE. Honest residuals: 791 bit-fields + 164 field-resets not lowered (UNLOCATED, the located-fields-only rule); 3 `isf_rule_conflict` on `ATB` (the ISF-RULE-CONFLICT-RESIDUAL family — correctly RESIDUALIZED, NOT emitted, so unlike LPI #7 the `.isf` stays strict-clean) | renderable (`dp.isf`, 828 storage(reset) / 19 enums / 4 rules / 5 signals) | **0 diagnostics** ✓ (strict-clean) |
| 24 | `100806_0200` CoreSight SoC-600 0200 TRM | 761 | **GENUINE refresh (same #23/#17 class — stale evidence predated the `.1a`/`.1b` consolidation gates + transaction recognizer):** register_records 631 held (exact, no fresh-Docling variance this time), **actor_signal_relations 47→35 / actors 46→37 / interfaces 13→6** (consolidation folds fragment/phantom actors), **transactions 0→1** (recognizer fires); conditional_rules 51 held; 0 message-fields (honest). `validate` no stage-staleness (semantic+intent relations both 35 → non-zero, silent), score 52/100 ADEQUATE. Honest residuals: register bit-fields + field-resets not lowered (UNLOCATED); 1 `isf_temporal_unrepresentable` (a temporal_signal_constraint FSMGen can't represent — honest, not emitted). Completes the CoreSight SoC-600 corpus cluster's 0200 version (0701 = #23; 0100 pending) | renderable (`dp.isf`, 631 storage(reset) / 9 enums / 3 rules / 4 signals) | **0 diagnostics** ✓ (strict-clean) |
| 25 | `100806_0100` CoreSight SoC-600 0100 TRM | 702 | **GENUINE refresh (same #23/#24/#17 class) — completes the CoreSight SoC-600 cluster (0701/0200/0100 all done):** register_records 597 held (exact), **actor_signal_relations 41→31 / actors 44→34 / interfaces 12→5** (consolidation), **transactions 0→1** (recognizer fires); conditional_rules 46 held; 0 message-fields (honest). `validate` no stage-staleness (semantic+intent relations both 31 → non-zero, silent), score 52/100 ADEQUATE. Honest residuals: register bit-fields + field-resets not lowered (UNLOCATED); 1 `isf_temporal_unrepresentable` (honest, not emitted). The 3 SoC-600 versions are UNIFORMLY the consolidation+transaction refresh class (stale evidence had `.10c` registers but predated the `.1a`/`.1b` consolidation gates + transaction recognizer) | renderable (`dp.isf`, 597 storage(reset) / 12 enums / 3 rules / 4 signals) | **0 diagnostics** ✓ (strict-clean) |
| 26 | `5_0_…_intel_…_directed_io` Intel VT-d 5.0 | 354 | **GENUINE refresh — CORRECTS the #22 "already current-binary-equivalent" survey expectation:** the one-pass register-count survey flagged VT-d's 103 registers as current, but its stale evidence predated BOTH the message-field family AND the section-heading transaction recognizer → register_records 103 held (exact), **`message_field_records` (key absent)→15 / 4 containers** (NEW — the `.10c` structure-field family fires on VT-d's `Root-Entry Format`/context-table structures; this **actualizes the book's existing `.10c` "VT-d gains 15 typed structure fields" claim** that the stale persisted evidence never reflected, taking the corpus-wide in-memory-structure total 1,220/11→1,235/12 docs), **transactions 0→2** [`device_tlb_operation`, `set_root_table_pointer_operation` — both recognition-only, honest], actors 14→13 (consolidation), signal_constraints 1→4, temporal_rules 1→4; 0 relations (memory-mapped register spec — honest absence, `KG-ISF-COMPLETENESS.3`). `validate` no stage-staleness (0-vs-0 honest absence, silent), score 8/100 INCOMPLETE (honest for a memory-mapped register/structure spec lacking wire direction/width/clock/reset — the #21 RISC-V-IOMMU class). Honest residuals: register bit-fields + field-resets UNLOCATED (6 adapter `residual_decisions`); doc-completeness gauge 7/17 unexplained intent-bearing tables (Lever-D structure-table recall opportunity, like #21) | renderable (`agent.isf`, 510 signals / 103 storage(reset) / 4 enums / 44 rules / 0 txns) | **0 diagnostics** ✓ (strict-clean) |
| 27 | `jesd84_b50` JEDEC eMMC 5.0 | 296 | **MIXED refresh — genuine transaction win BUT a phantom-actor explosion (the FIRST descriptive-prose register/protocol spec re-ingested):** **transactions 0→6** [`boot`/`alternative_boot`/`device_lock_unlock`/`dual_data_rate_mode`/`background`/`h_w_reset`_operation — 5 with grounded signal set (9 members), all real eMMC ops], register_records 17 held, interfaces 9→38, signal_constraints 0→2, 0 message-fields (honest — eMMC has no bit-position structure tables). **BUT actors 20→153 / relations 23→349:** the fresh Docling extraction (7017 statements vs stale 6544) surfaces ~90+ phantom sentence-fragment actors (`host has`/`host to`/`cache in`/`B write`/`CMD to`/`device behaves`/`following`/`value`) that the `.1a`/`.1b` agent-identity gates — measured clean on AMBA-style docs — do NOT gate for eMMC's descriptive prose (esp. real-agent+trailing-auxiliary/preposition `host has`/`host to`). The emitter lowers only the renderable `host` actor (the 153 raw `actors[]` are never lowered), so the `.isf` stays strict-clean. `validate` no stage-staleness (349 non-zero, silent), score 51/100 ADEQUATE. **Surfaces NEW spun-out lever E (agent-identity precision for descriptive-prose / non-AMBA specs — `KG-ISF-COMPLETENESS` family).** | renderable (`host.isf`, 62 signals / 17 storage(reset) / 28 enums / 16 rules / 0 txns) | **0 diagnostics** ✓ (strict-clean — `host` actor only) |
| 28 | `jesd235a` JEDEC HBM2 DRAM | 172 | **MIXED refresh — CONFIRMS the phantom explosion is PROSE-SPECIFIC (HBM2 is a register/timing-table DRAM spec, the diagnostic contrast to #27 eMMC):** actors **52→38 CONSOLIDATED DOWN** (the `.1a`/`.1b` gates fold fragments, like the AMBA/CoreSight class — NOT the eMMC explosion), relations 89→55, interfaces 72→31, **transactions 0→3** [`read`/`write`/`trr_mode`_operation, recognition-only], register_records 17 held, 0 message-fields (honest). **BUT `.isf` strict-FAILS → NEW lever F:** the emitter built a generic-named `(type TABLE (bits 6))` mega-enum conflating ~10 distinct doc tables (REPAIR_LANE codes + microbump-pitch descriptions + test-operation list + IDD currents + mode-register refs) with restarting/duplicate values, and the REPAIR_LANE members are BINARY codes (`0,1,10,11,…,1000,1111`) emitted as bare decimal-looking tokens → FSMGen rejects `TABLE.REPAIR_LANE_8` value `1000` (package symbol value must be a width/radix-qualified scalar literal like `4'b1000`) AND the bare token is semantically wrong (binary 1000 ≠ decimal 1000). Lever F = ISF enum-member value literal format + binary-radix preservation (kin to Lever A, the `ISF-*-EMIT` value-literal family), plus a related enum-extraction-precision concern (generic `TABLE` mega-enum conflation). `validate` 55/100 ADEQUATE, no stage-staleness. | renderable (`hbm.isf`, 15 signals / 17 storage / 7 enums / 28 rules / 2 txns) | **✅ 0 diagnostics** (was 1 ERROR enum value `1000`; **RESOLVED by `KG-ISF-COMPLETENESS.2a.iv` Lever F** — the malformed `TABLE` enum is now residualized, not emitted) |
| 29 | `ihi0098_a` AMBA CHI C2C | 108 | **MARQUEE message-field refresh — the `.10` families fire on CHI packet/flit field tables that were ABSENT in the stale pre-`.10` evidence:** **`message_field_records` 0→149 / 13 containers**; normalized bundle RESTORED (was missing; 108pp / 88 visual / 0 residuals; Docling CPU; RAM steady ~70% free). 0 register_records / 0 relations (CHI is a coherency/message protocol — intent lives in message fields, the honest-absence class `KG-ISF-COMPLETENESS.3`); conditional_rules 17 + transactions 2 (recognition-only) held; statements 2239. **Surfaced (extraction-precision, NOT emitter → honest residuals, NOT fixed in a re-ingest slice):** the emitter built a generic `(type TABLE (bits 8))` mega-enum conflating many distinct field-value tables (duplicate values + sentence-fragment member names — the #28 HBM2 generic-`TABLE` conflation class), and a few interface ports are prose-acronym noise (`AES`/`AMBA`/`ARM` — the `KG-ISF-COMPLETENESS.4` signal-inventory prose-noise class). `validate` no stage-staleness (0-vs-0 honest absence, silent). | renderable (`agent.isf`, 69 ports / 3 enums / 1 rule / 0 txn bodies) | **0 diagnostics** ✓ (strict-clean, `--strict --check --json`) |
| 30 | `1_0_2025_03_12` RISC-V AIA | 89 | **bundle-restoration + current-binary confirmation (the #21/#22 class — NOT pre-`.10` stale):** all deterministic surfaces byte-near-identical to the retained-`source_ir` evidence — `register_records` 0 held, `message_field_records` 0, `actor_signal_relations` 0, `conditional_rules` 39 held, `extracted_statements` 1193 held; intent 8 actors / 0 rel / 0 txns. **normalized bundle RESTORED** (was missing; 89pp / 105 visual / 0 residuals; Docling CPU; RAM ~75% free). Honest absence: 0 registers/relations (memory-mapped interrupt-controller ISA — its APLIC/IMSIC register-layout tables sit in the **12 `unknown`-classified structured tables** that don't match the `.10b`/`.10c`/`.10g` families → a **Lever-D structure-table recall opportunity**, the #21 RISC-V IOMMU class; NOT a regression). The 8 `encoding` tables feed the 5 emitted enums. | renderable (`agent.isf`, 91 ports / 5 enums / 22 rules) | **0 diagnostics** ✓ (strict-clean, `--strict --check --json`) |
| 31 | `ihi0098_a_b` AMBA CHI C2C (2026) | 122 | **MARQUEE message-field refresh (CHI C2C 2026 variant, sibling of #29 → confirms the CHI-C2C family is a real marquee, not a one-off):** **`message_field_records` 0→143 / 12 containers** (the `.10` families fire on CHI packet/flit field tables absent in the stale pre-`.10` evidence); transactions 2→3 (recognition-only); the lone stale `actor_signal_relations` 1→0 (a fragment relation the current agent-identity gates drop — CHI is a coherency/message protocol, 0 relations is honest per `KG-ISF-COMPLETENESS.3`); conditional_rules 25 held; 2364 statements; 0 registers. **normalized bundle RESTORED** (122pp / 96 visual / 0 residuals; Docling CPU; RAM ~75% free). Same surfaced extraction-precision residuals as #29 (generic-`TABLE` mega-enum conflation + signal-acronym prose noise — not fixed in-slice). | renderable (`agent.isf`, 69 ports / 3 enums / 0 rules) | **0 diagnostics** ✓ (strict-clean, `--strict --check --json`) |
| 32 | `jesd235_2013_10_hbm_dram` JEDEC HBM gen1 | 6 | **bundle-restoration + CORPUS-PROVENANCE finding (NOT a real-delta doc): the library `JESD235_2013-10_HBM_DRAM.pdf` is a 6-page LEGAL-EXHIBIT COVER** (markdown reads `DOCKET`/`ALARM`/`JEDEC STANDARD … HBM DRAM … OCTOBER 2013`/`Netlist Inc.`/`Netlist Ex 2021` + 12 cover images, **0 tables**), **not the full HBM gen1 standard** — so the thin yield is HONEST SOURCE-DRIVEN ABSENCE, not an extraction gap (contrast #28 HBM2 `jesd235a`, the real 172pp standard, which carried registers/transactions). All deterministic surfaces byte-near-identical to the retained-`source_ir` stale evidence: `register_records` 0 held / `message_field_records` 0 / `actor_signal_relations` 0 / `signal_constraints` 0 / `conditional_rules` 0; `extracted_statements` 69→68; intent `actors` 2→2 / `interfaces` 1→7 (minor current-binary regroup) / 0 txns / 3 behaviors / 5 constraints. `adapt` recovers 16 signals on actor `channel` but **BLOCKS honestly** — `is_renderable: false`, the single blocking reason `no behavioral content (temporal/conditional rules, signal constraints, or control blocks)`, 1 residual_decision (the correct `CORPUS-COVERAGE.0` "intent-no-isf" class; no `.isf` fabricated). **normalized bundle RESTORED** (was missing; 6pp / 12 visual / 0 residuals; Docling CPU; RAM ~83% free). After #32: **24 real chip-spec docs still normalized-missing.** | **BLOCKED** (honest — no behavioral content; 0 emitted `.isf`) | n/a (nothing emitted) |
| 33 | `usb_3_2_revision_1_0_2017_09` USB 3.2 | 548 | **full current-binary cascade + completed semantic-fidelity repair:** guarded CPU ingest restores 548 pages / 507 visuals / 283 tables / 5,830 elements. SourceIR 6,584→5,830 is cleaner visual-label suppression, not prose loss. The initial cascade exposed a strict-clean but false four-port/29-rule actor; `.2.33d` repaired the three EvidenceIR authority seams, the independent authority-empty SemanticIR fallback, and stale adapter-output convergence without any document/token exception. Final EvidenceIR is 8,267 statements / 0 relations / 10 signal constraints / 542 conditional rules / 102 timing constraints. Final IntentIR is 18 actors / 0 interfaces / 0 ports or relations / 2,705 behaviors / 3,014 constraints / 27 transactions / 1 register. Adapter lowering blocks honestly on no declared interface signals: 0 signals/transactions/rules, 1 storage, 15 residuals, no emitted target, and exactly `adapter.json`. After #33: 80/2/80/79 stage census and **23 real chip-spec refreshes remain.** | **BLOCKED** (honest signal-authority boundary; 0 emitted `.isf`) | n/a (nothing emitted) |
| 34 | `usb4_inter_domain_service_specification_v2_0_2025_11` USB4 Inter-Domain Service | 51 | **portable current-binary refresh + second-document semantic-authority transfer proof:** two guarded CPU ingests restore 51 pages / 82 visuals / 49 tables / 603 elements at 18% peak system memory used. All 51 page JSON image paths are final-rooted and repository-relative; exactly those paths differ from the preserved defective run, while all nine final IR/report/adapter hashes reproduce. Source elements 615→603; Evidence statements 1,260→1,025, spans 1,037→1,025, relations 1→0, while 27 conditionals / 6 registers / 1 protocol state remain. Semantic actors 12→11 and Intent actors 11→10; interfaces/ports/relations/connectivity all 1→0. The old one-signal/two-rule/eight-enum `USB4` adapter blocks honestly: 0 signals/transactions/rules/constants/enums, six storage records, one residual, no emitted target, exactly `adapter.json`. Only two `USB4` conditional consequents remain as source evidence, not interface authority. After #34: 80/3/80/79 stage census and **22 real chip-spec refreshes remain.** | **BLOCKED** (honest signal-authority boundary; 0 emitted `.isf`) | n/a (nothing emitted) |
| 35 | `usb4_connection_manager_guide_v2_0_2025_11` USB4 Connection Manager Guide | 96 | **portable current-binary refresh + third semantic-authority transfer:** guarded CPU ingest produces 96 pages / 46 visuals / 23 tables / 1,313 elements at 28% peak system memory used; 96/96 page paths are final-rooted and repository-relative. Source 1,509→1,313 is exactly body-text visual cleanup; all other element classes and structural counts hold. Evidence statements 1,680→1,421, spans 1,617→1,421, relations 13→0, signal constraints 2→6, conditionals 19 held, protocol states 0→2. Semantic/Intent actors 21→12; interfaces 4→0, ports 11→0, relations 13→0, connectivity 3→0; two recognition-only operations survive. The stale `SB`/`USB`/`USB4` three-signal/three-rule/one-enum adapter blocks honestly with no declared interface signals, four residuals, no emitted target, and exactly `adapter.json`. Two cascades reproduce all nine final hashes. After #35: 80/4/80/79 stage census and **21 real chip-spec refreshes remain.** | **BLOCKED** (honest methodology-guide/signal-authority boundary; 0 emitted `.isf`) | n/a (nothing emitted) |
| 36 | `den0068_2018_07_23_coresight_base_system_architecture` CoreSight Base System Architecture | 29 | **portable current-binary refresh + cat-3 authority/topology transfer:** guarded CPU ingest produces 29 pages / 19 visuals / 12 tables / 404 elements at 17% peak system memory used; all 29 page paths are final-rooted and repository-relative. Source structure and all EvidenceIR counts hold exactly (464 statements / 464 spans / 55 anchors / 19 visuals / 15 links / seven conditionals / zero relations or signal declarations). Current generic authority removes 88 stale heuristic interfaces; semantic actors 8, intent actors 5, behaviors 46, invariants 106, and conditionals 7 hold. Intent constraints 124→106; the stale 100-signal/two-rule `agent.isf` blocks honestly with no declared interface signals, no residuals, no emitted target, and exactly `adapter.json`. Validation surfaces the real residual: this self-declared architecture is under-extracted, with 19 unenriched visuals, 87 partially structured normative statements, and seven temporal-source rules lacking typed grounding—the known upstream cat-3 capture-recall frontier, not license to fabricate topology. Two cascades reproduce all nine final hashes. After #36: 80/5/80/79 stage census and **20 real chip-spec refreshes remain.** | **BLOCKED** (honest cat-3 capture-recall/signal-authority boundary; 0 emitted `.isf`) | n/a (nothing emitted) |
| 37 | `102196_0100_01_2022_05_05_aarch64_external_debug_guide` AArch64 External Debug Guide | 25 | **portable current-binary refresh + fourth semantic-authority transfer:** guarded CPU ingest produces 25 pages / 11 visuals / two tables / 36 sections / 266 elements at 19% peak system memory used; all 25 page paths are final-rooted and repository-relative. Source 347→266 removes exactly 81 visual-label `body_text` records on seven visual-bearing pages while every structural class holds. Evidence has 36 anchors / 243 spans / 11 visuals / seven links / 243 statements, five conditionals, two normative facts, and zero relations/signals. Validation classifies the self-declared document as a high-confidence methodology guide. Current authority removes 64 stale heuristic interfaces, the visual-label `host` actor, and the 73-signal `agent.isf`; five actors / 22 phases / 54 invariants / 56 gates / two assertions / 28 decompositions / five conditionals and 78 behaviors remain. Lowering blocks with no declared interface signals, zero residuals, no emitted target, and exactly `adapter.json`. Two cascades reproduce all nine hashes. After #37: 80/6/80/79 stage census and **19 real chip-spec refreshes remain.** | **BLOCKED** (honest methodology-guide/signal-authority boundary; 0 emitted `.isf`) | n/a (nothing emitted) |

**Cumulative (33 docs, protocol + register/arch phases): hundreds of registers + ~385 message fields + transaction recognition surfaced that were ABSENT in stale evidence** — registers e.g. SMMU-arch 1→89, GIC-arch 17→90, MMU-700 13→63, GIC-600 15→33, TMC 2→30; message fields DTI 0→159, CHI-C2C 0→210, VT-d 0→15; transactions e.g. eMMC 0→6, HBM2 0→3 (real ops). **Every current renderable adapter is FSMGen-strict clean; USB #33 and HBM-gen1 #32 block honestly and emit nothing.** **#27 eMMC surfaced a phantom-actor explosion (20→153) on descriptive prose → NEW lever E (agent-identity precision for non-AMBA prose specs); the `.isf` is unaffected. #28 HBM2 confirmed E is PROSE-SPECIFIC (HBM2 consolidated cleanly) but surfaced lever F (enum-value literal format + generic-`TABLE` mega-enum precision).** **Refined #22/#23 phase note:** "already current-binary-equivalent" is not uniform — a doc's stale evidence sits at whatever binary last rebuilt it, so re-ingest can still deliver a real KG refresh when that binary predated a later gate: #22 Cortex-A76 was fully current (byte-identical), but #23 CoreSight SoC-600 predated the `.1a`/`.1b` consolidation + transaction recognizer and got a genuine actor-consolidation + 2-transaction refresh (the #17 Avalon class). So the remaining tail is a MIX of pure confirmation (#21/#22) and consolidation/recognition refreshes (#17/#23), with marquee table-family jumps now the exception. **NEW phase finding (#21):** some of the 57 normalized-missing docs are NOT pre-`.10` stale — their evidence was rebuilt Jun-15 from the retained `source_ir.json` (only the heavyweight `normalized/` page-image bundle was disk-reclaimed) so it already carried the `.10` register families; for those, re-ingest is **normalized-bundle restoration + current-binary confirmation** (modest deterministic delta), distinct from the genuinely pre-`.10` docs (SMMU/GIC/MMU big jumps). The IOMMU also surfaced a **RISC-V STRUCTURE-table recall opportunity** (device-context/command-queue tables not matching the `.10b`/`.10d`/`.10e` two-column families — Lever D). **PHASE FINDING (#17–#19, the non-AMBA protocol specs):** their refresh value is NOT marquee table-family gains — Avalon = consolidation + current-binary freshness; **Wishbone + OpenCAPI-TL surfaced a doc-style TABLE-RECOGNITION recall gap** (Wishbone 1 `signal_description`/25 tables; OpenCAPI 0/246 — both predominantly `unknown`-classified; non-AMBA `Signal\|Direction\|Width` shapes), so their `.isf` degenerate to 1 signal. **REGRESSION RULED OUT** (DTI 159 / MMU-700 63 / AHB 66 all hold on the current binary) → these are the upstream Lever-D signal/table-recall family, not emitter bugs. The high-value AMBA-style protocol specs are now exhausted; **next phase = the register/TRM/ISA docs** (the `.10` register families demonstrably fire there — CoreSight/GIC/JEDEC/RISC-V system-IP). Remaining queue = OpenCAPI×13 (PHY/mechanical/TL-variants, expected thin)/JEDEC/USB/RISC-V system-IP/VT-d/Cortex-A76/GIC-400/CoreSight-SoC-600×3/overview/guides. **#29 (`2026-06-24`) — AMBA CHI C2C `ihi0098_a` is a genuine marquee exception to the "tail is mostly bundle-restoration" phase finding:** its stale evidence was truly pre-`.10` (0 message fields), so the current binary's `.10` families surfaced `message_field_records` 0→149 / 13 containers — a real KG-completeness gain, `.isf` strict-clean. After #29: 29 docs re-ingested, **27 real chip-spec docs still normalized-missing** (plus the project README, not a chip spec). CHI C2C also re-confirmed two standing extraction-precision residuals (generic-`TABLE` mega-enum conflation; signal-inventory prose-acronym noise) — surfaced, not fixed in-slice. **#30 (`2026-06-24`) — RISC-V AIA `1_0_2025_03_12` is the contrasting bundle-restoration case:** all deterministic surfaces byte-near-identical (statements 1193, registers 0, cond_rules 39 all held), so re-ingest value = normalized-bundle restoration + current-binary confirmation, `.isf` strict-clean — and it re-confirms the #21 RISC-V-IOMMU Lever-D recall opportunity (the APLIC/IMSIC register layouts sit in 12 `unknown` structured tables the `.10` families don't match). After #30: **26 real chip-spec docs still normalized-missing**. Phase pattern holds: marquee message-field/register jumps (#29 CHI) now interleave with bundle-restoration confirmations (#30 AIA) in the tail. **#31 (`2026-06-24`) — CHI C2C 2026 variant `ihi0098_a_b` re-confirms the CHI-C2C family marquee:** `message_field_records` 0→143 / 12 containers (sibling of #29's 0→149), `.isf` strict-clean; the lone stale relation 1→0 (fragment drop). After #31: **25 real chip-spec docs still normalized-missing.**

**Cumulative update after #37:** 37 current-binary refreshes are complete and 19 real chip-spec documents remain.
AArch64 External Debug joins CoreSight Base System, USB4 Connection Manager, USB4 Inter-Domain, USB 3.2, and HBM-gen1 as honestly blocked/non-emitting; 66 current renderable
adapters remain, all FSMGen-strict clean. The preceding long-form cumulative paragraph is the historical
through-#33 snapshot; this update is the current corpus frontier.

**Spun-out CODE levers (`.2` re-ingest is SURFACING + scoping these — measurement-first; each needs its OWN owned leaf under the `ISF-*-EMIT` family, with WIRE-BASED-100 + register/wire golds + `kg-bench` gating; do NOT fix inside a re-ingest slice):**

- **Lever B — ISF module-name HDL-sanitization — ✅ DONE `2026-06-21` (`KG-ISF-COMPLETENESS.2a.iii`, owner-chosen).** GIC-600's `.isf` had failed FSMGen strict with `Malformed top-level FSM source '?fsm:redistributor→_distributor…'`. Fixed by flipping `sanitize_isf_name` (`ir/isf_ir.rs`) from a char denylist (which missed the arrow `→`) to an allowlist (`[A-Za-z0-9_]`, else `_`) and routing `derive_isf_actor_name`'s module label through it. GIC-600 now re-checks FSMGen `success=true` / 0 diagnostics; wire golds byte-identical; `run_ci.sh` GREEN (lib 1679); `kg-bench` 156/156. KM `isf-module-name-hdl-sanitization`. (Latent agent-identity gap — an arrow-phrase actor slipping the `.1a`/`.1b` gates — remains a separate future candidate.)
- **Lever A — ISF value width-alignment — ✅ RESOLVED (re-verified strict-CLEAN `2026-06-23`; the #8 log was STALE).** Re-emitting DTI (`ihi0088_g`) with the current binary + `subs/fsmgen/bin/fsmgen --strict --check --json` returns **success / 0 diagnostics** — `ATST` now emits `1'd1` (width-1), not the `2'b1` the #8 row recorded. The width-alignment was delivered by `ISF-VALUE-WIDTH-EMIT.2` (README line 110); the #8 strict-FAIL entry was never re-verified after that landed. No code needed; drift corrected here. (Caught by the `KG-ISF-COMPLETENESS.2a.iv` probe re-verifying the strict tally.)
- **Lever C — rule-write conflicts — ✅ RESOLVED (`2026-06-23`, `KG-ISF-COMPLETENESS.2a.v`).** LPI: `isf_conflicting_rule_writes` on `PREQ` — `rule_5` (PREQ←1, an UNCONDITIONAL rule_action) conflicts with `temporal_..._dyn_sigcon_0012` (PREQ←0, a guarded temporal signal constraint). The same-guard `dedup_conflicting_rules` keyed on `(signal, guard)` so missed it; FSMGen flags it because an unconditional rule's firing set ⊇ every guard (its `_condition_terms_prove_disjoint` can never prove an absent condition disjoint). Fixed by a new `drop_unconditional_overlap_conflicts` post-pass (keep the unconditional value, residualize the conflicting rule; the `(priority …)` hatch was rejected as ungrounded precedence). **MEASUREMENT correction:** a fresh re-emit + FSMGen sweep showed the conflict was NOT LPI-only — the AXI/AHB/AXI-Stream wire golds + LTI + NVMe were ALL failing on it (the cached `.isf` were byte-identical to fresh AND already FAIL); the fix takes **6 docs FAIL→PASS**, 0 PASS→FAIL, 100/107 `.isf` byte-identical, post-fix 97/107 `.isf` PASS. The combined AXI+ACE `ihi0022_h_c` stays FAIL on the orthogonal pre-existing `(port expr)` grammar (Non-Goal).
- **Lever F — ISF enum value-literal emit-gate — ✅ DONE `2026-06-23` (`KG-ISF-COMPLETENESS.2a.iv`).** HBM2's `.isf` had failed FSMGen strict on `TABLE.REPAIR_LANE_8` value token `1000`. FSMGen rejects a bare token of only `0`/`1` digits with length >= 4 (an un-qualified binary literal — verified by value sweep: `1000`/`1111` fail, `69152`/`999`/`4'b1000` pass); HBM2's REPAIR_LANE values are binary codes mis-read as bare decimals in a mega-conflated `TABLE`. The emitter now drops an enum carrying such a value to an `isf_enum_value_literal_*` residual (honest residual over fabricating a radix). HBM2 now FSMGen `--strict --check` **success / 0 diagnostics** (only `TABLE` dropped; `EXTEST_RX`/`DWORD_MISR` kept); ONLY HBM2 `.isf` changes corpus-wide (wire golds + all enum docs byte-identical); `run_ci.sh` GREEN (lib 1706); `kg-bench` 156/156. KM `isf-enum-value-literal-emit-gate`. (The upstream mega-enum conflation stays an honest residual → a future extraction-precision lever.)

**Running strict tally (post-`.2.37`, measured on the current converged output set):** `adapt` emits one
primary-actor `.isf` per renderable document. The cache now contains **66 current `.isf`: 66 PASS / 0 FAIL**;
USB #33, USB4 #34, USB4 Connection Manager #35, CoreSight Base System #36, and AArch64 External Debug #37 are honestly blocked and emit none. This confirms both that the grammar frontier remains closed and that
strict syntax is not a semantic-fidelity oracle. **Levers A, B, C, F + the rule-value gate all ✅ resolved:** Lever B (module-name HDL-sanitization, `.2a.iii`); Lever A (value width-alignment, `ISF-VALUE-WIDTH-EMIT.2`, re-verified clean `2026-06-23`); Lever F (enum value-literal emit-gate, `.2a.iv`, HBM2 strict-clean); **Lever C (unconditional-rule-overlap conflict residual, `.2a.v`, `2026-06-23` — 6 docs FAIL→PASS incl. all 3 wire golds + LPI/LTI/NVMe, 0 regressions)**; **`.2a.vi` rule-drive-value validity gate (`2026-06-23` — closed the last AXI+ACE `ihi0022_h_c` `(port expr)` FAIL by residualizing its 4 prose-valued loopback rules)**. **The ISF-emit strict-FAIL frontier is fully CLOSED — every renderable doc emits FSMGen-`--strict`-clean `.isf`.** **Surfaced lever D (upstream/non-emitter, now seen on 4 docs):** non-AMBA table/signal recognition — Wishbone-style `*_O`/`*_I` suffix-notation + prose signal-lists, OpenCAPI-style packet/command tables left `unknown` (219/246), and the RISC-V-IOMMU/VT-d in-memory STRUCTURE tables — a `PDF-VARIANT-DIGESTION` signal/table-recall family (kin to the parked `.9.10`). **Surfaced lever E (#27 eMMC — KG-precision, NOT emitter):** **agent-identity precision for descriptive-prose / non-AMBA specs** — fresh Docling of prose-heavy specs mints ~90+ phantom sentence-fragment actors (`host has`/`host to`/`cache in`/`B write`) the `.1a`/`.1b` gates (measured clean on AMBA docs) don't catch; needs its OWN owned `KG-ISF-COMPLETENESS` leaf (probe-first, structural gates not denylists — owner steer). eMMC itself remains ISF-unaffected; USB #33's semantic-trust repair is now closed and is not a syntax strict-FAIL. Watch the long-tail for width-align (A) / rule-conflict (C) / non-AMBA-table-recall (D) / prose-actor-precision (E) / enum-value-literal (F) recurrence to scope those.

**PHASE FINDING (#22, the already-current-binary-equivalent class is now the RULE, not the exception):** a one-pass survey of the 35 remaining normalized-missing docs' persisted evidence (`2026-06-23`) shows the register-heavy ones the frontier had flagged as "genuinely pre-`.10`-stale" are ALREADY current — CoreSight SoC-600 ×3 = 597/631/833 `register_records`, AMD-IOMMU `48882` = 217 `message_field_records`, Intel VT-d = 103 registers — because their evidence was rebuilt from the retained `source_ir.json` before the `normalized/` bundle was reclaimed (the #21/#22 class). So the marquee-jump phase (SMMU/GIC/MMU/DTI/CHI-C2C) is effectively complete; the remaining ~35 slices are predominantly **bundle-restoration + current-binary confirmation + honest-absence / Lever-D-recall documentation**, which is still genuine corpus-coverage work (a doc that depends only on a retained `source_ir.json` is one disk-reclaim away from losing its evidence; restoring `normalized/` makes it rebuildable). A true pre-`.10`-stale jump remains possible in the long tail but is now the exception.

## Changelog

- `2026-08-09`: `.2.38a` CODE/DATA/DOC DONE. Validating copied #38 artifacts exposed that all five persistence
  helpers backannotated IR through embedded canonical layouts while reports followed the copy. The explicit-path
  writer now updates only the CLI artifact plus its adjacent report. One hermetic all-stage regression and one
  real five-stage release replay keep the complete canonical tree byte-exact; canonical-path tests stay green.
  Clippy, 1,788 tests / five ignored, rustdoc, mdBook, doctrines, persisted paths, and locality pass. The required
  status record triggers an exact rollover of 12 oldest live duplicates into sealed segment `0005`, leaving 60
  warning-safe records. Parent #38 resumes from authenticated-source regeneration.
- `2026-08-09`: `.2.37` DATA/DOC DONE. Guarded AArch64 External Debug ingest produces 25 pages / 11 visuals /
  two tables / 36 sections / 266 elements at 19% peak system memory used; 25/25 page paths are final and
  repository-relative. Exactly 81 stale visual-label body records disappear while every structural class holds.
  Evidence retains 243 statements / five conditionals / two normative facts and zero relations or declared
  signals. Current generic authority removes 64 heuristic interfaces, one visual-label actor, and the 73-signal
  `agent.isf`; the high-confidence methodology guide blocks honestly and leaves only `adapter.json`. Two
  cascades reproduce nine hashes; WIRE/I2C/SWD, KG 156/156, 66/66 strict, persisted-path, and locality gates
  pass. Corpus is 37 done / 19 remaining; frontier → select and own #38.
- `2026-08-09`: `.2.36` DATA/DOC DONE. Guarded CoreSight Base System ingest produces 29 pages / 19 visuals /
  12 tables / 404 elements at 17% peak system memory used; 29/29 page paths are final and repository-relative.
  Source structure and EvidenceIR counts hold exactly, including zero signal declarations/relations. Current
  generic authority removes 88 stale heuristic interfaces and the 100-signal/two-rule `agent.isf`; lowering
  blocks on no declared interface signals and leaves only `adapter.json`. Validation surfaces an honest upstream
  capture-recall residual—19 unenriched visuals, 87 partially structured normative statements, and seven
  temporal-source rules without typed grounding. Two cascades reproduce nine hashes; WIRE/I2C/SWD, KG 156/156,
  67/67 strict, mdBook/doctrines/path/locality, and exact cleanup/residue gates pass. Corpus is 36 done / 20
  remaining; frontier → select and own #37.
- `2026-08-09`: `.2.35` DATA/DOC DONE. Guarded USB4 Connection Manager ingest produces 96 pages / 46 visuals /
  23 tables / 1,313 elements at 28% peak system memory used; 96/96 page paths are final and repository-relative.
  The stale 13-relation / four-interface / 11-port chain emitted `SB`/`USB`/`USB4`, three rules, and one enum;
  current generic authority retains source evidence but yields zero relations/interfaces/ports/connectivity.
  The methodology guide blocks on no declared interface signals, removes `device_also.isf`, and leaves only
  `adapter.json`. Two cascades reproduce nine hashes; WIRE/I2C/SWD, KG 156/156, 68/68 strict, mdBook/doctrines/
  locality, and exact cleanup/residue gates pass. Corpus is 35 done / 21 remaining; frontier → select and own #36.
- `2026-08-09`: `.2.34b.ii.b` DATA/DOC DONE; `.ii`/`.b`/`.2.34` CLOSED. Two guarded USB4 ingests reproduce
  51 pages / 82 visuals / 49 tables / 603 elements at 18% peak system memory used. All 51 sidecars are
  repository-relative and final-rooted; their path field is the only difference from the preserved defective
  bundle, and every downstream artifact hash is byte-identical. Current generic authority removes the stale
  one-signal/two-rule/eight-enum model; the adapter blocks at zero interfaces/signals/rules and leaves exactly
  `adapter.json`. Nine WIRE/I2C/SWD datasets, KG 156/156, 69/69 FSMGen strict, full CI/book/doctrines/locality,
  and exact residue gates pass before authenticated rollback/evidence deletion. Corpus is 34 done / 22 remaining;
  frontier → select and own #35.
- `2026-08-09`: `.2.34b.ii.a` CODE DONE. Fresh USB4 exposed 51 page JSON sidecars retaining absolute staging
  image paths. Added a pre-swap page-sidecar rewrite that cross-checks summary identity, rejects traversal and
  symlink escape, persists the final repository-relative image path, keeps the intentional no-image `null`, and
  deletes only staging on any malformed record. Four focused page tests, existing metadata tests, two stub
  ingests, formatting, and warning-deny Clippy pass. Frontier → `.ii.b` real USB4 rerun/signoff.
- `2026-08-09`: `.2.34b.i` DATA/DOC RECOVERY DONE. A temporary fresh-SWD oracle command resolved its artifact
  root to the canonical cache. Preserved the untouched downstream chain, then validated and rebuilt it from the
  promoted EvidenceIR. Current generic dense-prose authority removes `LEVEL`, three phrase-shaped actors, and
  four relations while retaining all 29 serial facts. Two cascade replays are byte-identical; SWD derivation,
  base relation, FSMGen strict, focused/WIRE, and KG 156/156 gates pass. The closing locality gate separately
  exposed 51 USB4 page sidecars with absolute staging paths, now owned by `.2.34b.ii.a`; the fresh USB4 chain was
  preserved exactly and canonical USB4 restored to its authenticated baseline. Frontier → `.ii.a`, then `.ii.b`.
- `2026-08-09`: `.2.34a` PROBE/DOC DONE; `.2.34b` locality-blocked before ingest. Selected USB4 Inter-Domain
  Service because its retained one-signal/two-rule `USB4` adapter directly tests #33's generic bus-acronym repair.
  The host-library symlink still resolves to a 935 MiB boot-volume Git checkout; no matching SSD checkout/PDF was
  found, contrary to the director's storage statement. Candidate PDF SHA-256 is `ab337460…396`. No copy, link,
  pipeline, rollback, or generated artifact changed. Frontier awaits the intended SSD route or explicit verified
  one-time read-only copy authority.
- `2026-08-09`: `.2.33d.iv.c` DATA/DOC DONE; `.iv`/`.d`/`.2.33` CLOSED. Authenticated the 9-file /
  38,981,655-byte rollback and reproduced byte-identical downstream USB stage artifacts with release
  `946766cb…488` before and after documented SourceIR validation backannotation.
  Final typed surfaces are 0 actor relations/interfaces/interface records/ports/adapter signals/transactions/rules,
  one storage record, 15 adapter residuals, and exactly `adapter.json`. Five focused command groups, all nine
  provider-free WIRE datasets, KG 156/156, and full CI (1,783 pass / five ignored) plus rustdoc/mdBook/six
  doctrines/path/locality pass. Only then was the exact rollback deleted; exact-path and task-id residue are zero.
  Corpus emit cache is 70 current `.isf`, all strict-clean. Frontier → select and own refresh #34 under `.2`.
- `2026-08-09`: `.2.33d.iv.b` CODE/DOC DONE. Corpus measurement found 21 authority-empty documents with
  5,527 interfaces / 18,397 all-low records feeding 4,060 adapter signals; DTI's lone `DOWNSTREAM` relation is
  prose, not a wire-authority exception. The replacement preserves signal-led deontic `VALID`/`READY` evidence
  but all 21 affected documents still dry-run to zero interfaces; wire-gold surfaces are byte-equivalent. Real
  USB blocks at zero signals/rules and leaves only `adapter.json`. WIRE 1.000, KG 156/156, and full CI (1,783
  pass / five ignored) are green; frontier → `.iv.c`, rollback still preserved.
- `2026-08-09`: `.2.33d.iv.a` CODE/DOC DONE. Real USB reproduced a stale prior actor file outside the current
  adapter manifest. Successful writes now reconcile obsolete regular/symlink `.isf` leaves, including blocked
  transitions, while preserving unrelated material. Eight focused adapter tests, warning-deny Clippy, full CI
  (1,780 pass / five ignored), rustdoc, mdBook, doctrines, and locality pass. The cascade separately proves an
  authority-empty 918-interface/556-output blocker; frontier → `.iv.b`, and the exact rollback remains preserved.
- `2026-08-09`: `.2.33d.iii` MEASURE/DOC DONE. Both affected Rust files remain byte-identical to `.d.ii`; runtime
  path inspection and five focused tests confirm relations/directions cannot originate a name rejected at the
  three catalog authorities. The downstream backstop is closed as redundant; `.d.iv` is active for real USB
  cascade signoff.
- `2026-08-09`: `.2.33d.ii` CODE/DOC DONE. Both declaration catalogs now require canonical
  `Signal <identifier> is <predicate>` grammar (including the broad-gate-preserved `inout` contract), `bus` is
  no longer a width-one parenthetical head, and port/pin tables need explicit compact inventory structure. The
  combined regression closes `AT`/`USB`/`ENHANCED`/`NO` across catalog, relation, direction, and provenance;
  equal producer ties remain lexicographically last. Serial/table/full-library, WIRE, KG, Clippy, full CI,
  mdBook, doctrines, and locality pass. The mandatory engineering-notes rollover seals 27 records exactly;
  its archive index is newly at the controlled 81.2% lines / 81.5% bytes warning, below 90% rollover. Frontier
  after commit → `.2.33d.iii` measured-unnecessary closure.

- `2026-08-09`: `.2.33d.i` PROBE/DOC DONE. The all-artifact census separates three unsupported signal-authority
  promotions from the downstream direction amplifier: 97 weak sentence-start declaration matches, 17
  bus-acronym width-one captures, and 11 accepted port/pin-only table shapes. Selected the bounded `.d.ii`
  grammar/structure repairs, proved `.d.iii` needs no separate heuristic if convergence/USB regressions close,
  measured producer ties in nine IntentIR docs, and routed the full evidence to the research catalog and durable
  fact. No production/generated artifact changed. Frontier after commit → `.2.33d.ii`.

- `2026-08-09`: `.2.33c` DATA/DOC DONE. Completed and validated USB 3.2 through SemanticIR, IntentIR,
  adapter, and real FSMGen strict. SourceIR 6,584→5,830 is visual-label suppression, not prose loss: every
  loss page has a visual, short affected-page fragments fall 543→3, and 548/507/283 pages/visuals/tables hold.
  The emitted ISF is syntactically clean but semantically blocked: four weak false-signal seeds reinforce through
  relation-derived directions, then a phantom request actor wins initiator selection. Added the durable fact and
  pending universal repair `.2.33d`; removed the exact one-file rollback after comparison and zero-residue proof.
  Corpus now 80 SourceIR / 2 normalized / 80 EvidenceIR / 79 downstream chains; 33 refreshes complete / 23 left.
  KG bench 156/156; doctrine/book/locality gates complete the commit workflow. Frontier → `.2.33d`.
- `2026-08-09`: `.2.33b` CODE/DOC DONE. Both declaration catalogs now call one valid-prefix boundary helper;
  a multi-byte-marker non-declaration and a real declaration after non-ASCII prose directly prove safety and
  retained behavior. Rebuilt release `1710e5…` constructs/validates USB EvidenceIR from the promoted SourceIR
  without re-ingest (918 anchors / 8,267 spans / 507 visuals / 22,029 links / 8,412 statements). Focused
  EvidenceIR 252/252 plus five ignored and full CI 1,776/1,776 plus five ignored pass with all doctrines,
  warning-deny Clippy, rustdoc, and mdBook. The mandatory changelog rollover seals 29 aged-out records in
  authenticated segment 0001 and restores the 85 ordinary + two detached live view. After commit, frontier →
  `.2.33c` downstream cascade and quality disposition.
- `2026-08-09`: `.2.33a` PROBE DONE. Preserved a 4.9 MiB byte-identical same-volume SourceIR baseline;
  rebuilt release `9cd700…`; authorized-external CPU ingest produced USB 3.2 SourceIR at 548 pages / 507 visual /
  283 tables / 5,830 elements / zero residuals with 81–82% RAM free. The repo-relative symlink launch first
  failed closed as designed. Evidence then panicked before output on `- U+F0B7 signal ...`: both declaration
  catalogs slice `idx - 2..idx` even though `match_indices` returns a byte offset. Added the durable fact and
  split the repair/resume into `.2.33b`/`.2.33c`. The architecture entry also triggered its declared line-based
  rollover; four whole post-capsule records now live content-identically and in order in sealed segment 0001
  (canonical terminal newline), and the current window is 60 records. Commit: see
  `CORPUS-COVERAGE.2.33a — surface USB UTF-8 EvidenceIR blocker`.
- `2026-08-09`: `.3` lifecycle/currentness audit DONE. A live census found 80 SourceIR / 1 normalized / 79
  EvidenceIR / 78 downstream chains, apparently contradicting `.2`'s 32 restored bundles. Durable history
  root-caused this to the intentional `2026-07-05` normalized-cache cleanup; all five retained stages for the 32
  completed refresh keys remain present (160/160). Progress is therefore 32 current-binary refreshes complete /
  24 real chip-spec documents unrefreshed, independent from ephemeral normalized retention. Added KM fact,
  corrected the AIA book drift, and passed Knowledge Map/catalog, mdBook doctests/build, and locality checks.
  Commit: see `CORPUS-COVERAGE.3 — reconcile refresh progress with normalized-cache cleanup`. Frontier returns
  to `.2`; next owned leaf will be `.2.33` USB 3.2.
- `2026-06-24`: `.2` re-ingest **#31 — AMBA CHI C2C 2026 variant `ihi0098_a_b`** (`ihi0098_a_b_2026_02_03_amba_chi_chip_to_chip_c2c_architecture_specification`, 122pp / 96 visual) — fresh-session PNT slice (binary current). Docling CPU (0 residuals; RAM ~75% free) → deterministic cascade. **Marquee `message_field_records` 0→143 / 12 containers** (sibling of #29's 0→149 — confirms the CHI-C2C family is a genuine marquee, not a one-off); transactions 2→3 (recognition-only); lone stale `actor_signal_relations` 1→0 (fragment drop by the current agent-identity gates — coherency/message protocol, honest 0); conditional_rules 25 held; 0 registers; normalized bundle RESTORED. `agent.isf` renderable (69 ports / 3 enums / 0 rules), **FSMGen `--strict --check --json` 0 diagnostics**. Same surfaced residuals as #29 (generic-`TABLE` mega-enum + signal-acronym noise — not fixed in-slice). After #31: 31 re-ingested, 25 real chip-spec docs still normalized-missing.
- `2026-06-24`: `.2` re-ingest **#30 — RISC-V AIA `1_0_2025_03_12`** (`1_0_2025_03_12_risc_v_advanced_interrupt_architecture`, 89pp / 105 visual) — fresh-session PNT slice (continuing the `.2` sweep, binary already current from #29 so no rebuild). Docling CPU (0 residuals; RAM ~75% free) → deterministic cascade. **Bundle-restoration + current-binary confirmation (the #21/#22 class):** all deterministic surfaces byte-near-identical to the retained-`source_ir` evidence (statements 1193, register_records 0, message_field_records 0, actor_signal_relations 0, conditional_rules 39 — all held); intent 8 actors / 0 rel / 0 txns. **normalized bundle RESTORED** (was missing). Honest absence: 0 registers/relations (memory-mapped interrupt ISA whose APLIC/IMSIC register layouts sit in 12 `unknown`-classified structured tables that don't match the `.10` families — re-confirms the #21 RISC-V-IOMMU Lever-D structure-table recall opportunity, not a regression). `agent.isf` renderable (91 ports / 5 enums / 22 rules), **FSMGen `--strict --check --json` success / 0 diagnostics**. After #30: 30 re-ingested, 26 real chip-spec docs still normalized-missing.
- `2026-06-24`: `.2` re-ingest **#29 — AMBA CHI C2C `ihi0098_a`** (`ihi0098_a_2024_02_07_amba_chi_chip_to_chip_c2c_architecture_specification`, 108pp / 88 visual) — fresh-session PNT slice (first eligible leaf of the first active tree after `KG-ISF-COMPLETENESS` ran out of immediately-buildable leaves). Release binary first rebuilt to current (the persisted one predated `.2a.v`/`.2a.vi`). Docling CPU (0 residuals; RAM steady ~70% free) → deterministic `evidence→semantic→intent→adapt` cascade. **Marquee message-field refresh: `message_field_records` 0→149 / 13 containers** (the stale evidence was truly pre-`.10`; the current `.10` families fire on CHI's packet/flit field tables — a real KG-completeness gain, not bundle-restoration). 0 registers / 0 relations (coherency/message protocol — honest absence, `KG-ISF-COMPLETENESS.3`); conditional_rules 17 + transactions 2 held; normalized bundle RESTORED. `agent.isf` renderable (69 ports / 3 enums / 1 rule) and **FSMGen `--strict --check --json` success / 0 diagnostics**. Surfaced (extraction-precision, not emitter; not fixed in-slice): generic-`TABLE` mega-enum conflation (#28 class) + signal-inventory prose-acronym noise (`AES`/`AMBA`/`ARM` — `KG-ISF-COMPLETENESS.4` class). After #29: 29 re-ingested, 27 real chip-spec docs still normalized-missing. WIRE-BASED-100 orthogonal (gold docs not re-ingested); `generated/` git-ignored → the `.2` log table is the durable trace.
- `2026-06-23`: **Lever F RESOLVED + Lever A drift CORRECTED (via `KG-ISF-COMPLETENESS.2a.iv`).** A fresh-session
  probe re-verified the "3 strict-FAIL" tally against the current binary + real FSMGen: **(a) DTI (Lever A) is
  already strict-CLEAN** — `ATST` emits `1'd1`, FSMGen `--strict --check` 0 diagnostics; the #8 row's `2'b1`
  strict-FAIL was a STALE log entry (the width-alignment landed in `ISF-VALUE-WIDTH-EMIT.2` and was never
  re-verified). **(b) HBM2 (Lever F) NOW strict-CLEAN** — `.2a.iv` added an enum value-literal emit-gate: FSMGen
  rejects a bare `[01]`-only token of length >= 4 (an un-qualified binary literal; verified by value sweep), and
  HBM2's `TABLE.REPAIR_LANE` binary codes were mis-read as bare decimals, so the malformed mega-enum is now
  residualized (`isf_enum_value_literal_table`) instead of emitted; only `TABLE` dropped, `EXTEST_RX`/`DWORD_MISR`
  kept; HBM2 FSMGen `--strict` 0 diagnostics. **(c) LPI (Lever C) remains the ONLY open strict-FAIL** (cross-surface
  rule-conflict on `PREQ`). **Running tally: 27 of 28 renderable docs strict-clean, 1 FAIL (LPI).** Corpus-wide
  byte-identical EXCEPT HBM2 `hbm.isf` (wire golds + GIC-600/CoreSight/ARM-Debug/Avalon enums all carry legit
  decimals the binary-token criterion never flags); `run_ci.sh` GREEN (lib 1706); `kg-bench` 156/156; no re-ingest
  (read-only verification + emitter-only code change). See `docs/tasks/KG-ISF-COMPLETENESS.md` `.2a.iv`.
- `2026-06-23`: `.2` re-ingest **#28 — JEDEC HBM2 DRAM** (`jesd235a_2015_11_hbm2_dram`, 172pp / 224 visual) —
  fresh-session PNT slice, register/TRM phase, **deliberate diagnostic pick (DRAM register/timing spec — the structured
  contrast to #27's descriptive prose)**. Docling CPU (0 residuals; RAM 81% free; ~2 min) → deterministic cascade.
  **MIXED refresh:**
  - **CONFIRMS the #27 phantom explosion is PROSE-SPECIFIC:** HBM2 actors **52→38 CONSOLIDATED DOWN** (the `.1a`/`.1b`
    agent-identity gates fold fragments, exactly like the structured AMBA/CoreSight class — the OPPOSITE of eMMC's 20→153
    explosion), relations 89→55, interfaces 72→31. So lever E (prose-actor-precision) is scoped to descriptive-prose docs,
    not register/timing-table docs. **GENUINE win:** transactions 0→3 [`read`/`write`/`trr_mode`_operation, recognition-only],
    register_records 17 held, 0 message-fields (honest). `validate` no stage-staleness, 55/100 ADEQUATE.
  - **BUT `.isf` strict-FAILS → NEW spun-out lever F (ISF-emitter, kin to Lever A):** the emitter built a generic-named
    `(type TABLE (bits 6))` mega-enum conflating ~10 distinct doc tables (REPAIR_LANE codes + microbump-pitch descriptions +
    test-operation list + IDD currents + mode-register refs) with restarting/duplicate values, and emits the REPAIR_LANE
    BINARY codes (`0,1,10,11,…,1000,1111`) as bare decimal-looking tokens → FSMGen rejects `TABLE.REPAIR_LANE_8` value `1000`
    (package symbol values must be width/radix-qualified scalar literals like `4'b1000`); the bare token is ALSO semantically
    wrong (binary 1000 ≠ decimal 1000). Lever F = ISF enum-member value literal format + binary-radix preservation (the
    `ISF-*-EMIT` value-literal family), plus a related enum-extraction-precision concern (the generic `TABLE` mega-enum that
    conflates many distinct tables — an enum analogue of the #27 phantom-actor precision gap).
  - **Phase finding:** two consecutive re-ingests (#27/#28) each surfaced a NEW substantive lever (E precision, F enum-literal)
    rather than a clean refresh — the re-ingest tail is now reliably a LEVER-SURFACING exercise. The diagnostic value (scoping
    E to prose, finding F) is high, but it reinforces that the next high-value work is ACTING on the surfaced levers, not
    grinding more re-ingests.
  - **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction; binary
    current; `scripts/check_doctrines.sh` GREEN; no book change (HBM2 adds 0 message-fields). 25/28 re-ingested docs strict-clean
    (3 FAIL: DTI Lever A, LPI Lever C, HBM2 Lever F). Coverage: 28 of 57.
- `2026-06-23`: `.2` re-ingest **#27 — JEDEC eMMC 5.0** (`jesd84_b50_2013_09_emmc_5_0`, 296pp / 344 visual) —
  fresh-session PNT slice, register/TRM phase, **first descriptive-prose register/protocol spec re-ingested**. Docling
  CPU (0 residuals; confidence high; RAM steady 80–82% free, `.4a` guard armed, Ollama idle, ~5 min) → deterministic
  cascade. **MIXED refresh — genuine transaction win BUT a phantom-actor explosion:**
  - **GENUINE wins:** **transactions 0→6** [`boot`/`alternative_boot`/`device_lock_unlock`/`dual_data_rate_mode`/`background`/`h_w_reset`_operation
    — 5 with grounded signal set (9 members), all real eMMC operations]; register_records 17 held; interfaces 9→38;
    signal_constraints 0→2; 0 message-fields (honest — eMMC has no bit-position structure tables); normalized bundle
    restored (rebuildable). `.isf` renderable (`host.isf`, 62 signals / 17 storage(reset) / 28 enums / 16 rules / 0 txns);
    real `fsmgen --strict --check --json` **success / 0 diagnostics** (strict-clean — the emitter lowers only the
    renderable `host` actor). `validate` no stage-staleness (relations 349 non-zero, silent), quality 51/100 ADEQUATE.
  - **REGRESSION surfaced (honest, NOT papered over):** **actors 20→153 / relations 23→349** — the fresh Docling
    extraction (7017 statements vs stale 6544) mints ~90+ phantom sentence-fragment actors (`host has`/`host to`/`cache in`/`B write`/`CMD to`/`device behaves`/`following`/`value`)
    that the `.1a`/`.1b` agent-identity precision gates — measured clean on AMBA-style docs — do NOT gate for eMMC's
    descriptive prose (esp. real-agent+trailing-auxiliary/preposition `host has`/`host to` that `.1b.i`'s trailing-verb
    strip misses). The phantoms never reach `.isf` (the raw `actors[]` is not lowered), so this is a **KG-fidelity gap
    (north-star bar #1), not a strict-FAIL**. → **NEW spun-out lever E: agent-identity precision for descriptive-prose /
    non-AMBA specs** (`KG-ISF-COMPLETENESS` family; probe-first, structural gates not denylists per owner steer), HIGH-VALUE
    because the remaining prose tail (USB/HBM/guides) likely recurs it.
  - **Phase finding:** the descriptive-prose doc-class behaves OPPOSITE to the structured AMBA/CoreSight class — fresh
    re-ingest *worsens* the actor surface (more fragment candidates) rather than refining it. The genuine refresh value
    (transactions, registers, bundle restoration) is real but is mixed with the phantom-actor noise.
  - **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction;
    binary current; `scripts/check_doctrines.sh` GREEN; `mdbook build` GREEN (no book number changed — eMMC adds 0
    message-fields). 25/27 re-ingested docs strict-clean. Coverage: 27 of 57.
- `2026-06-23`: `.2` re-ingest **#26 — Intel VT-d 5.0** (`5_0_2024_08_intel_virtualization_technology_for_directed_io_specification`,
  354pp / 810 visual) — fresh-session PNT slice, register/TRM phase, **first distinct-vendor (Intel) re-ingest of the tail**.
  Docling CPU (0 residuals; confidence high; RAM steady 80–83% free, `.4a` guard armed, Ollama idle, ~4 min) → deterministic
  cascade `evidence`→`semantic`→`intent`→`adapt --target isf`. **GENUINE refresh — CORRECTS the #22 "already
  current-binary-equivalent" survey expectation:** the one-pass register-count survey flagged VT-d's 103 registers as current,
  but its stale evidence predated BOTH the message-field family AND the section-heading transaction recognizer →
  register_records 103 held (exact), **`message_field_records` (key absent)→15 / 4 containers** (NEW — the `.10c`
  structure-field family fires on VT-d's `Root-Entry Format`/context-table structures; **actualizes the book's existing
  `.10c` "VT-d gains 15 typed structure fields" claim** the stale persisted evidence never reflected → corpus-wide
  in-memory-structure total measured 1,220/11→**1,235/12 docs**), **transactions 0→2** [`device_tlb_operation`,
  `set_root_table_pointer_operation`, both recognition-only — honest], actors 14→13 (consolidation), signal_constraints
  1→4, temporal_rules 1→4; 0 relations (memory-mapped register spec — honest absence, `KG-ISF-COMPLETENESS.3`).
  **`.isf` renderable** (`agent.isf`, 510 signals / 103 storage(reset) / 4 enums / 44 rules / 0 txns); real
  `subs/fsmgen/bin/fsmgen --strict --check --json` **success / 0 diagnostics** (strict-clean). `validate` no stage-staleness
  (0-vs-0 honest absence, silent), quality 8/100 INCOMPLETE (honest for a memory-mapped register/structure spec lacking
  wire direction/width/clock/reset grounding — the #21 RISC-V-IOMMU class). **Honest residuals:** register bit-fields +
  field-resets UNLOCATED (6 adapter `residual_decisions`); doc-completeness gauge 7/17 unexplained intent-bearing tables
  (a Lever-D structure-table recall opportunity, like #21's RISC-V structure tables). **Phase correction:** the one-pass
  register-count survey is NOT sufficient to call a doc "already current" — a register-current doc can still predate the
  message-field/transaction surfaces and carry a real refresh; the marquee table-family jump remains the exception but the
  refresh value is consistently under-counted by register count alone. Binary current (no rebuild). No code change →
  WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction. Book `document-categories.md`
  in-memory-structure count synced 1,220/11→1,235/12 (reconciles a latent inconsistency vs `evidenceir.md` `.10c`).
  Coverage: 26 of 57.
- `2026-06-23`: `.2` re-ingest **#25 — CoreSight SoC-600 0100 TRM** (`100806_0100`, 702pp / 1157 visual) —
  fresh-session PNT slice, register/TRM phase; **completes the CoreSight SoC-600 cluster** (0701=#23, 0200=#24,
  0100=#25). Docling CPU (0 residuals; RAM steady 67–77% free, `.4a` guard armed, Ollama idle; ~5 min) →
  deterministic cascade. **GENUINE refresh (same #23/#24/#17 class):** register_records 597 held (exact),
  **actor_signal_relations 41→31 / actors 44→34 / interfaces 12→5** (consolidation), **transactions 0→1**
  (recognizer fires); conditional_rules 46 held; 0 message-fields (honest). **`.isf` renderable** (`dp.isf`, 597
  storage(reset) / 12 enums / 3 rules / 4 signals); real `fsmgen --strict --check --json` **success / 0
  diagnostics** (strict-clean). `validate` no stage-staleness (relations 31-vs-31), quality 52/100 ADEQUATE.
  Honest residuals: register bit-fields + field-resets not lowered (UNLOCATED); 1 `isf_temporal_unrepresentable`
  (honest, not emitted). The 3 SoC-600 versions are uniformly the consolidation+transaction refresh class. Binary
  current (no rebuild). No code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by
  construction. Coverage: 25 of 57.
- `2026-06-23`: `.2` re-ingest **#24 — CoreSight SoC-600 0200 TRM** (`100806_0200`, 761pp) — fresh-session PNT
  slice, register/TRM phase. Docling CPU (761 page artifacts / 1302 visual / 0 residuals; RAM steady 74–79% free,
  `.4a` guard armed, Ollama idle; ~5 min) → deterministic cascade (evidence). **GENUINE refresh (same #23/#17
  class):** the stale evidence predated the `.1a`/`.1b` consolidation gates + transaction recognizer →
  register_records 631 held (exact, no fresh-Docling variance), **actor_signal_relations 47→35 / actors 46→37 /
  interfaces 13→6** (consolidation folds fragment/phantom actors), **transactions 0→1** (recognizer fires);
  conditional_rules 51 held; 0 message-fields (honest). **`.isf` renderable** (`dp.isf`, 631 storage(reset) / 9
  enums / 3 rules / 4 signals); real `fsmgen --strict --check --json` **success / 0 diagnostics** (strict-clean).
  `validate` no stage-staleness (semantic+intent relations both 35 → non-zero, silent), quality 52/100 ADEQUATE.
  Honest residuals: register bit-fields + field-resets not lowered (UNLOCATED); 1 `isf_temporal_unrepresentable`
  (a temporal_signal_constraint FSMGen can't represent — honest, not emitted). Completes the CoreSight SoC-600
  cluster's 0200 version (0701 = #23; 0100 pending). Binary current (no rebuild). No code change → WIRE-BASED-100 +
  register/wire golds + `kg-bench` 156/156 orthogonal by construction. Coverage: 24 of 57.
- `2026-06-23`: `.2` re-ingest **#23 — CoreSight SoC-600 0701 TRM** (`100806_0701`, 842pp) — fresh-session PNT
  slice, register/TRM phase. Docling CPU re-ingest (842 page artifacts / 1935 visual / automation_confidence high /
  0 residuals; RAM steady 71–80% free, `.4a` guard armed, Ollama idle; ~9 min) → deterministic cascade
  `evidence`→`semantic`→`intent`→`adapt --target isf` (evidence 17445 statements). **GENUINE refresh (NOT
  byte-identical like #22):** the stale evidence predated the `.1a`/`.1b`/`.1b.iv` agent-identity consolidation
  gates AND the section-heading transaction recognizer → register_records 833→828 (fresh-Docling table-boundary
  variance, minor), **actor_signal_relations 64→41 / actors 60→47 / interfaces 20→8** (consolidation folds the
  fragment/phantom actors the older evidence carried — cleaner KG, the #17 Avalon class), **transactions 0→2** (NEW
  typed surface, recognizer now fires), conditional_rules 74 held, 0 message-fields (no packet/structure tables —
  honest). **`.isf` renderable** (`dp.isf`, 828 storage(reset) / 19 enums / 4 rules / 5 signals); real
  `fsmgen --strict --check --json` **success / 0 diagnostics** (strict-clean — the 3 `isf_rule_conflict` on `ATB`,
  the ISF-RULE-CONFLICT-RESIDUAL family, correctly RESIDUALIZE and are NOT emitted, so unlike LPI #7 the file stays
  strict-valid). `validate` no stage-staleness (semantic+intent relations both 41 → non-zero, silent), quality
  53/100 ADEQUATE. **Honest residuals:** 791 bit-fields + 164 field-resets not lowered (UNLOCATED, located-fields-only
  rule); the 3 `ATB` rule-conflicts. **Phase refinement:** "already current-binary-equivalent" is not uniform — #22
  was fully current (byte-identical) but #23 got a genuine consolidation+transaction refresh, so the tail is a MIX
  of pure confirmation and #17/#23-class refreshes. Binary current (no rebuild). No code change → WIRE-BASED-100 +
  register/wire golds + `kg-bench` 156/156 orthogonal by construction. Coverage: 23 of 57 normalized-missing docs
  re-ingested.
- `2026-06-23`: `.2` re-ingest **#22 — Cortex-A76 TRM** (`100798_0401`, 620pp) — fresh-session PNT slice,
  register/TRM phase. Docling CPU re-ingest (620 page artifacts / 476 visual / automation_confidence high /
  0 residuals; RAM steady 73–83% free, `.4a` guard armed, Ollama idle) → deterministic cascade
  `evidence`→`semantic`→`intent`→`adapt --target isf` (evidence 9534 statements / 185 links). **Honest finding —
  evidence was ALREADY current-binary-equivalent (NOT pre-`.10` stale, the #21 class):** register_records 42 held,
  conditional_rules 26 held, 0 message-fields / 0 relations / 0 interfaces / 0 transactions; IntentIR unchanged
  (9 actors / 7 interfaces / 707 behaviors / 393 constraints / 0 relations) — the `.10c`/`.10g` register families
  predate this doc's stale evidence (rebuilt from the retained `source_ir.json` before the `normalized/` bundle was
  reclaimed), so re-ingest's value is **normalized-bundle restoration + current-binary confirmation**, not a marquee
  jump. **`.isf` renderable** (`agent.isf`, 189 signals / 42 storage(reset) / 13 enums); real
  `fsmgen --strict --check --json` **success / 0 diagnostics** (strict-clean). `validate` shows **no stage-staleness**
  (semantic+intent relations both 0 → 0-vs-0 honest absence, the `.1` detector correctly silent) and IntentIR quality
  24/100 INCOMPLETE (honest for a CPU-core TRM lacking wire direction/width/clock/reset grounding — graph direction
  0%, semantic role 0%). **Honest absences:** 0 relations/interfaces (register/behavioral intent, not wire relations,
  `KG-ISF-COMPLETENESS.3`); 0 message-fields (no packet/structure tables); the 42 registers' bit-fields largely
  UNLOCATED → honest `.isf` residuals (469 bit-fields + 17 field-resets not lowered — the located-fields-only rule,
  no fabrication). **Survey correction:** a one-pass scan of the 35 remaining normalized-missing docs shows the named
  register-heavy candidates (CoreSight SoC-600 ×3 = 597/631/833 regs, AMD-IOMMU = 217 msg-fields, VT-d = 103 regs)
  are ALSO already current-binary-equivalent → the marquee-jump phase is effectively over; the remaining slices are
  the #21/#22 confirmation+restoration class. Binary current (no rebuild). No code change → WIRE-BASED-100 +
  register/wire golds + `kg-bench` 156/156 orthogonal by construction. Coverage: 22 of 57 normalized-missing docs
  re-ingested.
- `2026-06-22`: `.2` re-ingest **#21 — RISC-V IOMMU Architecture Spec** (`1_0_1_2026_02_22_risc_v_iommu_architecture_specification`,
  108pp) — fresh-session PNT slice, register/TRM/ISA phase. Docling CPU (108 pages / 196 visual / 0 residuals / confidence high;
  RAM steady 76–78% free, `.4a` guard armed, Ollama idle) → deterministic cascade `evidence`→`semantic`→`intent`→`adapt --target
  isf`. **Honest finding — evidence was ALREADY current-binary-equivalent (NOT pre-`.10` stale):** register_records 33 held / 147
  fields, signal_constraints 6→8, conditional_rules 47; source_ir is near-identical to the retained Jun-8 capture (13-byte diff) and
  the IntentIR is unchanged (12 actors / 628 constraints / 608 behaviors / 426 ifaces). The `.10c`/`.10g` register families predate
  this doc's Jun-15 stale evidence (which had been rebuilt from the retained `source_ir.json` before the `normalized/` page-image
  bundle was disk-reclaimed), so re-ingest's value here is **normalized-bundle restoration + current-binary confirmation**, not a
  marquee table-family jump. **`.isf` renderable** (`agent.isf`, 175 signals / 33 storage(reset) / 5 enums / 45 rules); real
  `fsmgen --strict --check --json` **success / 0 diagnostics** (strict-clean); `validate` shows **no stage-staleness** (0-vs-0 honest
  absence — the `.1` detector correctly silent) and IntentIR quality 3/100 INCOMPLETE (honest for a memory-mapped register/structure
  spec lacking wire-signal direction/width/clock/reset grounding). **Honest absences:** 0 relations / 0 interfaces (intent lives in
  registers, `KG-ISF-COMPLETENESS.3`); 0 `message_field_records` — of 83 structured tables, the IOMMU's in-memory
  device-context/process-directory/command-queue **STRUCTURE** tables don't match the `.10b`/`.10d`/`.10e` two-column bit-position
  families → a surfaced **RISC-V structure-table recall opportunity** (Lever D), not a regression; 0 transactions (command vocabulary
  not in the section-heading recognizer — honest, like TileLink). Binary current (no rebuild). No code change → WIRE-BASED-100 +
  register/wire golds + `kg-bench` 156/156 orthogonal by construction. Coverage: 21 of 57 normalized-missing docs re-ingested.
- `2026-06-22`: `.2` re-ingest **#20 — GIC-400 TRM** (`ddi0471`, 57pp) — fresh-session PNT slice, first of the
  register/TRM phase pivot. Docling CPU (57 pages / 38 visual / 0 residuals; RAM steady 74–77% free) → deterministic
  cascade (evidence 873 statements). **Healthy refresh (NOT thin):** 5 `signal_description` of 25 Docling tables → a
  16-signal AXI-slave-interface `.isf` with **5 transactions + 4 storage (reset) + 6 relations** held; `fsmgen --strict`
  success / 0 diagnostics; `validate` no stage-staleness, score 56/100. register_records 3→4 only (modest — GIC-400's
  registers largely sit in 14 `unknown`-classified tables, an older/different table style than GIC-600's `.10c`-shaped
  15→33; a minor Lever-D recall opportunity, not the marquee `.10c` gain). Confirms the register/TRM phase yields healthy
  multi-signal `.isf` even when the big `.10c` jump doesn't apply. Binary current (no rebuild). No code change → golds +
  `kg-bench` orthogonal. Coverage: 20 of 57 normalized-missing docs re-ingested.
- `2026-06-22`: `.2` re-ingest **#19 — OpenCAPI 4.0 Transaction-Layer Arch** (`opencapi_4_0_transactionlayer_arch`, 240pp) —
  fresh-session PNT slice. Docling CPU (240 pages / 376 visual / 0 residuals; RAM steady 73% free) → deterministic cascade
  (evidence 3528 statements, IntentIR 797 free-text constraints + 1 transaction + 12 actors). **Honest doc-style
  recognition-gap finding, REGRESSION RULED OUT:** of 246 fresh Docling tables, 0 classify as `signal_description` (219
  `unknown` / 20 encoding / 7 feature_matrix) → relations 17→0, signal_constraints 8→0, 0 message fields; `.isf` THIN
  (`channel.isf`, 1 signal), FSMGen `--strict` success / 0 diagnostics; `validate` no stage-staleness (0-vs-0), score 38/100.
  Verified the current binary is NOT regressed — DTI (159 msg-fields) / MMU-700 (63 registers) / AHB (66 relations) all
  hold. OpenCAPI TL is a packet/command-layer spec; its tables don't match the AMBA `Signal|Direction|Width|Description`
  shape → the 219 `unknown` tables are the recall opportunity (Lever D family, table-recognition for non-AMBA styles),
  spun-out, not fixed in-slice. **Phase finding:** the high-value AMBA-style protocol specs are now exhausted (#17–#19 are
  thin non-AMBA); next phase pivots to the register/TRM/ISA docs where the `.10` families demonstrably fire. Coverage:
  19 of 57 normalized-missing docs re-ingested.
- `2026-06-22`: `.2` re-ingest **#18 — Wishbone B4** (`wbspec_b4`, 128pp) — fresh-session PNT slice. Docling CPU
  re-ingest (128 pages / 356 visual assets / 0 residuals; RAM steady 72–78% free, `.4a` guard armed, Ollama idle) →
  deterministic cascade (evidence 2005 spans / 2041 statements). **Result is an honest extraction-gap finding:** the
  fresh evidence carries **0 interfaces / 0 signal_records → 0 actor_signal_relations** (the lone stale relation was a
  fragment the `.1a` gate drops — not a regression; the stale build also had ~1). IntentIR still captures 215 free-text
  constraints + 2 transactions + 8 actors, so the intent is present as obligations, just not as the typed wire surface.
  `.isf` renders but THIN (`arbiter.isf`, 1 signal); **FSMGen `--strict --check` success / 0 diagnostics**; `validate`
  shows no stage-staleness (0-vs-0 honest absence) and score 42/100. **Root cause (read-only probe):** Wishbone documents
  its signals in the `SIGNAL_O()`/`SIGNAL_I()` suffix-notation + prose signal-list style, which the current
  signal-table/prose extractors don't recognize → a SURFACED upstream signal-recall lever (Lever D) for the
  `PDF-VARIANT-DIGESTION` family (kin to the parked `.9.10` prose-bus-line lever), NOT an emitter lever and per the tree
  rule NOT fixed inside a re-ingest slice. Binary already current (no rebuild). No code change → golds + `kg-bench`
  orthogonal. Coverage: 18 of 57 normalized-missing docs re-ingested.
- `2026-06-22`: `.2` re-ingest **#17 — Avalon Interface Spec** (`683091`, 63pp) — fresh-session PNT slice. Docling
  CPU re-ingest (63 pages / 192 visual assets / automation_confidence high / 0 residuals, RAM steady 77–78% free,
  built-in `.4a` guard armed, Ollama idle) → deterministic cascade `evidence`→`semantic`→`intent`→`adapt --target isf`.
  Refresh result: relations 126→111 / actors 64→50 (current `.1a`/`.1b`/`.1b.iv` agent-identity consolidation gates
  fold fragment/phantom actors that the stale Jun-7 evidence still carried), registers 8 held, transactions 5 held;
  **honest absence of message-field/presence surfaces** (Avalon carries no such table families). `.isf` renderable
  (`source.isf`, 26 signals / 8 storage / 7 enums / 1 txn body), **FSMGen `--strict --check` success / 0 diagnostics**;
  `validate` shows **no stage-staleness warning** (fresh cascade). Release binary rebuilt first (was stale — `isf_ir.rs`
  newer; `CARGO_BUILD_JOBS=2`, 1m30s) so the cascade ran HEAD. No extraction code change → WIRE-BASED-100 + register/wire
  golds + `kg-bench` orthogonal by construction. Coverage: 17 of 57 normalized-missing docs re-ingested.
- `2026-06-21`: `.2` re-ingest batch OWNED + provisioning set up. Owner re-provisioned the host-local spec
  library (`chipdoc`, 88 PDFs); chosen mechanism = a git-ignored symlink `.cache/local-references/chipdoc`
  (no `corpus/` copy → no git bloat; absolute library path never tracked). `/.cache/` added to `.gitignore`.
  Frontier now ACTIVE (was standing/blocked): RAM-guarded per-doc re-ingest with the current binary to refresh
  the 57 docs' STALE EvidenceIR (unlock `.10`/`.12`/`.2` extractor families) and cascade to `.isf`,
  protocol-specs-first. Ownership slice — no extraction code change.
- `2026-06-17`: `.1` stage-staleness validator DONE (CODE). `validate <intent-ir>`/`<semantic-ir>` now emits a
  `stage_staleness` Warning when the downstream carries 0 `actor_signal_relations` while its upstream (loaded
  via the carried path) carries some — false-positive-free (gating never empties a non-empty set), I/O paid
  only when empty (`let`-chain), skipped when the upstream is off-disk. Pure helper +3 unit tests; live-verified
  (positive fires, nvme/healthy silent); ADR-0006; `run_ci.sh` green (lib 1660, +3); `kg-bench` 156/156;
  WIRE-BASED-100 unaffected by construction. Book `quality/validation.md`; KM `stage-staleness-validate-detector`.
- `2026-06-17`: Created on the owner's substantive-gap-#2 directive. `.0` build-out + census DONE — corpus
  IntentIR coverage 36→78 (42 evidence-only docs + 3 stale built, 0 failures, 0 stale remaining; 75/78 isf
  with 3 honest behavioral-content blocks); RAM steady 77%; deterministic, no LLM/Docling. Frontier → `.1`
  stage-staleness `validate` detector + the gated re-ingest of the 57 normalized-missing docs.
