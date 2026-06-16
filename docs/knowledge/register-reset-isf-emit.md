---
id: register-reset-isf-emit
title: SpecForge extracts register-field reset_value but the ISF emitter drops it — (storage (var … (reset V))) is unfed; 1508 registers are strictly composable (446 with V>0, all in the 3 CoreSight SoC-600 TRMs), ALL 4 wire docs emit ZERO reset (byte-identical), and 169 need a var-width fix (spun to .3)
answers:
  - "does SpecForge lower register reset values into the .isf (no — extracted + carried but dropped at the ISF emit boundary)"
  - "where does the ISF emitter drop register reset_value (ir/isf_ir.rs:730 builds IsfStorageVar{name,width}, :375 renders (var NAME (width W)), :81 IsfStorageVar has no reset field)"
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
reverify: "Confirm the gap is still open: grep -n 'reset' crates/specforge/src/ir/isf_ir.rs around the IsfStorageVar struct (~:81) + the (var …) render (~:375) — until ISF-REGISTER-RESET-EMIT.2 lands, IsfStorageVar has only name+width and the render has no (reset …). Reproduce the measurement (read-only python over generated/intent_ir/*/intent_ir.json): for each register_records[].fields[], parse reset_value (dec/0x/0b/…h), locate via bits_high/bits_low/bit_width, LSB-tile V = OR(parse_int(reset_i) << bits_low_i), and bucket strictly-composable (located + parseable-fits-field + no-overlap) / fits-current-width(max-field-extent) / V>0 / needs-wider-width / wire-doc — expect 1508 composable, 1339 fits-current, 446 V>0 (all 3 CoreSight SoC-600 TRMs: 199/127/120), 169 over-width, and ZERO composable resets in APB/AHB/AXI/SWD (AXI's 71 register_records are positionless encoding pseudo-tables). FSMGen contract: grep -n 'reset' subs/fsmgen/docs/book/src/13k-isf-feature-support-matrix.md (line ~42: (storage (var NAME (width N) [(reset V)])) shipped, optional, in-width non-neg int, omission = all-0s, over-width/non-int fails closed)."
---

**Established `2026-06-16`** (`ISF-REGISTER-RESET-EMIT.1`, read-only over the 36 persisted
`generated/intent_ir/*/intent_ir.json`; report `docs/research/register-reset-emit-measurement.md`).

SpecForge extracts register-field `reset_value` (`ir/source.rs:432`, populated from register tables
`ir/evidence.rs:11509`/`:11838`) and carries it to IntentIR (`ir/intent.rs:193`), and the ISF emitter
already builds a `(storage …)` block per register (`ir/isf_ir.rs:730`) — but it renders
`(var NAME (width W))` (`:375`) and **drops `reset_value`** (`IsfStorageVar` has only `name`+`width`,
`:81`). FSMGen's `(storage (var NAME (width N) [(reset V)]))` is already `shipped`
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

Decision: **GO**. `.2` emits `(reset V)` for the strictly-composable, V>0, fits-current-width
registers (purely additive), records the rest as honest residuals; `.3` reconciles the var width.

See `[[transaction-capture-census]]` for the parallel ISF-fidelity (transactions) lens; the
governing north star is `[[project_kg_isf_completeness]]` (bar #6: every IntentIR fact appears in the
`.isf` or an explicit residual).
