---
id: isf-enum-value-literal-emit-gate
title: The `.isf` emitter drops an enum whose member value is a bare binary-looking token (only `0`/`1` digits, length >= 4 — an un-qualified binary literal FSMGen rejects) and records an `isf_enum_value_literal_<name>` residual, instead of emitting an invalid literal — so a mega-conflated binary-as-decimal enum (HBM2 `TABLE.REPAIR_LANE`) no longer breaks the whole `.isf` (KG-ISF-COMPLETENESS.2a.iv, Lever F, surfaced by CORPUS-COVERAGE.2 on JEDEC HBM2)
answers:
  - "why does an .isf enum get dropped / held out of the emitted .isf"
  - "what is KG-ISF-COMPLETENESS.2a.iv (ISF enum value-literal emit gate / Lever F)"
  - "why did HBM2's .isf fail fsmgen strict with enum member 'TABLE.REPAIR_LANE_8' value token '1000'"
  - "what enum member values does FSMGen reject (a bare token of only 0/1 digits with length >= 4 — an un-qualified binary literal; verified by value sweep: 1000/1010/1111/10000 fail, 999/1020/69152 and 0/1/111 and 4'b1000/16'd1000 pass)"
  - "is the enum drop a width-overflow rule (no — count-derived width 2a.iv hypothesis was DISPROVEN; GIC-600 emits 69152 strict-clean; FSMGen accepts bare decimals of any magnitude)"
  - "what does isf_enum_value_is_emittable_literal / isf_enum_is_emittable do in ir/isf_ir.rs"
  - "where is the dropped enum surfaced (an isf_enum_value_literal_<name> ResidualDecisionPacket via enum_residuals(), wired into adapters.rs residual_decisions)"
  - "are the wire-gold / other .isf affected by the enum emit gate (no — byte-identical; only HBM2 hbm.isf changes corpus-wide; the criterion never flags a legit decimal)"
  - "why drop the enum instead of width/radix-qualifying it (the value is a binary code mis-read as a decimal — the emitter can't recover the radix without fabricating; honest residual over fabrication)"
date: 2026-06-23
tags: [kg-isf-completeness, isf, emitter, enum, fsmgen-strict, package-symbol, adr-0006, residual, corpus-coverage, lever-f, fix]
evidence: crates/specforge/src/ir/isf_ir.rs (isf_enum_value_is_emittable_literal — bare [01]-only len>=4 token rejected; isf_enum_is_emittable; emitted_enums filters on it; enum_residuals + enum_value_literal_residual_packet); crates/specforge/src/ir/adapters.rs (residual_decisions extends isf_model.enum_residuals()); docs/tasks/KG-ISF-COMPLETENESS.md (.2a.iv node + acceptance checklist); docs/tasks/CORPUS-COVERAGE.md (.2 Lever F, HBM2 re-ingest #28)
reverify: "RAM-safe, no VLM/Docling. cargo build --release -p specforge. HBM2: target/release/specforge adapt generated/intent_ir/jesd235a_2015_11_hbm2_dram/intent_ir.json --target isf; perl subs/fsmgen/bin/fsmgen --strict --check --json generated/adapters/isf/jesd235a_2015_11_hbm2_dram/hbm.isf -> success=true, 0 diagnostics; adapter.json residual_decisions carries exactly ['isf_enum_value_literal_table'] (EXTEST_RX 212 / DWORD_MISR 19 KEPT). FSMGen rule sweep (minimal .isf, type bits 16, enum value v): 1000/1010/1111/10000 -> success=false; 999/1020/69152/0/1/10/111/4'b1000/16'd1000 -> success=true. Corpus byte-identical: re-emit all generated/adapters/isf/*/*.isf and diff vs a pre-change baseline -> only jesd235a_2015_11_hbm2_dram/hbm.isf changes (wire golds APB/AHB/AXI/SWD + Avalon + CoreSight SoC-600 + GIC-600 69152 + ARM-Debug 3360 byte-identical). run_ci.sh GREEN (lib 1706); kg-bench 156/156."
---

**Built `2026-06-23` (`KG-ISF-COMPLETENESS.2a.iv`, CODE — Lever F).** Surfaced by `CORPUS-COVERAGE.2` re-ingest
#28 (JEDEC HBM2 DRAM): FSMGen `--strict --check` rejected the WHOLE `.isf` with `Package … contains '+enums'
entry for enum member 'TABLE.REPAIR_LANE_8' with value token '1000', but package symbol values currently must
resolve to literal scalar values such as '0', '8'3', '8'hA5'`.

## Measurement-first correction (a wrong hypothesis caught by the diff + an FSMGen sweep)
The initial root-cause hypothesis was **count-derived width overflow** — `IsfIr::from_intent_ir`
(`ir/isf_ir.rs:878`) sets the enum's backing `(type … (bits B))` width from member COUNT
(`ceil(log2(count))`), so a value `>= 2^B` overflows. A width-fits gate was coded and then **DISPROVEN**: a
before/after `.isf` diff showed it wrongly dropped legitimate AXI `AWATOP`(49)/`AWSNOOP`/`ARSNOOP` and AHB
`TABLE`(64) enums, and a value sweep against the real FSMGen showed FSMGen ACCEPTS bare decimals of ANY
magnitude (GIC-600's `TABLE` carries `69152` and is strict-clean). So width is NOT the rule.

## The actual FSMGen rule (verified by value sweep)
FSMGen's package-symbol parser rejects a **bare token consisting ONLY of binary digits (`0`/`1`) with length
`>= 4`** — it treats it as an un-qualified binary literal. `1000`/`1010`/`1111`/`10000` fail; `0`/`1`/`10`/`111`
(<= 3 binary digits) and any value containing a 2-9 digit (`999`/`1020`/`69152`) pass at any magnitude; a
width/radix-qualified literal (`4'b1000`/`16'd1000`/`8'hA5`) passes at any magnitude. HBM2's
`TABLE.REPAIR_LANE` values (`1000,1001,1110,1111`) are exactly that shape — **binary codes the extractor
mis-read as bare decimals** in a generic `TABLE` mega-enum that conflates ~10 distinct doc tables.

## Fix (emitter-side, honest residual over fabrication)
`emitted_enums()` (`isf_ir.rs`) previously gated only on `is_safe_isf_scalar_value` (whitespace-free), so a
bare `1000` reached FSMGen. New `isf_enum_value_is_emittable_literal(value)` is FALSE only for the
binary-token shape; `isf_enum_is_emittable(e)` requires every member a safe scalar AND emittable-literal;
`emitted_enums()` filters on it. `enum_residuals()` records an `isf_enum_value_literal_<name>` packet for an
enum dropped *specifically* by this gate (all members safe-scalar but >= 1 binary token), wired into
`adapters.rs` `residual_decisions`. The emitter cannot width/radix-qualify the value without fabricating a
radix (the true binary intent was already lost upstream), so the enum is dropped, not guessed. Universal
token grammar, no name list (ADR 0006). The upstream mega-enum conflation + binary-as-decimal mis-read stays
an honest residual → a future extraction-precision lever.
