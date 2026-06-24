---
id: generic-enum-conflation
title: The `.isf` generic-`TABLE` (and `FIGURE`/`DATA`/…) mega-enum is an EXTRACTION-born conflation — `derive_encoding_enum_name`'s fallback (evidence.rs) names an unmatched encoding table after its caption keyword (`Table N -` → `TABLE`), and `build_symbol_definitions` (semantic.rs) merges every same-named table into one junk enum; measured 56/78 docs / 96 generic + 271 real-named-but-junk enums (KG-ISF-COMPLETENESS.5). FIXED by `.5.i` (`2026-06-24`): the fallback now keeps a candidate ONLY when independently evidenced (a declared signal OR a column-header reference token), else returns None; + the emitter gates the (types) block by emitted_enums() — corpus generic enums 82→8, total enum records 422→105, WIRE-BASED-100 held 1.000
answers:
  - "is the generic-TABLE enum conflation fixed / what did KG-ISF-COMPLETENESS.5.i do (LANDED 2026-06-24: derive_encoding_enum_name fallback keeps the candidate only when independently evidenced — a declared signal OR a column-header reference token of the table — else None; emitter isf_ir.rs gates the (types) block by emitted_enums() so a member-dropped enum leaves no orphan (type ...). Corpus generic enums 82->8 / total enum records 422->105 across 33 rebuildable docs; real signal-match enums byte-identical; WIRE-BASED-100 1.000 before==after; fsmgen --strict 0 diagnostics)"
  - "why do 8 generic-named enums survive .5.i (they are document-evidenced — the token IS a declared signal or a column header in that doc, e.g. CCIX 'Table of Contents' header cells keep a 'TABLE' enum; the structural gate correctly cannot drop them without a forbidden name-list — honest .5.ii member-quality residuals)"
  - "does .5.i change anything besides enums (yes, beneficially — dropped Enum statements leave discovered_values, so off-gold junk value-constraints derived from junk-enum members also disappear, e.g. AXI ACTIVATEACK A -> grounded ACTIVATEACK 1; distinct constraint facts identical, WIRE-BASED-100 unaffected)"
  - "why does the .isf emit a generic (type TABLE (bits N)) enum / what is the TABLE mega-enum"
  - "where does the generic enum name TABLE/FIGURE/DATA come from (derive_encoding_enum_name fallback, evidence.rs:4457-4461 — first caption token passing is_hardware_signal_token at evidence.rs:7106, which accepts 'Table'->'TABLE')"
  - "why are many distinct value-tables merged into one enum (build_symbol_definitions accumulates members by enum_name key, semantic.rs:2782-2789 — every 'TABLE'-named table fuses into one SymbolDefinitionRecord)"
  - "is the generic-enum conflation an emitter bug or an extraction bug (EXTRACTION-born in evidence.rs + semantic.rs; isf_ir.rs:889-912 lowers it faithfully)"
  - "what is KG-ISF-COMPLETENESS.5 (the generic-enum-conflation measurement + decision packet)"
  - "how many docs / enums are affected (56/78 docs carry a generic-named enum; 96 generic vs 493 real; but a name-only gate misses 271 real-named-but-junk fragment/dup enums — the real defect is member quality)"
  - "is a name-only gate enough to fix the generic enum (no — 271 real-named enums like COMMAND/DWORD_MISR/AMBA are themselves fragment-heavy/dup-heavy; the load-bearing signal is member quality)"
  - "what is the recommended fix (.5.i extraction-side fallback name-gate: derive_encoding_enum_name must return None unless the candidate token is a declared signal -> no enum minted; + emitter orphan-type fix isf_ir.rs:403-409 gate types block by emitted_enums(); .5.ii member-quality gate for the 271 real-named junk enums, calibration-gated)"
  - "is removing the generic enums WIRE-BASED-100-safe (scores ORTHOGONAL/SAFE — generic enums are in no scored gold; but the .isf BYTES change on all 4 wire golds — APB/AHB/AXI/SWP each emit a junk TABLE; AHB's TABLE fuses HTRANS+HSIZE which already have correct enums — a strict improvement needing a deliberate snapshot refresh, NOT byte-identical)"
  - "is the orphan (type TABLE) line a separate emitter bug (yes — isf_ir.rs:403-409 emits all self.types unconditionally, so a Lever-F-residualized enum still leaves an orphan (type ...) line; gate by emitted_enums())"
date: 2026-06-24
tags: [kg-isf-completeness, isf, enum, extraction, evidence-ir, semantic-ir, emitter, adr-0006, corpus-coverage, measurement, fidelity, bar-6]
evidence: crates/specforge/src/ir/evidence.rs (derive_encoding_enum_name :4457-4461 caption-keyword fallback; is_hardware_signal_token :7106 accepts 'TABLE'; synthesize_encoding_declarations_for_enum :11898/:11952 member synthesis); crates/specforge/src/ir/semantic.rs (build_symbol_definitions :2782-2789 merge-by-name, record :2865-2877); crates/specforge/src/ir/intent.rs (:189 verbatim copy to IntentIR); crates/specforge/src/ir/isf_ir.rs (:889-912 faithful enum lowering; :403-409 unconditional types block = orphan-type bug; :376 Lever-F value gate); docs/research/generic-enum-conflation-measurement.md; docs/tasks/KG-ISF-COMPLETENESS.md (.5 node)
reverify: "RAM-safe, no VLM/Docling/rebuild. target/release/specforge adapt generated/intent_ir/jesd235a_2015_11_hbm2_dram/intent_ir.json --target isf; grep '(type TABLE' generated/adapters/isf/jesd235a_2015_11_hbm2_dram/hbm.isf -> '(type TABLE (bits 6))'. Corpus census: for each generated/intent_ir/*/intent_ir.json, count SymbolDefinition enums whose symbol_name is a doc-structure token (TABLE/FIGURE/DATA/COLUMN/ANNEX/NOTE/...) -> 96 generic across 56 docs vs 493 real. HBM2 TABLE: 57 members, 7 value-restart runs (dup 0..N seven times), 30 sentence-fragment member names. Wire golds: grep '(type TABLE' over generated/adapters/isf/{ihi0024_*,ihi0033_c,ihi0022_l,*swp*}/*.isf -> each emits a junk TABLE."
---

**Measured `2026-06-24` (`KG-ISF-COMPLETENESS.5`, read-only — no code).** Surfaced by the
`CORPUS-COVERAGE.2` re-ingests of JEDEC HBM2 (#28) and AMBA CHI C2C (#29); explicitly
deferred by `KG-ISF-COMPLETENESS.2a.iv` (Lever F) as "a future extraction-precision lever".

## What it is

The `.isf` emitter lowers IntentIR enums to `(type NAME (bits N) (M VALUE)…)`. HBM2's
`hbm.isf` emits `(type TABLE (bits 6))` — a junk-named enum fusing ~7 unrelated value-tables
(lane-remap binary codes + microbump geometry + IEEE-1500 opcodes + IDD currents + a footnote
+ a mode-register caption), with 29 duplicate values and 30 sentence-fragment member names.

## Why (extraction-born)

`derive_encoding_enum_name` (`evidence.rs:4457-4461`), when the known-signal match fails,
names the table after the first caption token passing `is_hardware_signal_token`
(`evidence.rs:7106`, which accepts `Table`→`TABLE`). HBM2's captions all read `"Table N - …"`,
so every unmatched encoding table is named `TABLE`, and `build_symbol_definitions`
(`semantic.rs:2782-2789`) merges all `TABLE`-named tables into ONE enum by name. The emitter
(`isf_ir.rs:889-912`) lowers it faithfully — it is not the source of the conflation.

## The deeper defect

A name-only gate catches the 96 generic enums but misses **271 real-named-but-junk** enums
(`COMMAND`/`DWORD_MISR`/`AMBA`, fragment names + restarting values). Of 493 real-named enums,
only 222 are clean. The generic name is the visible symptom of a **member-quality** defect.

## Decision (GO, decomposed)

`.5.i` — extraction-side fallback name-gate (`derive_encoding_enum_name` returns `None`
unless the candidate token is a declared signal → no enum minted; kills the 96 generic +
the conflation) + emitter orphan-`(type)` fix (`isf_ir.rs:403-409` gate the types block by
`emitted_enums()`). `.5.ii` — per-table member-quality gate for the 271 real-named junk
enums (calibration-gated). Universal structural rule, ADR-0006 (no chip-name list).

**WIRE-BASED-100:** scores orthogonal/safe (enums unscored); but the `.isf` bytes change on
all 4 wire golds (each emits a junk `TABLE`; AHB's fuses HTRANS+HSIZE) — a strict improvement
needing a deliberate snapshot refresh, deferred to a focused slice for signoff quality.

## `.5.i` LANDED (`2026-06-24`)

Two edits. **(1)** `derive_encoding_enum_name`'s fallback (`evidence.rs`) keeps the first
`is_hardware_signal_token` caption token **only when it is independently evidenced** — a declared
signal (`known_signals`) **or** a column-header reference token of the table — else returns `None`
(the existing `continue` contract → no enum minted → honest residual). **(2)** the emitter
(`isf_ir.rs`) gates the `(types …)` block by `emitted_enums()`, so a member-dropped enum leaves no
orphan `(type …)`. Two measurement corrections (both cleaner): the genuinely-named enums come from
the **signal-match loop above the fallback** (byte-identical), so the structural gate is *strictly
better* than a name-only gate — it also drops fallback-origin "real-named-but-junk" (`COMMAND`/`AMBA`/
`READ`/`CACHE`/`RELEASE`); and the gate also removes off-gold junk **constraints** whose value was a
junk-enum member (`discovered_values` coupling) — score-orthogonal, distinct facts identical.

**Measured:** corpus census (33 rebuildable docs) generic-named enums **82→8** / total enum records
**422→105**; the 8 survivors are document-evidenced column-header/declared-signal tokens (CCIX "Table
of Contents", gic_600 `DATA`) → `.5.ii` residuals. WIRE-BASED-100 **1.000 before==after** (proven via
before/after `eval-extraction` on rebuilt gold evidence); nvme-registers + i2c golds identical;
`kg-bench` 156/156; `run_ci.sh` GREEN (lib 1712); all affected `.isf` FSMGen-`--strict` 0 diagnostics.
ADR-0006 proven structural: `DATA` KEPT where a real gic_600 signal, DROPPED where a bare HBM2 caption
word. Report `docs/research/generic-enum-conflation-measurement.md` §`.5.i LANDED`.

Links: [[isf-enum-value-literal-emit-gate]] (Lever F — the value-literal gate that deferred
this), [[behavior-temporal-lowering-broader-corpus]] (`.4`, which spun out enum/signal
precision), [[project_kg_isf_completeness]], [[feedback_scoring_rigor]],
[[feedback_avoid_denylists_prefer_structural]].
