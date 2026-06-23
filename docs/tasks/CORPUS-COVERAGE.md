# CORPUS-COVERAGE: build every ingested doc through to IntentIR/.isf + keep downstream stages non-stale

## Metadata

- Tree ID: `CORPUS-COVERAGE`
- Status: `active` (`.0` build-out + `.1` stage-staleness validator both done `2026-06-17`; `.2` re-ingest batch ACTIVE `2026-06-21` — owner re-provisioned the host-local library, so the 57 normalized-missing docs are now re-ingestable to refresh their STALE EvidenceIR with the current binary)
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

## Task Tree

- ID: `CORPUS-COVERAGE` · Status: `active` · Children: `.0` (build-out + census, done), `.1` (stage-staleness
  validator, done), `.2` (host-local re-ingest batch, active)
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
- ID: `CORPUS-COVERAGE.2` · Status: `active` (`2026-06-21`) · Goal: **RAM-guarded re-ingest of the 57
  normalized-missing docs with the CURRENT binary**, now that the owner has re-provisioned the host-local
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
- Frontier (active): `CORPUS-COVERAGE.2` — re-ingest the 57 normalized-missing docs from the
  `.cache/local-references/chipdoc` symlink, register/TRM/ISA phase, one doc per slice (26 of 57 done after
  #26 Intel VT-d 5.0 — a GENUINE refresh that **CORRECTS the #22 survey expectation**: VT-d was flagged "already
  current-binary-equivalent" off its 103 registers, but its stale evidence predated the message-field family AND the
  transaction recognizer → it gained 15 message-fields + 2 transactions, so the one-pass register-count survey is NOT
  sufficient to call a doc current; #25 CoreSight SoC-600 0100 TRM completed the SoC-600 cluster 0701/0200/0100, all
  three genuine consolidation+transaction refreshes). **Measured-corrected expectation (`2026-06-23`, #22–#23, refined #26):** a one-pass survey of
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

| # | doc (key) | pages | refreshed surfaces (after) | `.isf` | fsmgen `--strict` |
|---|---|---|---|---|---|
| 1 | `ihi0079` AMBA CXS | 54 | message_field_records 0→1; transactions →2 (recognition now fires); 11 signal ports emitted (CXSDATA/CXSVALID/CXSCRDGNT/…); stale evidence had message/temporal/presence fields ABSENT | renderable (`transmitter.isf`, 11 ports) | **0 diagnostics** ✓ |
| 2 | `ihi0083` AMBA GFB | 45 | transactions →5 (recognition now fires); relations 16 (held); signal_constraints 14→7 (current stricter declared-subject gate drops ungrounded); no msg/reg fields (GFB has none — honest) | renderable (`device.isf`, 7 ports) | **0 diagnostics** ✓ |
| 3 | `ihi0076` ACC | 90 | register_records 2→4 (the `.10g` section-heading register-field family fires — ACC is one of its 5 docs); rich `.isf` 133 ports (vs stale 2-register); 0 txns/relations (register/debug-channel arch — honest) | renderable (`agent.isf`, 133 ports) | **0 diagnostics** ✓ |
| 4 | `ihi0082` ATP | 84 | transactions →5 (recognition fires); relations 11→9 — the CLEAN deterministic re-ingest drops the garbled-VLM-fragment actors the `KG-ISF-COMPLETENESS.4` census flagged (`"RREADY is RBR"`), so ATP no longer needs the owner-gated VLM lever just to get a clean baseline; reg 4 held | renderable (`rate_parameter.isf`, 1 port) | **0 diagnostics** ✓ |
| 5 | `tilelink_1_7_1` TileLink 1.7.1 | 107 | relations 39→33 (current agent-identity consolidation gates `.1a`/`.1b` — which postdate the stale evidence — fold fragment actors); renderable per-channel `.isf` (`channel_b`); transactions 0 (TileLink's Get/Put op vocabulary not yet in the section-heading recognizer — honest) | renderable (`channel_b.isf`, 5 ports) | **0 diagnostics** ✓ |
| 6 | `tilelink_1_8_0` TileLink 1.8.0 | 111 | relations 40→34 (same `.1a`/`.1b` consolidation as 1.7.1); transactions 0 (op vocabulary — honest); per-channel `.isf` | renderable (`channel_b.isf`, 6 ports) | **0 diagnostics** ✓ |
| 7 | `ihi0068` AMBA LPI | 66 | transactions →1 (recognition fires); relations 9→8; constraints 17 held | renderable (`controller.isf`, 5 ports) | **1** — `isf_conflicting_rule_writes` on `PREQ` (overlapping rule writes pick different values, `rule_5`): the `ISF-RULE-CONFLICT-RESIDUAL` family (same as pre-existing AHB `HAUSER`/AXI `ASKSTOP`); honest residual, NOT introduced by this slice (emitter unchanged) → candidate for that tree |
| 8 | `ihi0088` AMBA DTI | 146 | **`message_field_records` 0→159 / 17 containers** (the `.10f` section-heading message-field family fires — the marquee re-ingest win; totally ABSENT in stale evidence); transactions →5; constraints 30→16 (stricter gate) | renderable (`channel.isf`, 294 ports) | **1 ERROR (strict FAIL)** — FSMGen HDL-gen: `assignment to 'ATST' uses RHS 2'b1 (width 2) for LHS width 1` (implicit truncation blocked). A real SpecForge ISF **width-alignment** emitter bug (emits a width-2 value to a 1-bit signal), EXPOSED not caused by this slice (emitter unchanged) → **spun-out CODE lever** (see below) |

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

**Cumulative (26 docs, protocol + register/arch phases): hundreds of registers + ~385 message fields surfaced that were ABSENT in stale evidence** — registers e.g. SMMU-arch 1→89, GIC-arch 17→90, MMU-700 13→63, GIC-600 15→33, TMC 2→30; message fields DTI 0→159, CHI-C2C 0→210, VT-d 0→15. **24/26 strict-clean `.isf`** (after `.2a.iii` fixed GIC-600; #17 Avalon + #18 Wishbone + #19 OpenCAPI-TL + #20 GIC-400 + #21 RISC-V-IOMMU + #22 Cortex-A76 + #23/#24/#25 CoreSight-SoC-600 + #26 VT-d clean); 2 strict-FAIL remain (DTI width-align = Lever A, LPI rule-conflict = Lever C). **Refined #22/#23 phase note:** "already current-binary-equivalent" is not uniform — a doc's stale evidence sits at whatever binary last rebuilt it, so re-ingest can still deliver a real KG refresh when that binary predated a later gate: #22 Cortex-A76 was fully current (byte-identical), but #23 CoreSight SoC-600 predated the `.1a`/`.1b` consolidation + transaction recognizer and got a genuine actor-consolidation + 2-transaction refresh (the #17 Avalon class). So the remaining tail is a MIX of pure confirmation (#21/#22) and consolidation/recognition refreshes (#17/#23), with marquee table-family jumps now the exception. **NEW phase finding (#21):** some of the 57 normalized-missing docs are NOT pre-`.10` stale — their evidence was rebuilt Jun-15 from the retained `source_ir.json` (only the heavyweight `normalized/` page-image bundle was disk-reclaimed) so it already carried the `.10` register families; for those, re-ingest is **normalized-bundle restoration + current-binary confirmation** (modest deterministic delta), distinct from the genuinely pre-`.10` docs (SMMU/GIC/MMU big jumps). The IOMMU also surfaced a **RISC-V STRUCTURE-table recall opportunity** (device-context/command-queue tables not matching the `.10b`/`.10d`/`.10e` two-column families — Lever D). **PHASE FINDING (#17–#19, the non-AMBA protocol specs):** their refresh value is NOT marquee table-family gains — Avalon = consolidation + current-binary freshness; **Wishbone + OpenCAPI-TL surfaced a doc-style TABLE-RECOGNITION recall gap** (Wishbone 1 `signal_description`/25 tables; OpenCAPI 0/246 — both predominantly `unknown`-classified; non-AMBA `Signal\|Direction\|Width` shapes), so their `.isf` degenerate to 1 signal. **REGRESSION RULED OUT** (DTI 159 / MMU-700 63 / AHB 66 all hold on the current binary) → these are the upstream Lever-D signal/table-recall family, not emitter bugs. The high-value AMBA-style protocol specs are now exhausted; **next phase = the register/TRM/ISA docs** (the `.10` register families demonstrably fire there — CoreSight/GIC/JEDEC/RISC-V system-IP). Remaining queue = OpenCAPI×13 (PHY/mechanical/TL-variants, expected thin)/JEDEC/USB/RISC-V system-IP/VT-d/Cortex-A76/GIC-400/CoreSight-SoC-600×3/overview/guides.

**Spun-out CODE levers (`.2` re-ingest is SURFACING + scoping these — measurement-first; each needs its OWN owned leaf under the `ISF-*-EMIT` family, with WIRE-BASED-100 + register/wire golds + `kg-bench` gating; do NOT fix inside a re-ingest slice):**

- **Lever B — ISF module-name HDL-sanitization — ✅ DONE `2026-06-21` (`KG-ISF-COMPLETENESS.2a.iii`, owner-chosen).** GIC-600's `.isf` had failed FSMGen strict with `Malformed top-level FSM source '?fsm:redistributor→_distributor…'`. Fixed by flipping `sanitize_isf_name` (`ir/isf_ir.rs`) from a char denylist (which missed the arrow `→`) to an allowlist (`[A-Za-z0-9_]`, else `_`) and routing `derive_isf_actor_name`'s module label through it. GIC-600 now re-checks FSMGen `success=true` / 0 diagnostics; wire golds byte-identical; `run_ci.sh` GREEN (lib 1679); `kg-bench` 156/156. KM `isf-module-name-hdl-sanitization`. (Latent agent-identity gap — an arrow-phrase actor slipping the `.1a`/`.1b` gates — remains a separate future candidate.)
- **Lever A — ISF value width-alignment.** DTI's 294-port `.isf` fails strict: a rule/drive lowers a width-2 literal (`2'b1`) onto the 1-bit signal `ATST` (FSMGen OperandContract blocks implicit truncation). The emitter doesn't width-align an emitted value to its declared signal width. Value-specific (CHI-C2C with 210 fields is strict-CLEAN → not universal).
- **Lever C (smaller) — rule-write conflicts.** LPI: `isf_conflicting_rule_writes` on `PREQ` (the `ISF-RULE-CONFLICT-RESIDUAL` family; same as pre-existing AHB `HAUSER`/AXI `ASKSTOP`).

**Running strict tally (26 renderable docs, post-`.2a.iii`):** **24 clean** (incl. GIC-600, now fixed, + GIC-arch + SMMU-arch + Avalon #17 + Wishbone #18 + OpenCAPI-TL #19 + GIC-400 #20 + RISC-V-IOMMU #21 + Cortex-A76 #22 + CoreSight-SoC-600 #23/#24/#25 + Intel VT-d #26). **2 FAIL** = DTI (Lever A, value width-align), LPI (Lever C, rule-conflict). Lever B (module-name) ✅ resolved. **Surfaced lever D (upstream/non-emitter, now seen on 2 docs):** non-AMBA table/signal recognition — Wishbone-style `*_O`/`*_I` suffix-notation + prose signal-lists, and OpenCAPI-style packet/command tables left `unknown` (219/246) — a `PDF-VARIANT-DIGESTION` signal/table-recall family (kin to the parked `.9.10`). Watch the long-tail for width-align (A) / rule-conflict (C) / non-AMBA-table-recall (D) recurrence to scope those.

**PHASE FINDING (#22, the already-current-binary-equivalent class is now the RULE, not the exception):** a one-pass survey of the 35 remaining normalized-missing docs' persisted evidence (`2026-06-23`) shows the register-heavy ones the frontier had flagged as "genuinely pre-`.10`-stale" are ALREADY current — CoreSight SoC-600 ×3 = 597/631/833 `register_records`, AMD-IOMMU `48882` = 217 `message_field_records`, Intel VT-d = 103 registers — because their evidence was rebuilt from the retained `source_ir.json` before the `normalized/` bundle was reclaimed (the #21/#22 class). So the marquee-jump phase (SMMU/GIC/MMU/DTI/CHI-C2C) is effectively complete; the remaining ~35 slices are predominantly **bundle-restoration + current-binary confirmation + honest-absence / Lever-D-recall documentation**, which is still genuine corpus-coverage work (a doc that depends only on a retained `source_ir.json` is one disk-reclaim away from losing its evidence; restoring `normalized/` makes it rebuildable). A true pre-`.10`-stale jump remains possible in the long tail but is now the exception.

## Changelog

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
