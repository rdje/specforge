# Generic-`TABLE` mega-enum conflation — measurement (`KG-ISF-COMPLETENESS.5`)

Read-only measurement, `2026-06-24`. No code changed, no rebuild, no test suite run.
Measured via the current `target/release/specforge` (built `2026-06-24`, post-`.2a.vi`)
and the persisted `generated/` corpus. Surfaced by the `CORPUS-COVERAGE.2` re-ingests of
JEDEC HBM2 (#28) and AMBA CHI C2C (#29), and explicitly deferred by `KG-ISF-COMPLETENESS.2a.iv`
(Lever F) as "a future extraction-precision lever".

## The defect (bar #6: every `.isf` element is a faithful protocol fact)

The `.isf` emitter lowers IntentIR enums to `(type NAME (bits N) (M VALUE)…)`. HBM2's
`hbm.isf` emits `(type TABLE (bits 6))` — a generic, junk-named enum that fuses ~7
unrelated value-tables — beside genuinely-named enums (`DM`, `COMMAND`, `EXTEST_RX`, …).
A reader of the `.isf` cannot tell `TABLE` is noise; it claims one coherent 6-bit enum
where the document has none.

## 1. Origin — EXTRACTION, not the emitter

The conflation is minted in `crates/specforge/src/ir/evidence.rs`; the emitter is a
faithful pass-through.

- **The generic name is created** in `derive_encoding_enum_name` (`evidence.rs:4457-4461`):
  when the known-signal caption/header/section match fails, the fallback picks the **first
  whitespace token of the caption that passes `is_hardware_signal_token`**. That predicate
  (`evidence.rs:7106`) accepts any token of length ≥ 2 whose first char is uppercase and
  whose chars are `[A-Z0-9_]` — so `Table` → `TABLE` passes (likewise `Column`/`Figure`/
  `Data`/`Annex`/`Note`…). In `source_ir`, 83 of 84 captioned HBM2 tables begin `"Table N - …"`,
  so every encoding-classified table without a signal match is named `TABLE`.
- **The members are synthesized** in `synthesize_encoding_declarations_for_enum`
  (`evidence.rs:11898/11952`): one statement per body row, `format!("Enum {enum_name} {member_name} = {value}.")`,
  the member NAME being the row's name-cell sanitized to `[A-Z0-9_]` — so a whole footnote
  sentence becomes one giant member name.
- **The merge** happens in `build_symbol_definitions` (`semantic.rs:2782-2789`): members are
  accumulated **by `enum_name` key**, so every fallback table sharing the key `"TABLE"`
  fuses into ONE `SymbolDefinitionRecord { symbol_name: "TABLE", kind: Enum }`
  (`semantic.rs:2865-2877`), copied verbatim into IntentIR (`intent.rs:189`) and lowered
  faithfully by `isf_ir.rs:889-912`.

**Verdict: extraction-born.** The fix belongs in `evidence.rs` (and `semantic.rs`'s merge),
not the emitter — though the emitter carries one related bug (§4).

## 2. Corpus measurement (78 persisted `intent_ir.json`)

Generic-name set (document-structure words — a *structural* class, not a chip-name list):
`TABLE, COLUMN, DATA, FIGURE, BIT(S), FIELD, REGISTER, PAGE, ANNEX, NOTE, CHAPTER, ENCODING, STATE, SECTION`.

| metric | value |
|---|---|
| docs with ≥1 generic-named enum | **56 / 78** |
| real (named) enums corpus-wide | 493 |
| generic-named enums corpus-wide | **96** (`TABLE` 54, `FIGURE` 14, `DATA` 7, …) |
| generic `(type …)` lines reaching `.isf` | ~95 across 55 actor `.isf` |

Worst offenders (members | dup member-VALUES | sentence-fragment member NAMES | ~distinct conflated tables):

| doc | enum | members | dup-vals | frag-names | ~src tables |
|---|---|---|---|---|---|
| cortex_a76 TRM | `TABLE` | 497 | 251 | 232 | ~96 |
| coresight_soc_600 (0200) | `TABLE` | 488 | 394 | 61 | ~184 |
| ihi0050_g CHI arch | `TABLE` | 417 | 375 | 230 | ~89 |
| gic_600 TRM | `TABLE` | 407 | 248 | 199 | ~83 |
| ihi0098_b CHI C2C | `TABLE` | 404 | 331 | 153 | ~89 |
| nvme_base 2.0a | `FIGURE` | 278 | 236 | 240 | ~43 |
| amd_iommu | `TABLE` | 220 | 186 | 201 | ~63 |
| **ihi0098_a CHI C2C (#29)** | `TABLE` | 183 | 144 | 32 | ~37 |
| **jesd235a HBM2 (#28)** | `TABLE` | 57 | 29 | 30 | ~7 |

**HBM2 `TABLE` — proof of conflation (7 value-restart runs = 7 fused source tables):**

| run | members | value range | example member |
|---|---|---|---|
| 0 | 12 | 0..1111 | `REPAIR_LANE_0` (lane-remap binary codes) |
| 1 | 5 | 0..4 | `HORIZONTAL_PITCH_OF_TWO_ADJACENT_MICROBUMPS` (geometry) |
| 2 | 20 | 0..19 | `BYPASS` (IEEE-1500 instruction opcodes) |
| 3 | 2 | 2..3 | `ONLY_APPLICABLE_IF_LOOPBACK_IS_ENABLED_…` (whole footnote) |
| 4 | 4 | 1..10 | `TABLE_21__MODE_REGISTER_11__MR11` (a table caption as a member) |
| 5 | 5 | 1..8 | `INTERNAL_VREF` |
| 6 | 9 | 0..8 | `IDD0` |

Values restart from 0 seven times → 29 duplicate values; 30 sentence-fragment names.
Unambiguously junk: ≥7 unrelated value-tables fused, no coherent encoding.

## 3. The deeper defect — member quality, not just the name

A **name-only** gate catches the 96 generic enums but **misses 271 real-named-but-junk
enums** whose members are themselves sentence fragments / restarting values:
HBM2 `COMMAND` (members like `THE_UNUSED_UPPER_COLUMN_ADDRESS_BIT_CA6_MUST_BE_DRIVEN…`),
HBM2 `DWORD_MISR` (`DWORD3__SAME_BIT_ORDERING_AS_DWORD0`), CHI `AMBA` (49 members like
`B2_2_4_DATA_FIELDS…`). Of the 493 real-named enums, **271 are fragment-heavy/dup-heavy;
only 222 are clean**. The generic name is the most visible symptom of a **member-quality**
defect (prose member names + restarting/duplicate values from conflated tables).

## 4. Decision — GO, extraction-side, decomposed

A clean, universal, ADR-0006-safe structural gate exists (this is **not** an honest residual,
and **not** an emitter-origin defect). Decompose:

- **`.5.i` (safe first code step):** in `derive_encoding_enum_name`
  (`evidence.rs:4457-4461`), the fallback must **not** return a name that is merely a
  document-structure token — gate it positively (the candidate token must independently be a
  declared signal / column header) and return `None` otherwise (a `None` fallback is the
  *existing* contract — both call sites `continue` on `None` → no enum minted → honest
  residual). This kills all 96 generic enums **and the conflation** (the merge is keyed on
  the shared name). Plus the **emitter orphan-`(type)` fix:** `isf_ir.rs:403-409` emits ALL
  `self.types` unconditionally, so even when Lever F drops an enum's members, its orphan
  `(type TABLE (bits 6))` still emits — gate the types block by `emitted_enums()`.
- **`.5.ii` (harder, needs threshold calibration):** a per-table **member-quality** gate at
  synthesis time (`synthesize_encoding_declarations_for_enum`) refusing to emit when the
  name column holds sentence fragments (identifier-shape / word-count / length criterion) or
  the value sequence restarts — catching the 271 real-named junk enums a name gate misses,
  without dropping the 222 clean enums. Requires its own precision/recall measurement of the
  threshold before landing.

### WIRE-BASED-100 safety

- **Scored surfaces — ORTHOGONAL / SAFE.** WIRE-BASED-100 scores `signal_constraint` /
  `relation` / `temporal_rule` facts (`eval-extraction` vs the `seed_{apb,ahb,axi}.json`
  golds). Generic enums are in no scored gold → dropping them **cannot move the scores**.
- **`.isf` BYTES — change (a strict improvement), NOT byte-identical.** All 4 wire golds DO
  emit a generic `TABLE` enum (APB `(type TABLE (bits 3))`; AHB `(type TABLE (bits 5))`
  fusing HTRANS+HSIZE values — both already correctly named `HTRANS`/`HSIZE` in the same
  actor; AXI 33-member; SWP `TABLE`+`ANNEX`). A fix **removes** this junk, leaving the
  correctly-named enums intact. No tracked test asserts on those bytes (grep of `crates/`
  found none), and the `*_passes_fsmgen_strict_validation` canaries stay green (removing junk
  keeps strict-clean). But because it changes byte-output, the code lever (`.5.i`) must land
  as a deliberate, snapshot-refreshing change with per-doc before/after evidence — **not**
  under the usual byte-identical-orthogonality argument.

## 5. Reproduce

```bash
# inspect the junk enum (RAM-safe; no VLM/Docling/rebuild)
target/release/specforge adapt generated/intent_ir/jesd235a_2015_11_hbm2_dram/intent_ir.json --target isf
grep -nE '\(type TABLE' generated/adapters/isf/jesd235a_2015_11_hbm2_dram/hbm.isf
# corpus generic-enum census: scan generated/intent_ir/*/intent_ir.json for SymbolDefinition
# enums whose symbol_name is a document-structure token (TABLE/FIGURE/DATA/…) and count
# members / duplicate values / fragment names per doc.
```

## Conclusion

The generic-`TABLE` mega-enum conflation is a real, corpus-wide (56/78 docs) bar-#6 ISF
fidelity defect, **extraction-born** in `evidence.rs`'s enum-name fallback + `semantic.rs`'s
merge-by-name. A clean universal structural gate exists (no chip-name list — ADR 0006). The
code lever is owned as `.5.i` (safe name-gate + emitter orphan-type fix) and `.5.ii`
(member-quality gate, calibration-gated). Because the fix changes wire-gold `.isf` bytes
(a strict improvement; scores orthogonal), it is recorded here as a deliberate
snapshot-refreshing change and **deferred to a focused/fresh slice** for signoff quality —
not rushed at the tail of a long measurement session.

---

## `.5.i` LANDED (`2026-06-24`, fresh focused session) — results

**The gate (two edits).** (1) `derive_encoding_enum_name`'s fallback (`evidence.rs`) now keeps
the first `is_hardware_signal_token` caption token **only when it is independently evidenced** —
a declared signal (`known_signals`) **or** a column-header reference token of the table — else
returns `None` (no enum minted; the existing `continue` contract → honest residual). (2) The
emitter (`isf_ir.rs`) gates the `(types …)` block by `emitted_enums()`, so a `(type)` is emitted
only when its enum is, removing the orphan `(type TABLE …)` a member-dropped enum left behind.

**Two corrections to the measurement's expectations (both make the result cleaner):**
- The genuinely-named enums (HBM2 `EXTEST_RX`/`DWORD_MISR`/`UPDATEWR`; wire-gold `PPROT`/`HTRANS`/
  `HSIZE`/`HPROT`/`HRESP`/`WLAST`/`RLAST`/`BRESP`/`RRESP`/`TKEEP`) come from the **signal-match loop
  ABOVE the fallback**, so the gate never touches them — they are **byte-identical**. The fallback
  only ever produced document-structure/caption words, so the structural gate is *strictly better*
  than a name-only gate: it ALSO drops the fallback-origin "real-named-but-junk" enums
  (`COMMAND`/`AMBA`/`READ`/`CACHE`/`RELEASE`/`BYTE`), which the `.5` measurement had assigned to
  `.5.ii`. The `.5.ii` residual is now precisely the **signal-match-origin** junk-member enums.
- The change is **NOT** confined to the enum surface. The dropped `Enum X = V` statements also
  leave `discovered_values`, which feeds value-constraint extraction — so a handful of off-gold junk
  constraints whose value WAS a junk-enum member also disappear (AXI: `ACTIVATEACK A` → the grounded
  reset value `ACTIVATEACK 1`; spurious `ARDOMAIN SHAREABLE`/`ARMMU* 0B0` removed). This is a
  **beneficial, score-orthogonal** side effect — the AXI *distinct* `signal_constraints` fact set is
  identical (0 added / 0 removed; the count drop 64→51 is duplicate records), and WIRE-BASED-100 is
  unchanged (proven below). The `.5` claim "orthogonal because enums are unscored" is *corrected*: it
  is orthogonal because the affected constraints are off-gold junk (the `.6/.7` filter already drops
  them) — proven empirically by the before/after eval, not assumed.

**Per-doc before→after (rebuilt with baseline vs gated binary; fsmgen `--strict --check --json`):**

| doc | `.isf` (type) before → after | fsmgen before/after | non-enum body |
|---|---|---|---|
| HBM2 `hbm.isf` | 15 → 6 (drop `TABLE`/`COLUMN`/`COMMAND` + 6 orphan types; keep `UPDATEWR`/`EXTEST_RX`/`DATA`/`HBM_RESET`/`DWORD_MISR`/`CHANNEL_ID`) | 0 / 0 | identical |
| APB `ihi0024_e` | `PPROT`+`TABLE` → `PPROT` | 0 / 0 | identical |
| AHB `ihi0033_c` | drop `TABLE`/`AMBA`; keep `HTRANS`/`HSIZE`/`HPROT`/`HRESP` | 0 / 0 | identical |
| AXI `ihi0022_l` | drop `TABLE`/`BYTE`/`READ`/`CACHE`/`RELEASE`/`AMBA`; keep 22 real signal enums | 0 / 0 | junk-constraint cleanup (distinct facts identical) |
| SWD `ihi0074_a` | drop `TABLE`/`REGISTER`/`ACCESSING`/`OF`/`USAGE`/`DRIVES`/`ARM`/`ATTRIBUTES`/`OK`; keep `TDI` | 0 / 0 | identical |
| AXI-Stream `ihi0051_b` | `TKEEP` (unchanged) | 0 / 0 | identical |
| SWP `etsi…` | drop `TABLE`/`CLT`/`ANNEX` → none | 0 / 0 | identical |
| amd_iommu | 34 → 5 types | 0 / 0 | identical |
| gic_600 | 23 → 6 | 0 / 0 | identical |
| coresight_soc_600 | 19 → 5 | 0 / 0 | identical |
| cortex_a76 | 16 → 7 | 0 / 0 | identical |
| CHI `ihi0050_g` | 14 → 4 | 0 / 0 | identical |
| CHI-C2C `ihi0098_b` | 7 → 1 | 0 / 0 | identical |
| nvme | 2 → 0 | 0 / 0 | junk-constraint cleanup |

**Corpus-wide census (33 rebuildable docs, persisted-before vs gated-after):** generic-named enums
**82 → 8**; total enum records **422 → 105** (317 junk records removed — generic + fallback-origin
real-named-but-junk + 0-member orphan types). The **8 survivors** (`DATA` in gic_600/coresight-TMC,
`TABLE` in CCIX×2/VT-d, `CACHE`+`ENCODING` in AXI+ACE, `ATTRIBUTES` in SMMU) survive precisely
because the token IS a declared signal or a column header in *that* document — e.g. CCIX has 48
header cells reading "Table of Contents", so `TABLE` is structurally indistinguishable from a signal
name without a forbidden name-list. They keep junk members and are the explicit `.5.ii`
(member-quality) target; this is the **structural gate behaving correctly** (ADR 0006: `DATA` is
KEPT where it is a real gic_600 signal and DROPPED where it is a bare HBM2 caption word).

**No regression (oracles):** WIRE-BASED-100 = **1.000, before == after** (proven by rebuilding each
gold doc's evidence with both binaries and running `eval-extraction --provider skip`): APB
signal_constraint 6/6 · relation 5/5; AHB 6/6 · 6/6; AXI 3/3 · 6/6; APB/AHB/AXI temporal 3/3·4/4·3/3;
SWD relation 1/1, lone constraint the documented promotion-only 0/1 — all identical old-vs-new.
nvme-registers and i2c-signals golds identical old-vs-new. `kg-bench` **156/156**. `run_ci.sh`
**GREEN** (lib **1712** passed, +new tests; warning-deny clippy/fmt/rustdoc + mdBook). Every affected
`.isf` stays FSMGen-`--strict` **0 diagnostics** (the renderable corpus stays strict-clean). Unit
tests: `encoding_enum_name_fallback_drops_document_structure_keyword`,
`encoding_enum_name_fallback_keeps_column_header_candidate`,
`encoding_enum_name_keeps_declared_signal_but_drops_unevidenced_caption_word`, and the strengthened
`binary_looking_enum_value_is_excluded_and_recorded_as_residual` (asserts the orphan `(type table)`
is now dropped). **Frontier → `.5.ii`** (member-quality gate for the 8 document-evidenced survivors +
the signal-match-origin junk-member enums; calibration-gated).
