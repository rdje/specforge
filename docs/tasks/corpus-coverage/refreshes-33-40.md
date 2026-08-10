# CORPUS-COVERAGE — refreshes 33–40

- Part ID: `refreshes-33-40`
- State: `legacy`

<!-- corpus-task-source-region:refreshes-33-40:start -->
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

- ID: `CORPUS-COVERAGE.2.38` · Status: `done` (`2026-08-09`, DATA/DOC) · Result: authenticated the source PDF
  `08a37c35…162`, current release binary `5175bf62…8f4`, original six hashes/counts, and complete validator-side-
  effect snapshot before regeneration. The repository-relative symlink form failed closed before mutation;
  resolved caller-authorized SSD input is correctly labeled external. Two guarded CPU ingests produce 32 pages /
  25 visuals / four tables / 42 sections / 250 elements and zero residuals; the measured replay peaked at 44%
  system memory used. All 32 page image and layout paths are final repository-relative values with no staging
  residue. Source 333→250 is a net removal of 83 flattened visual-label body records: the 88 old-only records are
  83 diagram labels plus five prose/header records contaminated by appended labels, each replaced by its clean
  prose/header text; list items, captions, pages, visuals, tables, and section counts hold.

  Evidence becomes 42 anchors / 228 spans / 25 visuals / 14 links / 228 statements, with six normative facts,
  one signal constraint, and zero relations, declarations, conditionals, registers, or timing records. The nine
  retained relations were not grounded topology: examples include `RAM is reads APB`, `means drives ATB`, and
  `debugger does reads DRW`. Current generic authority removes them, the six acronym/section-title interfaces,
  eight ports, four connectivity edges, and four-signal/two-enum adapter surface. Semantic/Intent retain four
  actors, 13 phases, 21 invariants, 36 gates, one assertion, 24 decompositions, 49 behaviors, 22 constraints, and
  four assumptions. Validation classifies the self-declared document as a high-confidence methodology guide.
  Lowering blocks on `no signals declared in interface`, retains one honest unsupported temporal residual, emits
  no target, and leaves exactly `adapter.json`. Two complete cascades reproduce all 12 IR/report/manifest hashes.
  Nine provider-free WIRE/I2C/SWD gates, KG 156/156, all 66 current emitted ISFs through FSMGen strict, persisted-
  path and project-data locality gates pass. No product code changed in the parent.
  Commit: `CORPUS-COVERAGE.2.38 — sign off Introducing CoreSight refresh`.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.38`

- [x] **REPRODUCE / MEASURE** — authenticate the source PDF, original six hashes/counts, nine relations, six
  interfaces/eight ports/four connectivity edges, blocked four-signal adapter, release binary, and preserved
  validator-side-effect set; record guarded-ingest counts, peak memory, final hashes, and typed deltas.
- [x] **ROOT CAUSE (WHY + WHERE)** — classify every material delta against current generic relation, topology,
  signal/interface, and behavioral authority rather than treating guide vocabulary or strict syntax as trust.
- [x] **ADDRESSED (verified)** — promote only complete validated stages; require every page path final and
  repository-relative; converge obsolete adapter targets; run FSMGen strict only if honestly renderable.
- [x] **NO REGRESSION** — reproduce the deterministic cascade, hold nine provider-free WIRE/I2C/SWD gates and KG
  156/156, sweep all current emitted ISFs through FSMGen strict, and pass mdBook/doctrine/path/locality gates plus
  broader Rust CI if any product code changes.
- [x] **GENERICITY** — no CoreSight/Arm/guide/document/token exception, manual generated-output edit, relaxed
  validator, fabricated topology/behavior, or LLM/VLM inference enters the slice.
- [x] **LOCKSTEP** — generated artifacts, #38 row/count/frontier, live docs, durable fact when warranted, mdBook,
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

- ID: `CORPUS-COVERAGE.2.39` · Status: `done` (`2026-08-09`, DATA/DOC) · Result: authenticated source PDF
  `7df9af5f…0365`, current release binary `5175bf62…8f4`, and the exact six-file / 225717-byte retained chain before
  mutation. The retained SourceIR still named the obsolete boot-volume input; regeneration now records the
  caller-authorized SSD source as external provenance. Two guarded CPU ingests each peak at 18% system memory
  used and reproduce 10 pages / six visuals / five tables / 23 sections / 105 content elements / zero SourceIR
  residuals. All 10 page image paths and 10 layout paths are final repository-relative values with no staging
  residue.

  Evidence 113→112 removes only synthetic `statement_0114`, `Signal DL is width 1.`; all 105 SourceIR content
  elements are unchanged. Its span points into the headerless `Terms` table, whose `DL` row defines OpenCAPI data
  link layer beside abbreviations such as `AFU`, `DLx`, `DUT`, and `PHY`. It is a glossary, not a signal inventory
  or formal interface declaration. Current evidence retains 23 anchors, 112 spans, six visual records, 10 links,
  107 source facts, two normative statements, two timing statements, and one conditional statement, with zero
  typed relations, declarations, constraints, conditionals, registers, or timing-table records.

  Current generic authority therefore removes high-confidence
  `interface_explicit_interface_section_0018_terms` and the blocked adapter's one-bit `DL` output. SemanticIR
  preserves two actors, three phases, four invariants, six contracts, three gates, one assertion, and 13
  decompositions; IntentIR preserves one actor, 12 behaviors, five constraints, and four temporal invariants.
  Validation classifies the low-structured-design-intent document as a guide while leaving purpose honestly
  unresolved at low confidence between physical/electrical and methodology/guide; two partially structured
  normative statements remain visible candidate misses. Lowering blocks on no declared signals plus no behavior,
  emits no target, and leaves only `adapter.json` plus its report. Two complete ingests/cascades reproduce all 12
  artifact/manifest/report hashes. Nine provider-free WIRE/I2C/SWD gates, KG 156/156, all 66 current emitted ISFs
  through FSMGen strict, persisted-path and project-data-locality gates pass. Only after the complete gate set
  passed, the exact 29-file / 804-KiB same-volume rollback/task bundle was deleted with zero task-id residue. No
  product code changed.
  Commit: `CORPUS-COVERAGE.2.39 — sign off OpenCAPI Ready note refresh`.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.39`

- [x] **REPRODUCE / MEASURE** — authenticate the source PDF, retained six-file chain, 10-page source structure,
  zero-relation/declaration evidence, one-interface/one-signal blocked adapter, release binary, and same-volume
  rollback; record guarded-ingest counts, peak memory, final hashes, and typed before→after deltas.
- [x] **ROOT CAUSE (WHY + WHERE)** — classify every material delta against current generic document-class,
  interface, signal, and behavioral authority rather than treating an acronym or strict syntax as trust.
- [x] **ADDRESSED (verified)** — promote only complete validated stages; require every page path final and
  repository-relative; converge obsolete adapter targets; run FSMGen strict only if honestly renderable.
- [x] **NO REGRESSION** — reproduce the deterministic cascade, hold nine provider-free WIRE/I2C/SWD gates and KG
  156/156, sweep all current emitted ISFs through FSMGen strict, and pass mdBook/doctrine/path/locality gates plus
  broader Rust CI if any product code changes.
- [x] **GENERICITY** — no OpenCAPI/vendor/note/document/acronym exception, manual generated-output edit, relaxed
  validator, fabricated interface/behavior, or LLM/VLM inference enters the slice.
- [x] **LOCKSTEP** — generated artifacts, #39 row/count/frontier, live docs, durable fact when warranted, mdBook,
  and `MEMORY.md` agree; delete only authenticated rollback/task evidence after every final gate is green.

- ID: `CORPUS-COVERAGE.2.40` · Status: `done` (`2026-08-09`, DATA/DOC) · Result: authenticated source PDF
  `b427a302…2902`, current release binary `5175bf62…8f4`, and the exact six-file / 278090-byte retained chain before
  mutation. Two guarded CPU ingests peak at 21% and 22% system memory used and reproduce 13 pages / three visuals /
  two tables / 25 sections / 173 content elements / zero SourceIR residuals. All 13 page image paths and 13 layout
  paths are final repository-relative values; regenerated external provenance names the caller-authorized SSD
  input rather than the obsolete boot-volume route.

  Evidence 173→172 removes only synthetic `statement_0174`, `Signal DL is width 1.`; all 173 SourceIR content
  elements are unchanged. The cited `Terms` table defines data link layer and belongs to the same glossary shape
  as `.2.39`, not a signal inventory or formal interface declaration. Current evidence retains 25 anchors, 172
  spans, three visual records, 169 source facts, two normative statements, and one conditional statement, with
  zero links, typed relations, declarations, constraints, conditional rules, registers, or timing records.

  Current generic authority therefore removes high-confidence
  `interface_explicit_interface_section_0016_terms` and the blocked adapter's one-bit `DL` output. SemanticIR
  preserves two actors, four phases, four invariants, one contract, six gates, one assertion, and 15
  decompositions; IntentIR preserves one actor, 11 behaviors, four constraints, and four temporal invariants.
  Validation classifies the note as a low-structured-design-intent guide while purpose remains honestly unresolved
  at low confidence between physical/electrical and methodology/guide; two partially structured normative
  statements and three unenriched visuals stay visible as candidate misses. Lowering blocks on no declared signals
  plus no behavior, emits no target, and leaves only `adapter.json` plus its report.

  Two complete ingests/cascades reproduce all 12 artifact/manifest/report hashes. Nine provider-free
  WIRE/I2C/SWD gates, KG 156/156, all 66 current emitted ISFs through FSMGen strict, persisted-path, and
  project-data-locality gates pass. The signoff lockstep audit also restores the accidentally omitted `.2.39`
  per-document table row; its already-committed task result and all other durable surfaces were correct. Only
  after the complete final gate set passed, the exact 29-file / 932010-byte / 968-KiB same-volume rollback/task
  bundle was deleted with zero task-id residue. No product code changed.
  Commit: `CORPUS-COVERAGE.2.40 — sign off OpenCAPI Certified note refresh`.

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.40`

- [x] **REPRODUCE / MEASURE** — authenticate the source PDF, retained six-file chain, 13-page source structure,
  zero-relation/constraint evidence, one-interface/one-signal blocked adapter, release binary, and same-volume
  rollback; record guarded-ingest counts, peak memory, final hashes, and typed before→after deltas.
- [x] **ROOT CAUSE (WHY + WHERE)** — classify every material delta against current generic document-class,
  interface, signal, and behavioral authority rather than assuming the `.2.39` sibling result.
- [x] **ADDRESSED (verified)** — promote only complete validated stages; require every page path final and
  repository-relative; converge obsolete adapter targets; run FSMGen strict only if honestly renderable.
- [x] **NO REGRESSION** — reproduce the deterministic cascade, hold nine provider-free WIRE/I2C/SWD gates and KG
  156/156, sweep all current emitted ISFs through FSMGen strict, and pass mdBook/doctrine/path/locality gates plus
  broader Rust CI if any product code changes.
- [x] **GENERICITY** — no OpenCAPI/vendor/note/document/acronym exception, manual generated-output edit, relaxed
  validator, fabricated interface/behavior, or LLM/VLM inference enters the slice.
- [x] **LOCKSTEP** — generated artifacts, #40 row/count/frontier, live docs, durable fact when warranted, mdBook,
  and `MEMORY.md` agree; delete only authenticated rollback/task evidence after every final gate is green.

<!-- corpus-task-source-region:refreshes-33-40:end -->
