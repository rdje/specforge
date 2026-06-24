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

---

## `.5.ii` measurement (`2026-06-24`, read-only) — the member-quality gate is PER-MEMBER, not per-enum

Read-only measurement over all 78 persisted `intent_ir.json` (561 enums with ≥1 member, 12 509
members). Reproducer: `member_final.py` over `generated/intent_ir/*/intent_ir.json` — count, per enum
member, the `_`-separated tokens of the synthesis-sanitized `member_name` and test for an
English **sentence-spine** token. No code, no rebuild.

### Finding 1 — the recorded `.5.ii` plan (whole-enum drop on name-fragments OR value-restart) is DISPROVEN

The `.5` decision packet proposed *"refuse to **emit an enum** whose name column holds sentence
fragments OR whose value sequence restarts."* Inspecting the actual members of the post-`.5.i`
survivors overturns both halves of that whole-enum rule:

- **A whole-enum drop destroys real codes — the surviving junk enums are CONFLATIONS of a junk table
  and a clean table.** AXI-gold `BRESP` (`ihi0022_l`) has **16 members**: 7 sentence-fragment prose
  members (`NON_EXCLUSIVE_WRITE__THE_TRANSACTION_WAS_SUCCESSFUL…`) + a leaked `BRESP_WIDTH`, **fused
  with** the 8 genuine codes `OKAY/EXOKAY/SLVERR/DECERR/DEFER/TRANSFAULT/RESERVED/UNSUPPORTED`. A
  whole-enum drop would throw away the real codes. The correct granularity is **per-member**: drop the
  prose members, keep the codes.
- **Value-restart is NOT a junk signal.** AHB-gold `HPROT` (`ihi0033_c`) has `restarts=2` (3 fused
  HPROT sub-encodings: protection bits + memory-type + cacheability) yet **every one of its 15 members
  is a clean identifier** (`DATA_INST`/`PRIVILEGED`/`BUFFERABLE`/`WRITE_BACK__SHAREABLE`/…). Dropping it
  on the restart signature is a **false positive** that loses real protocol intent. Restart correlates
  with conflation but conflation of *clean* tables is still all-real-members — so restart cannot gate a
  drop. (Splitting a clean conflation into sub-enums is a deeper refinement, deferred — see residuals.)

### Finding 2 — the load-bearing signal is per-member NAME shape (sentence-spine fragment)

The synthesis (`synthesize_encoding_declarations_for_enum`) sanitizes each name-cell to `[A-Z0-9_]`
uppercased, so a whole prose sentence in a name-cell becomes one giant `_`-joined member name
(`THE_TRANSACTION_MUST_BE_LOOKED_UP_IN_A_CACHE_BECAUSE…`). A **real** hardware enum symbol is an
identifier and never contains the **spine of an English sentence** — a copula/auxiliary/modal
(`IS`/`ARE`/`WAS`/`BE`/`HAS`/`MUST`/`SHALL`/…), article/demonstrative (`THE`/`AN`/`THIS`/`THAT`/…), or
relativizer/subordinator (`WHICH`/`WHEN`/`IF`/`BECAUSE`/`WHILE`/…). That gives a precise, universal,
ADR-0006-safe per-member predicate: **a member whose token set contains a sentence-spine word is a
prose fragment → drop it.**

**Collision exclusions (the `.1a` discipline).** Five spine-shaped tokens collide with real hardware
identifiers and are EXCLUDED from the lexicon — the same structural-collision reasoning `.1a` used for
the agent gate: `A` (article vs. a single-letter port/version suffix — `MASKLANE_A`, `DAT0CREDIT_A`),
`I` (pronoun vs. the letter), `ITS` (pronoun vs. the GIC **ITS** component — `ITS_COMMAND_QUEUE`), `CAN`
(modal vs. the **CAN** bus), `MAY` (modal vs. the month). They cost essentially zero recall here
(`GROUP_CAN_SEND_MSI`, `MAY_3__2019`-a-date) and remove the only genericity risk on other corpora.

### Finding 3 — precision/recall (per-item, `[[feedback_scoring_rigor]]`)

| anchor | members | result |
|---|---|---|
| **CLEAN anchor** (wire-gold `HTRANS`/`HSIZE`/`HRESP`/`HPROT`/`TKEEP` members + the canonical short codes `OKAY/EXOKAY/SLVERR/DECERR/IDLE/BUSY/NONSEQ/SEQ/BYTE/HALFWORD/WORD/DOUBLEWORD`) | 115 | **0 flagged → precision 1.000 (zero false positives)** |
| **JUNK anchor** (hand-picked genuine prose fragments: `…TRANSACTION_WAS_SUCCESS…`, `IS_NOT_SUPPORTED`, `MUST_BE_SET`, `WHENASSERTED…`, `THE_REQUEST_HAS…`, `WRITE_WAS_UNSUCCESSFUL…`) | 269 | **269 caught → recall 1.000 (0 missed)** |

Corpus-wide, the predicate drops **3 781 / 12 509 (30.2 %)** members — the dominant junk class.
(The 8 "clean-anchor FP" my first pass reported were a mislabeled anchor: they were `ARCACHE`/`AWCACHE`
in the *dense-prose* AXI+ACE manual `ihi0022_h_c` whose flagged members are genuine giant sentence
fragments — true positives, not false positives.)

### Finding 4 — per-enum effect on the wire-gold + AXI+ACE class (the high-stakes blast radius)

| doc | enum | members before → after | outcome |
|---|---|---|---|
| AXI `ihi0022_l` | `BRESP` | 16 → 9 | **recovers** `OKAY/EXOKAY/SLVERR/DECERR/DEFER/…` from the conflation |
| AXI `ihi0022_l` | `RRESP` | 8 → 1 | drops all 7 prose; leaves the `_WIDTH` leak (residual) |
| APB `ihi0024_e` | `PPROT` | 3 → 0 | **dropped** (all 3 are prose bit-descriptions, not enum-shaped) |
| AXI+ACE `ihi0022_h_c` | `ARCACHE`/`AWCACHE`/`AWTAGOP`/`READ`/`RLAST`/`WLAST` | 4/4/4/7/2/2 → 0 | **dropped** (pure prose) |
| AHB `ihi0033_c` | `HSIZE`/`HTRANS`/`HRESP` | unchanged | untouched (no prose member) |

This changes wire-gold `.isf` bytes (a strict improvement, like `.5.i`) → the code slice (`.5.ii`
proper) must land as a deliberate snapshot-refreshing change with a before/after WIRE-BASED-100 eval,
NOT under a byte-identical argument. Scores stay orthogonal (enums unscored; the dropped members'
`discovered_values` only feed off-gold junk value-constraints, the `.5.i`-proven coupling).

### Honest residuals (deeper member-quality classes, NOT gated by the spine filter — deferred)

The spine filter is the clean, FP-free FIRST member-quality gate; it deliberately leaves harder
classes as honest residuals (completeness/precision both protected — no overfitting):
- **Glossary cross-references** (`SEE_CACHE_MISS`, `SEE_ALSO_COHERENCY_GRANULE`) — a `SEE`/`SEE_ALSO`
  lead, not a sentence spine.
- **Front-matter / ToC members** (`NON_CONFIDENTIAL_PROPRIETARY_NOTICE`, `RELEASE_INFORMATION`).
- **Section-caption members** (`B2_3_1_READ_TRANSACTIONS`) — a leading section-number token shape.
- **`_WIDTH` parameter leaks** (`BRESP_WIDTH`, `RRESP_WIDTH`, `AWCMO_WIDTH`).
- **Value-restart conflations with all-clean members** (`HPROT`) — kept intact (every member real);
  splitting into sub-enums is a separate, riskier refinement.

### Decision — GO, per-member sentence-spine fragment drop at synthesis

A clean, universal, ADR-0006-safe, false-positive-free per-member gate exists. Land it in
`synthesize_encoding_declarations_for_enum` (`evidence.rs`): skip a member whose sanitized name
contains a sentence-spine token (lexicon above, collisions excluded); if every member is dropped the
enum empties and is not minted (the existing no-statements → no-`SymbolDefinition` contract → honest
residual). The harder residual classes above stay honest residuals for a later refinement. WIRE-BASED-100
+ `kg-bench` + FSMGen `--strict` + `run_ci.sh` are hard gates on the code slice; before/after eval proof
required (byte-changing).

### Reproduce

```bash
# member-quality census over the persisted corpus (RAM-safe; no VLM/Docling/rebuild):
# for each generated/intent_ir/*/intent_ir.json SymbolDefinition enum, split each member_name on '_'
# and flag the member if any token is an English sentence-spine word (copula/aux/modal/article/
# demonstrative/relativizer/subordinator), EXCLUDING the collisions A/I/ITS/CAN/MAY/AM.
# -> 3781/12509 members flagged; clean-anchor 0/115 flagged; junk-anchor 269/269 flagged.
```

---

## `.5.ii` LANDED (`2026-06-24`, same fresh focused session) — results

**The gate.** `is_prose_fragment_member_name(member_name)` returns true when any `_`-separated token
(exact, lowercased) is in `PROSE_SENTENCE_SPINE_WORDS` (the 38-word copula/auxiliary/modal/article/
demonstrative/relativizer/subordinator lexicon, collisions `a`/`i`/`its`/`can`/`may`/`am` excluded). The
member loop in `synthesize_encoding_declarations_for_enum` (`ir/evidence.rs`) `continue`-skips a fragment
member, so a conflated enum keeps its genuine codes and a pure-prose table emits no statements → no
`SymbolDefinition` → honest residual. One seam covers both call paths (`None` / `Some(known_signals)`).

**Landed wire-gold census** (gold evidence rebuilt with the preserved baseline = post-`.5.i` vs the gated =
post-`.5.ii` binary; counting the `Enum X = V` statements):

| doc | enum-member statements before → after | highlight |
|---|---|---|
| AXI `ihi0022_l` | 148 → 91 | `BRESP` 16→9 recovers `OKAY/EXOKAY/SLVERR/DECERR/DEFER/TRANSFAULT/RESERVED/UNSUPPORTED`; `ARTAGOP`/`AWTAGOP`/`RLAST`/`WLAST` → 0 (pure prose) |
| APB `ihi0024_e` | 7 → 4 | prose `PPROT[n] is used…` rows dropped |
| AHB `ihi0033_c` / SWD / nvme / i2c | unchanged | no surviving enum carries a prose member |

The emitted AXI `manager.isf` now declares `(BRESP (BRESP_WIDTH 0) (OKAY 0) (EXOKAY 1) (SLVERR 2)
(DECERR 3) (DEFER 4) (TRANSFAULT 5) (RESERVED 6) (UNSUPPORTED 7))` — a faithful response enum where a junk
`(type TABLE …)` (pre-`.5.i`) and a 16-member prose-fused `BRESP` (pre-`.5.ii`) used to be. CCIX `TABLE`
stays 27 clean-identifier members (the cross-table-merge-of-clean residual — correctly untouched: every
member is a real symbol; dropping them would lose intent).

**No regression (oracles).** WIRE-BASED-100 = **1.000, before == after** (PROVEN by rebuilding each gold
doc's evidence with both binaries and running `eval-extraction --provider skip` over all 10 seeds — the
scored surface is byte-identical old-vs-new, only the `evidence_root`/temp-path lines differ): APB
constraint 6/6·relation 5/5; AHB 6/6·6/6; AXI 3/3·6/6; temporal APB/AHB/AXI 3/3·4/4·3/3; SWD relation 1/1
+ documented promotion-only 0/1; SWD-derivation serial-frame 11/11·swd-op 4/4·protocol-state 13/13; i2c
declared_signal 6/6. Rebuilt AXI + APB `.isf` pass FSMGen `--strict --check --json` (`success: true`).
`kg-bench` **156/156**. `run_ci.sh` GREEN (lib **1716**, +4 tests; warning-deny clippy/fmt/rustdoc + mdBook).
Unit tests: `prose_sentence_spine_words_excludes_identifier_collisions`, `prose_fragment_member_predicate`,
`encoding_member_synthesis_drops_prose_fragments_keeps_codes`,
`encoding_member_synthesis_all_prose_yields_no_statements`. **`.5` enum-surface fidelity is now built**
(`.5.i` name-gate + `.5.ii` member-gate); the deeper member-quality residual classes stay honest residuals.

---

## `.5.iii` measurement (`2026-06-24`, read-only) — the `_WIDTH` parameter-leak class is the one buildable deeper residual

`.5.ii` left five deeper member-quality classes as honest residuals (glossary `SEE…`,
front-matter/ToC, section-caption `B2_3_1_…`, `_WIDTH` parameter leaks, value-restart-of-all-clean).
This slice MEASURES them per-item over the persisted corpus to decide which (if any) is worth a code
gate. Reproducer `scripts/measure_enum_width_leak.py` (read-only, deterministic, RAM-safe — no
VLM/Docling/rebuild). The persisted corpus is a MIX (AXI/APB/CCIX were re-emitted post-`.5.ii` during
that slice's verification; the rest is pre-`.5.i`), so the spine-flag tally reads 3375/11721 here vs the
`.5.ii` 3781/12509 — immaterial to the residual-class question, which scans whatever members survive.

### Finding 1 — most deeper-residual members live in `.5.i`-DROPPED generic enums (no `.isf` reach)

Of 54 corpus members ending in `_WIDTH`, **47 sit in generic-named enums** (`TABLE`/`TRANSLATION`/
`DESCRIPTION`/`BIT`: `TABLE:TBUCFG_SID_WIDTH`, `TRANSLATION:LTI_ID_WIDTH`, `BIT:WIDTH`) — exactly the
enums `.5.i`'s name-gate already drops whole, so they never reach any `.isf`. Likewise the section-caption
class: the 319 leading-`[A-Z]?digit` survivors are dominated by generic enums (`DEBUG:D1_1`,
`DEBUG:D2_3`), and `DEBUG` is a caption word `.5.i` kills. **So the deeper classes are largely SUBSUMED
by `.5.i`'s whole-enum drop** — there is nothing left to fix for them in a post-`.5.i` world.

### Finding 2 — the `_WIDTH` leak DOES reach a wire-gold `.isf`, and it is materially damaging

The remaining **7 `_WIDTH` members live in real-signal-named enums** that survive `.5.i` and emit — all
in the AXI **gold** `ihi0022_l` (`BRESP_WIDTH`, `RRESP_WIDTH`, `RCHUNKNUM_WIDTH`, `RCHUNKSTRB_WIDTH`,
`AWSNOOP_WIDTH`/`ARSNOOP_WIDTH` in `AXSNOOP`, `AWCMO_WIDTH`). Inspecting the emitted `manager.isf` shows
real damage (a width PARAMETER mis-read as an encoding VALUE, literal `0`):
- `(BRESP (BRESP_WIDTH 0) (OKAY 0) (EXOKAY 1) …)` — `BRESP_WIDTH 0` is a junk member that **duplicates
  the value `0`** already used by the real code `OKAY`.
- `(RRESP (RRESP_WIDTH 0))` — the real RRESP codes are **entirely replaced** by the lone width junk; the
  `.isf` falsely states RRESP's only encoding value is "RRESP_WIDTH = 0".
- `(AXSNOOP (AWSNOOP_WIDTH 0) (ARSNOOP_WIDTH 1))` — a whole enum of pure `_WIDTH` junk.
- `(AWCMO (AWCMO_WIDTH 0) (CLEAN_AND_INVALIDATE 0) (CLEAN_ONLY 1))` — `AWCMO_WIDTH 0` duplicates
  `CLEAN_AND_INVALIDATE 0`.

This is a genuine north-star bar-#6 fidelity defect (the `.isf` carries a false protocol fact). It did not
trip WIRE-BASED-100 because the scored surface is constraints/relations/temporal — the enum surface is
emitter-only/orthogonal (the same reason `.5.i`/`.5.ii` byte-changed the golds without moving the score).

**Root cause** (`generated/evidence_ir/ihi0022_l*`): the member's synthesized statement is literally
`"Enum BRESP BRESP_WIDTH = 0."` — a configuration/parameter row (`BRESP_WIDTH = 0`, the bit-width of the
BRESP signal) leaked into the BRESP value enum via `synthesize_encoding_declarations_for_enum`, the same
seam `.5.ii` gates.

### Finding 3 — the discriminator is FP-free and document-grounded (ADR 0006)

The clean rule: **drop a synthesized encoding member named `<X>_WIDTH` iff `X` is a declared signal in
the document OR the enum's own name** (the width of signal `X` is a parameter, never an encoding value).
Per-item over the corpus:
- All 6 distinct caught prefixes (`BRESP`/`RRESP`/`RCHUNKNUM`/`RCHUNKSTRB`/`AWSNOOP`/`ARSNOOP`/`AWCMO`)
  are **TRUE declared signals** in the AXI doc — the gate is grounded in the document's own evidence,
  exactly like `.5.i`'s "independently evidenced" rule, NOT a name list.
- **False-positive set is EMPTY**: no legit width-VALUE such as `FULL_WIDTH`/`HALF_WIDTH`/`QUARTER_WIDTH`
  exists anywhere in the corpus, and — decisively for future docs — the declared-signal discriminator
  would never catch one, because `FULL`/`HALF`/`QUARTER` are size words, not declared signals. Precision
  1.000 by construction (genericity guardrail satisfied: a real link-width enum is preserved).
- Per-member, not per-enum (parallel to `.5.ii`): dropping `BRESP_WIDTH` keeps BRESP's 8 codes; dropping
  the lone `RRESP_WIDTH`/`AXSNOOP` members empties those enums → not minted (honest residual, strictly
  better than emitting a false single-value enum).

### Finding 4 — the other deeper classes are NO-GO / honest residual

- **Section-caption / table-reference** (`D6_4`, `B2_3_1_…`): no false-positive-free structural gate.
  The leading `[A-Z]?digit` token collides with real codes (`D1`/`D2`/`D3` debug states, `L1`/`L2` cache
  levels) — the census matched `DEBUG:D1_1` etc., which are legitimate-shaped. And the genuine table-refs
  (`CACHE:D6_4` in the dense AXI+ACE `ihi0022_h_c`) sit in enums of dubious standing. No clean win.
- **Value-restart-of-all-clean** (`HPROT`): `.5.ii` already established restart is not a junk signal (all
  members clean); the 101 restart enums are dominated by `.5.i`-dropped generics. Splitting a clean merged
  enum into sub-enums is a refinement with no fidelity defect to fix — deferred, riskier.
- **Glossary `SEE_…`** (≤6) and **front-matter** (`NON_CONFIDENTIAL_…`): tiny prevalence, name-ish (a
  single lead/section word), mostly in dropped enums — low value, no clean structural gate.

### Decision — GO on the `_WIDTH` parameter-leak member gate; NO-GO on the rest

Land a per-member `_WIDTH` parameter-leak drop in `synthesize_encoding_declarations_for_enum`
(`evidence.rs`), parallel to the `.5.ii` spine gate: skip a member named `<X>_WIDTH` when `X` is the enum
name or a declared signal. Implementation threads the existing `known_signals` set (already in scope at
the signal-match caller, `evidence.rs:4700` `Some(known_signals)`) into the synthesis; the enum-self-name
check needs no plumbing and covers the `None` caller. Byte-changing on the AXI wire gold → before/after
WIRE-BASED-100 eval REQUIRED on the code slice (a strict improvement: the scored surface stays
byte-identical because enums are unscored, but `manager.isf` recovers clean BRESP/AWCMO and drops the
false RRESP/AXSNOOP enums). Universal structural rule, ADR-0006 (no chip-name list). The section-caption /
restart-of-clean / glossary / front-matter classes stay honest residuals.

### Reproduce

```bash
# read-only, deterministic, RAM-safe (no VLM/Docling/rebuild):
python3 scripts/measure_enum_width_leak.py
# -> 54 _WIDTH members; 7 CAUGHT (all AXI-gold, all declared-signal-grounded);
#    47 UNCAUGHT (generic .5.i-dropped enums); 0 legit-width-value FALSE POSITIVES; VERDICT GO.
# inspect the damage in the emitted .isf (RAM-safe):
grep -nE '_WIDTH|BRESP|RRESP|AWCMO|AXSNOOP' \
  generated/adapters/isf/ihi0022_l_2025_08_amba_axi_protocol_specification/manager.isf
```

---

## `.5.iii` LANDED (`2026-06-24`, same fresh focused session) — results

**The gate.** `is_width_parameter_leak_member(member_name, enum_name, known_signals)` (`ir/evidence.rs`)
returns true when a synthesized member is named `<X>_WIDTH` and `X` is the enum's own name OR a declared
signal (`known_signals`, matched case-insensitively). The member loop in
`synthesize_encoding_declarations_for_enum` `continue`-skips such a member right after the `.5.ii`
sentence-spine gate (one seam). `known_signals` is now threaded from the signal-match caller
(`evidence.rs:4708` `Some(known_signals)`); the `synthesize_encoding_declarations` `None` caller covers
the enum-self-name case. A pure-parameter enum empties → no statements → `build_symbol_definitions` mints
no `SymbolDefinition` (honest residual), exactly the `.5.ii` contract.

**Effect (live, new binary).** Rebuilding the AXI gold `ihi0022_l` evidence drops EXACTLY the 7 measured
leaks (`Enum BRESP BRESP_WIDTH = 0.`, `RRESP_WIDTH`, `RCHUNKNUM_WIDTH`, `RCHUNKSTRB_WIDTH`, `AWSNOOP_WIDTH`,
`ARSNOOP_WIDTH`, `AWCMO_WIDTH`): `extracted_statements` 6414→6407, and the **non-Enum statement TEXT set is
byte-identical** old-vs-new. The rebuilt `manager.isf` now emits
`(BRESP (OKAY 0) (EXOKAY 1) (SLVERR 2) (DECERR 3) (DEFER 4) (TRANSFAULT 5) (RESERVED 6) (UNSUPPORTED 7))`
(8 clean codes, no value-`0` dup) and `(AWCMO (CLEAN_AND_INVALIDATE 0) (CLEAN_ONLY 1))`; the false
`(RRESP (RRESP_WIDTH 0))`, `(RCHUNKNUM …)`, `(RCHUNKSTRB …)`, and `(AXSNOOP …)` `_WIDTH`-only enums are
**gone** (emptied → honest residual). FSMGen `--strict --check --json` → **success / 0 diagnostics**. The
`SECSID_WIDTH`/`SID_WIDTH`/`SSID_WIDTH` self-named pseudo-enums (the enum NAME ends `_WIDTH`, prefix not a
declared signal) stay an honest residual — a name-level case overlapping `.5.i`, out of this gate's scope.

**No regression.** WIRE-BASED-100 = **1.000 before==after** (PROVEN): AXI before/after
`eval-extraction --provider skip` on baseline-vs-gated evidence is identical (seed_axi 4/4 constraint +
6/6 relation; seed_axi_temporal 3/3); APB/AHB/SWD/i2c evidence rebuilt with both binaries is
**byte-identical** (the gate is inert — the leaks are AXI-only). Full wire eval on the canonical new
evidence holds the documented state (APB/AHB/AXI all 1.000; SWD relation 1/1 + the documented
promotion-only constraint 0/1; SWD-derivation 11/11·4/4·13/13; i2c 6/6). `kg-bench` 156/156. `run_ci.sh`
GREEN (lib 1718, +2 tests: `width_parameter_leak_member_predicate` pins `FULL_WIDTH` KEPT;
`encoding_member_synthesis_drops_width_parameter_leak_keeps_codes`).
