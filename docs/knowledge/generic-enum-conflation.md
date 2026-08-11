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
  - "how is the .5.ii member-quality gate designed / what did the .5.ii calibration find (measured 2026-06-24 read-only over 78 docs/561 enums/12509 members: the gate is PER-MEMBER not per-enum — a whole-enum drop destroys AXI BRESP's real codes OKAY/EXOKAY/SLVERR/DECERR which are FUSED with prose fragments in one conflated enum; value-restart is NOT a junk signal — AHB HPROT restarts but every member is a clean identifier. The load-bearing signal is per-member NAME shape: an English sentence-SPINE token marks a prose fragment. Land a per-member sentence-spine fragment drop at synthesize_encoding_declarations_for_enum)"
  - "is the .5.ii enum member-quality gate landed (yes, LANDED 2026-06-24: is_prose_fragment_member_name + PROSE_SENTENCE_SPINE_WORDS in ir/evidence.rs gate the member loop in synthesize_encoding_declarations_for_enum, one seam for both call paths; AXI manager.isf now emits (BRESP (OKAY 0)(EXOKAY 1)(SLVERR 2)(DECERR 3)(DEFER 4)(TRANSFAULT 5)(RESERVED 6)(UNSUPPORTED 7)) recovering codes from the 16-member prose-fused enum; WIRE-BASED-100 1.000 before==after across all 10 seeds, FSMGen --strict success on AXI+APB, kg-bench 156/156, run_ci GREEN lib 1716 +4 tests. .5 enum-surface fidelity now built)"
  - "what is the .5.ii sentence-spine member-fragment predicate (a synthesized enum member_name is a prose fragment if any _-token is an English sentence-spine word — copula/aux/modal IS/ARE/BE/HAS/MUST/SHALL, article/demonstrative THE/THIS/THAT, relativizer/subordinator WHICH/WHEN/IF/BECAUSE — EXCLUDING the .1a collisions A/I/ITS/CAN/MAY/AM. Precision 1.000 (0/115 clean-anchor flagged), recall 1.000 (269/269 junk-anchor caught), 30.2% of members drop; universal grammar ADR-0006, no name list)"
  - "why not gate the whole enum on value-restart for .5.ii (DISPROVEN false-positive: AHB HPROT has value restarts=2 from 3 fused sub-encodings but all 15 members are clean identifiers DATA_INST/PRIVILEGED/BUFFERABLE/...; dropping it loses real intent. Restart correlates with conflation but conflation-of-clean-tables is all-real-members, so restart cannot gate a drop — keep it, sub-enum splitting deferred)"
  - "is removing the generic enums WIRE-BASED-100-safe (scores ORTHOGONAL/SAFE — generic enums are in no scored gold; but the .isf BYTES change on all 4 wire golds — APB/AHB/AXI/SWP each emit a junk TABLE; AHB's TABLE fuses HTRANS+HSIZE which already have correct enums — a strict improvement needing a deliberate snapshot refresh, NOT byte-identical)"
  - "is the orphan (type TABLE) line a separate emitter bug (yes — isf_ir.rs:403-409 emits all self.types unconditionally, so a Lever-F-residualized enum still leaves an orphan (type ...) line; gate by emitted_enums())"
  - "what are the deeper enum member-quality residual classes after .5.ii / what did the .5.iii measurement find (measured 2026-06-24 read-only, reproducer scripts/measure_enum_width_leak.py: of the 5 deferred classes — glossary SEE…, front-matter/ToC, section-caption B2_3_1_…, _WIDTH parameter leaks, value-restart-of-clean — most are SUBSUMED by .5.i (47/54 _WIDTH members and the bulk of 319 section-caption survivors sit in generic-named enums .5.i drops whole), EXCEPT the _WIDTH leak which reaches the AXI wire-gold .isf and is materially damaging)"
  - "why is the _WIDTH enum-member leak a real fidelity defect (.5.iii: 7 _WIDTH members in real-signal-named enums in AXI gold ihi0022_l reach manager.isf — (BRESP (BRESP_WIDTH 0)(OKAY 0)…) duplicates value 0, (RRESP (RRESP_WIDTH 0)) REPLACES the real RRESP codes, (AXSNOOP (AWSNOOP_WIDTH 0)(ARSNOOP_WIDTH 1)) pure junk; a width PARAMETER 'Enum BRESP BRESP_WIDTH = 0.' mis-read as an encoding VALUE — a false bar-#6 fact, unscored by WIRE-BASED-100 since enums are emitter-orthogonal)"
  - "what is the .5.iii _WIDTH parameter-leak gate / is it landed / is it ADR-0006 safe (LANDED 2026-06-24: is_width_parameter_leak_member + a continue-skip in synthesize_encoding_declarations_for_enum after the .5.ii spine gate, known_signals threaded from the signal-match caller. Drops a synthesized encoding member named <X>_WIDTH iff X is a declared signal OR the enum's own name — document-grounded like .5.i, NOT a name list; corpus FP set EMPTY: no legit FULL_WIDTH/HALF_WIDTH value exists and the declared-signal arm never catches one since FULL/HALF are not signals; per-member not per-enum so BRESP keeps its codes and RRESP/AXSNOOP empty to honest residuals. AXI manager.isf now (BRESP (OKAY 0)(EXOKAY 1)...) + (AWCMO (CLEAN_AND_INVALIDATE 0)(CLEAN_ONLY 1)); false RRESP/AXSNOOP/RCHUNK* _WIDTH enums gone; FSMGen --strict success/0; WIRE-BASED-100 1.000 before==after; kg-bench 156/156; run_ci GREEN lib 1718 +2)"
  - "can an encoding table's column header SOURCE an enum name rather than only veto one / what did .5.iv measure (measured 2026-08-11 read-only, reproducer scripts/measure_encoding_enum_header_naming.py: derive_encoding_enum_name sources candidates ONLY from caption_text or the section title (evidence.rs:4698-4703) and then validates them against known_signals + the header (:4715-4733), so the header is a veto and never a source. Corpus: 2,540 encoding tables -> 281 header-nameable <FIELD> value|Description -> 134 minting a non-empty enum after the .5.ii spine gate, in 10 docs. GO on the lever, NO-GO on the naive predicate; CODE deferred to .5.iv.a)"
  - "does header-sourced enum naming re-create the merge-by-name conflation (NO — 28 of 28 collision groups agree on every shared value, 0 conflicts. Structural, not lucky: a caption keyword like Table is shared by unrelated tables, but a header names the actual field and a field encodes the same way throughout a document. Worked example SMMU SH: 11 tables in ihi0070_e_a, every shared value identical 0b00=NON_SHAREABLE/0b10=OUTER_SHAREABLE/0b11=INNER_SHAREABLE/0b01=RESERVED — the merge IS the correct encoding)"
  - "what must .5.iv.a exclude before header-sourced naming can land (four measured junk classes among the 134: OFFSET-headed register-offset tables where the header names a column concept not a field (3, CoreSight SDC-600); *_WIDTH self-named pseudo-enums whose only member is LEGAL_VALUES (the .5.iii honest residual, reappearing from the header side); garbled members (AXADDR -> VA_40/NUM_2_0_A); and 12 RESERVED-only enums carrying no intent. It is byte-changing on the AXI wire gold ihi0022_l (a new AWATOP enum) so it needs the full before/after WIRE-BASED-100 protocol)"
  - "would header-sourced naming have recovered the Arm SMMU guide's SEC_SID enum (NO — honest correction recorded at .5.iv: that table's members are whole description sentences, so the .5.ii spine gate drops them all and the enum empties however it is named. The lever is real but does not help the document that surfaced it)"
  - "why are section-caption / value-restart enum residuals NO-GO (.5.iii: section-caption/table-ref has no FP-free gate — leading [A-Z]?digit token collides with real codes D1/D2/L2 e.g. DEBUG:D1_1; restart-of-clean has no fidelity defect — .5.ii proved restart is not junk, all members real, mostly .5.i-dropped; glossary SEE…/front-matter are tiny + name-ish -> honest residuals)"
date: 2026-08-11
tags: [kg-isf-completeness, isf, enum, extraction, evidence-ir, semantic-ir, emitter, adr-0006, corpus-coverage, measurement, fidelity, bar-6]
evidence: crates/specforge/src/ir/evidence.rs (derive_encoding_enum_name :4457-4461 caption-keyword fallback; is_hardware_signal_token :7106 accepts 'TABLE'; synthesize_encoding_declarations_for_enum :11898/:11952 member synthesis); crates/specforge/src/ir/semantic.rs (build_symbol_definitions :2782-2789 merge-by-name, record :2865-2877); crates/specforge/src/ir/intent.rs (:189 verbatim copy to IntentIR); crates/specforge/src/ir/isf_ir.rs (:889-912 faithful enum lowering; :403-409 unconditional types block = orphan-type bug; :376 Lever-F value gate); docs/research/generic-enum-conflation-measurement.md; docs/tasks/KG-ISF-COMPLETENESS.md (.5 node); scripts/measure_encoding_enum_header_naming.py (.5.iv reproducer)
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

## `.5.ii` measurement (`2026-06-24`) — member-quality gate is PER-MEMBER

Read-only census over all 78 persisted IntentIR docs (561 enums / 12 509 members) overturns the
recorded `.5` plan. **Per-member, not per-enum:** a whole-enum drop destroys real codes — AXI-gold
`BRESP` fuses 7 prose fragments + `BRESP_WIDTH` WITH the 8 genuine codes
`OKAY/EXOKAY/SLVERR/DECERR/DEFER/TRANSFAULT/RESERVED/UNSUPPORTED` (drop the prose MEMBERS, keep the
codes → 16→9). **Value-restart is NOT a junk signal:** AHB-gold `HPROT` restarts (3 fused sub-encodings)
yet all 15 members are clean identifiers → a restart-gate is a false positive; keep it (sub-enum
splitting deferred). **The signal is per-member NAME shape:** the synthesis sanitizes a name-cell to
`[A-Z0-9_]`, so a prose sentence becomes one `_`-joined member name; a real symbol never contains an
English **sentence-spine** token (copula/aux/modal `IS`/`ARE`/`BE`/`HAS`/`MUST`/`SHALL`; article/
demonstrative `THE`/`THIS`/`THAT`; relativizer/subordinator `WHICH`/`WHEN`/`IF`/`BECAUSE`). Drop a
member carrying a spine token; an emptied enum is not minted (honest residual). **Collisions EXCLUDED**
per the `.1a` discipline: `A`/`I`/`ITS`/`CAN`/`MAY`/`AM`. **Precision 1.000** (0/115 clean anchor flagged)
/ **recall 1.000** (269/269 junk anchor caught); 30.2 % of members drop. Honest residuals deferred:
glossary `SEE…`, front-matter/ToC, section-caption `B2_3_1_…`, `_WIDTH` leaks, restart-of-clean. **GO**
— land at `synthesize_encoding_declarations_for_enum` (`evidence.rs`); byte-changing on wire golds →
before/after WIRE-BASED-100 eval required. Report §`.5.ii measurement`.

**`.5.ii` LANDED (`2026-06-24`).** `is_prose_fragment_member_name` + `PROSE_SENTENCE_SPINE_WORDS`
(`ir/evidence.rs`) gate the member loop in `synthesize_encoding_declarations_for_enum` (one seam → both
call paths): a member whose `_`-token set carries a spine word is skipped. AXI `manager.isf` now emits
`(BRESP (OKAY 0)(EXOKAY 1)(SLVERR 2)(DECERR 3)(DEFER 4)(TRANSFAULT 5)(RESERVED 6)(UNSUPPORTED 7))` (codes
recovered from the 16-member prose-fused enum). WIRE-BASED-100 **1.000 before==after** (all 10 seeds, gold
evidence rebuilt with baseline vs gated binary; scored surface byte-identical); FSMGen `--strict` `success`
on AXI+APB; `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1716, +4 tests). `.5` enum-surface fidelity now
built; deeper member-quality classes (glossary/front-matter/section-caption/`_WIDTH`/restart-of-clean) are
honest residuals. Report §`.5.ii LANDED`.

## `.5.iii` measurement (`2026-06-24`) — the `_WIDTH` parameter-leak is the one buildable deeper residual

Read-only census over the 78 persisted IntentIR docs of the five classes `.5.ii` deferred (reproducer
`scripts/measure_enum_width_leak.py`). **Most are SUBSUMED by `.5.i`:** 47 of 54 `_WIDTH` members and the
bulk of the 319 section-caption survivors live in generic-named enums (`TABLE`/`TRANSLATION`/`DEBUG`) that
`.5.i` already drops whole → no `.isf` reach. **But the `_WIDTH` leak reaches the AXI WIRE-GOLD `.isf` and
is materially damaging:** 7 members in real-signal-named enums in `ihi0022_l` emit
`(BRESP (BRESP_WIDTH 0)(OKAY 0)…)` (dup value 0), `(RRESP (RRESP_WIDTH 0))` (real codes replaced),
`(AXSNOOP (AWSNOOP_WIDTH 0)(ARSNOOP_WIDTH 1))` (pure junk), `(AWCMO (AWCMO_WIDTH 0)…)` (dup). Root cause:
a config/parameter row (`Enum BRESP BRESP_WIDTH = 0.`) leaked into the value enum — a width PARAMETER, not
an encoding VALUE. Unscored by WIRE-BASED-100 (enums emitter-orthogonal — why it held 1.000 while the
`.isf` carried junk). **GO** on a per-member gate: drop `<X>_WIDTH` iff `X` is a declared signal OR the
enum's own name — document-grounded (ADR 0006, like `.5.i`), corpus FP set EMPTY (no `FULL_WIDTH`-style
value exists; `FULL`/`HALF` are never declared signals so a real link-width enum is preserved). Per-member,
not per-enum (keeps BRESP's codes; empties RRESP/AXSNOOP → honest residual). **NO-GO** on section-caption
(leading `[A-Z]?digit` collides with real codes `D1`/`L2`), restart-of-clean (no defect), glossary/
front-matter (tiny). Byte-changing on the AXI gold → the code slice needs a before/after WIRE-BASED-100
eval. Report §`.5.iii measurement`.

**`.5.iii` LANDED (`2026-06-24`).** `is_width_parameter_leak_member` (`ir/evidence.rs`) + a `continue`-skip
in `synthesize_encoding_declarations_for_enum`'s member loop after the `.5.ii` spine gate; `known_signals`
threaded from the signal-match caller for the declared-signal arm. AXI evidence rebuild drops EXACTLY the 7
leaks (statements 6414→6407; non-Enum statement set byte-identical); `manager.isf` now emits
`(BRESP (OKAY 0)(EXOKAY 1)(SLVERR 2)(DECERR 3)(DEFER 4)(TRANSFAULT 5)(RESERVED 6)(UNSUPPORTED 7))` +
`(AWCMO (CLEAN_AND_INVALIDATE 0)(CLEAN_ONLY 1))`, and the false `RRESP`/`RCHUNKNUM`/`RCHUNKSTRB`/`AXSNOOP`
`_WIDTH`-only enums are gone; FSMGen `--strict` success/0. WIRE-BASED-100 **1.000 before==after** (AXI eval
identical; APB/AHB/SWD/i2c evidence byte-identical → gate inert); `kg-bench` 156/156; `run_ci.sh` GREEN
(lib 1718, +2). The `SECSID_WIDTH`/`SID_WIDTH`/`SSID_WIDTH` self-named pseudo-enums stay an honest residual.
Report §`.5.iii LANDED`.

Links: [[isf-enum-value-literal-emit-gate]] (Lever F — the value-literal gate that deferred
this), [[behavior-temporal-lowering-broader-corpus]] (`.4`, which spun out enum/signal
precision), [[project_kg_isf_completeness]], [[feedback_scoring_rigor]],
[[feedback_avoid_denylists_prefer_structural]].
