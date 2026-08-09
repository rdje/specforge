# PDF-VARIANT-DIGESTION — activities 10a–10c table shapes

- Part ID: `activity-10a-10c-table-shapes`
- State: `legacy`

<!-- active-task-source-region:activity-10a-10c:start -->
structural probe, run `2026-06-10`).** · Status: `in_progress` (`.10p` probe DONE; build
sub-leaves pending, probe-first per family). The corpus-wide census over all 77 persisted
SourceIRs: **7,012 `unknown`-kind structured tables**, and the top normalized header
signatures are overwhelmingly REGISTER-FIELD variants (~2,000 tables) — all structural
header vocabulary, zero chip names (ADR 0006). Crossing each doc's register-shaped
unknown-table count against its extracted register yield separates healthy docs (extraction
works without the kind label: CoreSight-0701 598→833 regs/2,978 fields; NVMe 199→42/201;
VT-d 152→103/318; RISC-V Debug 59→44/179) from the REAL gaps:
  | doc | reg-shaped unknown tables | extracted regs/fields | dominant unexplained signature |
  | --- | --- | --- | --- |
  | AMD IOMMU 48882 | 167 | 11/**0** | `bits \| description` (+287-count family ×3 docs) |
  | CCIX ×4 versions | ~151 each | 8/11 each | `bit location \| register description \| attributes` (481 corpus-wide) + `byte location \| size \| register description \| attributes \| m/o` |
  | CoreSight TMC ddi0461 | 56 | 2/50 | `bits \| name \| description` family |
  | GIC-600 TRM | 72 | 15/214 | `bits \| name \| function` (129 corpus-wide, 5 docs) |
  | MMU-700 TRM | 68 | 13/91 | `bits \| name \| description` (68) |
  | CHI C2C (ihi0098) | 84 | 76/315 | `bits \| field \| description \| access type \| reset` (65) — partial |
  | CHI G (ihi0050) | 28 | **0/0** | degraded ingest (host-local re-provide blocks full fix) |
  Build sub-leaves (each probe-first on ITS family, structural only, no lists/case,
  wire-docs + register golds stay green): `.10a` the `bit location`-keyed register/field
  vocabulary (CCIX family, ~600 tables across 4 docs — biggest single unlock); `.10b` the
  two-column `bits \| description` shape (AMD IOMMU + relatives, ~290); `.10c` the
  `name \| function` column synonym (GIC-600 +4 docs, ~129); `.10d` the offset-suffixed
  dword-relative bit-cell family (`31:28 +04`; 41 AMD tables / 184 rows — spun from the
  `.10b` quantified residuals); `.10e` the `byte location \| size` placement-table family
  (CCIX ×4); `.10f` the section-HEADING prose message-field family (DTI message protocols —
  the field layout is in `<NAME>, bits [hi:lo]` headings, not tables); `.10g` (in_progress)
  the section-heading REGISTER-field form (GIC/SMMU/CoreSight/ACC/ARM-Debug → the register
  surface; the register-routed twin of `.10f` via the SAME shared container-walk, plus a
  per-document name-uniqueness residual gate so a short mnemonic reused across access-port
  blocks is never over-counted or conflated).
  **`.10b` first probe pass (`2026-06-10`): the family is AMD IOMMU 160 + NVMe 130 + eMMC 1 —
  NVMe is a GOLD-measured doc, so any gate change hits its measured surface; AND the rows
  describe in-memory STRUCTURE entries (AMD Device Table Entry "Field Definitions"
  page-fragments with caption-less continuations; NVMe queue/data structures), NOT MMIO
  registers — `.10b` must first decide the typed home (register surface vs a structure
  surface) and design a gate WITHOUT access/reset vocabulary (two columns have none;
  candidate anchors: the "Field Definitions" caption family, all-rows-bit-range column
  shape, fragment continuation merging). Heavier design slice than `.10a` — full
  probe-first treatment in a fresh leaf, golds re-measured.** Honest boundary: a shape is
  recovered only when its rows actually parse as bit-range + field semantics — otherwise it
  stays an explicit residual (no fabrication).

**`PDF-VARIANT-DIGESTION.10a` — `bit location` register-field vocabulary (CCIX family).**
· Status: **DONE `2026-06-10`** (probe → measured design → build → live per-item verification).
**Verification log:** lib tests 1546→1553 (7 new hermetic: gate vocabulary, identifier-shape,
paren+frame, all four leading-identifier bleed gates, caption-offset grammar, full
synthesize integration incl. the byte-location negative twin); kg-bench **154/154**; full
`scripts/run_ci.sh` GREEN. **Old-vs-new dry-run parity sweep over ALL 12 intact-bundle
docs (NVMe, RISC-V Debug, APB/AHB/AXI/AXI-Stream, SWD/ADI, I2C, SMBus, I2S, CAN, SWP):
byte-IDENTICAL** — zero drift on every gold-measured doc, promoted canonical surfaces
untouched. **Live run on the real persisted CCIX/CoreSight tables** (stub-markdown /tmp
copies; canonical untouched — their normalized bundles are host-local-blocked like
`.6`/`.7`): CCIX rev2.0 **8 regs/11 fields → 143 regs/389 fields** (259 named + 130 honest
bit-range residuals, 40 registers with recovered byte offsets; e.g. `CCIX PL DVSEC Header
at Byte Offset 04h` = CCID 15:0 RO / DVSECRevID 19:16 RO / DVSECLength 31:20 RO);
CoreSight 0100/0200 upgrade EXACTLY the 13 predicted fields each (`ATDATA127`, `AFVALID`,
`ID0_20_2F`…, bit positions preserved) and the predicted single residual mis-name exists
exactly once corpus-wide (`register_table_0149`, page-wrap bleed). Canonical CCIX/CoreSight
EvidenceIRs refresh whenever those PDFs are re-provided and re-ingested. Book:
`pipeline/evidenceir.md` subsection; KM card `bit-location-register-field-vocabulary`.
**Family probe over the persisted corpus:** 542 `bit location`-headed unknown tables exist
ONLY in the 4 CCIX SourceIRs (0 outside — gate extension corpus-safe); 1,432 data rows;
98.3% of bit cells parse as bit ranges. The register identity lives in the caption
(`<Name> Register fields at Byte Offset 04h` / `<Name> Register at Byte Offset-0Ch`); the
FIELD NAME is fused into the description cell as its leading identifier (`CCID This field
indicates …`), beside `Reserved …` rows (395) and honest residual rows (`See Table 7-1 …`,
wrapped continuation bleed). Why today yields 8 regs/11 fields per doc: the
`is_register_field_header` gate misses (`bit location` ≠ `bit`/`bits`, no `field`/`name`
cell), and in the `field description` variants the description column itself is taken as
the name column, so sentence-length "names" die at the >4-words gate.
**Measured design (every gate demonstrated per-item on the corpus):**
  1. gate + bits-column vocabulary unified: a bit-POSITION header (`bits`/`bit`/`bit
     range`/`position`/`bit location`) counts as the field/bits column evidence
     (probe P2: the full unified vocabulary newly admits ONLY the 542 CCIX tables);
  2. a name-ish header that also contains `description` is a DESCRIPTION column, never the
     field-name column (probe P1: only the 52 CCIX `field description` tables change);
  3. mnemonic recovery on the existing bit-range-name path gains two grammar forms after
     the untouched `(MNEMONIC):` form: **paren+frame** `Full Name (Ident) This
     field/bit/value/register …` (9/doc, recovers mixed-case `SevNocomm`/`LogLen` class) and
     **leading-identifier** `<Ident> This field …` gated by: identifier-shaped token (not a
     plain Titlecase/lowercase English word — `See`/`Indicates`/`Error` rejected), ≠ the
     row's own access cell (`RO Reserved bit …` bleed), remainder not `Reserved…`-led,
     unique among the table's leading tokens (kills ADI `ASCII Identity code` ×3), and no
     LATER mid-cell defined-term `<Ident> This field` (kills `… (except FLR).
     LinkCreditSendEnable This field …` continuation bleed);
  4. caption locator `… at Byte Offset <tok>` → `offset_address` (verbatim token; `from …
     through …` ranges stay None — never collapsed to a guessed point).
**Measured outcome:** CCIX ×4: 9 form1 + 208–249 form2 mnemonics/doc (vs 11 fields TOTAL
today); CoreSight 0100/0200 TRMs: +13 genuine each (`ATDATA127`, `ID0_20_2F`, `ATREADYS` —
all eyeballed); NVMe (63 residual rows), RISC-V, ADI, SWP, I2C: UNCHANGED by construction
(probe-proven 0 new accepts). Known residuals (honest, quantified): ONE wrong-name row
corpus-wide (rev2.0 `table_0149` — `CCIX` from HAQREQ continuation bleed with no
defined-term marker; locally indistinguishable from a genuine row, documented not gamed);
the `attibutes` typo family (4 tables) stays residual (typo vocabulary is a list — not
added); `byte location \| size \| register description` tables (~60) are register-AT-OFFSET
structures, NOT bit fields — treating byte offsets as bit ranges would fabricate, so they
stay residual (future lever: map them to register-map records). Side effect noted:
`audit-extraction` uses the same gate for its sampling labels — classification labels
shift on CCIX-class tables only.

**`PDF-VARIANT-DIGESTION.10b` — the two-column `bits | description` STRUCTURE-field family.**
· Status: **DONE `2026-06-10`** (probe → typed-home decision → measured design → build →
live per-item verification).
**Verification log:** lib tests 1553→1557 (4 new hermetic: strict cell parser incl. the
`318:32` fabrication case; container-label grammar incl. the ref-must-carry-a-digit guard;
full surface integration with the DTE-shaped capless-head chain, fresh-structure
non-adoption, 3-cell value-row skip, Reserved skip, offset/symbolic whole-table rejection,
kind-gate exclusion, cross-strategy id density; the sense-guard + label-disagreement chain
fixture); kg-bench **154/154**; full `scripts/run_ci.sh` GREEN. **Old-vs-new dry-run parity
over ALL 12 intact-bundle docs: every extraction surface byte-identical; the ONLY delta
anywhere is `extraction_manifest` gaining exactly the new registered-extractor entry**
(by design — the framework records eligible entries for all registered extractors; the
manifest IS the behavioral fingerprint), **plus NVMe's additive `message_field_records`
0 → 216**. NVMe register gold re-measured before/after IDENTICAL (0.966/0.966 register-field
views, 42/42 named, 201/201 with bit positions — the documented gold-authoring state); full
battery green (constraints 1.000×3, filtered relations 1.000×4, temporal 3/3+4/4+3/3,
SWD/I2C/RISC-V at documented states). **Live: NVMe 216 fields / 113 containers (`CID` at
`31:16` of `Command Dword 0` …); AMD (stub-copy protocol, canonical untouched) 82 / 15 with
the stitched DTE carrying 31 fields (`Mode0FC` 247 → `IV` 128 → … → `V` 0 across 9
fragments); USB 3.2 / USB4 / TMC ddi0461 / eMMC = 0 records each (measured — their
value-encoding rows recover no names, correctly).** Canonical NVMe evidence deliberately
NOT rebuilt this slice: a rebuild drops its standing Pattern gauge (the owner-visible
`LLM-PRIMARY-PROMOTION.4` sweep state) and re-measurement needs the live NLI provider — the
additive refresh rides the next gauge-bearing sweep. Book: `pipeline/evidenceir.md`
subsection (+ a drift-guard clause in the FIELD.2 subsection: its zero-yield claim is
scoped to the field-titled reader); KM `bit-position-structure-field-extraction`.
Follow-on levers recorded: the 41 offset-suffixed AMD tables (`31:28 +04`, dword-relative —
an offset-aware future leaf) and NVMe `table_0447` (one symbolic row).
**Family census** (measured with the `.10a` unified bit-position header vocabulary; effective
2-column header = bit-position word + a `description`-containing word, consecutive `col_span`
duplicates collapsed): **334 `unknown`-kind tables across 6 docs** — AMD IOMMU 163, NVMe 156
(a GOLD-measured doc), USB 3.2 ×10 (plus 4 already `signal_description`-classified, untouched
by kind-gate), USB4 inter-domain ×3, CoreSight TMC ddi0461 ×1, eMMC ×1. Wire-based docs carry
ZERO family tables (byte-identity by construction).
**TYPED-HOME DECISION (the leaf's first question): `message_field_records`** — the existing
"structured content fields, NOT wires" surface — via a NEW second extractor strategy
`message_fields.bit_position_table`; **NOT the register surface**. Why: the rows describe
in-memory STRUCTURE entries (AMD Device Table Entry, NVMe command dwords / queue entries /
data structures), the family carries no access/reset vocabulary anywhere (claiming MMIO
register semantics would fabricate them), and the register surface — including NVMe's measured
register gold — stays byte-identical by construction. The message-field module doc already
reserves "field shapes without a field-name column … for later strategies"; this is that
strategy. `MessageFieldRecord` gains an additive `bit_range: Option<(u32, u32)>`
(serde-skipped; `SerialFrameField` precedent) — old artifacts load unchanged and the existing
field-titled strategy emits `None`, so CHI-class docs stay byte-identical.
**Measured gates (each demonstrated per-item over the persisted family):**
  1. table gate: effective table kind `unknown`, effective 2-column header (bit-position
     vocabulary + `description`-containing), ≥1 eligible row, and **ALL eligible rows'
     bit cells strict-parse** as pure bit positions (`255:248`, `247`, `[7:4]` — a NEW strict
     cell parser; the lenient `parse_bit_range` would mis-read NVMe's symbolic
     `31 + (Element Length*8) :32` as `318:32`). Eligible row = exactly 2 effective cells —
     NVMe's 196 three-cell value-encoding sub-rows and 2 footnote rows self-exclude.
     Measured: AMD 122/163 tables pass (the 41 failures are the offset-suffixed family),
     NVMe 155/156 (the 1 failure is `table_0447`, one symbolic row — its 5 good rows stay
     honest residuals), and every small-doc table passes-but-yields-0-names (below);
  2. container from the caption: strip a leading `Table|Figure <ref> [:.-–]` label, a trailing
     `(Continued)` marker, then a trailing `Field Definitions` / `Field Descriptions` /
     `Fields` suffix (`Device Table Entry (DTE) Field Definitions (Continued)` → `Device Table
     Entry (DTE)`; mid-label `Fields, PR=0` qualifiers are kept — `PR=0` vs `PR=1` are distinct
     containers). NO structure-noun requirement: the probe showed caption nouns mislead
     (`Reservation Register - Command Dword 10` is a COMMAND name, not an MMIO register) and
     the SHAPE gate + name grammar already carry the precision (structural over lists);
  3. caption-less fragment chains: a capless family table joins the preceding chain ONLY on
     **bit-exact adjacency** (descending `first_hi == prev_last_lo - 1`, or the ascending
     mirror) with page distance ≤ 1 and caption-label agreement; chain container = the
     captioned members' shared label; **a chain with no captioned member yields nothing**.
     Measured: the AMD DTE chain alternates capless/`(Continued)` fragments with EXACT
     adjacency at every hop (255:248→…→208 | 207→…→185 | … → 1→0; the chain HEAD `table_0025`
     is itself capless, so chains must form by adjacency first, container second); corpus-wide
     23 AMD capless fragments adopt exactly, 80 do NOT (fresh 31:0/63:0 structures restart at
     a width boundary — zero ambiguous "below-but-gap" cases), NVMe 1 adopts (`table_0367`) /
     7 stay residual, USB ascending splinters adopt-then-yield-0-names;
  4. field names REUSE the `.10a` mnemonic chain `recover_field_mnemonic` verbatim (access
     cell = None): NVMe rows carry the untouched `Full Name (MNEMONIC):` form (215 named:
     `CID`, `SQID`, `PSDT`, `PRP1`…), AMD rows carry the gated leading-identifier form
     (313 named: `Mode0FC`, `vImuEn`, `GuestPagingMode`, `SysMgt`, `IntCtl`…) — zero new
     name grammar needed, zero suspicious accepts in the per-item eyeball; `Reserved…`-led
     rows are SKIPPED (padding is not a named field; 156 AMD + 98 NVMe), and a row with no
     recoverable name yields no record (honest absence).
**Quantified honest residuals (documented, not gamed):** the dword-relative offset-suffixed
cells (`31:28 +04`; 184 rows / 41 AMD tables — capturing the bit range while dropping the
`+04` byte offset would MISREPRESENT absolute position; a future offset-aware lever);
capless chains with no captioned member (80 AMD + 7 NVMe lost-caption tables); name-less
parsed rows (140 AMD / 58 NVMe — including the `GDeviceID[15:0]:` bracket-slice leading form
and the `GCR3 …` repeated-leading-token amphiboly the `.10a` uniqueness gate rightly rejects);
NVMe `table_0447`'s 5 good rows (one symbolic sibling row fails the all-rows gate); the
small-doc value-encoding tables (eMMC power-class codes, USB suspend options, TMC
scatter-gather) pass the shape gate but recover 0 names → 0 records, correctly.
**Build-safety facts (verified in code before building):** `message_field_records` is consumed
only by LLM-gated opt-in paths (`extract-constraints-llm` entity typing) and kg-bench — the
deterministic Pattern `signal_constraints` surface never reads it; `validate` has zero
`message_field` consumption, so `document_class` / completeness gauges are unchanged;
registering extractor #2 on the surface adds a manifest `ExtractorRunEntry` on EVERY doc, so
the parity claim is: all extraction surfaces byte-identical on the 11 other intact docs with
ONLY the manifest gaining the new entry; NVMe additionally gains additive
`message_field_records`/`bit_range` content. NVMe register-gold eval re-measured before/after.

**`PDF-VARIANT-DIGESTION.10c` — the `bits | name | function/description` three-column family.**
· Status: **DONE `2026-06-10`** (probe → bucket design → unified-collector build → live
per-item verification; the heading lever probe-REJECTED).
**Verification log:** lib tests 1557→1560 (3 new: caption register-name grammar incl. the
`<n>`/space-`n` array forms and the Titlecase/`Reservation Register` protection; the strict
footnote-letter parser; the full chains-route integration — caption-named register chain
with capless fragment + footnote cell + `-`→bit-range residual + Reserved kept +
`(continued)` `.4c` merge + structure-side colon-form names + register-worded-unnamed
residual); kg-bench **154/154**; full `scripts/run_ci.sh` GREEN. **Parity: ALL 12 intact
docs manifest-only delta (the `registers.bit_assignment_table` entry); NVMe's 216 message
fields BYTE-IDENTICAL through the unified-collector refactor** (the `.10b` lock tests +
sweep prove the 2-col path unmoved); golds at documented states (constraints 1.000×3,
filtered relations 1.000×4, temporal 3/3+4/4+3/3, NVMe 0.966/0.966 + 42/42 + 201/201,
RISC-V 0.588/0.000 authoring state, I2C 6/6). **Live (stub protocol, canonical untouched):
GIC-600 15→33 regs (+18/79 fields: `GICD_CTLR`, `GICD_TYPER`, array `GICD_CHIPR<n>`…);
MMU-700 13→63 (+50/180: `TCU_CTRL`…, space-`n` arrays `TCU_NODE_CTRL n`); TMC ddi0461 2→30
(+28/87: `RSZ`, `RRP`, `CTL`…); SDC-600 0→5 (+5/25); GIC-400 +1 (`GICC_IIDR`); CHI-C2C-b +5
property registers (`C2C_Prop*Tx1` 22 fields via a chained `Continued from previous page`
fragment). Structure side: VT-d +15 message fields (`Root-Entry Format` `CTP`/`P`,
Scalable-mode entries — colon-form name cells `CTP: Context-table Pointer`); C2C Rx1
container 21.** Two measured caption gates added during live verification (per-item, then
fixture-locked): a label ending with a sentence period is caption BLEED (2 of 314 family
captions, both GIC-400 prose) and a `Continued from previous page` label is no label at
all (1 of 314 — the fragment then chains bit-exactly to its true home, which is HOW
`C2C_Prop*Tx1` got its 22 fields); a register-WORDED caption that grounds no identifier is
register-shaped-but-unnamed → residual on BOTH surfaces (the MMU-700 space-`n` leak found
live and killed, then the space-`n` grammar recovered those 2 registers properly).
**Honest residuals (quantified):** 68+ capless chains with no labeled member (lost-caption
register tables — the re-ingest lever); 15 same-heading-but-bit-OVERLAPPING capless tables
(why the heading lever was rejected); `RES0`-named fields kept (the document's own declared
name); `C2C_Prop*Rx1 field positions` lands as a structure container because its caption —
unlike its `B15.76` sibling — omits the word register (evidence-honest; re-provided PDFs
may improve captions). GOTCHA recorded: `cargo test` does NOT rebuild the bin — measure
live runs only with a freshly `cargo build`-copied binary (one stale-binary misread caught
same-session).
**Census** (effective 3-column `unknown`-kind header: bit-position vocabulary + name-ish
(`name`/`field`/`field name`/`bit name`) + `function`/`description`/`meaning`/`notes`, NO
access/reset anywhere): **270 tables across 11 docs** — GIC-600 71, MMU-700 66, CoreSight
TMC ddi0461 55, Intel VT-d 39, SDC-600 16, CHI-C2C ihi0098_b 8, SMBus 6, GIC-400 5, GIC
arch 2, SMMU arch 1, OpenCAPI 1. Signatures: `bits|name|function` 145, `bits|name|
description` 68, `bits|field|description` 48, rest singletons. These tables fail BOTH
current gates by design: `is_register_field_header` needs access/reset; the field-titled
message reader needs a container caption.
**Caption-evidence buckets (per-item over all 270):** `REGISTER-WORD` 88 (caption literally
says register: MMU-700 `TCU_CTRL register bit descriptions` 51, TMC `RSZ Register bit
assignments` 32, C2C 5) · `BITASSIGN-REGIDENT` 28 (`GICD_CTLR bit assignments` — single
register-shaped identifier before the universal TRM locution `bit assignments`; GIC-600 21,
SDC-600 6, GIC-400 1) + 4 `BITASSIGN-OTHER` incl. the `<n>` ARRAY-register idents
(`GICD_CHIPR<n>`, `GICT_ERR<n>STATUS`) · `STRUCT-NOUN` 13 (VT-d `Root-Entry Format` /
`Scalable-mode Context-Entry Format` 11 — in-memory STRUCTURES, the `.10b` class, here WITH
an explicit name column) · `CAPLESS` 133 · prose-bleed/continued 3.
**TYPED-HOME DIRECTION (per-table, by the document's own caption vocabulary):** the
register-evidence buckets (REGISTER-WORD + BITASSIGN-REGIDENT(+`<n>`)) go to the REGISTER
surface with the caption as the register-name source — the same evidence class as
`register_name_from_caption`/EXTRACTION-GAP-FIX.3 (a TRM caption `X register bit
descriptions` IS a register declaration; access/reset stay honestly absent on the fields).
The STRUCT-NOUN bucket extends the `.10b` bit-position message-field strategy with an
explicit-name-column variant (VT-d's Root/Context entries are the same in-memory-structure
class as AMD's DTE). Collision check DONE: the existing register records on these TRMs are
SUMMARY-caption records (GIC-600's 15 = `… registers summary` tables; `GICD_CTLR` etc. do
NOT exist yet) — the new path CREATES registers, no double-count; same-name fragment merge
stays safe by the `.4c` distinct-fields rule.
**Chain probe (capless adoption, .10b protocol):** adjacency is NOT sufficient here —
EXACT/NOT per doc: MMU-700 12/3, VT-d 11/17, GIC-600 9/39, TMC 2/17(+2 below-gap). The many
NOT cases are lost-caption tables for DIFFERENT registers (each register's table is small),
so the `.10c` chain design needs caption-`(continued)` family anchoring AND/OR
nearest-heading anchoring (TRMs head each register section `4.3.2 GICD_CTLR…`) — probe the
heading lever per-item BEFORE building; bit-exact adjacency remains the only
caption-free joiner (never guess).
**Row/name quality (measured):** strict-parse rows GIC-600 301 (12 noparse) / MMU-700 205
(3) / TMC 156 (99: 93 EMPTY bit cells = wrapped continuation rows + 6 `[31:0] a`
footnote-letter suffix) / VT-d 290 (40). Name cells: explicit column, no fused-name grammar
needed; `-` placeholder names (89 on GIC-600) and `Reserved` rows are honest skips; a
footnote-letter micro-grammar is a measured maybe.
**Verification plan:** 12-doc parity (register surface byte-identity expected everywhere
except… NONE of the 12 intact docs carry the family — SMBus does (6 tables: 4 capless +
value-encoding shapes) so SMBus golds/timing 84 must be re-measured), stub-protocol live
runs on GIC-600/MMU-700/TMC/VT-d, all register golds + battery, kg-bench, full CI.
**BUILD DESIGN (closed by the second probe pass, `2026-06-10`):** (1) the HEADING lever is
REJECTED BY MEASUREMENT — same-heading adoption adds exactly 1 safe table corpus-wide and
would wrongly adopt 15 bit-OVERLAPPING tables (page-granular headings mis-assign when
registers share pages); the `.10b` bit-exact adjacency rule stands UNCHANGED, and captioned
`(continued)` tables need no chain at all (the existing `.4c` same-name register fragment
merge consolidates them). (2) Caption register-name grammar measured: 116 captioned tables
yield a register ident (ident-before-`register`: `TCU_CTRL`, `RSZ`; single-ident before
`bit assignments`: `GICD_CTLR`; `<n>` array idents `GICD_CHIPR<n>` via extended ident
charset); honest misses = space-`n` forms (2), prose-bleed (1), and the LPI/SMBus captions
which correctly fall through to the STRUCTURE side (no single-ident head). (3) The 101
empty-bit-cell rows are VALUE-ENCODING sub-rows (name cell holds `0`/`1` enums) — skipped,
not counted against the gate; 9 footnote-letter cells (`[31:0] a`) parse via a strict
bracketed-range+single-letter form, 3-col rows only. (4) ARCHITECTURE: a unified
`collect_bit_layout_tables` collector (2-col + 3-col family scan) + ONE chain pass + label
routing (`Register(name)` chains → a third register-surface strategy
`registers.bit_assignment_table`, one RegisterRecord per chain, access/reset honestly
absent; `Container(label)` chains → the existing `message_fields.bit_position_table`
strategy, explicit name column for 3-col rows) — corpus-proven parity-safe (2-col and 3-col
docs overlap only on TMC, whose single 2-col table yields 0 records); `bit(s)` joins the
shared bit-position header vocabulary (one matcher, `.10a` principle). The `.10b` 2-col
behavior is LOCKED through the refactor by its own tests + the NVMe-216/AMD-82 re-proof.

**`PDF-VARIANT-DIGESTION.10d` — the offset-suffixed dword-relative bit-cell family.**
<!-- active-task-source-region:activity-10a-10c:end -->

