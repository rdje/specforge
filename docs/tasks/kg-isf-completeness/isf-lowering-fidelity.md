# KG-ISF-COMPLETENESS — isf lowering fidelity

- Part ID: `isf-lowering-fidelity`
- State: `legacy`

<!-- kg-isf-completeness-task-source-region:isf-lowering-fidelity:start -->
## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.2a.iv`
- [x] **REPRODUCE / MEASURE** — `adapt --target isf` on the persisted HBM2 `intent_ir.json` emits `hbm.isf`; the real `subs/fsmgen/bin/fsmgen --strict --check --json hbm.isf` returns `success:false` with `enum member 'TABLE.REPAIR_LANE_8' value token '1000'` rejected. A value sweep against the same FSMGen pins the rule: `1000`/`1010`/`1111`/`10000` fail, `999`/`1020`/`69152` and short `0/1/111` and qualified `4'b1000`/`16'd1000` pass.
- [x] **ROOT CAUSE (WHY + WHERE)** — `emitted_enums()` (`crates/specforge/src/ir/isf_ir.rs:352`) gated only on `is_safe_isf_scalar_value` (whitespace-free), so a bare binary-looking token (`1000`) reached FSMGen, which rejects an un-qualified `[01]`-only token of length >= 4. The HBM2 `TABLE` is a mis-extracted mega-enum (REPAIR_LANE binary codes mis-read as decimals + ~10 conflated tables). Initial "count-derived width overflow" hypothesis (`isf_ir.rs:878`) was DISPROVEN by a before/after `.isf` diff (it wrongly dropped AXI `AWATOP`/AHB `TABLE`) and the FSMGen value sweep (`69152` accepted).
- [x] **ADDRESSED (verified)** — new `isf_enum_value_is_emittable_literal` + `isf_enum_is_emittable` + `enum_residuals` (`isf_ir.rs`) + `adapters.rs` residual wiring. HBM2 `hbm.isf` now FSMGen `--strict --check --json` **success / 0 diagnostics**; only `TABLE` dropped → residual `isf_enum_value_literal_table`; `EXTEST_RX`(212)/`DWORD_MISR`(19) KEPT. 2 new unit tests pass.
- [x] **NO REGRESSION** — fresh re-emit + `diff` of all emitted `.isf`: the ONLY changed file is HBM2 `hbm.isf` — wire golds (APB/AHB/AXI/SWD) + Avalon + CoreSight SoC-600 + GIC-600 + ARM-Debug `.isf` BYTE-IDENTICAL. `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1706 passed, +2; clippy/fmt/rustdoc warning-deny + mdBook); WIRE-BASED-100 orthogonal (emitter-only, wire `.isf` byte-identical).
- [x] **GENERICITY (ADR 0006)** — universal token grammar (a bare `[01]`-only token of length >= 4 = an un-qualified binary literal FSMGen rejects), NOT a chip/vendor/protocol name list; verified against the real FSMGen, not guessed (`[[feedback_verify_fsmgen_before_fr]]`); every legitimate decimal / qualified literal passes, so currently-clean enums are byte-identical.
- [x] **LOCKSTEP** — README current-state bullet; book `pipeline/isf-adapter.md`; KM card `isf-enum-value-literal-emit-gate`; CHANGES.md / DEVELOPMENT_NOTES.md / LIVE_ACHIEVEMENT_STATUS.md / MEMORY.md; `CORPUS-COVERAGE.2` Lever-F + strict-tally corrected (DTI Lever A already resolved; HBM2 Lever F now resolved).

- ID: `KG-ISF-COMPLETENESS.2a.v` · Status: `done` (`2026-06-23`, measurement-first; LANDED + verified
  against the real FSMGen + full corpus sweep) · Goal: **close the open ISF strict-FAIL class — the
  `isf_conflicting_rule_writes` cross-guard conflict ("Lever C"), surfaced on the AMBA Low Power Interface
  (`ihi0068_d`) but MEASURED to also hit the AXI/AHB/AXI-Stream wire golds, LTI, and NVMe** (north-star
  bar #6: every renderable `.isf` is strict-valid; a dropped obligation becomes an honest residual, never a
  silent loss or a fabricated resolution). **MEASUREMENT-FIRST CORRECTION:** the resume-pointer tally
  "27/28 renderable clean, 1 FAIL (LPI)" was STALE — a full fresh current-binary re-emit + FSMGen sweep
  showed the conflict was NOT LPI-only (the cached wire-gold `.isf` were byte-identical to fresh AND already
  FSMGen-FAIL on this conflict). After the fix: **6 docs FAIL→PASS** (AXI `ihi0022_l`, AHB `ihi0033_c`,
  AXI-Stream `ihi0051_b`, LPI `ihi0068_d`, LTI `ihi0089_d`, NVMe), **0 PASS→FAIL regressions**. **TALLY
  CORRECTION (`2026-06-23`, same slice):** the per-`.isf` tally was first reported over a stale-inclusive
  107-file cache; `adapt` emits only ONE primary-actor `.isf` per doc, so 37 of those were STALE cruft from
  older binary versions (cleaned). Over the **70 current-emit `.isf` (one per doc)** the honest post-fix tally
  is **69/70 PASS** — the LONE remaining FAIL is `ihi0022_h_c` (combined AXI+ACE) on the ORTHOGONAL `(port
  expr)` grammar (a spun-out `ISF-VALUE-WIDTH-EMIT` Non-Goal), NOT this conflict class. So after Lever C the
  ISF-emit strict-FAIL frontier is effectively CLOSED (Levers A/B/C/F resolved).
  **REPRODUCE (read-only, real FSMGen):** `subs/fsmgen/bin/fsmgen --strict --check --json` on the persisted
  LPI `controller.isf` returns `success:false` / 1 diagnostic: `ISF conflict 'isf_conflicting_rule_writes' on
  target 'PREQ': … rule 'rule_5' (rule_action, <- 1) conflicts with rule
  'temporal_temporal_signal_constraint_dyn_sigcon_0012' (rule_action, <- 0)`. A second, identical-shape
  conflict on `PACCEPT` (`constraint_3` ←1 vs `temporal_…_dyn_sigcon_0011` ←0) is masked behind it (FSMGen
  confesses one conflict at a time). **ROOT CAUSE (WHY + WHERE):** `rule_5`/`constraint_3` are **unconditional**
  (`IsfRule.condition == ""` → rendered with no guard, always active) and drive `PREQ`/`PACCEPT` ←1;
  `..._dyn_sigcon_0012`/`..._0011` are **guarded** (`(== PACCEPT 0)`) and drive ←0. SpecForge's existing
  conflict dedup `dedup_conflicting_rules` (`crates/specforge/src/ir/isf_ir.rs:2512`) keys on
  `(signal, condition)` — the **same-guard** overlap model — so an unconditional rule (guard `""`) and a guarded
  rule (guard `(== PACCEPT 0)`) hash to different keys and the conflict is never detected, yet FSMGen flags it:
  its `_condition_terms_prove_disjoint` (`subs/fsmgen/perl/FSM/Scheduler/ISF/LoweringIR.pm:10458`) can NEVER
  prove an absent/empty condition disjoint, so an unconditional rule's firing set ⊇ every guard → it overlaps
  any different-value rule on the same target. **DESIGN (validated empirically, GO):** after the same-guard
  dedup (which leaves ≤1 unconditional value per signal — two unconditional rules on one signal share key
  `(S,"")`), add a second pass `drop_unconditional_overlap_conflicts`: for each signal `S` with a kept
  unconditional driver value `V`, DROP (+ honest `ResidualDecisionPacket`) every other rule driving `S` to a
  value `≠ V`. This is **precise** — it drops exactly FSMGen's flagged case (unconditional-overlap) and nothing
  else, so a currently-strict-clean doc (which by construction CANNOT contain such a config, or FSMGen would
  already reject it) re-emits byte-identical. The **`(priority …)` escape-hatch alternative was tested and
  REJECTED**: it cleared one pair but `..._0012` then conflicted with the next unconditional `PREQ←1` rule
  (`rule_6`), so keeping the guarded minority rule would require asserting an ungrounded precedence over EVERY
  same-value unconditional rule — fabrication, against `[[feedback_isf_no_hacks]]`. The general
  different-guard-overlap case (two non-empty guards that overlap) is NOT in LPI (measured) and stays an honest
  out-of-scope residual. Empirical validation on the real FSMGen: dropping `..._0011` + `..._0012` →
  `success:true` / 0 diagnostics. ADR-0006: structural (empty-guard ⇒ overlaps-all), no chip/vendor/protocol
  name list. KM card `[[isf-unconditional-rule-overlap-conflict]]` (to write). `[[project_kg_isf_completeness]]`
  / `[[feedback_verify_fsmgen_before_fr]]`. **Frontier → after this lands, the renderable strict tally is
  28/28; remaining `.2`/`.3` north-star work is corpus-refresh + relation-completeness (Docling/RAM-gated).**

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.2a.v`
- [x] **REPRODUCE / MEASURE** — `subs/fsmgen/bin/fsmgen --strict --check --json generated/adapters/isf/ihi0068_d_2021_10_amba_low_power_interface_specification/controller.isf` → `success:false`, 1 diagnostic `isf_conflicting_rule_writes` on `PREQ` (`rule_5` ←1 vs `temporal_temporal_signal_constraint_dyn_sigcon_0012` ←0); a second identical-shape `PACCEPT` conflict (`constraint_3` ←1 vs `..._0011` ←0) is masked behind it. A full fresh re-emit + FSMGen sweep further showed the AXI `ihi0022_l` / AHB `ihi0033_c` / AXI-Stream `ihi0051_b` wire golds + LTI `ihi0089_d` + NVMe were ALL FSMGen-FAIL on this same conflict class (the stale "27/28 clean" tally was wrong; cached wire-gold `.isf` byte-identical to fresh AND FAIL).
- [x] **ROOT CAUSE (WHY + WHERE)** — `dedup_conflicting_rules` (`crates/specforge/src/ir/isf_ir.rs`) keys conflict detection on `(signal, condition)` (same-guard only); an unconditional rule (`IsfRule.condition == ""`) and a guarded rule (`(== PACCEPT 0)`) hash to different keys so the overlap is missed, but FSMGen's `_condition_terms_prove_disjoint` (`subs/fsmgen/perl/FSM/Scheduler/ISF/LoweringIR.pm:10458`) never proves an absent condition disjoint, so the unconditional rule overlaps every guard on its target → `isf_conflicting_rule_writes` (`_build_conflict_issues:10888`). Confirmed by the FSMGen `--strict --check` diagnostic naming `rule_5` (guard-less) vs the guarded `..._0012`.
- [x] **ADDRESSED (verified)** — new pass `drop_unconditional_overlap_conflicts` (`isf_ir.rs`, after the same-guard dedup, before priority emit): per signal with an unconditional driver value `V`, drop every other rule driving it to `≠ V` + record `isf_unconditional_overlap_<name>` residual. LPI `controller.isf` (and `channel.isf`) now FSMGen `--strict --check --json` **success / 0 diagnostics**; exactly `..._0011` (`PACCEPT←0`) and `..._0012` (`PREQ←0`) dropped, both surfaced as `isf_unconditional_overlap_*` residuals (adapter `residual_decision_count` 4→6). Corpus: **6 docs FAIL→PASS** (AXI `ihi0022_l` manager, AHB `ihi0033_c` manager, AXI-Stream `ihi0051_b` transmitter, LPI controller, LTI `ihi0089_d` coherent_host, NVMe channel). +2 unit tests (`drop_unconditional_overlap_conflicts_drops_guarded_minority_keeps_unconditional`, `..._noop_without_unconditional_driver`).
- [x] **NO REGRESSION** — full re-emit diff (current binary WITH vs WITHOUT the change, via `git stash` of `isf_ir.rs`): exactly **7 `.isf` differ** (all 7 are current primary emits), every other emit byte-identical; FSMGen on the 7 → **6 FAIL→PASS, 0 PASS→FAIL**, the 1 still-FAIL (AXI+ACE `ihi0022_h_c` manager) fails on the **orthogonal** pre-existing `(port expr)` grammar (a spun-out `ISF-VALUE-WIDTH-EMIT` Non-Goal), unchanged. **Honest current-emit tally** (after cleaning 37 stale cache-cruft `.isf` from older binaries — `adapt` emits one primary actor per doc): **69 of 70 PASS**, the lone FAIL = `ihi0022_h_c` `(port expr)`. `kg-bench` **156/156**; `run_ci.sh` **GREEN** (lib **1708**, +2; clippy/fmt/rustdoc warning-deny + mdBook). **WIRE-BASED-100 orthogonal by construction** — the only Rust file changed is the `.isf` emitter `isf_ir.rs`; `eval-extraction` reads the IR, never the `.isf`.
- [x] **GENERICITY (ADR 0006)** — universal ISF-semantics rule (an empty-guard rule is unconditional ⇒ overlaps every guard on its target, mirroring FSMGen's own disjointness model), NOT a chip/vendor/protocol-name list; the `(priority …)` escape-hatch was empirically TESTED and rejected as ungrounded precedence (it cleared one pair then `rule_6` conflicted next — keeping the minority would require asserting a winner over every unconditional rule = fabrication, `[[feedback_isf_no_hacks]]`).
- [x] **LOCKSTEP** — README current-state bullet; book `pipeline/isf-adapter.md`; KM card `isf-unconditional-rule-overlap-conflict`; CHANGES.md / DEVELOPMENT_NOTES.md / LIVE_ACHIEVEMENT_STATUS.md / MEMORY.md; `CORPUS-COVERAGE.2` strict tally corrected (Lever C).

- ID: `KG-ISF-COMPLETENESS.2a.vi` · Status: `done` (`2026-06-23`, measurement-first; LANDED + verified) ·
  Goal: **close the LAST ISF-emit strict-FAIL — the AMBA AXI+ACE (`ihi0022_h_c`) `(port expr)` grammar
  failure** so the renderable corpus reaches **70/70 FSMGen-`--strict`-clean** (the spun-out
  `ISF-VALUE-WIDTH-EMIT` Non-Goal, now the frontier after Lever C). **REPRODUCE (real FSMGen):** `fsmgen
  --strict --check --json` on the AXI+ACE `manager.isf` returns `success:false` / `Error: rule
  'constraint_48' assignment actions require '(port expr)'`. **ROOT CAUSE (WHY + WHERE):** the rule's drive
  VALUE is free PROSE — `(RLOOP the value that was presented on the ARLOOP signal)` / `(BLOOP … AWLOOP …)`
  (a loopback-tag obligation the extractor captured as a sentence, not a literal). FSMGen requires a rule
  assignment action's RHS to be a renderable value expression `(port expr)`, so a multi-word prose value
  breaks the file. The emitter renders rule drive values VERBATIM (`render_isf_control_expression`,
  `ir/isf_ir.rs`) with NO validity gate on the value (the existing gates cover enum members `.2a.iv` and
  value width `ISF-VALUE-WIDTH-EMIT`, not the rule's own scalar). **MEASURED scope (read-only over all 70
  current-emit `.isf`):** EXACTLY 4 prose-valued drive lines, ALL in `ihi0022_h_c` (`constraint_48`/`_49` +
  the two `tinv_sc_llm_sigcon_0061`/`_0062` duplicates); ZERO other docs carry a whitespace-bearing rule
  drive value, so a value-validity gate is corpus-safe by construction. **FIX (drop + honest residual):**
  new pass `drop_unrenderable_rule_values` (`isf_ir.rs`, before the width/dedup passes) drops a rule whose
  ANY drive value is not `is_safe_isf_scalar_value` (the existing non-empty/whitespace-free scalar test) and
  records an `isf_rule_value_<name>` residual. The prose value is unrecoverable — "the value PRESENTED on
  ARLOOP" is a temporal loopback, not the current `(port ARLOOP)`, so recovering a `(port expr)` would
  fabricate the timing — honest residual over fabrication (`[[feedback_isf_no_hacks]]`). ADR-0006: a
  structural value-shape test, no chip/vendor/protocol name list. Empirically validated: dropping the 4 →
  AXI+ACE FSMGen `success:true` / 0 diagnostics → **70/70**. KM `[[isf-unrenderable-rule-value-residual]]`.

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.2a.vi`
- [x] **REPRODUCE / MEASURE** — `subs/fsmgen/bin/fsmgen --strict --check --json generated/adapters/isf/ihi0022_h_c_2021_01_amba_axi_and_ace_protocol_specification/manager.isf` → `success:false`, `Error: rule 'constraint_48' assignment actions require '(port expr)'`. Read-only scan of all 70 current-emit `.isf`: exactly 4 whitespace-bearing rule drive values, all in `ihi0022_h_c` (`constraint_48`/`_49`/`tinv_sc_llm_sigcon_0061`/`_0062` = `RLOOP`/`BLOOP` ← "the value that was presented on the ARLOOP/AWLOOP signal"); 0 elsewhere.
- [x] **ROOT CAUSE (WHY + WHERE)** — the `.isf` emitter renders a rule's drive value verbatim (`ir/isf_ir.rs`) with no value-validity gate; a constraint whose extracted value is PROSE (`the value that was presented on the ARLOOP signal`) emits `(RLOOP <prose>)`, which FSMGen rejects because a rule assignment action RHS must be a `(port expr)` (renderable value expression), not free text. Confirmed by the FSMGen `--strict --check` diagnostic naming `constraint_48`.
- [x] **ADDRESSED (verified)** — new pass `drop_unrenderable_rule_values` (`isf_ir.rs`, before the width/dedup passes) drops a rule whose any drive value fails `is_safe_isf_scalar_value` and records an `isf_rule_value_<name>` residual. AXI+ACE `manager.isf` now FSMGen `--strict --check` **success / 0 diagnostics**; exactly the 4 prose-valued rules (`constraint_48`/`_49`/`tinv_sc_llm_sigcon_0061`/`_0062`) dropped + residualized (`isf_rule_value_*`). +1 unit test (`drop_unrenderable_rule_values_drops_prose_keeps_scalars`).
- [x] **NO REGRESSION** — only `ihi0022_h_c` (the only doc with a prose-valued rule, measured) changes; every other current emit byte-identical. **Full FSMGen sweep over all 70 current-emit `.isf` → 70/70 strict-clean** (was 69/70; 0 PASS→FAIL). `kg-bench` 156/156; `run_ci.sh` GREEN (lib **1709**, +1; clippy/fmt/rustdoc warning-deny + mdBook); WIRE-BASED-100 orthogonal by construction (emitter-only — `eval-extraction` reads the IR, never the `.isf`).
- [x] **GENERICITY (ADR 0006)** — a structural value-shape test (a rule drive value must be a non-empty whitespace-free scalar token to be renderable), NOT a chip/vendor/protocol-name list; the prose loopback value is unrecoverable as an exact `(port expr)` (the temporal "was presented" semantics), so honest residual over a fabricated `(port ARLOOP)` approximation.
- [x] **LOCKSTEP** — README current-state bullet; book `pipeline/isf-adapter.md`; KM card `isf-unrenderable-rule-value-residual`; CHANGES.md / DEVELOPMENT_NOTES.md / LIVE_ACHIEVEMENT_STATUS.md / MEMORY.md; `CORPUS-COVERAGE.2` strict tally → 70/70.

- ID: `KG-ISF-COMPLETENESS.2b` · Status: `deferred` (measured-MARGINAL `2026-06-17`, read-only) · Goal:
  **ISF lowering-coverage visibility gauge** — make the lowering's per-surface coverage visible as adapter
  metadata + a `validate <intent-ir>` surface. **Measured marginal → not building now:** `.2` established
  the lowering is already faithful for grounded intent, so a coverage gauge is **noise-dominated** — a
  headline "257 / 16 463 temporal_invariants lowered" is dominated by the 16 179 ungroundable empty-subject
  invariants (ToC headings) `.2` showed are honest absences, so the number misleads rather than informs;
  the actionable signal (a grounded typed rule on a DECLARED signal that did not lower) is ≈0 on the wire
  docs and ~151 corpus-wide register-field paths that correctly belong to the register surface. An honest
  gauge would need full per-category breakdown for little operator value (the project already has rich
  `validate` inventories). Re-open only if an owner wants the categorized ISF-lowering residual surfaced.
  ADR-0006 (universal counts, no name list).
- ID: `KG-ISF-COMPLETENESS.3` · Status: `done` (`2026-06-24` — measurement DONE `2026-06-17`, CLOSED
  `2026-06-24`; read-only; owner-directed substantive north-star push after the owner pushed back on
  "buildable frontier exhausted") · Goal:
  **bar #2 relation-completeness** — investigate why whole docs carry actors+constraints but ZERO
  `actor_signal_relations` (`nvme`/`tilelink`/`wbspec`/`i2c`/`ccix`/VT-d/IOMMU), the exact "real-agent
  relation-incompleteness" the north star names. **Measured (read-only census over 78 evidence + 36 intent
  artifacts + content sampling): the 0-relation docs are NOT a relation-extraction gap.** Two distinct
  causes, cleanly separated:
  **(A) STALE IntentIR (recoverable).** The canonical `intent_ir.json` is stale relative to its
  `evidence_ir.json` corpus-wide: `tilelink_1_7_1` carries **39** relations in evidence but **0** in its
  (05-16-dated) intent; `tilelink_1_8_0` 40→0, `um10204` i2c 17→0, `wbspec` 1→0; plus a broader "intent
  older than evidence" set (gic_600 / mmu_700 / ihi0082 ATS / dti / opencapi×3 / usb4). A **deterministic
  `semantic`→`intent` rebuild** (no LLM, no Docling) recovers them — **PROVEN live on `tilelink_1_7_1`:
  relations 0→39, actor_ports 0→69, all 40 actors connected** (RAM steady 77%). Operational cause (evidence
  rebuilt under a sweep without cascading downstream; `converge` rebuilds the whole chain, stage commands do
  not auto-cascade), not a code bug.
  **(B) HONEST ABSENCE (not a gap).** `nvme`/`risc_v_iommu`/VT-d/`ccix` declare **~0 wire signals**
  (content-sampled: their drive/read "cues" are ToC entries, register-access descriptions `RO`/`RW`, and
  agent-MESSAGE/transaction prose — never agent-SIGNAL relations). These are register/command/coherency
  protocols whose intent lives in `register_records`/`message_field_records`/`transactions`, NOT in
  actor-signal relations. 0 relations is CORRECT; forcing them would FABRICATE (the north-star caution).
  **Genericity insight:** the actor-signal-relation surface is intrinsically wire-protocol-shaped;
  relation-completeness is the wrong bar dimension for register/message protocols. **Frontier → (i)** corpus
  refresh so the recovered relations land canonically (owner gap #2); **(ii)** a generic STAGE-STALENESS
  detector in `validate` so a stale downstream artifact silently dropping relations is surfaced, not hidden
  (candidate code slice — serves "the KG must be COMPLETE"). ADR-0006. Report
  `docs/research/relation-completeness-measurement.md`; KM `[[relation-completeness-staleness-vs-absence]]`.
  **CLOSED `2026-06-24` (verification-only, no code change).** Both frontier sub-steps satisfied:
  **(i) corpus refresh** — a full re-census over all 78 persisted `intent_ir.json` vs their
  `evidence_ir.json` finds **0 stale docs** (zero with `evidence>0 & intent==0`); the
  `CORPUS-COVERAGE.2` re-ingest sweep rebuilt the affected docs via `converge` (whole-chain cascade),
  so the recovered relations landed canonically — `tilelink_1_7_1` 33/33, `tilelink_1_8_0` 34/34,
  `um10204` I2C **17/17**, `gic_600` 101/101, `mmu_700` 25/25, ATS `ihi0082` 9/9, DTI 1/1, opencapi
  transaction-layer 15/15, USB4 13/13 (each `intent`==`evidence`); `wbspec` 0/0 (reclassified to
  honest-absence); 33 register/PHY/command docs at 0/0 = the correct (B) honest-absence class.
  **(ii) stage-staleness detector** — shipped + unit-tested as `CORPUS-COVERAGE.1`
  (`stage_staleness_relation_finding` → `semantic_stale_relations_dropped` /
  `intent_stale_relations_dropped`, category `stage_staleness`; KM `[[stage-staleness-validate-detector]]`).
  Bar #2 relation-completeness is therefore resolved for the recoverable class, correctly N/A for
  register/message protocols, and held at WIRE-BASED-100 = 1.000 on the wire class — no fabrication.
  Closure appended to the report; gates orthogonal (no code change); `check_doctrines.sh` GREEN.
- ID: `KG-ISF-COMPLETENESS.4` · Status: `done` (`2026-06-17`, read-only measurement, docs-only) · Goal:
  **bar #5/#6 re-assessment on the broader 78-doc corpus** — `.2` measured ISF-lowering fidelity over 36
  docs; `CORPUS-COVERAGE.0` then doubled the corpus to 78 (register/coherency/profile-heavy), so the
  resume-pointer standing candidate (b) was to re-check whether behavior/temporal rules are still
  carried-or-residual (no silent drop) at the new scale. **Measured (faithful Python replication of the
  three `IsfIr::from_intent_ir` `(rule)` filters over all 78 `intent_ir.json`):** `conditional_rules` 2237
  total / 516 lower / 1670 no-consequent / **51 undeclared-named**; `signal_constraints` 369 / 287 / 0 /
  **82**; `temporal_invariants` 29343 / 286 / 29030 empty-subject / **27**; the residualizing path
  (`temporal_rules` 320 + `actor_contracts` 211) is bar-#5-safe by construction
  (`[[isf-temporal-lowering-no-silent-drop]]`). The 30 700 empty-subject/no-consequent drops are correctly
  silent ("absence is not an event"). The **160 undeclared-NAMED-subject drops** (the only genuine bar-#5
  silent-loss candidate) were characterised item-by-item: 5 already on the register surface; the other 155
  are **field content + noise, NOT wire intent** — register/message FIELD mnemonics (NVMe `MTFA`/`HMDLAL`,
  CCIX `SAMH`/`ESMD`, RISC-V `DC.tc.SXL`; homed on the field surfaces by design — the `.isf` does not lower
  fields), DTI message-field obligations leaked into `signal_constraints` (DTI carries **no
  `message_field_records`** — a field-recognition gap, spun out), prose/hex noise (`DMA`/`TLB`/`PCI`/`IBM`/
  `FFFF`/`FFFFFFFF_FFFFFFFF`/`Reserved`/`this bit`), and the ATP `ihi0082` cluster of real AXI names from
  **garbled** VLM fragments (`"RREADY is RBR"`). **0 undeclared/silent drops on all four wire docs**
  (re-confirmed at 2× scale). **Conclusion: bar #5/#6 HOLDS on the broader corpus — no buildable
  lowering-residual lever; the adapter's silent skip of an undeclared-subject rule is CORRECT** (the genuine
  gaps are upstream: route field obligations to the field surface / filter prose noise — never residualize
  at the adapter, which would reintroduce exactly the noise `.2`/`.2b` rejected). CONFIRMS + extends `.2`
  to the doubled corpus. ADR-0006 (universal counts, structural classification, no name list).
  **Spun-out grounded observations** (each needs own measurement-first ownership): **(1)** DTI `ihi0088`
  message-field recognition gap → `EXTRACTION-QUALITY-GAUGE.FIELD`/`PDF-VARIANT-DIGESTION` (most
  actionable); **(2)** signal-inventory prose noise (`AMBA`/`ARM`/`APCI` minted as DTI signals) →
  signal-precision; **(3)** ATP VLM-fragment quality. Report
  `docs/research/behavior-temporal-lowering-completeness-78doc.md`; KM
  `[[behavior-temporal-lowering-broader-corpus]]`. `[[project_kg_isf_completeness]]` /
  `[[feedback_not_complete_attack_substantive_gaps]]`.

- ID: `KG-ISF-COMPLETENESS.5` · Status: `active` (umbrella; **measurement DONE `2026-06-24`**, read-only,
  docs-only; code → `.5.i`/`.5.ii`/`.5.iii` all LANDED `2026-06-24`, `.5.iv.a` LANDED `2026-08-31`;
  `.5.iv` measured `2026-08-11`, `.5.v` censused and `.5.vi` adjudicated `2026-09-21`; the open
  children are `.5.vi.a` (CODE — the caption-label identity gate) and `.5.vii` (positional values),
  in that order; node layout given by `LIVE-DOCUMENT-PRESSURE-HEADROOM.4e`) · Goal: **enum-surface fidelity (bar #6) — the generic-`TABLE`
  mega-enum conflation.** Surfaced by the `CORPUS-COVERAGE.2` re-ingests of JEDEC HBM2 (#28) and AMBA CHI C2C
  (#29), explicitly deferred by `.2a.iv` (Lever F) as "a future extraction-precision lever". **Measured
  (read-only, current binary + 78-doc persisted corpus; reproducer in the report):** the `.isf` emits a
  generic junk-named enum (HBM2 `(type TABLE (bits 6))`) fusing ~7 unrelated value-tables (29 dup values, 30
  sentence-fragment member names). **Origin = EXTRACTION, not the emitter:** `derive_encoding_enum_name`'s
  fallback (`evidence.rs:4457-4461`) names an unmatched encoding table after the first caption token passing
  `is_hardware_signal_token` (`evidence.rs:7106`, which accepts `Table`→`TABLE`), and `build_symbol_definitions`
  (`semantic.rs:2782-2789`) merges every same-named table into ONE enum by name (copied verbatim to IntentIR
  `intent.rs:189`; lowered faithfully `isf_ir.rs:889-912`). **Corpus:** 56/78 docs carry a generic-named enum
  (96 generic `TABLE`/`FIGURE`/`DATA`/… vs 493 real); ~95 reach `.isf`. **Key insight:** a name-only gate
  catches 96 generic but MISSES **271 real-named-but-junk** enums (`COMMAND`/`DWORD_MISR`/`AMBA` — fragment
  names + restarting values; only 222 of 493 real enums are clean) → the load-bearing signal is **member
  quality**, the name is the symptom. **Decision: GO, extraction-side, decomposed** (`.5.i` safe name-gate +
  emitter orphan-type fix; `.5.ii` member-quality gate). Universal structural rule, ADR-0006 (no chip-name
  list). **WIRE-BASED-100:** scores ORTHOGONAL (enums unscored — `eval-extraction` golds carry no enum), but
  the `.isf` BYTES change on all 4 wire golds (each emits a junk `TABLE`; AHB's fuses HTRANS+HSIZE which
  already have correct enums) — a strict IMPROVEMENT, not byte-identical, needing a deliberate snapshot
  refresh. No code → all oracles orthogonal by construction; `check_doctrines.sh` GREEN; KM 125→126. Report
  `docs/research/generic-enum-conflation-measurement.md`; KM `[[generic-enum-conflation]]`.
  `[[feedback_scoring_rigor]]` / `[[feedback_avoid_denylists_prefer_structural]]`.

- ID: `KG-ISF-COMPLETENESS.5.i` · Status: `done` (`2026-06-24`, fresh focused session per the high-stakes
  gate-code rule; LANDED + WIRE-BASED-100-verified + corpus-censused) · Goal: **the safe extraction-side
  fallback name-gate + the emitter orphan-`(type)` fix.** (1) `derive_encoding_enum_name` (`evidence.rs:4457-4461`)
  must NOT return a name that is merely a document-structure token — gate it positively (the candidate token
  must independently be a declared signal / column header) and return `None` otherwise (a `None` fallback is
  the EXISTING contract: both call sites `continue` → no enum minted → honest residual). Kills all 96 generic
  enums AND the conflation (the merge is keyed on the shared name). (2) `isf_ir.rs:403-409` emits ALL
  `self.types` unconditionally → a Lever-F-residualized enum still leaves an orphan `(type TABLE (bits 6))`
  line; gate the types block by `emitted_enums()`. **Design (locked by read-only probe `2026-06-24`):** the
  defect lives ONLY in the fallback; the genuinely-named enums (HBM2 `COMMAND`/`EXTEST_RX`/`DWORD_MISR`,
  wire-gold `PPROT`/`HTRANS`/`HSIZE`/`HPROT`/`HRESP`/`WLAST`/`RLAST`/`BRESP`/`RRESP`/`TKEEP`) come from the
  **signal-match path** (the candidate IS a declared signal matched in caption/section/header, returned BEFORE
  the fallback ever runs), so the fallback gate cannot touch them — they stay byte-identical. The fallback
  produces only document-structure/caption-word names (`TABLE`/`FIGURE`/`COLUMN`/`DATA`/`ANNEX`, and the AMBA
  caption words `AMBA`/`BYTE`/`READ`/`CACHE`/`RELEASE`), none of which is a declared signal or a column header
  → all dropped. The structural gate is therefore strictly BETTER than a name-only gate (it also catches the
  fallback-origin real-named-but-junk like `AMBA`); the SIGNAL-MATCH-origin real-named-but-junk
  (`COMMAND`/`DWORD_MISR` — declared signals with fragment members) is untouched and stays `.5.ii`'s job.
  Acceptance: see the dedicated checklist below.

<!-- kg-isf-completeness-task-source-region:isf-lowering-fidelity:end -->
