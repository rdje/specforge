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
