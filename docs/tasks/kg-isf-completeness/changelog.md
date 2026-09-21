# KG-ISF-COMPLETENESS — changelog

- Part ID: `changelog`
- State: `legacy`

<!-- kg-isf-completeness-task-source-region:changelog:start -->
## Changelog

- `2026-09-21`: **`.5.vi.a` LANDED — the caption label was mis-pointing the value column, not just
  adding a junk name.** The trial was reverted on `2026-09-21` because the rule widened `.5.vii`'s
  positional defect; `.5.vii.a` then deleted that fallback, so the blocker was re-derived and found
  CLEARED rather than assumed. `caption_descriptive_text` strips `<word> <numbering token>` at both
  caption readings, with a volume letter (`A7.4`) or every Arm caption is missed. Grammar, not a word
  list: **5,408 of 5,531 captions carry a label and the leading word is one of exactly three.** One
  guard — never strip a word the document DECLARES (0 documents declare `table`/`figure`/`chapter`);
  the header-echo guard was tested and REJECTED because **123 tables echo the caption into the
  header**, exactly the tables that need stripping. Replay over all 27 chains: **2 documents change,
  +40 statements, 0 removed, 0 non-`Enum` change**, every other surface byte-identical. **Root-caused
  why a NAME gate adds MEMBERS**: `infer_encoding_column_indices` takes the name as an argument, so
  `TABLE` let `code` inside `Opcode` claim the value column and the members were dropped as
  unparseable. 26 of the 40 are exactly correct AXI/ADIv6 encodings; 11 carry stated values with
  names from the wrong column, opened as `.5.xiii` (262 tables / 34 documents, upper bound).
  **WIRE-BASED-100 before == after, byte-identical across all five seeds, both binaries, rebuilt
  chains** — the leg the trial could not run. `kg-bench` 156/156. Frontier → `.5.xi`.

- `2026-09-21`: **`.5.viii` LANDED — the quote states the RADIX, and the prescribed repair was wrong.**
  `.5.vii` pinned *"a bare `'1110'` already parses as binary under the all-`0`/`1` arm, so stripping
  the wrapper is sufficient"*. It does not: `parse_encoding_numeric_literal` tries `parse::<u32>()`
  first, so that arm is unreachable below eleven digits and a stripped `'1110'` is **1110, not 14**.
  Scored over all 357 slots the prescribed repair is **wrong on 314 (88.0%)** and **strictly worse
  than the defect it replaces on 57**. Its stated acceptance bar — *"the quote-wrapped count goes to
  0"* — is satisfied identically by both repairs: **a COUNT cannot verify a VALUE.** The documents
  state the convention themselves, so `parse_encoding_value_literal` reads `'<bits>'` as binary and
  `x'<hex>'` as hex, at the member seam only. **The pinned hazard was too narrow**: the shared reader
  also gates `derive_header_sourced_enum_name` clause 5, which MINTS an enum, so widening it would
  have been identity-additive — the `.5.vi.a` trap, avoided by construction and pinned by a test.
  All **27 replayable chains byte-identical** (the 357 are legacy; effect today is 0 by design and
  `.5.ix` is why it lands anyway), `kg-bench` 156/156, 0 quoted cells in any tracked fixture.
  Opened `.5.x` (clause 5, 3 tables), `.5.xi` (decimal-before-binary for unquoted cells, 1,173 cells,
  158 replayable), `.5.xii` (grouped `0b`/`0x` refused, 77 cells, + a census instrument bias).
  Frontier → `.5.vi.a`.

- `2026-09-21`: **`.5.vii.a` LANDED — the fallback is gone, and the landing is 54, not 10,189.**
  `synthesize_encoding_declarations_for_enum` no longer substitutes `row_idx`. **The acceptance bar
  `.5.vii` wrote was corrected before any code was written**: 10,137 of the 10,189 values are in the
  51 unrebuildable legacy chains, so a replay can only observe 52 across 4 documents — the same
  stratum trap that reversed `.5.vi.a`, caught early this time. Measured: **54 statements removed, 0
  added, every one an `Enum`**; the 2 over prediction are AHB `HREADYOUT`, whose bare-digit members
  make its pairs producible by seven tables, so the census under-counted it — **578 slots (3.0%) are
  in that ambiguity class and the totals are FLOORS**, which the reproducer now prints itself.
  **The leg `.5.vi.a` could not run is green**: rebuild-then-validate per stage in order re-earns the
  proof seal, giving chain-currency **27/27 at all four stages** and letting `eval-extraction` run —
  WIRE-BASED-100 **before == after on both binaries**. `kg-bench` 156/156. 2 promoted symbols
  withdrawn (`AWDOMAIN`, `ARDOMAIN`), 4 more now fully document-stated.
  Frontier → `.5.vi.a`, whose blocker this removes.

- `2026-09-21`: **`.5.vii` MEASURED — the fallback is half the value surface, and it is not defensible.**
  Read-only over all 78 chains (`scripts/adjudicate_positional_enum_values.py`; report
  `docs/research/enum-value-provenance.md`). Replaying the producer instead of reading the value's
  SHAPE turns `.5.v`'s 160 enums into **10,189 of 19,023 values (53.6%) the document never stated**,
  because a shape test can see neither a mixed enum nor a gapped one. Only **84 of 512** promoted
  symbols carry no fabricated value. The ordinal is uncorroborated by its own row 94.5% of the time
  and **provably wrong in 5,688 slots** (it equals a value the same enum states elsewhere). Largest
  cause is a header word, not a missing column: `bit` matches `Bits`, a field-LAYOUT column. **DECISION
  GO** on removal → `.5.vii.a`. Two corrections published: `.5.vi`'s two protected continuations are
  legitimate in identity only and both are content junk — a continuation is the one case where a
  table-local ordinal cannot be right — and 357 fabrications are quoted literals the parser cannot
  read, split out as `.5.viii`. KM `[[minted-enum-values-are-mostly-row-indices]]`.
  Frontier → `.5.vii.a` (CODE).

- `2026-09-21`: **`.5.vi.a` TRIAL — the identity rule is not subtractive, and the sequencing reverses.**
  Implemented, measured corpus-wide, and reverted unlanded. `.5.vi` simulated the rule as removal only;
  the signal-match branch returns the LONGEST declared signal in the identity context, so deleting the
  caption label also changes which signal wins, and the rule ADDS enums. Replayed read-only over all 27
  rebuildable chains: **24 identical on the whole statement surface, 3 changed, every delta a purely
  additive `Enum` statement.** The AXI wire gold goes 79 -> 133 enum statements, unblocking genuine
  encodings the label had hidden (`AxBURST` FIXED/INCREMENTING/WRAPPING = 0/1/2, `AxSIZE`, `AxDOMAIN`,
  `AxMMUSECSID`, …) **and** two junk enums that promote encoding literals as member names.
  **All 54 enums `.5.vi` predicted it would clean are in legacy chains that cannot be rebuilt**, so the
  benefit is unrealised while the cost lands on current, scored documents — which reverses `.5.v`'s
  ordering: `.5.vii` first, or both together. `kg-bench` 156/156; APB/AHB still 1.000 and
  byte-identical; the AXI seed REFUSES until its chain is rebuilt, because a producer change stales the
  proof seal and `rebuild_stage_cascade.sh` never absorbs an intended content delta. **A prediction
  derived from a simulation of the rule is not a measurement of the rule.**

- `2026-09-21`: **`.5.vi` DONE — the case the leaf was told to protect is two enums, and neither of
  its two candidate rules survives anyway.** Adjudicating `.5.v`'s 231 conflations per item found
  **2 document-stated page-break continuations** corpus-wide, both in the AXI gold, against 22 that
  only *look* like one: that layout class **mixes** genuine AXI+ACE code-table continuations with
  RISC-V register-field tables sharing a header shape, so outside the two stated cases nothing
  separates them and a span-based rule cannot be made FP-free. The defect is upstream of the span:
  the caption's `Table N-M` label prefix names **1,634 of the 3,325** conflated tables and 6 of 316
  healthy ones, and forbidding it reaches exactly **54** enums named `TABLE`/`FIGURE`/`CHAPTER`.
  **A rule aimed at the symptom the census measured would have been wrong; the census's own
  dominant signal pointed one seam upstream.**
  **And this leaf published a wrong conclusion first.** Its initial run reported `0` stated
  continuations and concluded the legitimate case had no population at all — flattering the rule it
  was proposing. Three instrument defects caused it: a caption-label pattern anchored on a digit,
  which cannot see Arm's volume numbering (`Table A7.4`); a continuation test that accepted a
  shared `Chapter B15.` reference as the document's statement while a trailing-colon mismatch hid
  the two real pairs; and reading only one of the marker's **two dialects**, missing the trailing
  `(continued)` form that spans 20 documents to the prefix form's 7. **The near-zero was checked
  for vacuity — 209 tables do carry the prefix marker — and that check PASSED while the instrument
  was blind on most of the corpus**, which is the lesson: a non-vacuous denominator does not
  validate the numerator. `.5.vi.a` owns the code. Report
  `docs/research/enum-identity-adjudication.md`; reproducer
  `scripts/adjudicate_conflated_enum_identity.py`; KM `[[caption-label-is-not-table-identity]]`.

- `2026-08-31`: **`.5.iv.a` CORRECTION — two of the statements the landing published are withdrawn.** The
  director audited the five findings. Three re-derived exactly (the eval-extraction refusal and its
  relocation cause on the pre-change binary; 54 legacy / 24 current chains with APB/AHB/AXI legacy and only
  SWD+I2C rebuildable; 994 candidates / 285 accepted / 1 in a rebuildable document, `AWATOP` already 13
  members, SMMU `Table 3-1` minting nothing). Two did not, both inside the `RESERVED` argument, and both
  because they were read off a dump instead of computed. **(a) The split is SIX/SIX, not five/seven.**
  Modelling `build_symbol_definitions`' merge-by-name and conflicting-value drop: in the `.5.iv` census
  frame 6 of 12 `RESERVED` members are eliminated and 6 survive; in the shipped-predicate frame 4 of 10 are
  eliminated and the SAME 6 survive (`STALL_MODEL`, `TTENDIAN`, `HTTU`, `PGS`, `CONTFORMAT`, `EVENTTYPE`).
  Four conflict inside their own table; two only conflict once merge-by-name pulls in a sibling table, which
  is exactly what counting rows per table misses. **(b) The `DataSource` worked example is backwards.** It
  accumulates `DEFAULT_NO_USEFUL_INFORMATION = 0` plus `RESERVED` at 2 AND 3; the reserved rows disagree, so
  the rule drops `RESERVED` and the surviving enum is `(DATASOURCE (DEFAULT_NO_USEFUL_INFORMATION 0))` — the
  merge preserves no reserved encoding, the opposite of what was published. **The NO-GO decision does not
  move**: it rested on the elimination leg and the indistinguishability leg (6 survivors against 31
  legitimate single-member comparators, discriminable only by a spec-assigned word ADR 0006 forbids), and
  only the decorative third leg is gone. `scripts/measure_header_sourced_enum_naming.py --reserved-split`
  now models the merge and prints both frames, so the split is re-derivable instead of eyeballed — the same
  lesson `CLAIM-VERIFICATION-ADOPTION.9`'s ninth instance records. Report §`.5.iv.a correction`; KM
  `[[generic-enum-conflation]]`.

- `2026-08-31`: **`.5.iv.a` DONE — CODE: the encoding table's own column header may now SOURCE its enum
  name.** Added `derive_header_sourced_enum_name` (`ir/evidence.rs`) as a LAST resort after the signal-match
  loop and the `.5.i`-gated caption fallback, so the change is strictly additive. Its five clauses were
  measured over the population the SHIPPED scan visits, and that re-measurement corrected `.5.iv` three times:
  the population is **994 candidates / 285 accepted in 9 documents**, not 134 in 10 (`.5.iv` censused only
  `table_kind == "encoding"`, but `table_looks_like_encoding` admits `unknown`-kind tables too); the four
  exclusion classes `.5.iv` named are the wrong list (**positional headers** — `Bytes`/`Offset`/`Index`/`bits`,
  the column holds a POSITION so the table is a field LAYOUT — remove 448 alone and subsume two of the four,
  and **"no value cell parses as an encoding literal"** subsumes the `*_WIDTH` class *and* the
  glossary/notation/abbreviation class `.5.iv` never saw); and **`RESERVED`-only did not ship as an
  exclusion** — SIX of the twelve have their `RESERVED` member eliminated by `build_symbol_definitions`'
  conflicting-value rule and SIX survive, and those six are structurally identical to 31 legitimate
  single-distinct-member tables, so the only discriminator left is a spec-assigned value word ADR 0006
  forbids (corrected `2026-08-31`; first published as `five`/`seven`). **The node's own gate prediction was also false:**
  `ihi0022_l` already carries `AWATOP` (13 members) and cannot be rebuilt at all — legacy schema 1, one of 54
  legacy chains against 24 current — and exactly ONE accepted table sits in a rebuildable document (the SMMU
  guide `Table 3-1` that opened `.5.iv`), which mints nothing because `.5.ii` drops its sentence members.
  **NO REGRESSION:** `check_chain_currency.sh` **24 replayed / 24 current / 0 stale** at evidence, semantic,
  intent, and isf-adapter — every persisted artifact byte-identical; `kg-bench` **156/156**; `cargo test`
  green (+8 tests); `check_doctrines.sh` GREEN; the production-genericity flow census moved only
  `analyzed_functions`/`helper_edges`/`decision_sites`, with every authority and protection count unchanged.
  **Honest gap, not caused here:** `eval-extraction` refuses every corpus document on the PRE-change binary
  too, so the WIRE-BASED-100 before/after protocol could not run — isolated to an EvidenceIR proof that binds
  the artifact's own `artifact_layout`, and opened as `WIRE-BASED-100.8`. Report §`.5.iv.a`; reproducer
  `scripts/measure_header_sourced_enum_naming.py`; KM `[[generic-enum-conflation]]` +
  `[[evidence-proof-binds-artifact-location]]`. **`.5` is now complete** — `.5.i` name gate, `.5.ii` spine
  member gate, `.5.iii` `_WIDTH` member gate, `.5.iv` header-naming measurement, `.5.iv.a` header source.

- `2026-06-24`: **`.5.iii` DONE — CODE: per-member `_WIDTH` parameter-leak enum gate.** Same fresh-session
  PNT continuation (the two slices of `.5.iii`: measurement, then code). Added
  `is_width_parameter_leak_member` (`ir/evidence.rs`) + a `continue`-skip in
  `synthesize_encoding_declarations_for_enum`'s member loop after the `.5.ii` spine gate: a member named
  `<X>_WIDTH` where `X` is the enum's own name OR a declared signal (the `known_signals` set, now threaded
  from the signal-match caller) is a width PARAMETER → dropped, so a polluted enum keeps its codes (AXI
  `BRESP` keeps OKAY/EXOKAY/SLVERR/DECERR/…) and a pure-parameter enum empties (no statements → no
  `SymbolDefinition` → honest residual). +2 tests. **Verified per-item:** AXI evidence rebuild drops
  EXACTLY the 7 leaks (statements 6414→6407; non-Enum statement set byte-identical); the AXI `manager.isf`
  now emits `(BRESP (OKAY 0)(EXOKAY 1)(SLVERR 2)(DECERR 3)(DEFER 4)(TRANSFAULT 5)(RESERVED 6)(UNSUPPORTED
  7))` (8 clean codes, no dup) + `(AWCMO (CLEAN_AND_INVALIDATE 0)(CLEAN_ONLY 1))`, and the false
  `(RRESP (RRESP_WIDTH 0))`/`AXSNOOP`/`RCHUNK*` enums are gone; FSMGen `--strict --check` **success / 0
  diagnostics**. **NO REGRESSION:** WIRE-BASED-100 = **1.000 before==after** (AXI before/after eval
  identical; APB/AHB/SWD/i2c evidence byte-identical old-vs-new → gate inert); `kg-bench` 156/156;
  `run_ci.sh` GREEN (lib **1718**, +2). ADR-0006 (document-grounded, FP set EMPTY — `FULL_WIDTH` kept).
  **`.5` enum-surface fidelity now has all three member gates** (`.5.i` name + `.5.ii` spine + `.5.iii`
  `_WIDTH`); the `*_WIDTH`-named self-pseudo-enums (SECSID_WIDTH/SID_WIDTH/SSID_WIDTH) stay an honest
  residual. Report §`.5.iii LANDED`; KM `[[generic-enum-conflation]]` → `.5.iii` LANDED. Frontier →
  KG-ISF-COMPLETENESS has no further immediately-buildable leaf (`.1c.ii` upstream-NLP-gated, `.2b`
  measured-marginal); PNT advances to the next active tree.

- `2026-06-24`: **`.5.iii` MEASUREMENT DONE → GO on the `_WIDTH` parameter-leak gate (read-only, docs-only).**
  Fresh-session PNT continuation off `.5.ii` (the first active tree, north star; the resume pointer flagged
  the `.5.ii` deeper member-quality residuals as the re-open candidate "needing their own measurement"). A
  read-only census over all 78 persisted IntentIR docs (tracked reproducer `scripts/measure_enum_width_leak.py`)
  measured the five deferred classes. **The deeper classes are largely SUBSUMED by `.5.i`** (47/54 `_WIDTH`
  members and the bulk of the 319 section-caption survivors sit in generic-named enums `.5.i` already drops
  whole). **But the `_WIDTH` leak reaches the AXI WIRE-GOLD `.isf` and is materially damaging:** 7 members in
  real-signal-named enums (`BRESP`/`RRESP`/`RCHUNKNUM`/`RCHUNKSTRB`/`AXSNOOP`/`AWCMO`, all in `ihi0022_l`)
  emit `(BRESP (BRESP_WIDTH 0)(OKAY 0)…)` (dup value), `(RRESP (RRESP_WIDTH 0))` (real codes replaced),
  `(AXSNOOP (AWSNOOP_WIDTH 0)(ARSNOOP_WIDTH 1))` (pure junk) — a width PARAMETER mis-read as an encoding
  VALUE (`Enum BRESP BRESP_WIDTH = 0.`). Unscored by WIRE-BASED-100 (enum surface orthogonal — why it held
  1.000 while the `.isf` carried junk). **Clean, FP-free, ADR-0006 discriminator:** drop `<X>_WIDTH` iff `X`
  is a declared signal OR the enum's own name (all 6 caught prefixes are declared signals; corpus FP set
  EMPTY — no `FULL_WIDTH`-style value exists, and `FULL`/`HALF` are never declared signals). Per-member, not
  per-enum (keeps BRESP's codes; empties RRESP/AXSNOOP → honest residual). **NO-GO** on section-caption (FP
  risk: `D1`/`L2` real codes), restart-of-clean (no defect), glossary/front-matter (tiny). No code → all
  oracles orthogonal; `check_doctrines.sh` GREEN. Report §`.5.iii measurement`; KM `[[generic-enum-conflation]]`.
  `[[feedback_scoring_rigor]]` / `[[feedback_avoid_denylists_prefer_structural]]`. Frontier → the `.5.iii`
  CODE slice (the `_WIDTH` gate; before/after WIRE-BASED-100 eval required, byte-changing on the AXI gold).

- `2026-06-24`: **`.5.ii` DONE — CODE: per-member sentence-spine enum-fragment gate.** Same fresh focused
  session as the `.5.ii` measurement (the two slices of `.5.ii`: calibration, then code). Added
  `PROSE_SENTENCE_SPINE_WORDS` + `is_prose_fragment_member_name` (`ir/evidence.rs`) and a `continue`-skip in
  `synthesize_encoding_declarations_for_enum`'s member loop: a synthesized member whose `_`-token set holds
  an English sentence-spine word (copula/aux/modal/article/demonstrative/relativizer/subordinator;
  collisions `a`/`i`/`its`/`can`/`may`/`am` EXCLUDED per `.1a`) is a captured prose sentence → dropped, so a
  conflated enum keeps its genuine codes (AXI `BRESP` recovers `OKAY/EXOKAY/SLVERR/DECERR/…`) and a
  pure-prose table empties (no statements → no `SymbolDefinition` → honest residual). +4 tests. **NO
  REGRESSION:** WIRE-BASED-100 = **1.000 before==after** (PROVEN — before/after `eval-extraction` over all
  10 seeds on gold evidence rebuilt with the preserved baseline vs gated binary, scored surface
  byte-identical); rebuilt AXI + APB `.isf` pass FSMGen `--strict` `success`; `kg-bench` 156/156; `run_ci.sh`
  GREEN (lib **1716**, +4). ADR-0006 (universal grammar, no name list). **`.5` enum-surface fidelity built**
  (`.5.i` name-gate + `.5.ii` member-gate); deeper member-quality classes (glossary/front-matter/
  section-caption/`_WIDTH`/restart-of-clean) are honest residuals. Report §`.5.ii LANDED`; KM
  `[[generic-enum-conflation]]` → LANDED. Frontier → KG-ISF-COMPLETENESS has no further immediately-buildable
  leaf (`.1c.ii` upstream-NLP-gated, `.2b` measured-marginal); PNT advances.

- `2026-06-24`: **`.5.ii` MEASUREMENT/CALIBRATION DONE → design CORRECTED + GO (read-only, docs-only).**
  Fresh-session PNT continuation. Read-only member-quality census over all 78 persisted IntentIR docs (561
  enums / 12 509 members). **Overturns the recorded `.5` plan:** the gate is **per-member**, not per-enum (a
  whole-enum drop destroys AXI `BRESP`'s real codes `OKAY/EXOKAY/SLVERR/DECERR/…` fused with prose), and
  **value-restart is NOT a junk signal** (AHB `HPROT` restarts but every member is a clean identifier). The
  load-bearing signal is per-member NAME shape: an English **sentence-spine** token (copula/aux/modal/
  article/demonstrative/relativizer/subordinator) marks a prose-fragment member. Collisions EXCLUDED per the
  `.1a` discipline (`A`/`I`/`ITS`/`CAN`/`MAY`/`AM`). **Precision/recall:** clean anchor 0/115 flagged
  (precision 1.000), junk anchor 269/269 caught (recall 1.000); 30.2 % of members drop. Honest residuals
  deferred (glossary `SEE…`, front-matter, section-caption `B2_3_1_…`, `_WIDTH` leaks, restart-of-clean).
  **GO** — land a per-member sentence-spine fragment drop at `synthesize_encoding_declarations_for_enum`
  (`evidence.rs`); byte-changing on wire golds (strict improvement) → before/after WIRE-BASED-100 eval
  required on the code slice. No code → all oracles orthogonal; `check_doctrines.sh` GREEN. Report §`.5.ii
  measurement`; KM `[[generic-enum-conflation]]`. `[[feedback_scoring_rigor]]` /
  `[[feedback_avoid_denylists_prefer_structural]]`. Frontier → the `.5.ii` code slice.

- `2026-06-24`: **`.5.i` DONE — CODE: extraction-side fallback name-gate + emitter orphan-`(type)` fix.**
  Fresh focused session per the high-stakes gate-code rule (it changes wire-gold `.isf` bytes). Two edits:
  (1) `derive_encoding_enum_name`'s caption-token fallback (`ir/evidence.rs`) keeps the candidate ONLY when
  it is independently evidenced — a declared signal OR a column-header reference token of the table — else
  returns `None` (existing `continue` contract → honest residual), killing the generic enums AND the
  merge-by-name conflation; (2) the emitter (`ir/isf_ir.rs`) gates the `(types)` block by `emitted_enums()`
  so a member-dropped enum leaves no orphan `(type …)`. **Two corrections to the `.5` measurement, both
  making the result cleaner:** the genuinely-named enums come from the signal-match loop ABOVE the fallback
  (byte-identical), so the structural gate is *strictly better* than a name-only gate (it also drops
  fallback-origin "real-named-but-junk" like `COMMAND`/`AMBA`/`READ`/`CACHE`/`RELEASE`); and the gate also
  cleans off-gold junk constraints whose value was a dropped-enum member (AXI `ACTIVATEACK A` → grounded
  `ACTIVATEACK 1`), score-orthogonal (distinct constraint facts identical). **Measured:** corpus census (33
  rebuildable docs) generic-named enums **82→8** / total enum records **422→105**; per-doc before→after on
  HBM2 + 4 wire golds + AXI-Stream + SWP + 6 worst offenders (amd 34→5, gic_600 23→6, coresight 19→5,
  cortex_a76 16→7, CHI 14→4, CHI-C2C 7→1) — real signal enums byte-identical, all FSMGen `--strict` 0
  diagnostics. The 8 survivors are document-evidenced (column-header/declared-signal) honest `.5.ii`
  residuals. **NO REGRESSION:** WIRE-BASED-100 = 1.000 before==after (PROVEN by before/after eval on
  rebuilt gold evidence); nvme-registers + i2c golds identical; `kg-bench` 156/156; `run_ci.sh` GREEN (lib
  1712, +4 tests). ADR-0006 (structural, no name list — `DATA` kept where a real signal, dropped where a
  caption word). Report §`.5.i LANDED`; KM `[[generic-enum-conflation]]` → LANDED. Frontier → `.5.ii`.

- `2026-06-24`: **`.5` OPENED — generic-`TABLE` mega-enum conflation MEASURED (read-only, docs-only).**
  Fresh-session PNT pivot off `CORPUS-COVERAGE.2` (the re-ingest tail is now empirically low-value — #32 was a
  legal-exhibit excerpt — so per `[[feedback_not_complete_attack_substantive_gaps]]` the high-value move is
  acting on a surfaced upstream extraction-precision lever). A delegated read-only agent + my own verification
  localized the defect to EXTRACTION (`evidence.rs` enum-name fallback + `semantic.rs` merge-by-name, NOT the
  emitter), measured it corpus-wide (56/78 docs / 96 generic + 271 real-named-but-junk enums; HBM2 `TABLE` = 7
  fused tables), and established the decisive insight that a name-only gate is insufficient (the load-bearing
  signal is member quality). Decision: GO, extraction-side, decomposed into `.5.i` (safe name-gate + emitter
  orphan-`(type)` fix — the next code slice; deferred to a focused/fresh session because it changes wire-gold
  `.isf` bytes) and `.5.ii` (member-quality gate, calibration-gated). WIRE-BASED-100 scores orthogonal; `.isf`
  bytes change (strict improvement). No code → all oracles orthogonal; `check_doctrines.sh` GREEN; knowledge
  map 125→126. Report `docs/research/generic-enum-conflation-measurement.md`; KM `[[generic-enum-conflation]]`.

- `2026-06-24`: **`.3` CLOSED — bar #2 relation-completeness resolved (verification-only, no code change).**
  Fresh-session PNT pick (first eligible leaf of the first active tree). Re-census over all 78 persisted
  `intent_ir.json` vs `evidence_ir.json` → **0 stale docs** corpus-wide (the `CORPUS-COVERAGE.2` re-ingest
  sweep rebuilt the once-stale docs through `converge`, cascading the whole chain; recovered relations now
  land canonically: tilelink 33/34, I2C 17, gic_600 101, mmu_700 25, ATS 9, DTI 1, opencapi-TL 15, USB4 13 —
  each `intent`==`evidence`). Sub-step (ii), the generic stage-staleness `validate` detector, was already
  shipped + unit-tested as `CORPUS-COVERAGE.1` (`stage_staleness_relation_finding`). The 33 register/PHY/
  command docs at 0/0 are the correct (B) honest-absence class — relation-completeness is N/A there (their
  intent lives on register/message-field/transaction surfaces); wire protocols held at WIRE-BASED-100 = 1.000.
  No fabrication. Report closure appended (`docs/research/relation-completeness-measurement.md`); KM cards
  `[[relation-completeness-staleness-vs-absence]]` (closure addendum) + `[[stage-staleness-validate-detector]]`.
  All oracles orthogonal (no code change); `check_doctrines.sh` GREEN. Frontier → the tree's remaining open
  leaves are both deferred (`.1c.ii` upstream-NLP-gated, `.2b` measured-marginal), so KG-ISF-COMPLETENESS has
  no further immediately-buildable leaf; PNT advances to the next active tree.
- `2026-06-23`: **`.1c.ii` MEASURED → deferred as a bounded residual (read-only, docs-only).** Measured the
  bulk dense-prose phantom class (multi-word, single-`REL-INFERRED`, leading-noun subjects). A within-document
  structural gate is **disproven unsafe**: the `.1b.ii`-style connectivity fold mishandles the real cases on
  AXI+ACE `ihi0022_h_c` (`caching Manager`→`Manager` correct, but `Manager component`→`component` WRONG — the
  agent is the modifier, not the head; the agent token's position varies). Within one doc a real descriptive
  reference (`caching Manager`) and a phantom (`basic bus`) are structurally indistinguishable (same single
  relation, same provenance markers, same shape), and eMMC carries no `ProtocolActorRecord` grounding surface.
  A drop is forbidden by the genericity guardrail; a participation threshold by completeness. **The genuine
  fix is upstream relation-subject extraction precision on descriptive prose** (`[[project_nlp_shallow_parse_direction]]`),
  not a downstream rule → recorded as an honest residual (the phantoms never reach `.isf`). `.1c` umbrella
  outcome: `.1c.i` shipped, `.1c.ii` upstream-NLP-gated. Report §8; no code change → all oracles orthogonal;
  `check_doctrines.sh` GREEN.
- `2026-06-23`: **`.1c.i` DONE — CODE: dense-prose trailing preposition/auxiliary strip.** Same-session
  continuation of the `.1c` probe (fresh session, measurement in context). Extended
  `consolidate_trailing_fragment` (`ir/evidence.rs`) with a new `NON_ACTOR_TRAILING_FUNCTION_WORDS` const
  (prepositions + auxiliaries/modals, subset of the leading function-word lexicon, conjunctions excluded —
  drift-guarded), so a relation subject like `host has`/`host to`/`cache in` consolidates onto its leading
  agent (`host`/`cache`) the same way `.1b.i` folds `Subordinate extends`→`Subordinate`. **Verified:** eMMC
  live cascade actors 153→138 (the 4 aux/prep `host` variants merge onto `host`; 29 phantom names removed),
  `host.isf` strict-clean; **WIRE-BASED-100 = 1.000** (fresh-Pattern new-binary eval — constraints
  APB/AHB/AXI 6/6·6/6·3/3, relations APB/AHB/AXI/SWD 5/5·6/6·6/6·1/1, temporal 3/3·4/4·3/3); `kg-bench`
  156/156; `run_ci.sh` GREEN (lib 1704, +2 tests). ADR-0006 (closed-class grammar, corpus-safe: 0 ≥8-port
  actors are `X<aux/prep>` across all 78 docs). KM `[[agent-trailing-function-word-consolidation]]`;
  README/book/live-docs synced. Frontier → `.1c.ii` (single-relation noun-phrase precision, deferred).
- `2026-06-23`: **`.1c` OWNED + PROBE DONE — agent-identity precision for the DENSE-PROSE doc class (Lever E).**
  Fresh-session PNT pivot off `CORPUS-COVERAGE.2` (owner "attack substantive gaps, not easy incremental"): two
  consecutive re-ingests (#27 eMMC, #28 HBM2) each surfaced a NEW substantive lever, so the high-value move is
  ACTING on a surfaced lever, not grinding more re-ingests. Read-only measurement over the 78 persisted IntentIR
  docs (no code change). **Findings:** (1) the eMMC explosion (153 actors/349 rel vs stale 20/23) is a
  **relation-subject extraction-precision** problem — 148/153 actors are CONNECTED phantoms minted from a single
  `REL-INFERRED` relation whose leading token is a noun (so `.1a` passes, `.1b.iv` can't touch); only 5 are
  pure-unconnected (the grounded-keep role terms `.1b.iv` correctly preserves; 0 droppable). (2) A name-SHAPE-only
  drop is DISPROVEN unsafe — AMBA's real agents share the same shape classes as eMMC's phantoms → a shape drop
  would fail WIRE-BASED-100; the fix must be grammatical normalization or participation/grounding, never shape.
  (3) The doc class is DENSE-PROSE (AXI+ACE `ihi0022_h_c` 189 + CHI `ihi0050_g` 87 explode too; terse AXI gold
  `ihi0022_l` 21 clean). **Scoped:** `.1c.i` (extend `.1b.i` trailing-strip to prepositions+auxiliaries —
  corpus-wide safety MEASURED CLEAN, 0 ≥8-port actors are `X<aux/prep>` shaped across all 78 docs, wire golds
  untouched, reach 138 names/~17 docs) = the landable first gate; `.1c.ii` (single-relation noun-phrase phantom
  precision, the bulk ≈120) deferred-with-trigger pending its own measurement. Report
  `docs/research/agent-identity-prose-class-measurement.md`; KM `[[agent-identity-prose-class-measurement]]`; book
  honesty caveat added to `pipeline/evidenceir.md`. `check_doctrines.sh` GREEN; `kg-bench`/WIRE-BASED-100
  orthogonal (no extraction change). Frontier → `.1c.i`.
- `2026-06-18`: **`.2a.ii` DONE — CODE (OWNER-AUTHORIZED): initiator-perspective signal DIRECTION emission.** The
  owner's `2026-06-17` steer (AskUserQuestion: "Build it, initiator perspective") satisfied the `.2a` re-open
  trigger. The emitted `.isf` interface now lowers grounded actor-relative direction from the protocol's INITIATOR
  actor's perspective instead of defaulting non-`Input` signals to `(output)`. `select_initiator_actor` (`ir/isf_ir.rs`)
  picks the net-producer actor (out > in) maximizing `(out, in)` — structural, no name list (ADR 0006); validated
  per-item: AHB `Manager` (6/2), APB `Requester` (20/12), AXI `Manager` (116/52), SWD `debugger` (2/1).
  `initiator_perspective_directions` maps the initiator's `Drives`→`(output)`/`Reads`→`(input)` (ambiguous/InOut/
  Unknown omitted → residual); the signal loop prefers it, else flat hint, else `(output)`; `derive_isf_actor_name`
  (`ir/adapters.rs`) names the module after the same initiator (coherent). Strict-safe because the per-output
  named-drive block is already Output-filtered → inputs auto-suppressed from drives. **Measured flip (baseline-vs-after
  via `git stash`):** APB input 2→12, AXI 4→52, SWD 0→1 (faithful protocol direction); AHB 0→0 (honest residual —
  stale Manager grounding limited to sidebands + clock/reset). **Gates:** FSMGen `--strict` 0 NEW diagnostics on all
  4 wire docs (AHB `HAUSER` + AXI `ASKSTOP` pre-existing rule conflicts unchanged; APB/SWD PASS); `run_ci.sh` GREEN
  (lib 1677→1679, +2 tests); `kg-bench` 156/156; WIRE-BASED-100 orthogonal (emitter-only); ADR-0006. KM card
  `isf-initiator-perspective-direction`; book `pipeline/isf-adapter.md`. `.2a` fully resolved (width `.2a.i` +
  direction `.2a.ii`). `[[project_kg_isf_completeness]]`.
- `2026-06-17`: **`.4` measurement DONE** (read-only, docs-only) — bar #5/#6 re-assessment on the broader
  78-doc corpus (after `CORPUS-COVERAGE.0` doubled it 36→78). Faithfully replicated the three
  `from_intent_ir` `(rule)` filters: 160 undeclared-named-subject drops are field content + prose/VLM noise,
  not wire intent (0 on all 4 wire docs); the adapter's silent skip is correct; bar #5/#6 HOLDS (confirms +
  extends `.2`/`.2b`). Spun out the DTI message-field-recognition gap (most actionable next), signal-
  inventory prose noise, and ATP VLM-fragment quality. Report
  `docs/research/behavior-temporal-lowering-completeness-78doc.md`; KM
  `behavior-temporal-lowering-broader-corpus`.

- `2026-06-17`: **`.3` measurement DONE** (read-only; owner-directed substantive push after the owner
  pushed back on the "buildable frontier exhausted" framing). Bar #2 relation-completeness: the 0-relation
  docs are NOT a relation-extraction gap — (A) STALE intent_ir recoverable by a deterministic
  `semantic`→`intent` rebuild (PROVEN: `tilelink_1_7_1` relations 0→39, 40/40 actors connected; also
  tilelink_1_8_0/i2c/wbspec + a broader stale set), and (B) HONEST ABSENCE on register/command/coherency
  docs (nvme/iommu/vt-d/ccix declare ~0 wire signals; intent lives in register/message-field/transaction
  surfaces; forcing relations = fabrication). Frontier → corpus refresh (#2) + a generic stage-staleness
  `validate` detector. Report `docs/research/relation-completeness-measurement.md`; KM
  `relation-completeness-staleness-vs-absence`.
- `2026-06-17`: **`.1b.ii` DONE** — named-interface consolidation LANDED, completing the `.1b` umbrella
  (`.1b.i`/`.1b.ii`/`.1b.iii`/`.1b.iv` all done → with `.1a`, agent-surface fidelity `.1` is fully built).
  New `strip_interface_suffix` + `consolidate_interface_actor_relations` post-pass in
  `actor_signal_relation_surface` (`ir/evidence.rs`), after the `.1b.iii` split and before dedup, rewrites an
  `"X interface"` relation subject to the bare `"X"` ONLY when `X` is independently a connected relation
  subject in this doc — so `Subordinate interface`→`Subordinate` / `Transmitter interface`→`Transmitter`
  fold onto the real agent while the GIC `CPU interface` named block (whose `CPU` is never an agent on its
  own) is preserved. Per-doc connected-agent gate, no name list (ADR 0006) — the same token can be SAFE in
  one doc and a RISK in another (CoreSight `AXI interface`). Census: 6 docs carry an `"* interface"` actor
  (3 safe / 5 risk); none of the 4 WIRE-BASED-100 gold docs do, so the gold relation surface is structurally
  untouched. Live (fresh post-`.1a`/`.1b` rebuild): AXI-Stream `Transmitter interface`→`Transmitter` (23
  rels), AXI+ACE `Subordinate interface`→`Subordinate` (49 rels), 4 wire docs unchanged. WIRE-BASED-100 HELD
  1.000 (constraints + relations + temporal, APB/AHB/AXI); `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1657,
  +3 tests; conflation guard locked by the `CPU interface` unit test — the reclaimed CoreSight / missing GIC
  RISK docs are not live-rebuildable). Book `pipeline/intentir.md` "How the actor surface stays faithful";
  KM card `[[agent-interface-block-consolidation]]`; report §9.
- `2026-06-17`: **`.1b.iv` DONE** — Class-C PURE-INFERRED phantom drop LANDED (measurement-first; the
  substantive remaining buildable agent-surface lever after `.2b` measured-marginal). New
  `is_pure_inferred_phantom_role` + a guard in `build_intent_actors` (`ir/intent.rs`) skips an actor whose
  `responsibilities` is exactly the single SemanticIR Phase-2 term-scan marker
  ``"semantic role inferred around `X` evidence"`` — pure generic-vocabulary noise (0 ports, 0 relations,
  no phase/contract grounding), keyed on the marker SHAPE not a name list (ADR 0006). Measured on a fresh
  post-`.1a`/`.1b` rebuild of the 4 wire docs into a temp evidence-root (WRITE-PATH GOTCHA): drops exactly
  the 21 corpus-wide phantoms / 16 docs (wire: APB `controller`, AHB `agent`) with ZERO connected or
  grounded actors touched; genuinely-discussed-but-unwired agents (AXI `transmitter`, SWD `host`, GIC
  `arbiter`) deliberately KEPT (completeness over pruning). Stage-diff: evidence + semantic byte-identical,
  intent differs only by the dropped phantoms (+ the phantom id leaving global behaviors' `actor_ids`);
  `actor_signal_relations`/`signal_constraints`/`temporal_rules` byte-identical; `.isf` byte-identical
  (emitter never reads `actors[]`). WIRE-BASED-100 HELD 1.000 (constraints + relations + temporal,
  APB/AHB/AXI, fresh eval); `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1654, +2 tests). Book
  `pipeline/intentir.md` "How the actor surface stays faithful". KM card
  `[[agent-pure-inferred-phantom-drop]]`; report §8. The Class-C taxonomy is now closed for the
  unambiguous phantom subset; the PROSE-GROUNDED 0/0 set stays honestly kept. Frontier → `.1b.ii` /
  `.2a` direction (both deferred-with-trigger) / `KG-ISF-TRANSACTIONS` body-emission (FSMGen-parked).
- `2026-06-16`: **`.0` DONE** — tree created; owns the owner's `2026-06-16` north star (complete KG/
  IntentIR → faithful ISF; extraction serves ISF-fidelity; reverses "defer ISF"). Defined the checkable
  6-point ISF-complete-IntentIR bar; recorded the measured 2-defect agent baseline (precision noise +
  real-agent relation-incompleteness; the naive orphan filter disproven — junk agents carry ports).
  Frontier → `.1` (agent-surface fidelity, measurement-first). Memory `project_kg_isf_completeness`;
  reverses the `project_pdf_variant_digestion` "defer ISF" note.
- `2026-06-16`: **`.1` MEASUREMENT DONE** (read-only, docs-only — measurement-first before any code).
  Full per-actor port/relation census over the 4 wire docs + AXI-Stream, each noise actor's relations
  resolved to source text. Result: the agent surface has THREE structural defect classes (A Junk /
  B Fragment-of-real-agent / C Zero-evidence role-term). **Corrected the `.0` hypothesis with evidence:**
  the "disconnected real agents are a recoverable relation gap" claim is DISPROVEN — Class-C terms are a
  drive/read subject next to a known signal ≈0 times, so connecting them would be fabrication; the real
  completeness win is Class-B consolidation, and Class-C is dropped per-doc. **Genericity guardrail
  proven** (AXI `transmitter` 0/0 vs AXI-Stream `Transmitter` 22/23 → per-doc evidence-keyed, no name
  list). Spun the code into `.1a` (precision structural gate) + `.1b` (consolidation + zero-evidence
  honesty), each measurement-first + WIRE-BASED-100-gated. Report `docs/research/agent-surface-fidelity-measurement.md`;
  KM card `agent-surface-defect-taxonomy`. No code.
- `2026-06-16`: **`.1a` DONE** — precision structural agent-identity gate LANDED (first code in this
  tree). New `is_non_actor_phrase_fragment` in `ir/evidence.rs` rejects a relation-subject candidate
  whose FIRST content token is a universal function word (`NON_ACTOR_LEADING_FUNCTION_WORDS`) or a
  leading verb (`NON_ACTOR_LEADING_VERBS`) — ADR-0006-safe grammar, NOT a chip-name list — wired into
  the DRY seam `normalize_relation_actor_name` so all three relation paths gate at once. Measurement-first
  over all 36 IntentIR docs corrected the candidate lists twice (drop `i`/`its` for the `'I'`/GIC `ITS`
  collisions; exclude articles/determiners — `All Managers` is a `.1b` strip), landing on ZERO ≥8-port
  rejects / 9 targets caught / 63 non-agent rejects. Live AXI rebuild: actors 24→21, junk dropped, real
  agents byte-for-byte preserved (`Manager` 169 ports, `Subordinate` 168, `interconnect` 5),
  descriptor-noun class deferred per design. WIRE-BASED-100 held at 1.000 (constraints APB/AHB/AXI,
  relations ×4 docs, temporal ×3) on fresh-Pattern eval; `kg-bench` 156/156; `run_ci.sh` green (+3 tests,
  lib 1633 pass/2 ignored). KM card `agent-identity-structural-gate`. Frontier → `.1b`.
- `2026-06-16`: **`.1b` MEASUREMENT DONE + split into sub-leaves** (read-only over the persisted 36-doc
  IntentIR corpus, before any code). Confirmed the persisted corpus is the PRE-`.1a` baseline (the `.1a`
  rebuild went to a temp evidence-root, not canonical — the WRITE-PATH GOTCHA). Modelled each `.1b`
  transform corpus-wide: (i) the trailing verb/adverb strip is grammatically unambiguous and the clean
  win; the `"X interface"→"X"` half carries a named-block conflation risk (GIC `CPU interface` ≠ `CPU`)
  → `.1b.ii`; (iii) coordinated "X and Y" → `.1b.iii`; the Class-C 0/0 drop is 320 actors corpus-wide
  (only 21 PURE-INFERRED vs 223 PROSE-GROUNDED + 76 SECTION+INFERRED) and validated only on wire docs
  → `.1b.iv`, deferred until a defensible provenance re-check is designed. Report
  `docs/research/agent-surface-fidelity-measurement.md` §7. Frontier → `.1b.i`. No code.
- `2026-06-16`: **`.1b.i` DONE** — Class-B trailing-fragment consolidation LANDED. New
  `consolidate_trailing_fragment` in `ir/evidence.rs` strips a trailing `NON_ACTOR_LEADING_VERBS` token or
  a `NON_ACTOR_TRAILING_DISCOURSE_MARKERS` adverb (the latter a new const, drift-guarded ⊆ the leading
  function-word lexicon), keeping the leading agent noun, wired into the DRY seam
  `normalize_relation_actor_name` BEFORE the `.1a` reject — so `Subordinate extends`/`Subordinate then`→
  `Subordinate` and `decoder also`→`decoder` re-attribute their stranded relations onto the real agent
  (dedup merge) instead of surviving as phantom fragment actors. ADR-0006-safe (parts of speech, no name
  list); conjunctions (coordination, `.1b.iii`) and `"X interface"` (`.1b.ii`) deliberately excluded;
  byte-identical when nothing strips. Live AHB: `Subordinate` 25/23→27/26, `decoder` 0/0→4/2, actors
  25→19; AXI real agents byte-identical, actors 21. WIRE-BASED-100 HELD 1.000 (constraints ×3 / relations
  ×4 / temporal ×3) on fresh-Pattern eval; `kg-bench` 156/156; `run_ci.sh` GREEN (+2 tests, lib 1635
  pass/2 ignored). KM card `agent-trailing-fragment-consolidation`. Frontier → `.1b.ii`/`.1b.iii`/`.1b.iv`.
- `2026-06-17`: **`.2b` MEASURED-MARGINAL → deferred** (read-only, docs-only). Assessed the lowering-
  coverage gauge's value before building: `.2` showed the lowering is already faithful for grounded intent,
  so a coverage gauge is noise-dominated (a "257/16 463 temporal_invariants lowered" headline is dominated
  by 16 179 ungroundable ToC-heading invariants — honest absences, not a lowering gap; the actionable
  undeclared-but-grounded signal is ≈0 on wire docs / ~151 corpus register-field paths). Not worth a code
  slice. **Frontier → `.1b.iv`** (Class-C zero-evidence actor drop), now the substantive remaining
  buildable lever: a read-only count put zero-evidence (0 ports AND 0 relations) actors at **34% of all
  actors corpus-wide** (331/969; AXI 12/24, AHB 7/25, SWD 8/23, APB 3/10 on the pre-`.1a` persisted corpus
  — `agent`/`channel`/`consumer`/`producer`/`controller`/`device`/…). `.1b.iv` stays measurement-first +
  gate-risky (needs the PURE-INFERRED-vs-PROSE-GROUNDED provenance discriminator designed on freshly-rebuilt
  post-`.1a`/`.1b` evidence; WIRE-BASED-100 + the owner's completeness intent are hard gates) — a careful
  slice best done with fresh-session sharpness.
- `2026-06-17`: **`.2a.i` DONE** — signal width fidelity LANDED. The `.2a` FSMGen-contract trigger ran
  first (empirical binary probe + book contract, KM `fsmgen-ignores-signal-direction`): **direction is
  FSMGen-neutral** (flipping a driven output→input still passes `--strict`) → direction stays deferred
  (owner-philosophy only); **width** must resolve to a positive integer → emit the concrete grounded width.
  `actor_port_concrete_widths` (`ir/isf_ir.rs`) recovers each signal's single unambiguous concrete graph
  width (`actor_ports[].width_hint`) and the signal lowering prefers it over the width-1 default; never
  overrides a real >1 hint, never guesses on conflict (0 conflicts measured); ADR-0006, no name list; +1
  test. Live: AXI 32 + AHB 2 signals gain real widths (`ARSIZE→3`/`HSIZE→3`/…), non-signal lines 0,
  APB/SWD byte-identical, **0 new FSMGen `--strict` diagnostics** (AXI/AHB keep byte-identical pre-existing
  rule diagnostics), `run_ci.sh` green (lib 1652). WIRE-BASED-100 structurally unaffected (emitter-only).
  Report §6; KM cards `fsmgen-ignores-signal-direction` + `isf-lowering-fidelity-gauge`; book
  `pipeline/isf-adapter.md`.
- `2026-06-17`: **`.2` MEASUREMENT DONE** (read-only, docs-only — the `.2+` umbrella resolved into a
  concrete measurement leaf + two spun sub-leaves). Gauged which ISF-fidelity bar dimension has the largest
  faithful-lowering gap by reading the full ISF lowering and replicating its per-element filters over all 36
  IntentIR docs. **Verdict:** bar #6 (round-trip) is already honest for grounded wire intent — the 22k
  `behaviors`/`constraints` are free-text twins already lowered typed; the 17 425 typed-rule "drops" are
  un-groundable free-text (16 179 empty-subject temporal_invariants = ToC headings) / no-consequent prose /
  non-wire register-field paths (0 undeclared drops on all 4 wire docs), so mass-residualizing would be
  dishonest noise. The largest TRUE infidelity is **signal direction/width** (~98% defaulted output/width-1,
  ignoring the actor-relative graph) — but it is design-gated (relationship-relative + single flat module +
  the deliberate `R6-ISF-ADAPTER.4` default + an owner decision). Spun `.2a` (direction/width fidelity,
  deferred-with-trigger + owner steer) + `.2b` (lowering-coverage visibility gauge, buildable, additive).
  Report `docs/research/isf-lowering-fidelity-measurement.md`; KM card `isf-lowering-fidelity-gauge`. No
  code; WIRE-BASED-100 untouched.
- `2026-06-16`: **`.1b.iii` DONE** — coordinated-subject split LANDED. `split_coordinated_actor_subject` +
  `split_coordinated_actor_relations` (+ `relation_actor_id_slug`) in `ir/evidence.rs`, a post-pass in
  `actor_signal_relation_surface` before dedup: a relation whose subject is a coordinated "X and Y"
  ("*the Subordinate and decoder read HADDR*") is replaced by one relation per conjunct, so BOTH genuine
  agents connect to the signal they both act on. Splits on `and` ONLY (conjunction = both); `or` excluded
  (disjunction = ambiguous → fabrication); each conjunct re-validated through the full agent gate; fires
  only when ≥2 survive. Measured: 2 coordinated subjects corpus-wide (both AHB), both conjuncts real. Live
  AHB: `decoder` 4/2→8/6, `Subordinate` 27/26→32/31, `Exclusive Access Monitor` 4/2→5/3, fragments gone,
  actors 19→17. WIRE-BASED-100 safe — the AHB relation gold is 6 `drives` facts on unrelated statements,
  and the scorer doesn't penalize off-gold relations (AXI 343 rels / fp=0). WIRE-BASED-100 HELD 1.000;
  `kg-bench` 156/156; `run_ci.sh` GREEN (+2 tests, lib 1637 pass/2 ignored). KM card
  `agent-coordinated-subject-split`. Frontier → `.1b.ii`/`.1b.iv` (deferred-with-trigger) / `.2+`.
<!-- kg-isf-completeness-task-source-region:changelog:end -->
