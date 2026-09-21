# KG-ISF-COMPLETENESS — enum surface early leaves

- Part ID: `enum-surface-early-leaves`
- State: `legacy`

<!-- kg-isf-completeness-task-source-region:enum-surface-early-leaves:start -->
## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.5.i` — DONE `2026-06-24`
- [x] **REPRODUCE / MEASURE** — current `.isf` emits a generic junk enum: HBM2 `hbm.isf` `(type TABLE (bits 6))` fusing ~7 value-tables (57 members, 29 dup values, 30 fragment names); corpus-wide 96 generic-named enums across 56/78 docs reach `.isf`. Wire golds (BEFORE, `grep '(type'`): APB `(type TABLE (bits 3))` beside real `PPROT`; AHB `(type TABLE (bits 5))`+`(type AMBA (bits 4))` beside `HTRANS`/`HSIZE`/`HPROT`/`HRESP`; AXI `(type TABLE (bits 8))`+`BYTE`/`READ`/`CACHE`/`RELEASE`/`AMBA` beside `WLAST`/`RLAST`/`BRESP`/`RRESP`; SWP `(type TABLE (bits 5))`+`(type ANNEX (bits 2))` beside `CLT`. Report `docs/research/generic-enum-conflation-measurement.md`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `derive_encoding_enum_name` fallback (`crates/specforge/src/ir/evidence.rs:4457-4461`) names an unmatched encoding table after the FIRST caption whitespace token passing `is_hardware_signal_token` (`evidence.rs:7106`, which accepts `Table`→`TABLE`); `build_symbol_definitions` (`semantic.rs:2782-2789`) then MERGES every same-named table into one enum by name (→ the conflation), copied to IntentIR (`intent.rs:189`), lowered by `isf_ir.rs:889-912`. The emitter ALSO emits all `self.types` unconditionally (`isf_ir.rs:403-409`), orphaning a `(type)` whose enum is dropped by `isf_enum_is_emittable`. Both call sites (`scan_encoding_tables_by_signal_anchor`:4668 `Some(known_signals)`; `synthesize_encoding_declarations`:11907 `None`) funnel through the one fallback. Probe `2026-06-24`: HBM2's real enums are signal-match-origin; only `TABLE`/`COLUMN`/`DATA` are fallback-origin.
- [x] **ADDRESSED (verified, measured per-item)** — two edits: (1) `derive_encoding_enum_name` fallback keeps the candidate ONLY when independently evidenced (declared signal OR column-header reference token), else `None`; (2) emitter gates the `(types)` block by `emitted_enums()`. Per-doc before→after (baseline vs gated binary; full `evidence→semantic→intent→adapt` rebuild from persisted source_ir): **HBM2** types 15→6 (drop `TABLE`/`COLUMN`/`COMMAND` + 6 orphan types; keep `UPDATEWR`/`EXTEST_RX`/`DATA`/`HBM_RESET`/`DWORD_MISR`/`CHANNEL_ID` byte-identical); **APB** `PPROT`+`TABLE`→`PPROT`; **AHB** drop `TABLE`/`AMBA`, keep `HTRANS`/`HSIZE`/`HPROT`/`HRESP`; **AXI** drop `TABLE`/`BYTE`/`READ`/`CACHE`/`RELEASE`/`AMBA`, keep 22 real signal enums; **SWD** drop 9 (`TABLE`/`REGISTER`/`ACCESSING`/`OF`/`USAGE`/`DRIVES`/`ARM`/`ATTRIBUTES`/`OK`), keep `TDI`; **SWP** `TABLE`/`CLT`/`ANNEX`→none; worst-offender type-counts amd 34→5, gic_600 23→6, coresight 19→5, cortex_a76 16→7, CHI 14→4, CHI-C2C 7→1. **Corpus census (33 rebuildable docs):** generic-named enums **82→8**, total enum records **422→105**; the 8 survivors are document-evidenced column-header/declared-signal tokens (CCIX "Table of Contents" headers etc.) → honest `.5.ii` residuals. Report §`.5.i LANDED`; +4 unit tests (3 fallback-gate + strengthened orphan-type test) green.
- [x] **NO REGRESSION** — **WIRE-BASED-100 = 1.000, before == after (PROVEN)** by rebuilding each gold doc's evidence with BOTH binaries and running `eval-extraction --provider skip`: APB signal_constraint 6/6 · relation 5/5; AHB 6/6·6/6; AXI 3/3·6/6; APB/AHB/AXI temporal 3/3·4/4·3/3; SWD relation 1/1 + documented promotion-only 0/1 — all identical old-vs-new. nvme-registers + i2c-signals golds identical old-vs-new. **`kg-bench` 156/156.** **`run_ci.sh` GREEN** (lib **1712** passed; warning-deny clippy/fmt/rustdoc + mdBook). Every affected `.isf` stays **FSMGen `--strict` 0 diagnostics** (renderable corpus strict-clean). The real-named signal-match enums are **byte-identical**; the only `.isf` body change beyond the enum block is on AXI/nvme, where off-gold junk constraints derived from dropped-enum `discovered_values` disappear (distinct constraint facts identical, score-orthogonal) — corrects the `.5` "orthogonal because unscored" reasoning to "orthogonal because off-gold + filtered, proven by the eval".
- [x] **GENERICITY (ADR 0006)** — universal structural rule: a fallback enum name must be independently evidenced (a declared signal OR a column-header reference token of the table), NOT a chip/vendor/structure-word name list. Proven structural, not a denylist: `DATA` is KEPT where it is a genuine gic_600 signal but DROPPED where it is a bare HBM2 caption word; the generic-name SET in the report is a measurement aid, never the gate.
- [x] **LOCKSTEP** — README current-state bullet; book `pipeline/isf-adapter.md` enum-fidelity note; KM card `[[generic-enum-conflation]]` updated to LANDED; CHANGES.md / DEVELOPMENT_NOTES.md / LIVE_ACHIEVEMENT_STATUS.md / MEMORY.md.

- ID: `KG-ISF-COMPLETENESS.5.ii` · Status: `done` (**measurement DONE + gate LANDED `2026-06-24`**,
  before/after-WIRE-BASED-100-verified) · Goal: **the member-quality gate** for the
  residual junk enums a NAME gate cannot reach. After `.5.i` the residual is (a) the **8 document-evidenced
  generic-named survivors** (`DATA`/`TABLE`/`CACHE`/`ENCODING`/`ATTRIBUTES` the `.5.i` gate KEEPS because the
  token IS a declared signal or column header — e.g. CCIX "Table of Contents" cells, still cross-table-
  conflated) + (b) the **signal-match-origin junk-member enums** (NAME is a real signal, MEMBERS are
  sentence fragments — HBM2 `DWORD_MISR`, AXI `BRESP`/`RRESP`/`ARCACHE`).
  **MEASUREMENT DONE `2026-06-24`** (read-only over all 78 persisted IntentIR docs / 561 enums / 12 509
  members; report `docs/research/generic-enum-conflation-results.md` §`.5.ii measurement`; KM
  `[[generic-enum-conflation]]`) — it **OVERTURNS the recorded `.5` plan**:
  - **The gate is PER-MEMBER, not per-enum.** A whole-enum drop destroys real codes — the surviving junk
    enums are CONFLATIONS of a junk table and a clean table: AXI-gold `BRESP` fuses 7 prose fragments +
    `BRESP_WIDTH` WITH the 8 genuine codes `OKAY/EXOKAY/SLVERR/DECERR/DEFER/TRANSFAULT/RESERVED/UNSUPPORTED`.
    The fix drops the prose MEMBERS and keeps the codes (BRESP 16→9).
  - **Value-restart is NOT a junk signal.** AHB-gold `HPROT` restarts (3 fused sub-encodings) yet every one
    of its 15 members is a clean identifier — a restart-gate would be a false positive losing real intent.
    Restart is dropped from the rule; conflation-of-clean-tables stays kept (honest residual: sub-enum
    splitting deferred).
  - **The load-bearing signal is per-member NAME shape — an English sentence-SPINE token.** The synthesis
    sanitizes a name-cell to `[A-Z0-9_]`, so a whole prose sentence becomes one `_`-joined member name; a
    real hardware symbol never contains a copula/auxiliary/modal (`IS`/`ARE`/`BE`/`HAS`/`MUST`/…),
    article/demonstrative (`THE`/`THIS`/…), or relativizer/subordinator (`WHICH`/`WHEN`/`IF`/`BECAUSE`/…).
    A member with a spine token is a prose fragment → drop. **Collisions EXCLUDED** (the `.1a` discipline):
    `A`/`I`/`ITS`/`CAN`/`MAY`/`AM` (article-vs-suffix, GIC `ITS`, CAN-bus, month). Universal grammar, ADR
    0006 — NOT a chip/structure-word name list.
  - **Precision/recall (per-item, `[[feedback_scoring_rigor]]`):** clean anchor **0/115 flagged → precision
    1.000**; junk anchor **269/269 caught → recall 1.000**; corpus-wide 3 781/12 509 (30.2 %) members drop.
    Wire-gold blast radius: `BRESP` 16→9 (recovers codes), `PPROT`/`ARCACHE`/`AWCACHE`/`RLAST`/`WLAST`→0
    (pure prose dropped), `HSIZE`/`HTRANS`/`HRESP` untouched. **Honest residuals deferred:** glossary
    `SEE…` refs, front-matter/ToC members, section-caption (`B2_3_1_…`) members, `_WIDTH` leaks,
    value-restart-with-clean-members.
  - **GO** — land a per-member sentence-spine fragment drop in `synthesize_encoding_declarations_for_enum`
    (`evidence.rs`): skip a member whose sanitized name carries a spine token; an emptied enum is not minted
    (existing no-statements → no-`SymbolDefinition` contract → honest residual). Byte-changing on wire golds
    (strict improvement) → the code slice needs a before/after WIRE-BASED-100 eval, NOT a byte-identical
    argument; `kg-bench` + FSMGen `--strict` + `run_ci.sh` hard gates.
  - **LANDED `2026-06-24`** — `is_prose_fragment_member_name` + the `PROSE_SENTENCE_SPINE_WORDS` const
    (`ir/evidence.rs`) gate the member loop in `synthesize_encoding_declarations_for_enum` (the single
    member-synthesis seam, so both `None`/`Some(known_signals)` paths are covered): a member whose
    `_`-token set carries a spine word is skipped, so a conflated enum keeps its codes and a pure-prose
    table emits no statements → no enum. +4 tests (collision-exclusion drift guard, predicate unit, two
    integration tests on the synthesis seam). **Verified:** WIRE-BASED-100 = **1.000 before==after** (all 10
    seeds — before/after eval on gold evidence rebuilt with the preserved baseline vs gated binary; scored
    surface byte-identical); AXI `manager.isf` now emits `(BRESP (OKAY 0)(EXOKAY 1)(SLVERR 2)(DECERR 3)
    (DEFER 4)(TRANSFAULT 5)(RESERVED 6)(UNSUPPORTED 7))` (codes recovered) + APB pass FSMGen `--strict`
    `success`; `kg-bench` 156/156; `run_ci.sh` GREEN (lib **1716**, +4). Acceptance checklist below.

- ID: `KG-ISF-COMPLETENESS.5.iii` · Status: `done` (**measurement DONE + gate LANDED `2026-06-24`**,
  before/after-WIRE-BASED-100-verified) · Goal: **the deeper enum member-quality residual classes** `.5.ii` left
  open (glossary `SEE…`, front-matter/ToC, section-caption `B2_3_1_…`, `_WIDTH` parameter leaks,
  value-restart-of-all-clean) — measure them per-item, build only the one that is both material and cleanly
  gateable. **MEASUREMENT DONE `2026-06-24`** (read-only over the 78 persisted IntentIR docs; reproducer
  `scripts/measure_enum_width_leak.py`; report `docs/research/generic-enum-conflation-results.md`
  §`.5.iii measurement`; KM `[[generic-enum-conflation]]`):
  - **Most deeper-residual members are SUBSUMED by `.5.i`.** Of 54 `_WIDTH` members, 47 sit in
    generic-named enums (`TABLE`/`TRANSLATION`/`BIT`) that `.5.i` already drops whole → no `.isf` reach;
    likewise the 319 section-caption survivors are dominated by `.5.i`-dropped `DEBUG`-class enums.
  - **The `_WIDTH` leak DOES reach a wire-gold `.isf` and is materially damaging.** The 7 remaining
    `_WIDTH` members are in real-signal-named enums in the AXI **gold** `ihi0022_l`: `(BRESP (BRESP_WIDTH 0)
    (OKAY 0)…)` duplicates value `0`, `(RRESP (RRESP_WIDTH 0))` REPLACES the real RRESP codes,
    `(AXSNOOP (AWSNOOP_WIDTH 0)(ARSNOOP_WIDTH 1))` is pure junk, `(AWCMO (AWCMO_WIDTH 0)…)` duplicates `0`.
    A width PARAMETER (`Enum BRESP BRESP_WIDTH = 0.`) leaked into the value enum — a false bar-#6 fact
    (unscored by WIRE-BASED-100, hence held at 1.000 while the `.isf` carried junk).
  - **The discriminator is FP-free and document-grounded (ADR 0006):** drop a member named `<X>_WIDTH`
    iff `X` is a declared signal OR the enum's own name. All 6 caught prefixes are TRUE declared signals;
    the corpus FP set is EMPTY (no legit `FULL_WIDTH`/`HALF_WIDTH` value exists, and the declared-signal
    rule would never catch one — `FULL`/`HALF` are not signals). Per-member, not per-enum (keeps BRESP's
    codes; empties RRESP/AXSNOOP → honest residual).
  - **The other classes are NO-GO** — section-caption/table-ref has no FP-free gate (leading `[A-Z]?digit`
    collides with real codes `D1`/`L2`); restart-of-clean has no fidelity defect (all members real, mostly
    `.5.i`-dropped); glossary/front-matter are tiny + name-ish. Honest residuals.
  - **GO** — land a per-member `_WIDTH` parameter-leak drop in `synthesize_encoding_declarations_for_enum`
    (`evidence.rs`), parallel to the `.5.ii` spine gate; thread the existing `known_signals` set (in scope
    at the signal-match caller `evidence.rs:4700`) for the declared-signal arm, enum-self-name needs no
    plumbing. Byte-changing on the AXI wire gold → before/after WIRE-BASED-100 eval REQUIRED (a strict
    improvement: scored surface byte-identical, `manager.isf` recovers BRESP/AWCMO + drops RRESP/AXSNOOP).
    `kg-bench` + FSMGen `--strict` + `run_ci.sh` hard gates. Acceptance checklist below.
  - **CODE LANDED `2026-06-24`** — `is_width_parameter_leak_member(member, enum, known_signals)` +
    a `continue`-skip in `synthesize_encoding_declarations_for_enum`'s member loop right after the
    `.5.ii` spine gate (`ir/evidence.rs`): a member named `<X>_WIDTH` where `X` is the enum's own name
    OR a declared signal (the `known_signals` set, now threaded from the signal-match caller; the `None`
    caller covers the enum-self case) is a width PARAMETER → skipped; a pure-parameter enum empties (no
    statements → no `SymbolDefinition` → honest residual). +2 tests (predicate FP-guard incl. `FULL_WIDTH`
    kept; synthesis seam keeps BRESP codes / empties AXSNOOP). **Verified:** AXI evidence rebuild drops
    EXACTLY the 7 leaks (`BRESP/RRESP/RCHUNKNUM/RCHUNKSTRB/AWSNOOP/ARSNOOP/AWCMO _WIDTH`; statements
    6414→6407; non-Enum statement set byte-identical); AXI `manager.isf` now emits `(BRESP (OKAY 0)…)`
    (8 clean codes, no dup) + `(AWCMO (CLEAN_AND_INVALIDATE 0)(CLEAN_ONLY 1))` and the false
    `(RRESP (RRESP_WIDTH 0))`/`AXSNOOP`/`RCHUNK*` enums are gone; FSMGen `--strict --check` **success / 0
    diagnostics**. **WIRE-BASED-100 = 1.000 before==after** (AXI before/after eval identical; APB/AHB/SWD/i2c
    evidence byte-identical old-vs-new → gate inert). `kg-bench` 156/156; `run_ci.sh` GREEN (lib **1718**,
    +2). Acceptance checklist below. The `SECSID_WIDTH`/`SID_WIDTH`/`SSID_WIDTH` self-named pseudo-enums
    (enum NAME ends `_WIDTH`, prefix not a declared signal) stay an honest residual (out of scope — a
    name-level case overlapping `.5.i`).

- ID: `KG-ISF-COMPLETENESS.5.iv` · Status: `done` (**MEASUREMENT `2026-08-11`**, read-only, docs-only; the
  CODE slice is `.5.iv.a`, deliberately not started here) · Goal: **decide whether an encoding table's own
  column header may SOURCE an enum name, not merely veto one.** Surfaced by `CORPUS-COVERAGE.2.51`: the Arm
  SMMU *guide* carries `Table 3-1: Stream Security determination` with the header `SEC_SID value |
  Description`. `.5.i` correctly rejected the caption-derived `TABLE`, but the header — the most authoritative
  naming evidence the table has — is only ever read as a **veto** on a caption candidate, never as a
  **source**. `derive_encoding_enum_name` draws candidates from `caption_text` or the section title alone
  (`crates/specforge/src/ir/evidence.rs:4698-4703`), then validates them against `known_signals` and the
  header (`:4715-4733`). The asymmetry is incidental, not designed, and it sits directly on the roadmap's
  symbol surface ("all real agents, relations, constraints, behaviors, transactions, **symbols**, and
  storage") and its "prefer deterministic extraction … **enum rows**" doctrine.
  **Measured read-only over all 78 persisted SourceIRs** (tracked reproducer
  `scripts/measure_encoding_enum_header_naming.py`; no VLM/Docling/rebuild):

  | Population | Count |
  | --- | ---: |
  | `encoding` tables corpus-wide | 2,540 |
  | …matching a single-header-row `<FIELD> value \| Description` shape with exactly one non-generic field token | 281 |
  | …whose member names then survive the `.5.ii` sentence-spine gate (a non-empty enum would actually be minted) | 134 |
  | …documents involved | 10 |
  | distinct (document, candidate name) pairs | 82 |
  | pairs drawing the same name from more than one table in the same document | 28 |
  | **collision groups whose tables CONFLICT on any shared value→member mapping** | **0 of 28** |

  **The decisive result is the last row.** `.5.i`'s core objection to a name fallback was
  `build_symbol_definitions`' merge-by-name: many unrelated tables fusing into one junk enum. That failure
  mode **does not reproduce** for header-sourced names, and the reason is structural rather than lucky — a
  caption word like `Table` is shared by tables that have nothing to do with each other, whereas a header
  names the actual field, and one field encodes the same way throughout a document. The worked example is
  SMMU `SH`: eleven separate tables, every shared value identical
  (`0b00=NON_SHAREABLE`, `0b10=OUTER_SHAREABLE`, `0b11=INNER_SHAREABLE`, `0b01=RESERVED` where present), so
  the merge *is* the correct enum rather than a conflation of eleven.
  **Decision: GO on the lever, NO-GO on the naive predicate** — the measured 134 are a mix. Genuine field
  encodings (`AWATOP`, `ENDIAN`, `EXCL`, `RESPERR`, `ARCHID`, `DATASOURCE`, `ST_LEVEL`, `CD2L`, `VMID16`,
  `PRI`, and the 102-table SMMU architecture-spec body) sit beside four junk classes a code slice must
  exclude first: `OFFSET`-headed register-offset tables (three, CoreSight SDC-600) where the header names a
  column concept and not a field; the `*_WIDTH` self-named pseudo-enums whose only member is `LEGAL_VALUES`,
  already recorded as a `.5.iii` honest residual; garbled members (`AXADDR` → `VA_40`, `NUM_2_0_A`); and
  twelve `RESERVED`-only enums that carry no intent.
  **Honest correction to the observation that opened this leaf:** the SMMU *guide* table that motivated it
  would still mint nothing, because its members are description sentences that `.5.ii` correctly drops. The
  lever is real, but not for the document that surfaced it.
  **`.5.iv.a` (CODE) is deliberately deferred to its own focused slice.** It changes a shared extractor and
  would mint a new `AWATOP` enum on the AXI wire gold `ihi0022_l`, so it is byte-changing on a scored
  document and carries the full before/after WIRE-BASED-100 protocol on rebuilt gold evidence, a corpus-wide
  old-versus-new replay, and FSMGen `--strict` on every changed `.isf` — the same high-stakes gate-code rule
  `.5.i` was run under. No code changed in this leaf. Report
  `docs/research/generic-enum-conflation-results.md` §`.5.iv measurement`; KM `[[generic-enum-conflation]]`.

- ID: `KG-ISF-COMPLETENESS.5.iv.a` · Status: `done` (`2026-08-31`; CODE; the last leaf of `.5`) · Goal: **let an
  encoding table's own column header SOURCE an enum name, not merely veto one.** `.5.iv` named this slice as its
  frontier and measured it GO, but it was never given a node — so the lane's named next step had no owning leaf
  and could not legally be started (`docs/decisions/0003-task-tree-and-commit-doctrine.md`). Found and owned by
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.4e` while auditing the writer set of the report these leaves append to; the
  leaf records what `.5.iv` already decided rather than re-opening it.
  **Scope, from the `.5.iv` measurement.** Widen `derive_encoding_enum_name` (`crates/specforge/src/ir/evidence.rs`
  :4698-4703 draws candidates from `caption_text`/section title only; :4715-4733 validates against
  `known_signals` + the header) so a single-header-row `<FIELD> value | Description` table may take its name from
  the header. GO is on the lever, **NO-GO on the naive predicate**: of the 134 corpus tables that would mint a
  non-empty enum, four measured junk classes must be excluded first — `OFFSET`-headed register-offset tables
  (3, CoreSight SDC-600) where the header names a column concept and not a field; `*_WIDTH` self-named
  pseudo-enums whose only member is `LEGAL_VALUES` (the `.5.iii` honest residual, re-entering from the header
  side); garbled members (`AXADDR` → `VA_40`, `NUM_2_0_A`); and 12 `RESERVED`-only enums carrying no intent.
  The exclusion predicate must itself be measured FP-free before it lands, exactly as `.5.iii`'s `_WIDTH` gate
  was. Merge-by-name is safe here and this is measured, not assumed: 0 of 28 same-name collision groups conflict
  on any shared value.
  **Gate (high-stakes, same rule `.5.i` ran under).** It is byte-changing on a scored document — it mints a new
  `AWATOP` enum on the AXI wire gold `ihi0022_l` — so it carries the full before/after WIRE-BASED-100 protocol
  on rebuilt gold evidence, a corpus-wide old-versus-new replay, FSMGen `--strict --check --json` on every
  changed `.isf`, `kg-bench`, and `run_ci.sh`; it belongs in its own focused/fresh slice, not at the tail of
  another. ADR 0006 applies: the name must come from the document, never from a vocabulary list.
  Prerequisite: `KG-ISF-COMPLETENESS.5.iv` (met). Report
  `docs/research/generic-enum-conflation-results.md`; KM `[[generic-enum-conflation]]`.
  **LANDED `2026-08-31` — and it corrected three things this node recorded, each from measurement.**
  (1) **The population is twice what `.5.iv` counted.** `.5.iv` censused `table_kind == "encoding"` only, but
  the shipped scan skips just signal-description/register-map/timing-parameter and `table_looks_like_encoding`
  then admits any name+value header, so `unknown`-kind tables are in scope: 994 candidates reach the header
  path and 285 are accepted, not 134. (2) **The four exclusion classes are the wrong list.** The dominant junk
  is POSITIONAL headers (`Bytes`/`Offset`/`Index`/`bits` — the column holds a position, so the table is a
  field LAYOUT), which alone removes 448 candidates and subsumes the `OFFSET` and garbled-`AXADDR` classes;
  a second clause — at least one value cell must PARSE as an encoding literal — subsumes the `*_WIDTH` class
  *and* the glossary/notation/abbreviation class `.5.iv` never saw. **`RESERVED`-only did not ship as an
  exclusion**: SIX of the twelve have their `RESERVED` member eliminated by `build_symbol_definitions`'
  conflicting-value rule, the other SIX survive, and those six are structurally indistinguishable from 31
  legitimate single-distinct-member tables, so the only discriminator left is a spec-assigned value word
  ADR 0006 forbids. (Corrected `2026-08-31` — this node first published `five`/`seven`, hand-counted from a
  dump rather than derived; see the correction block below.) (3) **This node's gate prediction was false.**
  `ihi0022_l` already carries `AWATOP` with 13 members, and it cannot be rebuilt at all — its SourceIR is
  legacy schema 1, refused for canonical use, one of 54 legacy chains against 24 current ones. Exactly ONE
  accepted table sits in a rebuildable document (the SMMU guide's `Table 3-1`, the table that opened `.5.iv`)
  and it mints nothing, because `.5.ii` drops its sentence members — the mechanical confirmation of the honest
  correction `.5.iv` wrote in prose. The capability is therefore real but inert on the measurable stratum,
  and the replay proves that inertness instead of assuming it.
  Verification: `check_chain_currency.sh` 24 replayed / 24 current / 0 stale at every stage (evidence,
  semantic, intent, isf-adapter) — every persisted artifact byte-identical old-versus-new; `kg-bench` 156/156;
  `cargo test` green with +8 tests; `scripts/check_doctrines.sh` GREEN. Census reproducer
  `scripts/measure_header_sourced_enum_naming.py`.
  Commit: `KG-ISF-COMPLETENESS.5.iv.a — let an encoding table's own header source its enum name`

- ID: `KG-ISF-COMPLETENESS.5.v` · Status: `done` (`2026-09-21`; opened by `SIGNAL-CATALOG-CAPTURE-GAP.3`,
  MEASUREMENT first, no code — the measurement is the deliverable and it opened `.5.vi`/`.5.vii`) · Goal: **an enum whose NAME passes `.5.i` can still take its MEMBERS from
  the wrong column and its VALUES from the row index.** `.5.i` gated the name fallback to a candidate that
  is independently evidenced — a declared signal, or a **column-header reference token of this table** —
  and `.5.iv.a` let the header source a name outright. Both gates are about the NAME. Neither asks whether
  the members the reader then reads belong to that name.

  **The instance, found from another tree's end.** `SIGNAL-CATALOG-CAPTURE-GAP.3` gave Wishbone its first
  declared catalog, which unlocked `scan_encoding_tables_by_signal_anchor` (it returns early on
  `known_signals.is_empty()`). The first table it reached was *Table 3-1. Data Transfer Nomenclature*:

  ```text
  Nomenclature | Granularity | Description
  BYTE(N)      | 8-bit       | An 8-bit BYTE transfer at address 'N'.
  WORD(N)      | 16-bit      | A 16-bit WORD transfer at address 'N'.
  DWORD(N)     | 32-bit      | A 32-bit Double WORD transfer at address 'N'.
  QWORD(N)     | 64-bit      | A 64-bit Quadruple WORD transfer at address 'N'.
  ```

  It minted `Enum Nomenclature 8_BIT = 0.` … `64_BIT = 3.` — the enum named after the **first** column's
  header, its members read from the **second** column, and values `0..3` taken from the **row index**
  because the document states no code. All three are wrong about the same table: the document's
  nomenclature is `BYTE`/`WORD`/`DWORD`/`QWORD` and the granularity is a property of each, not an encoding
  of anything. `infer_encoding_column_indices` picks `name_col` and `value_col` by header keyword, and on a
  table whose merged title row is itself a header (`Table 3-1. Data Transfer Nomenclature` ×3, matching
  *transfer*) the two selections disagree with the table's real shape.

  **Bounded, and not promoted today**: `symbol_definitions` stays 0 on this document, so nothing reaches
  `.isf`. That is why this is a leaf rather than a stop-everything defect — but it is a real
  member/value-fidelity class that `.5.iii` explicitly left as an honest residual ("319 section-caption
  survivors, no FP-free gate") and it now has a concrete, reproducible instance.

  **First job is the population, not a rule.** Census corpus-wide, read-only: how many minted enums take
  their members from a column other than the one their name came from, and how many carry positional
  values against a document that states none. `.5.i`/`.5.ii`/`.5.iii` each earned their gate by measuring
  precision on the whole corpus first, and a positional-value gate that fires on a real code table would
  be worse than the residual. Do not write a rule before that number exists.
  Non-goal: re-opening the NAME gates, which are measured and correct; widening
  `scan_encoding_tables_by_signal_anchor`'s early return, which is what makes the convergence loop
  monotone.
  Prerequisite: none. Blocks: nothing.
  Verification: a read-only census reproducer under `scripts/`, plus the per-item adjudication `.5`'s
  children established as this tree's standard.

  **MEASURED `2026-09-21`** — `scripts/measure_enum_member_value_fidelity.py`, read-only over all 78
  persisted chains. **550 minted enums across 52 documents.** The two classes this leaf named are real
  and small; a third one it did not anticipate is neither.

  | class | enums | what it means |
  | --- | ---: | --- |
  | member-column drift | **21** | the name matched one column's header, the members came from another |
  | positional values | **160** | values are exactly `0..n-1` and appear in NO column, so they are `row_idx` |
  | both | 12 | the Wishbone instance's shape |
  | **cross-table conflation** | **231** | **no single table holds the members at all** |

  **The dominant class is the one nobody was looking for.** For **228 of those 231** (99%) no column of
  any table in the document contains every member, so the enum is a fusion of several tables under one
  name. They carry **19,447 of the corpus's 21,592 enum statements — 90% of all minted members** — and
  **3,159 of those members are longer than 60 characters**: whole description sentences promoted as enum
  member names. The longest is **2,344 characters**, a changelog paragraph. This is the class `.5.ii`'s
  `is_prose_fragment_member_name` gate exists to catch, still leaking at scale.

  **And this leaf's own severity assumption is refuted.** It recorded the defect as "bounded, and not
  promoted today: `symbol_definitions` stays 0 on this document". True of Wishbone, false of the corpus.
  **512 `symbol_definitions` reach `IntentIR` across 50 documents, and 226 of them — 44% — come from
  conflated enums**, carrying **9,999 of the 11,273 promoted members (89%)**, of which **2,782 are over
  60 characters**. Only **134 (26%)** of the promoted symbols come from an enum this census finds clean.
  The member/value-fidelity class is not bounded away from `.isf`; it is most of the symbol surface.

  | promoted symbols | count | members | members >60 chars |
  | --- | ---: | ---: | ---: |
  | from conflated enums | **226** | 9,999 | 2,782 |
  | from positional-value enums | 135 | 617 | 176 |
  | from member-column drift | 17 | 69 | 15 |
  | clean | 134 | 588 | 97 |

  **Each flag is a necessary condition, not a verdict**, and the census says so rather than implying a
  fix. A genuine code table may legitimately run `0..n-1`; a `name | description` table may legitimately
  source members from column 1; and a specification may legitimately split one code table across two
  pages, which would read as conflation. The >60-character member count is the independently damning
  signal, because no encoding names a member with a sentence.

  **Where a rule belongs, on this evidence.** Not at the column-selection seam `.5.v` first suspected —
  that is the 21-enum class. The 231-enum class is an **identity** failure: an enum whose members cannot
  come from one table is not one enum. `.5.vi` and `.5.vii` own the two rules separately, because they
  fire at different seams and must be measured apart.

  **Instrument correction, recorded because it changed a number.** The census first required the minted
  members to be a PREFIX of a column and left 240 enums unsourced. The minting loop SKIPS members its
  `.5.ii` and `.5.iii` gates reject, so the minted list is a subsequence; matching that way resolved 9
  and left 231. The remaining 231 are not a matcher limitation — 99% of them have no single column
  containing all their members under any matching rule.

<!-- kg-isf-completeness-task-source-region:enum-surface-early-leaves:end -->
