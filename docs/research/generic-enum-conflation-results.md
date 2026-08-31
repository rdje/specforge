# Generic-enum conflation — per-leaf gate results

Status: **retained verbatim from the conflation measurement; `.5.i`/`.5.ii`/`.5.iii`/`.5.iv.a` LANDED, `.5.iv` measured**
Owner: `KG-ISF-COMPLETENESS.5`
Partitioned: 2026-08-31 (`LIVE-DOCUMENT-PRESSURE-HEADROOM.4e`)
Measurement of record: [`generic-enum-conflation-measurement.md`](generic-enum-conflation-measurement.md)
Naming doctrine: [`ADR 0006`](../decisions/0006-no-hardcoded-chip-spec-vocabulary.md)

This record holds the per-leaf results of the `.5` enum-surface fidelity program. They were appended to the
`2026-06-24` conflation measurement as each leaf closed — three LANDED gate results and three follow-on
measurements over seven weeks — until that record reached 559 of its 640-line per-file ceiling with one
remaining writer still to report. The sections below are byte-identical to the measurement text they were
moved from and appear in its original order, which is the order they were appended. Nothing was reworded,
merged, or dropped.

The measurement retains its defect statement, extraction-side origin, corpus census, member-quality finding,
decomposed decision, reproducer, and conclusion, and states the composed result under `Outcome`. The
`.5.iv.a` CODE slice reports here, and its section corrects three things `.5.iv` recorded.

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

## `.5.iv` measurement (`2026-08-11`, read-only) — the header may source a name, and merge-by-name does not conflate

Surfaced by `CORPUS-COVERAGE.2.51`. Refreshing the Arm SMMU Software Guide retired its generic-`TABLE`
mega-enum exactly as `.5.i` intends, and in doing so exposed an asymmetry in the gate. The source table is
`Table 3-1: Stream Security determination`, whose header reads `SEC_SID value | Description`.
`derive_encoding_enum_name` draws its candidate from `caption_text` or the section title
(`crates/specforge/src/ir/evidence.rs:4698-4703`) and only then validates it against `known_signals` and the
header (`:4715-4733`). The header is therefore a **veto** and never a **source**: a table whose caption
carries no field token mints nothing even when its header names the field outright. The signal-match loop
above the fallback cannot close the gap either, because it fires only where the token is already a declared
signal — precisely the case that does not need help.

Reproducer: `scripts/measure_encoding_enum_header_naming.py` (read-only over the 78 persisted SourceIRs; no
VLM, Docling, or stage rebuild; repository-root-relative; `--json` for the machine-readable census).

| Population | Count |
| --- | ---: |
| `encoding` tables corpus-wide | 2,540 |
| …single header row shaped `<FIELD> value \| Description` with exactly one non-generic field token | 281 |
| …whose members survive the `.5.ii` sentence-spine gate, so a non-empty enum would be minted | 134 |
| …documents involved | 10 |
| distinct (document, candidate name) pairs | 82 |
| pairs drawing one name from more than one table in the same document | 28 |
| **collision groups conflicting on any shared value → member mapping** | **0 of 28** |

### Why the last row decides it

`.5.i`'s objection to a name fallback was never "the name looks wrong"; it was
`build_symbol_definitions`' merge-by-name (`ir/semantic.rs`), which fused every `TABLE`-named table into one
junk enum. That failure mode **does not reproduce** for header-sourced names, and the reason is structural
rather than lucky. A caption keyword like `Table` is shared by tables with nothing in common, whereas a
header names the actual field, and a field encodes the same way throughout a document — so the merge
reconstructs the field rather than fusing strangers.

The worked example is SMMU `SH`: eleven separate tables in `ihi0070_e_a`, every shared value identical
(`0b00=NON_SHAREABLE`, `0b10=OUTER_SHAREABLE`, `0b11=INNER_SHAREABLE`, and `0b01=RESERVED` where the table
lists it). Merging the eleven yields the correct Shareability encoding.

### Why the naive predicate is still a NO-GO

The 134 are a mix. Genuine field encodings — `AWATOP`, `ENDIAN`, `EXCL`, `RESPERR`, `ARCHID`, `DATASOURCE`,
`ST_LEVEL`, `CD2L`, `VMID16`, `PRI`, and the 102-table SMMU architecture-spec body — sit beside four classes
a code slice must exclude first:

- **`OFFSET`-headed register-offset tables** (three, CoreSight SDC-600): the header names a column concept,
  not a field; members are `RESERVED` / `CORESIGHT_MANAGEMENT_REGISTERS`.
- **`*_WIDTH` self-named pseudo-enums** (`NODEID_WIDTH`, `REQ_ADDR_WIDTH`, `DATA_WIDTH`) whose only member is
  `LEGAL_VALUES` — the family `.5.iii` already recorded as an honest residual, reappearing from the header side.
- **Garbled members**: `AXADDR` → `VA_40`, `NUM_2_0_A`.
- **Twelve `RESERVED`-only enums**, which carry no intent at all.

### Honest correction

The observation that opened this leaf is partly wrong. The SMMU *guide* table that motivated it would still
mint nothing under a header-sourced name, because its members are whole description sentences and the `.5.ii`
spine gate correctly drops them, emptying the enum. The lever is real, but it does not help the document that
surfaced it — which is the argument for measuring before implementing, not after.

### Handoff to `.5.iv.a` (CODE)

Deferred to its own focused slice under the high-stakes gate-code rule `.5.i` ran under. It changes a shared
extractor and would mint a new `AWATOP` enum on the AXI wire gold `ihi0022_l`, so it is byte-changing on a
scored document and requires the before/after WIRE-BASED-100 protocol on rebuilt gold evidence, a corpus-wide
old-versus-new `--dry-run` replay, and FSMGen `--strict --check --json` on every changed `.isf`. The exclusion
predicate must itself be measured FP-free before it lands, exactly as `.5.iii`'s `_WIDTH` gate was.

---

## `.5.iv.a` LANDED (`2026-08-31`) — the header sources the name, and the measured junk classes were wrong

The `.5.iv` census answered its own question correctly and named the wrong exclusions. Re-measuring the
population the SHIPPED scan actually visits — not the one `.5.iv` sampled — changed both the size of the
lever and the predicate it needs.

### Correction 1 — the population is twice what `.5.iv` counted

`.5.iv` filtered on `table_kind == "encoding"`. The shipped scan does not:
`scan_encoding_tables_by_signal_anchor` skips only signal-description, register-map, and timing-parameter
tables, and `table_looks_like_encoding` then admits any table whose header carries a name column and a value
column. `unknown`-kind tables are therefore in scope, and they carry a junk class `.5.iv` never saw at all.

| Population (all visited kinds) | Count |
| --- | ---: |
| tables the scan visits | 9,828 |
| …reaching the header path (one header row, two cells, a description-role right cell) | 994 |
| …**accepted** — an enum name is sourced | **285** in 9 documents, 164 distinct names |
| …declined: positional header | 448 |
| …declined: no encoding literal in the value column | 115 |
| …declined: ambiguous header (no single field token) | 146 |

Reproducer: `scripts/measure_header_sourced_enum_naming.py` (read-only over the persisted SourceIRs; mirrors
the shipped predicate clause for clause; `--json` for the machine-readable census).

### Correction 2 — the four classes `.5.iv` named are not the junk that matters

Measured against the whole population rather than the `encoding`-kind sample:

- **The dominant class is POSITIONAL, and `.5.iv` did not name it.** A header word such as `Bytes`, `Offset`,
  `Index`, or `bits` declares the left column to hold a position or an address, so the table lays out *where*
  a field sits rather than *what* its values mean. One clause removes 448 of the 709 declined candidates,
  including every NVMe `Bytes | Description` structure table, the CoreSight SDC-600 `Offset | Description`
  tables `.5.iv` listed, and the AXI `AxADDR bits` table whose members `.5.iv` recorded as "garbled".
- **The second class is "encodes nothing", and it subsumes two more of the four.** A glossary
  (`Term | Meaning`), a notation legend (`Notation | Meaning`), an abbreviation table
  (`Acronym | Description`), and a `<X>_Width value | Description` parameter table all share one structural
  property: **no value cell parses as an encoding literal**. Requiring at least one parseable value therefore
  disqualifies the `*_WIDTH` pseudo-enums `.5.iv` named *and* the glossary class it never saw, without a
  word list for either. This is the clause that keeps the I2C, OpenCAPI, and CoreSight-BSA glossary tables
  out — and those are the only accepted-shape tables in a document whose chain can still be rebuilt.
- **`RESERVED`-only is NOT an exclusion, and the record is corrected.** `.5.iv` called the twelve
  `RESERVED`-only enums junk that "carry no intent". Two measurements overturn it. First,
  `build_symbol_definitions` keys members by NAME and drops any member whose value conflicts, so the five
  multi-row cases self-eliminate downstream with no name-side gate. Second, the seven single-row cases are
  structurally indistinguishable from 31 legitimate single-distinct-member tables in the same population
  (`TTL 0b00 = NO_LEVEL_HINT_INFORMATION`, `CD2L 0b1 = 2_LEVEL_CD_TABLE_SUPPORTED`, `S1P`, `PRI`, `GRAN4K`…)
  — the only discriminator is the word `RESERVED` itself, which is exactly the spec-assigned value
  vocabulary ADR 0006 forbids. And in the merge they are not even vacuous: CHI `DataSource` fuses a
  meaning row with a reserved row into the field's correct encoding. So no `RESERVED` clause shipped.

### The predicate that shipped

`derive_header_sourced_enum_name` (`crates/specforge/src/ir/evidence.rs`) runs LAST, only after the
signal-match loop and the `.5.i`-gated caption fallback have both declined, so it is strictly additive: no
enum minted today changes name or disappears. Its five clauses are the corrections above:

1. one header row of exactly two cells;
2. the right cell names a description role;
3. the left cell carries no POSITIONAL role and, after document-structure and column-role words are stripped,
   leaves exactly one identifier — the field;
4. no value cell is a positional range (`03:02`, `[2:0]`), which marks a field-layout table; and
5. at least one value cell parses as an encoding literal.

### Correction 3 — the leaf's own prediction was false

`.5.iv` handed the code slice a gate: "byte-changing on a scored document — it mints a new `AWATOP` enum on
the AXI wire gold `ihi0022_l`". Both halves are wrong, and the reason is the corpus, not the lever:

- `ihi0022_l` already carries an `AWATOP` enum with 13 members, so the lever could at most add members.
- **`ihi0022_l` cannot be rebuilt at all.** Its persisted SourceIR is legacy schema 1, which the current
  binary refuses for canonical use (`legacy proofless SourceIR schema 1 … is inspection-only and must be
  rebuilt before canonical use`), and its normalized bundle is not retained. It is one of the 54 legacy
  chains, against 24 current ones.
- Of the 285 accepted tables, **1** sits in a rebuildable document — the Arm SMMU Software Guide's
  `Table 3-1: Stream Security determination`, the very table that opened `.5.iv` — and it mints nothing,
  because its members are description sentences the `.5.ii` spine gate drops. `.5.iv` predicted exactly that
  in prose; this is the mechanical confirmation.

So the change is **inert on the whole measurable stratum**, and the corpus-wide replay proves it rather than
assuming it: `check_chain_currency.sh` replays evidence, semantic, intent, and the ISF adapter for all 24
rebuildable documents against the patched binary and finds every persisted artifact byte-identical — and
states its own limit in the same line: `0 emitted .isf file(s), 24 blocked/no-file state(s)`, so the FSMGen
`--strict` leg has nothing to run on here and is vacuous by construction rather than passed. The capability
is real and reaches those 8 documents the moment their chains are re-ingested.

### Found while gating this slice — the WIRE-BASED-100 scoring oracle cannot run

The before/after WIRE-BASED-100 protocol this leaf inherited could not be executed, and not because of this
change: `eval-extraction` refuses **every** document in the corpus, on the pre-change binary too. Isolated to
a two-line probe — relocating an EvidenceIR by rewriting only its `artifact_layout`, every other byte
identical, fails canonical verification with `registered derivation 'evidence.claim.schema_version.root'
output or input topology is stale`, while the byte-identical copy that keeps its original layout verifies
— and `extract_on_copy` must relocate, so the corpus is never mutated. Owned by `WIRE-BASED-100.8`.
