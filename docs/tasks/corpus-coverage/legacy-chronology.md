# CORPUS-COVERAGE — legacy chronology

- Part ID: `legacy-chronology`
- State: `legacy`

<!-- corpus-task-source-region:legacy-chronology:start -->
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
<!-- corpus-task-source-region:legacy-chronology:end -->
