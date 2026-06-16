---
id: register-reset-isf-emit
title: SpecForge now lowers register-field reset_value to ISF (storage (var … (reset V))) at the true register width (ISF-REGISTER-RESET-EMIT CLOSED 2026-06-16) — was dropped at the emit boundary before .2/.3; 1508 registers strictly composable, all 4 wire docs byte-identical, FSMGen-strict-valid, symbolic/partial/unfit are honest residuals
answers:
  - "does SpecForge lower register reset values into the .isf (YES as of ISF-REGISTER-RESET-EMIT.2/.3 — composed from per-field reset_value and emitted at the true register width; it was dropped at the emit boundary before)"
  - "where does the ISF emitter lower register reset_value (ir/isf_ir.rs: IsfStorageVar.reset + classify_register_reset + register_var_width; render emits (var NAME (width W) (reset V)); before .2/.3 IsfStorageVar had only name+width and dropped it)"
  - "where is register-field reset_value extracted and carried (ir/source.rs:432 RegisterFieldRecord.reset_value; ir/evidence.rs:11509/11838 populate it; ir/intent.rs:193 clones register_records to IntentIR)"
  - "does FSMGen support a register reset value in storage (yes — (storage (var NAME (width N) [(reset V)])) is shipped per 13k:42 + 13m:48-68; optional, in-width non-negative int, omission = all-0s byte-identical, over-width/non-integer fails closed)"
  - "how many registers can SpecForge compose a faithful ISF reset for (1508 strictly composable corpus-wide; 1339 fit the current emit width; 446 have V>0 — the real .isf diff)"
  - "which docs gain a register reset in the .isf (only the 3 CoreSight SoC-600 TRMs — 199/127/120 V>0 resets; the register-heavy non-wire docs)"
  - "do the wire docs (APB/AHB/AXI/SWD) change when register reset is lowered to ISF (no — ZERO composable resets, .isf byte-identical, WIRE-BASED-100 holds trivially)"
  - "why does AXI have 71 registers but zero composable resets (AXI register_records are encoding pseudo-tables — Valid and Ready signals / Resource plane number properties — fields with no bit positions and symbolic resets like -, False, AxPROT[1])"
  - "how to compose a register-level ISF reset from per-field reset_value (LSB-tiling: V = OR(parse_int(reset_i) << bits_low_i), mirroring ir/register_bits.rs; only when every field is located + parseable non-neg int fitting its field width + no overlap)"
  - "what reset_value shapes are composable vs residual (numeric dec/0x/0b/…h compose; UNKNOWN, IMPLEMENTATION DEFINED, 0x-------- partial-unknown, -, X, Impl Spec, Configuration dependent, enum-annotated are honest residuals — ADR-0006, no name list)"
  - "why are 169 composable registers not yet emittable (their composed reset needs more bits than the current storage-var width which is max-field-extent not register width — e.g. CoreSight DPIDR V=0x1c013477 at width 11 over-width; var-width reconciliation spun to ISF-REGISTER-RESET-EMIT.3)"
  - "is the ISF storage-var width the register width (no — ir/isf_ir.rs uses max single-field extent, a latent bug; the true width is size_bits or max(bits_high)+1)"
  - "should a documented reset of 0 be emitted as (reset 0) in ISF (no — omit; the FSMGen all-0s default faithfully represents it; 893 of the fits-current registers are V==0)"
date: 2026-06-16
tags: [isf-register-reset-emit, isf, storage, registers, reset, intent-ir, adr-0006, measured, fsmgen-contract, north-star, ir-isf-ir]
evidence: docs/research/register-reset-emit-measurement.md (the full .1 corpus measurement + GO decision); crates/specforge/src/ir/isf_ir.rs:81/730/375 (IsfStorageVar + the storage build + the (var …) render that drops reset); crates/specforge/src/ir/source.rs:432 (RegisterFieldRecord.reset_value); crates/specforge/src/ir/intent.rs:193 (register_records carry-through); subs/fsmgen/docs/book/src/13k-isf-feature-support-matrix.md:42 + 13m-local-variables.md:48-68 (the shipped (reset V) contract); generated/intent_ir/*/intent_ir.json (the 36 docs scanned)
reverify: "CLOSED 2026-06-16 (ISF-REGISTER-RESET-EMIT.2 emit + .3 var-width). Confirm it is lowered: grep -n 'reset\\|register_var_width\\|classify_register_reset' crates/specforge/src/ir/isf_ir.rs — IsfStorageVar now has a reset field, classify_register_reset LSB-tiles, register_var_width = size_bits ⊔ max(bits_high)+1, render emits (var NAME (width W) (reset V)). Live: rebuild a CoreSight SoC-600 doc (specforge adapt generated/intent_ir/100806_0100_*/intent_ir.json --target isf) → its .isf has 183 (var … (reset …)) lines incl. (var table_9_3_dpidr_bit_assignments (width 32) (reset 469841015)); rebuild AXI (ihi0022_l) → .isf byte-identical to the pre-.2 baseline (0 storage resets); subs/fsmgen/bin/fsmgen --strict --check --json on the CoreSight .isf → success:true/0 errors. The measurement still reproduces (read-only python over generated/intent_ir/*/intent_ir.json): 1508 strictly-composable, 446 fits-current-width V>0 (all 3 CoreSight TRMs), 1045/2108 vars change width under .3 (0 in wire docs). NVMe/HBM2 keep PRE-EXISTING rule-conflict/enum-literal strict diagnostics (unrelated to storage)."
---

**Established `2026-06-16`** (`ISF-REGISTER-RESET-EMIT.1`, read-only over the 36 persisted
`generated/intent_ir/*/intent_ir.json`; report `docs/research/register-reset-emit-measurement.md`).
**CLOSED `2026-06-16`** — `.2` (emit `(reset V)`) + `.3` (var-width reconciliation) landed the lowering;
the prose below describes the original gap and the measurement that drove it.

SpecForge extracts register-field `reset_value` (`ir/source.rs:432`, populated from register tables
`ir/evidence.rs:11509`/`:11838`) and carries it to IntentIR (`ir/intent.rs:193`). Before `.2`/`.3` the
ISF emitter built a `(storage …)` block per register but rendered `(var NAME (width W))` and **dropped
`reset_value`** (`IsfStorageVar` had only `name`+`width`); now it composes the reset by LSB-tiling and
emits `(var NAME (width W) (reset V))` at the true register width (`register_var_width`), with
symbolic/partial/unfit resets left as one honest adapter residual. FSMGen's
`(storage (var NAME (width N) [(reset V)]))` is already `shipped`
(`subs/fsmgen/docs/book/src/13k-isf-feature-support-matrix.md:42`, `13m-local-variables.md:48-68`) —
`(reset V)` optional, V a non-negative integer fitting the width, omission = all-0s (byte-identical),
over-width/non-integer fails closed — so closing the gap needs **no FSMGen FR**.

**Measured surface:** 19 of 36 docs carry registers (2561 total). **1508 are strictly composable**
(every field located + `reset_value` parseable as a non-neg int fitting its field width + no overlap;
LSB-tiled to a register `V`). 1339 fit the current (max-field-extent) emit width; of those **446 have
V>0** — the real `.isf` diff, **entirely in the 3 CoreSight SoC-600 TRMs** (199/127/120). 893 are V==0
(faithfully the FSMGen all-0s default → omit). **All 4 wire docs (APB/AHB/AXI/SWD) emit ZERO reset**
→ `.isf` byte-identical, WIRE-BASED-100 trivially held (AXI's 71 "registers" are positionless encoding
pseudo-tables; AHB/APB/SWD have ~no field resets). 169 composable registers need a **wider var width**
than the current max-field-extent (the latent width bug — true width is `size_bits`/`max(bits_high)+1`);
their reset would be over-width and FSMGen would fail it closed, so the var-width reconciliation is
spun out as `ISF-REGISTER-RESET-EMIT.3` and they are honest residuals for `.2`.

Decision: **GO** — **DONE.** `.2` emitted `(reset V)` for the strictly-composable V>0 registers and
`.3` set the var to the true register width (`size_bits ⊔ max(bits_high)+1`) so the over-width ones also
emit; the rest are honest residuals. Live: AXI/AHB/APB/SWD `.isf` byte-identical, CoreSight SoC-600 183
resets, FSMGen-strict-valid (NVMe/HBM2 keep pre-existing unrelated rule/enum diagnostics), WIRE-BASED-100
1.000, `kg-bench` 156/156, `run_ci.sh` green.

See `[[transaction-capture-census]]` for the parallel ISF-fidelity (transactions) lens; the
governing north star is `[[project_kg_isf_completeness]]` (bar #6: every IntentIR fact appears in the
`.isf` or an explicit residual).
