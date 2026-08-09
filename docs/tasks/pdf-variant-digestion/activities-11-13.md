# PDF-VARIANT-DIGESTION — activities 11–13

- Part ID: `activities-11-13`
- State: `legacy`

<!-- active-task-source-region:activity-11:start -->
· Status: **DONE `2026-06-11`** (probe → measured scope decisions → build → live CLI
verification). The deferred lever
recorded since `.10b`: `validate <evidence-ir>` has ZERO `message_field` consumption —
the AMD-class 217-field / NVMe 216-field inventories (`message_field_records`, now with
`bit_range`/`byte_offset`) and the `message_field_constraints` surface are invisible to
the user-facing report, the `document_class` decision, and the per-document completeness
gauge. A structure-heavy document (command/queue/descriptor layouts) currently looks like
a low-intent doc to the classifier even when its message-field surface is rich.
**Scope (decide from the probe, bounded):** (1) report honest metrics on the evidence
report — field count, container count, `with_bit_range`, `with_byte_offset`,
`message_field_constraints` count — plus an Info finding; (2) `document_class`: decide BY
MEASUREMENT whether `message_field_records` should join the census (it is a LOW-NOISE
table-derived surface like registers — but the persisted-corpus impact must be measured
per-doc: which docs reclassify, and is each reclassification more honest? A guide must
never become `protocol` from noise; the `.5a` exclusion of `conditional_rules` is the
precedent for rejecting a surface); (3) completeness gauge: class-aware gaps for the
field surface — only dimensions the probe shows are real. CAVEAT recorded: most persisted
canonical evidence predates `.10b` (canonical NVMe deliberately NOT rebuilt — standing
Pattern gauge), so the corpus census must distinguish "surface absent because never
rebuilt" from "honest zero"; the class decision must not penalize either.
**PROBE RESULT (decisive):** ZERO of the 78 persisted evidence docs carry any
`message_field` surface (22 have manifests, i.e. were rebuilt since the framework — the
field-bearing docs simply were never rebuilt after FIELD.2/`.10b`: canonical NVMe keeps
its standing gauge, CHI/AMD bundles are host-local). The only real field-bearing data is
the AMD/NVMe dry-runs. **Measured scope decisions:** (1) metrics + Info finding = BUILD;
(2) `document_class` census = NO CHANGE — measured on the two real docs, NEITHER would
reclassify (AMD is `interface`, 64 connectivity edges; NVMe is `register`, 42 regs), and
with n=2 any new classifier arm is overfitting (`feedback_genericity_guardrail`; the
`.5a` `conditional_rules` exclusion is the precedent) — REVISIT TRIGGER recorded: when
the corpus re-ingest sweep rebuilds field-bearing docs at scale, re-measure per-doc.
**REVISIT DONE (`2026-06-14`, via `.13d`): the census decision STANDS — NO new arm.**
Five field-bearing docs now exist on canonical: AMD-IOMMU (217 fields/30 containers) →
`interface`; CCIX×4 (r1.0 92 / r1.0a 86 / rev1.1 86 / rev2.0 45 fields; 131–143 recovered
registers each) → `register` ×4. Message-field richness did NOT cluster into a new class
nor force a misclassification — each doc classifies honestly on its DOMINANT typed surface
(AMD on signals/relations, CCIX on its large register map), so a `message_field` classifier
arm would still be overfitting (n=5, splitting 1 interface / 4 register). Measured
confirmation either way, as required;
(3) completeness gauge = NO CHANGE — a width-only field table states no positions, so a
"fields without bit positions" dimension would mislabel honest document absence as an
extraction gap (fabricated expectation).
**Verification log:** lib tests 1564→1565 (1 new hermetic: the inventory metrics +
finding through the REAL pipeline from a `.10b`/`.10d`-shaped table — counts 2 fields /
1 container / 2 with bit_range / 1 with byte_offset, summary text locked; plus the
absence case extended onto the provenance test — zeros and NO finding when the surface
is empty); kg-bench **154/154**; full `scripts/run_ci.sh` GREEN. **Live CLI verification
(temp-root build, canonical untouched, post-fmt binary):** NVMe evidence rebuilt into a
/tmp root reports `message_field_records (from tables): 216 / containers: 113 /
with_bit_range: 216 / with_byte_offset: 0 / message_field_constraints: 0`, the
`[info:message_fields]` inventory finding renders with the honest counts, and the
manifest line shows `message_fields[message_fields.bit_position_table]` fired. New
validate surface: 5 metrics (`message_field_records`, `message_field_containers`,
`message_fields_with_bit_range`, `message_fields_with_byte_offset`,
`message_field_constraints`) + the `evidence_message_field_inventory` Info finding
(emitted only when the surface is non-empty — absence is not an event). KM card
`message-field-validate-integration`; book `quality/validation.md` note.

**`PDF-VARIANT-DIGESTION.12` — header-trapped SIGNAL tables (the `.9.11` lever's signal
<!-- active-task-source-region:activity-11:end -->

<!-- active-task-source-region:activity-12:start -->
sibling; AXI/ACE/APB/LTI presence matrices).** · Status: `in_progress` (census probe DONE
`2026-06-11`; **per-item probe DONE `2026-06-11` — all 4 items measured, leaf split
`.12a` build (this slice) + `.12b` presence surface (next)**). Found while
surveying the WIRE-BASED-100.3 residual ledger: APB `table_0018` ("signals trapped in its
header rows") is not an isolated docling artifact — the corpus census (signal-worded
first header row, ≥2 header rows, ≤1 body row, rows recovering under the EXACT `.9.11`
structural rule: value cells all `is_header=false`, non-empty label) finds **21 real
tables across 4 docs (+1 Wishbone TOC false-positive that self-excludes: 0 identifier-led
rows; +1 USB `timing_parameter` table that belongs to the `.9.11` timing domain):**
AXI ihi0022_l **12 tables / 306 distinct identifier tokens** (the `B2.2`/`B2.3` "Summary
of (check) signal presence for each interface class" + `A13.3` families — `AWSUBSYSID ||
SUBSYSID_WIDTH > 0 || O || N` rows: signal + presence-property + per-class Y/N/O/C);
ACE ihi0022_h_c 5 / 134 (incl. the `Signal matrix` caption); LTI ihi0089 2 / 44;
APB ihi0024_e 1 / 3 (`PWDATACHK`/`PREADYCHK`/`PBUSERCHK` — the exact `.3a` ledger
residual). 4 of the AXI tables are `unknown`-kind `Continued from previous page`
fragments of the captioned `B2.2`/`B2.3` chains. Census identifier gate is CASE-SOFT by
measurement: the strict all-caps probe undercounted by exactly `ARESETn`/`RESETn` and the
9 `Ax*` generics (19 tables/475 tokens → 21/487 at ≥60%-uppercase participation).
**Per-item probe RESULTS (`2026-06-11`, all 4 items):**
(1) **new-vs-declared** — the right keys: evidence-level inventory =
`collect_known_signal_names` over `extracted_statements` ("Signal X is …" declarations);
canonical = SemanticIR `interfaces[].signal_records[].signal_name` (the earlier probe's
`interface_signals` key does not exist — AXI canonical reads 289 names through the right
key). Diff per doc: **AXI 306/306 covered** (297 literal in the evidence inventory + 9
`Ax*` generic-name rows whose AW/AR expansions are BOTH declared, 18/18 verified;
`signal_alias_map` is EMPTY so the generics have no grounded literal expansion — A13.3
stays an honest residual until `.12b` captures its rows literally); **APB 3/3 covered**
(the `.3a` duplicate-presentation precedent CONFIRMED); **ACE 9 genuinely NEW wires**
(`AWBAR` w2, `AWDOMAIN` w2, `AWSNOOP` w4, `CRRESP` w5, `CDDATA` wV, 4×`BROADCAST*` w1 —
the matrix carries real `Width|Source|Default` declaration columns); **LTI 3 NEW**
(`LASECSID`/`LASID`/`LASSIDV`) but C5.1 carries NO width/direction columns → no honest
declaration content (their existence grounds via `.12b` presence capture).
(2) **gauge state** — 17/21 tables are flagged `unexplained_intent_bearing_tables`
candidate misses (within AXI 39 / ACE 39 / LTI 5 / APB 1); the 4 AXI `unknown`-kind
fragments are INVISIBLE to the gauge (non-accounted kind) — an honesty gap the build
closes. The gauge recomputes at VALIDATE time from persisted records + tables, so
coverage-rule fixes reach the promoted AXI/APB artifacts WITHOUT any rebuild.
(3) **typed capture (measured)** — presence-CONDITION intent is real at scale: AXI 157
condition-bearing rows / 73 distinct property expressions (`SUBSYSID_WIDTH > 0`,
`AXI_Transport == Ready`), LTI 23/15 → a NEW typed surface, split to **`.12b`**; ACE's 9
new wires mint through the EXISTING declaration content rules (width column present);
APB `table_0018` is cell-fused/rotated garble (signal names land in different columns,
cells fuse `PRDATACHK DATA_WIDTH/8`) → coverage-marking only, never typed capture from
its rows. Duplicates NEVER mint: the trapped-row declaration pass is gap-fill GATED on
"name absent from the already-built declared inventory".
(4) **chaining** — 9/9 AXI `Continued from previous page` fragments resolve to their
captioned heads by caption parent-number (`Table B2.2 …` ↔ `Table B2.2: Summary …`) +
EXACT first-header-row signature match (measured; incl. all 4 `unknown`-kind). LTI's
sub-header row (`A A.b|B|C|D`, value cells `is_header=true`) is correctly REFUSED by the
`.9.11` rule — genuine multi-row headers stay headers.
**`.12a` BUILD — DONE `2026-06-11`:** (i) **continuation-kind inheritance**
(`continuation_inherited_table_heads` in `ir/evidence.rs`): an `unknown`-kind fragment
inherits its captioned chain head's kind only when the caption-stated parent reference AND
the exact first-header-row signature both ground the join (evidence-level view only —
SourceIR is never mutated; an unclassified or signature-mismatched head inherits nothing);
consumed by the gap-fill, `unexplained_intent_bearing_tables`, and the validate
denominator (`intent_bearing_table_count`) so numerator and denominator cannot disagree.
(ii) **trapped-row-aware coverage**: the `.9.11` rule now has ONE shared definition
(`recovered_trapped_data_rows` — timing recovery refactored onto it, pure code motion)
and `densest_signal_name_column_tokens` chains it after `body_rows`, same strictness.
(iii) **gap-fill declaration synthesis** (`synthesize_trapped_row_signal_declarations`,
runs LAST in the seed so the inventory gate sees the complete declared universe): mints
under the body-row path's own content rules (direction or width — a presence-only row
states no declaration) AND only for names absent from the inventory (duplicates are
coverage-marked, never re-minted); fragments qualify through their chain head's
top-level gate. **MEASURED (final post-fmt binary):** 13/13 intact bundles old-vs-new
`evidence --dry-run` BYTE-IDENTICAL (gap-fill mints nothing on the rebuildable corpus —
exactly the probe's prediction; zero extraction-behavior change on every gold doc);
validate-time gauges on canonical artifacts: **APB 1→0 (the `table_0018` WIRE-BASED-100.3
residual CLOSES — 0/9 unexplained on the gold doc)**, AXI 39→32 of 94→98 (B2.2+B2.3
chains covered incl. the 4 newly-accounted `unknown` fragments; A13.3 stays honest),
ACE 39→36 (3 all-declared `*CHK` matrices covered; the 2 matrices carrying the 9 new
wires stay flagged until re-ingest), **LTI 5→6 — the honesty gap closing: `table_0080`
(a third C5.1 fragment, exact header match, cell-fused body rows carrying
LTVALID/LTCREDIT/LTCTAG cache-channel presence) was INVISIBLE to the gauge and is now an
accounted, honestly-flagged candidate miss**; standing extraction-quality gauges
re-display at documented states (APB 5/21 qwen2.5:14b-instruct); +7 hermetic tests
(inheritance grounding/refusals; gap-fill mints-only-undeclared, presence-row refusal,
fragment-reach; trapped coverage covered/strict-flagged; fragment accounting); lib 1574,
kg-bench 154/154, full `run_ci.sh` GREEN. RESIDUAL: the ACE +9 declarations (AWBAR w2,
AWDOMAIN w2, AWSNOOP w4, CRRESP w5, CDDATA wV, 4×BROADCAST* w1 — per-item verified in
the probe) land at the host-local re-ingest sweep; expected post-re-ingest: ACE 36→34
via gap-fill provenance + inventory coverage.
**`.12b` — the presence-CONDITION typed surface** · Status: **DONE `2026-06-11`** (design
census DONE `2026-06-11`; **corpus-wide GATE census DONE `2026-06-11` — the structural gate
(signal-worded first header + ≥2 all-code variant columns over identifier-led body+trapped
rows) fires on 33 tables / 7 docs: the 22 known PLUS 11 verified-per-item new candidates**;
BUILD landed `2026-06-11`, full log after the design text below). NEW per-item findings that shape the build: (a) the
ATB/AHB/APB version-matrix family (ATB 0016; AHB 0033-0035 — a GOLD doc; APB_d 0013-0015;
APB_e 0016-0017) is cyclically ROTATED — the signal name lands in the LAST column while
the header says col 0, so presence capture MUST reuse the `.5h` content-based rotation
detection and remap variant labels by the same offset, or refuse when the remap is not
clean (fused `Y Y` cells in APB_d 0013 / APB_e 0016 mark unclean rotations — refuse those
rows); (b) ACE mid-chain fragments 0272/0273 are clean UNROTATED body-row matrices whose
LAST variant column is a fused two-label header (`ACE5-Lite ACE5-LiteACP`) with
consistently fused two-code cells (`OC N`) — a measured pairwise split (N label tokens ↔
N code tokens) recovers them literally; cells that do not pair-split stay refused;
(c) ACE 0279 is the clean matrix tail (3 `*CHK` rows incl. an `I` source); (d) integrity
gates from the APB garble class: refuse the whole table when the content-densest signal
column ≠ the header-designated signal column AND no clean cyclic remap exists, and refuse
any table where ≥2 columns carry ≥2 signal tokens each with no consistent rotation
(the split-spill `table_0018` shape — its rows are individually refused by the per-row
code rule anyway). The 21+1 presence matrices carry CONFIGURATION
intent no existing surface types: per-variant signal presence (`AWSUBSYSID` is `O` in
AXI5, `N` in ACE5-LiteACP) plus property-conditioned existence (`SUBSYSID_WIDTH > 0` —
AXI 157 condition rows / 73 distinct exprs, LTI 23/15). **Design census (measured
`2026-06-11`, all 22 tables incl. LTI `table_0080`): 489 identifier-led data rows; 465
(95.1%) are WELL-FORMED — every variant cell a single 1-2-uppercase-letter code or `-`
(vocabulary: N 893, O 816, C 289, Y 205, OC 142, YS 54, OO 23, NS 17, YM 12, OI 6, OM 4 —
codes stay LITERAL strings, never interpreted; the legend prose differs per doc); 24 rows
malformed (fused `N N`/`C N`/`OM OM`, rotated `LTI_CACHE_SUPPORT O`, empty) → FULL-ROW
refusal, honest residuals (the APB 0018 / LTI 0080 garble class).** Typed shape (decide
final names at build): a new additive EvidenceIR surface (the `message_field_records`
precedent — `#[serde(default)]`, schema-stable) of records `{signal_name (literal case),
presence_condition: Option<literal expr> (None when '-'), variant_presence: [{variant_label
(literal column header; LTI's fused sub-header `A A.b` stays literal), code (literal)}],
table_id provenance}`; the A13.3 `Ax*` generic rows capture LITERALLY (the doc's own
generic-name convention — never expanded, the alias map is empty). Gates: the table
qualifies structurally (signal-worded first header + variant columns + `.9.11`/body data
rows — NO caption-word list beyond the existing structural census rule); per-row capture
only when the name cell is identifier-led AND every variant cell parses as a single code
or `-`; rows with a Width/Source column (the ACE matrix) still capture presence ALONGSIDE
the `.12a` declaration gap-fill (different intent, different surface). Open build items:
validate metrics + Info finding (the `.11` pattern: emitted only when non-empty);
residual accounting (a presence-captured table is EXPLAINED — wire into
`unexplained_intent_bearing_tables` like provenance; expected post-build: AXI 32→24-25,
LTI 6→3-4, ACE 36→34); kg-bench fixture pair (gold + malformed-refusal negative);
GOLD-SAFETY unchanged (additive surface; AXI gold+promoted — no canonical rebuild
without gauge re-measure; 13-doc old-vs-new parity must hold except the new surface +
manifest on docs that carry matrices).
**GOLD-SAFETY BAR (non-negotiable):** AXI + APB are
gold-measured docs with PROMOTED canonical surfaces and standing Pattern gauges — every
gold gate must re-measure at its documented state; additive-only; restore-the-Pattern-
baseline protocol does NOT apply here (no promotion measurement) but canonical artifacts
are NOT rebuilt without re-measuring the gauges they carry (live verification = temp-root
rebuilds from persisted SourceIR, canonical untouched — the `.10b`/`.10e` protocol).
Cross-link: `WIRE-BASED-100.3` (this leaf owns its `table_0018` residual line).
**`.12b` BUILD — DONE `2026-06-11`.** What landed: (i) the additive EvidenceIR surface
`signal_presence_records` — `SignalPresenceRecord { presence_id, signal_name (literal
case), presence_condition: Option<literal expr> (None when '-'), variant_presence:
[{variant_label (literal, incl. fused sub-headers and pair-split halves), code (literal
1–2-uppercase-letter string, NEVER interpreted)}], table_id }` (`#[serde(default)]` +
skip-if-empty — schema-stable, the `message_field_records` precedent); (ii) ONE shared
pure capture (`capture_signal_presence_rows` in `ir/evidence.rs`) implementing gate +
`.5h` rotation remap + split-spill integrity (orphan identifiers in a second column →
whole-table refusal) + measured fused-pair split (N label tokens ↔ N code tokens in
EVERY identifier-led row, else single-label and fused cells refuse their rows) + per-row
all-code capture, consumed by BOTH the registered extractor `signal_presence.matrix_table`
(`run_surface`, content-key dedup so page-break re-listed rows merge first-wins, ids
post-merge) and the completeness coverage (validate-time like `.12a` — STRICT: ≥1
captured row AND zero refused, so partial capture never hides a miss); (iii) validate:
4 metrics (`signal_presence_records`/`_signals`/`_conditioned`/`_variant_labels`) +
console block + `evidence_signal_presence_inventory` Info finding (non-empty only — the
`.11` pattern); (iv) kg-bench expectation surface (`signal_presence_count`/`_include`
with per-variant label↔code locks and condition/condition-absent/`_signal_names_exclude`)
+ fixture pair `signal_presence_matrix_gold` (trapped + rotated + fused-pair tables; also
locks presence NEVER mints canonical signals) and
`signal_presence_malformed_refusal_negative` (fused `Y Y` row refusal + split-spill
whole-table refusal). **GATE-CENSUS CORRECTIONS (measured per-item before coding):** the
structural gate fires on **36 tables / 8 docs**, not 33/7 — the +3 are AXI-Stream
`ihi0051_b` 0015–0017, genuine rotated version matrices the census sweep missed
(per-item verified against the document: `TVALID` Y/Y, `TDATA` C/O under the
`Tdata_Width` property, `TWAKEUP` C/N, the `*CHK` matrices C/N), and LTI `table_0080`
does NOT fire the gate (only one clean code column survives its garble; same honest
outcome — its rows were the census's malformed class). Pre-code probe re-derivations:
AXI condition census EXACT (157 rows / 73 distinct exprs), ACE known-5 = 134 tokens
EXACT (0271/0275/0276/0277/0278), the 24 malformed rows EXACT (ACE 0275's 21
inconsistently-fused + APB 0018's per-row 3), APB_d 0013 1 + APB_e 0016 2 fused-row
refusals EXACT; the `.12` LTI 23/15 condition numbers did NOT reproduce under the design
rules — corrected measurement: 31 condition rows / 17 distinct exprs over 0078+0079
(29 distinct condition-bearing names). **MEASURED (final post-fmt binary):** 12/12
rebuildable corpus bundles old-vs-new `evidence --dry-run` — every byte identical EXCEPT
the new surface + the manifest's `signal_presence` entry (`.12a` counted 13 bundles; one
normalized bundle has since been reclaimed, today's rebuildable universe is 12); records
mint ONLY on the 4 matrix docs — **AXI 306 rows (157 conditioned / 73 exprs — the census
numbers live), APB_e 20 (0018 refused whole), AHB 40, AXI-Stream 22** — and the 8
non-matrix docs read honest zero. Spot-checked per-item against the documents: the
leaf's own worked example (`AWSUBSYSID` cond `SUBSYSID_WIDTH > 0`, O in AXI5 / N in
ACE5-LiteACP), `AWREADY` cond `AXI_Transport == Ready` O/Y/Y/Y/Y, APB `PADDRCHK`
Check_Type C/N/N/N, `PPROT` O/O/N/N, AHB `HCTRLCHK2` Exclusive_Transfers C/C, the A13.3
`Ax*` family literal with per-version codes (`AxMMUATST` Version 1=C only — later-version
`-` cells honestly omitted). **Residual accounting (validate-time on canonical artifacts,
NO rebuild):** AXI 32→31 (`table_0188` A13.3 closes — the generic rows' typed home; the
predicted 24-25 band over-counted because the B2.2/B2.3 chains were ALREADY
`.12a`-inventory-covered — presence now double-covers them), ACE 36→35 (`table_0271`
closes — the AWBAR/AWDOMAIN/AWSNOOP fragment; `table_0275` stays honestly flagged: its
21 rows are exactly the garble class the design census itself refused, so the predicted
34 was internally inconsistent with the census — measurement settles it), LTI 6→4
(0078+0079 close; 0080 stays), APB_e/APB_d/ATB/AXI-Stream 0→0, AHB 4→4 (its matrices
were already inventory-covered; the 4 are non-matrix residuals). **GOLD-SAFETY
re-measured:** APB/AHB/AXI signal-constraint + relation + temporal golds all 1.000, SWD
1.000 (incl. derivation 11/4/13), I2C declared-signal 1.000, APB standing
extraction-quality gauge re-displays 5/21 `qwen2.5:14b-instruct`; canonical artifacts
NOT rebuilt (presence records land on canonical surfaces at the next rebuild/re-ingest;
the accounting already reaches them through the validate-time rule). +9 hermetic tests
(lib 1574→1583); kg-bench **156/156**; fmt + clippy `-D warnings` clean; full
`scripts/run_ci.sh` GREEN. RESIDUALS owned here: ACE `table_0275` + LTI `table_0080`
(garble class — full-row/whole-table refusals; their 6 + 3 new wires' declarations land
at the host-local re-ingest via the `.12a` gap-fill), APB_e `table_0018` (split-spill,
inventory-covered), ACE `table_0270` "Key for Signal Matrix" legend (Code|Meaning — a
pre-existing non-matrix residual this slice deliberately does not touch).

<!-- active-task-source-region:activity-12:end -->

<!-- active-task-source-region:activity-13:start -->
**`.13` — corpus re-ingest sweep (owner-unblocked `2026-06-11`)** · Status: `in_progress`
(`.13a` import landing). The owner granted the host-local spec library on request (the
path is deliberately NOT recorded in any tracked file — `feedback_source_pdfs_in_repo`;
PDFs are copied into `corpus/` and git-tracked there, registry-listed with library-relative
source paths only). This leaf lands the RECORDED pending re-ingest wins — the docs whose
`normalized/` bundles artifact-cleanup reclaimed, making evidence rebuilds impossible
until their PDFs live in-repo. Bounded scope, 10 PDFs (~45 MB): ACE `IHI0022_H.c`
(the `.12a` +9 wires AWBAR/AWDOMAIN/AWSNOOP/CRRESP/CDDATA/4×BROADCAST* mint via gap-fill;
presence records on 0271/0272/0273/0274/0276–0279; expected accounting 35→down), LTI
`IHI0089_D` (presence records on 0078/0079; garbled 0080 stays refused), ATB `IHI0032_C`
+ APB_d `IHI0024_D` (presence records on their rotated version matrices), CHI `IHI0050_G`
(unblocks `EXTRACTION-QUALITY-GAUGE.3c` — that measurement stays owned by its own tree),
CCIX ×4 + AMD IOMMU (the `.10a`/`.10d`/`.10e` message-field surfaces finally land on
CANONICAL artifacts; triggers the `.11` recorded revisit — re-measure the
`document_class` census per-doc once field-bearing docs exist at scale). **GAUGE
SAFETY (verified per-doc before any rebuild):** ACE 76/94, LTI 37/41, APB_d 4/13, CHI
9/13 carry standing `qwen2.5:14b-instruct` gauges — an evidence rebuild DROPS a gauge by
construction, so each gauge doc gets a fresh `nli-verify` re-measure after its rebuild
(Ollama verified live with `qwen2.5:14b-instruct` + `qwen2.5vl:7b`); ATB/CCIX×4/AMD
carry none; NONE of the 10 has persisted semantic/intent stages, so the sweep is
ingest (`DOCLING_DEVICE=cpu`, staged-swap protects `normalized/`, `source_ir` never
deleted before success) → `evidence` → `validate` → gauge re-measure where applicable.
**GOLD-SAFETY:** none of the 10 is a gold or promoted doc; the 12 corpus golds are
untouched by this sweep. Slices:
- `.13a` · `done` (commit `94468930`) · import the 10 PDFs into `corpus/` (mirroring the library's
  vendor layout: `arm/amba/core/axi/legacy/`, `arm/amba/core/chi/current/`,
  `arm/amba/supporting/atb/current/`, `arm/amba/specialized/lti/current/`,
  `arm/amba/core/apb/legacy/`, `cxl/ccix/current/`, `amd/system-ip/iommu/current/`) +
  `SOURCE_PDF_REGISTRY.md` rows. Filenames kept verbatim so the derived `document_key`s
  match the persisted artifacts exactly (verified per-item for all 10 before import).
- `.13b` · `done` · the 4 AMBA matrix docs (ACE/LTI/ATB/APB_d): re-ingest + evidence
  rebuild + measure — presence records mint on canonical, the ACE +9 wires mint via the
  `.12a` gap-fill (per-item width verification against the probe record), accounting
  deltas recorded, ACE/LTI/APB_d gauges re-measured live. APB_d/ATB/LTI landed pre-crash
  (session `2026-06-11d`, artifacts re-verified per-item post-crash); ACE was blocked by
  `.13b.1` — fixed, then measured this session.
  **ACE DONE (`2026-06-14`):** fresh `evidence` rebuild completes (52.3 MB max RSS, no OOM).
  **201 `signal_presence_records`** on exactly tables 0271/0272/0273/0274/0276/0277/0278/0279
  (0275 honestly refused — the garble class), 59 property-conditioned. **+9 wires minted via
  the `.12a` gap-fill, per-item-verified widths:** AWBAR w2, AWDOMAIN w2, AWSNOOP w4, CRRESP w5,
  CDDATA wV, BROADCASTATOMIC/BROADCASTCACHEMAINT/BROADCASTINNER/BROADCASTOUTER w1 (CRRESPCHK w1
  / CDDATACHK w DATA_WIDTH/8 already existed). `validate`: `document_class: protocol` /
  declared `specification`; **`unexplained_intent_bearing_tables` 35 → 30** on the freshly
  materialized canonical surface; signal-presence inventory finding fires (201 rows / 201 signals
  / 59 conditioned / 6 variant labels); manifest fires `signal_presence.matrix_table`. **Gauge
  re-measured live** (`nli-verify --vlm-provider ollama --model qwen2.5:14b-instruct`):
  **78/97 not-entailed (80.4%), 19 entailed, 0 abstained** — consistent with the standing
  pre-`.12b` 76/94 (~80%; ACE is a dense coherency-extension Pattern surface, majority-flagged),
  re-persisted on canonical. `generated/` is untracked, so this slice commits the recorded
  measurements only (the presence + gap-fill code shipped in `.12a`/`.12b`).
- `.13b.1` · `done` · BLOCKER fix (surfaced resuming `.13b` after the host crash):
  the ACE evidence rebuild is jetsam-SIGKILLed (17.2 GB max RSS / 80.3 GB peak footprint,
  419 s, exit 137 on a 24 GB host). Root cause measured per-item with a throwaway
  instrumented worktree probe (never committed): `replace_term_with_placeholder`
  (`crates/specforge/src/ir/prior_memory.rs`) copies non-matching bytes via
  `bytes[index] as char`, mangling every non-ASCII UTF-8 byte into a 2-byte char;
  `normalize_prior_phrase` chains ONE full replacement pass per MULTI-WORD term, so each
  pass re-doubles the previously mangled bytes — exponential growth in the number of
  multi-word signal/actor replacement terms. ACE's semantic-hints surface derives 173
  multi-word actor names (AXI: 3), far past the ~30 doublings that reach tens of GB, and
  the `•`-bearing `Signal | Width | Description` cells of `table_0201` trigger it — ONE
  `infer_signal_semantic_tags_from_description` call hangs for minutes growing the string
  until the kernel kills the process (sampled stacks: 100% in `normalize_prior_phrase`;
  the tables loop up to that row completes in 10 ms). Every other persisted doc carries
  too few multi-word terms to detonate, which is why the defect stayed invisible. FIX:
  char-correct copying in `replace_term_with_placeholder` (byte-for-byte identical
  behavior on pure-ASCII inputs; non-ASCII text preserved verbatim instead of mojibake;
  a match is only taken on char boundaries) + hermetic regression tests (non-ASCII
  preservation, no-growth under chained multi-word passes, replacement still fires beside
  non-ASCII). Verify: focused tests + kg-bench + full CI, intact-bundle byte-stability
  re-proof, then the ACE rebuild must complete and `.13b` resumes.
  **DONE (`2026-06-14`):** the `else` arm of `replace_term_with_placeholder` now copies one
  whole UTF-8 char (`text[index..].chars().next()` → `push(ch)` → `index += ch.len_utf8()`)
  instead of `push(bytes[index] as char)`; `index` is always on a char boundary (it advances
  by an ASCII match's `term_bytes.len()` or by one whole char), so on pure-ASCII input it is
  byte-for-byte identical to the old copy. +4 hermetic tests (non-ASCII-only verbatim;
  replacement-fires-beside-non-ASCII; 40 chained non-matching passes are a no-op; the
  end-to-end `normalize_prior_phrase` ACE-shape stays bounded with 173 multi-word actor
  terms + a `•`). Verified: lib 1583→**1587**; kg-bench **156/156**; full `run_ci.sh` GREEN
  (fmt/clippy-deny/rustdoc/mdBook/memory-arch/knowledge-map). **Byte-stability re-proof
  (pre-fix vs post-fix fresh `evidence --dry-run` — the clean isolation; ACE excluded because
  it OOMs pre-fix): all 15 non-ACE intact bundles BYTE-IDENTICAL** → the fix is a pure no-op
  on the intact corpus. **ACE evidence now COMPLETES: 21 s / 56 MB max RSS** (was 419 s /
  17.2 GB RSS / SIGKILL exit 137). KM card `prior-phrase-utf8-byte-as-char`.
- `.13c` · `done` (`2026-06-14`) · CHI: rebuild + gauge re-measure; report the field/constraint
  surfaces now on canonical; hand the `EXTRACTION-QUALITY-GAUGE.3c` unblock back. **No re-ingest
  needed** — CHI's `source_ir.json` + 528 MB `normalized/` bundle from `.2` were intact, so
  `specforge evidence` rebuilt from the persisted source (RAM stayed ≥78% free — the `.13b.1`
  UTF-8 fix holds on a 585p doc; section_anchors 1202 / evidence_spans 10624 / extracted_statements
  11708). **Field surface now on CANONICAL: `message_field_records` 0 (stale) → 106 across 4
  containers** (the `.FIELD.2` "CHI ~106" figure lands — the manifest's
  `message_fields.container_field_table` strategy fired; 0 with literal bit positions — CHI's flit
  fields are width/role tables, honest). Constraint surface: 13 Pattern signal_constraints,
  conditional_rules 190, relations 79; downstream rebuilt (semantic 87 actors, intent 2388
  behaviors) so the canonical chain is consistent. **Gauge re-measured (qwen2.5:14b-instruct,
  ~13 NLI calls): 9/13 not-entailed (69.2%)**, persisted + reported by `validate` (majority-
  erroneous warning). The not-entailed list is exactly the `EXTRACTION-QUALITY-GAUGE.3c` material:
  field obligations mis-attributed to channels by the Pattern path (`TagOp`/`PBHA` "must be 0" →
  "REQ must be 0"; MPAM "must be included" → "REQ must be I") + descriptive behaviors read as
  invariants ("the receiver sets REQLCRDV HIGH" → "REQLCRDV must be HIGH"). With the
  `LLM-PRIMARY-PROMOTION.5` flip now the converge default, a CHI converge would route the field
  obligations to `message_field_constraints` and clean the channel surface (the `.4` sweep measured
  CHI 69.2%→16.7% promoted) — that canonical CHI promotion is a natural flip-consistent follow-up,
  not required by this measurement leaf. **`.13` corpus sweep COMPLETE** (`.13a` import · `.13b`
  AMBA-matrix + ACE · `.13b.1` UTF-8 fix · `.13c` CHI · `.13d` CCIX×4 + AMD all done). validate
  also: document_class=protocol; completeness 20 gaps (signals_without_direction 17/28,
  unexplained_tables 3/27). EXTRACTION-QUALITY-GAUGE.3c is UNBLOCKED.
- `.13d` · `done` (`2026-06-14`) · CCIX ×4 + AMD: re-ingest + rebuild; the message-field surfaces
  land on canonical (expected ≈161 CCIX fields across versions, AMD 217/30 per the
  `.10d`/`.10e` measurements — re-verify live, never assume); run the `.11` revisit:
  re-measure `document_class` over the now-field-bearing docs and record whether the
  census decision stands (it must be a MEASURED confirmation either way).
  - **AMD-IOMMU DONE (`2026-06-14`):** re-ingest via the bounded batched path
    (`SPECFORGE_INGEST_BATCH_THRESHOLD=128` → 64-page batches, `DOCLING_DEVICE=cpu`,
    built-in `.4a` RAM guard) — **staged-swap clean, RAM stayed ≥49% free throughout, no
    OOM** (a live end-to-end validation of the `MEMORY-BOUNDED-INGEST.4a`–`.4c` guards on a
    310p doc). source_ir: 310p / 354 tables / 416 figures / 2060 elements. Evidence rebuild
    bounded (**peak RSS 41 MB** — the `.13b.1` UTF-8 fix holds). **Message-field surface now
    on CANONICAL evidence: 217 `message_field_records` / 30 containers** (217 with literal
    `bit_range`, 114 dword-relative with `byte_offset`) — **matches the `.10d`/`.10e`
    measurement EXACTLY** (217/30, 114 byte-offset). Top containers are real AMD structures
    (Device Table Entry 33 fields, IO_PAGE_FAULT log entry 16, ILLEGAL_DEV_TABLE_ENTRY 13).
    `register_records` 11 (stale pre-`.10`) → **8** — CORRECT `.10e` behavior (in-memory
    structure tables route to message-fields instead of fabricating MMIO registers), not a
    regression. `validate`: `document_class: interface` / declared `specification`;
    message-field inventory finding fires (217/30, 114 byte-offset); manifest fires
    `message_fields.bit_position_table` (+ register_map / relations.prose / semantic_hints.prose).
    **`.11` revisit datum #1:** AMD carries 217 fields yet classifies `interface` (3 signals /
    98 relations / no behavioral obligations) — its field richness did NOT reclassify it, so a
    message-field document-class arm is not yet motivated; revisit stays open pending the CCIX×4
    data points (need the family at scale before deciding — `.11` overfitting guard). `generated/`
    is untracked → this records the measurement; the extractor code shipped in `.10d`/`.10e`.
  - **CCIX ×4 DONE (`2026-06-14`):** all four re-ingested via the bounded batched path
    (threshold=128/64-page batches, cpu, `.4a` guard) — exit 0 each, ~3–4 min each, staged-swap
    clean, RAM stayed ≥41% free, no OOM. Evidence rebuilds bounded (CPU, fast). **Canonical
    message-field + register surfaces landed, matching the `.10a`/`.10e` measurements:**
    r1.0 = 92 fields / 11 containers / 131 registers; r1.0a = 86 / 10 / 131; rev1.1 = 86 / 11 / 132;
    rev2.0 = 45 / 6 / 143. **byte_offset fields total 45+36+35+45 = 161 = the `.10e` "≈161 fields"
    figure EXACTLY**; rev2.0's 143 registers = the `.10a` "143 registers" figure. All four
    `validate` → `document_class: register` (dominant surface = the recovered register map).
  - **`.11` `document_class` revisit COMPLETE — decision STANDS, NO new arm.** Five field-bearing
    docs now on canonical: AMD (217 fields) → `interface`; CCIX×4 (86–92 fields, 131–143 regs) →
    `register` ×4. Field richness did not cluster into a new class nor force a misclassification —
    each classifies on its dominant surface, so a message-field classifier arm would be overfitting
    (n=5, 1 interface / 4 register). Measured confirmation; no code change. **`.13d` complete.**

**(SUPERSEDED active note) `PDF-VARIANT-DIGESTION.6`/`.7`** — item ② (`.5`) COMPLETE and `.8` (broaden
prose-actor capture) DONE. **`.5a` + `.5b` + `.5c` are DONE** — structural doc-class routing
(protocol/register/interface/guide), the front-matter doc-type signal (true guide vs under-extracted spec), and
the class-aware per-doc completeness gauge (`.5b`) are all live in `validate` with honest guide reporting. **`.8`
DONE** — robust structural agent-definition grammar broadens prose-actor capture (projected 14 → 20 docs,
garbage-free, no denylist). Remaining: `.6` (VLM levers on zero-yield) / `.7` (USB 3.2 evidence-fail). Live distribution over the 74 persisted docs: protocol 25 /
register 11 / interface 22 / guide 16; `.5c` further split the 16 guides into 11 TRUE guides + 5 UNDER-EXTRACTED
specs (front-matter self-declares a spec → flagged for the VLM frontier `.6`, not silently dismissed). **`.4` (correctness/precision verification, item ①) is COMPLETE** — both
`.4a` (per-fact gold on register fields + prose signals) and `.4b` (VLM proposer/verifier audit) done.
**`.4b` DONE** (`.4b.1` harness + `.4b.2` live measurement): the `audit-extraction` VLM audit gives a
table-kind precision ESTIMATE that **discriminates extraction quality and independently corroborates `.4a`** —
RISC-V Debug **0.250/0.375** (register tables flagged for lacking in-table bit positions, matching `.4a.2`'s
bit-extent 0/179) vs NVMe **0.750** (register tables confirmed, matching `.4a.3`'s 0.931 bit-structure recall;
caught a real feature-matrix→timing misclassification). **`.4a` is DONE** (`.4a.1`–`.4a.5`): the
register-field surface is
measured on two opposite-shaped docs (RISC-V field-name recall 0.588; NVMe bit-structure recall 0.931) and the
declared-signal surface on I2C (recall 1.000 / precision 0.600). Five extraction-fix targets are now quantified
(RISC-V register-name + bit-graphic; NVMe mnemonic; I2C acronym/condition filter; …). The `.2`–`.3b` leaves
below are DONE (Lever A + B); `.4`–`.8` are the "make the breadth trustworthy" program.

- `PDF-VARIANT-DIGESTION.2` (Lever A, deterministic strategy) — **DONE**: `synthesize_register_field_tables`
  recovers register-FIELD tables the classifier left `unknown` (header-in-body `Field|Description|Access|
  Reset`, `Bits|Type|Reset|Description`, …) → `RegisterRecord`s. Designed from a corpus survey of real
  register-table shapes/access-notations/bit-formats ([[project_flexible_register_model]]); access/reset are
  free strings, bit ranges parse zero-padded, header-echo legend rows dropped. **RISC-V Debug: 60 regs / 179
  fields** from previously-`unknown` tables; APB/AHB/AXI/SWD source-tolerant stay 1.000 (additive); +4
  hermetic tests; full `run_ci.sh` green. RISC-V Debug PDF copied into `corpus/`. KM
  `register-field-table-extraction`.
- `PDF-VARIANT-DIGESTION.2b` (Lever A, VLM strategy) — **DONE (classification)**: Qwen2.5VL reads the
  `table_region` images and reclassifies `unknown` tables (validated live — it read the RISC-V `dmcontrol`
  table's kind + all 5 field names from the image). `enrich --vlm-provider ollama` now runs
  `classify_unknown_tables_via_vlm` (shared `vlm_image_query`; `parse_vlm_table_kind`), best-wins (only
  `unknown` tables touched; `register_field`/TOC/other not reapplied — grammar path / noise), writes
  `table_kind` back so a re-run of `evidence` fires the deterministic extractor. Gated (`skip` = no-op);
  encryption irrelevant (docling renders the images). **VERIFICATION GATE** (`vlm_kind_structurally_consistent`):
  the VLM proposes, structure disposes — a kind is applied only when the table header matches it, so the VLM's
  over-classifications (register-field / operation tables → `signal_description`) are rejected. Measured on
  RISC-V: without gate 22 reclassified (+9 real DMI signals but +5 garbage); WITH gate **1 reclassified → 9
  genuine DMI signals (`REQ_*/RSP_*`), 0 garbage**. +2 hermetic tests; full CI green. KM `vlm-table-strategy`.
- `PDF-VARIANT-DIGESTION.2b'` (Lever A, VLM extraction) — **DONE**: VLM GRID REPAIR. ~10% of corpus tables
  (262) are degenerate (Docling failed to structure them: ≤1 column). `enrich --vlm-provider` now runs
  `repair_degenerate_tables_via_vlm` — the VLM transcribes the table image to a JSON grid
  (`build_table_extract_prompt`/`parse_vlm_grid`, serde_json, ≥2 columns), REPLACING the degenerate
  header/body so the deterministic extractors run (best-wins at the STRUCTURE level: Docling grid vs VLM
  grid). Kind set only when the repaired header is structurally consistent (`.2b` gate). +1 hermetic test;
  full CI green; gated (skip = no-op).
- `PDF-VARIANT-DIGESTION.2c` (model flexibility) — **DONE (core)**: the register model now carries, all
  backward-compatible: `RegisterRecord.size_bits`; `RegisterFieldRecord.bit_width` (a field is
  `name + offset(=bits_low, LSb) + width`; range/single-bit/offset+width all map) + `enumerated_values`
  (`RegisterFieldEnumRecord { value, meaning }`); access/reset stay FREE strings. Populated deterministically
  where data exists: width from `[high:low]`, register size from max field MSb, inline enums from
  binary/hex/Verilog literals in descriptions (conservative — no bare-int false positives). A no-garbage
  filter drops bit-LAYOUT grids (register-diagram tables whose "fields" are bare bit numbers). +3 hermetic
  tests. **Real-data demo: NVMe → 44 registers / 199 fields, all with `bit_width`** (3 bit-layout grids
  filtered); RISC-V unaffected (60/179); APB/AHB/AXI/SWD stay 100%. NVMe added to `corpus/`. Owner-confirmed
  model
  ([[project_flexible_register_model]]). **Width is MANDATORY (physical):** a register is bit-storage so a
  width always exists; an unresolved `size_bits` is a COMPLETENESS GAP (parametric XLEN / cross-document),
  not optional — `validate` reports `registers_unresolved_width` (RISC-V 60/60 vs NVMe 0/44). REMAINING:
  register NAME from preceding heading (synthetic today — tables aren't in Docling's content_elements
  reading-order, so reliable association is deferred, not faked); block/base grouping; array/instance;
  parametric `size_expr` + cross-document width resolution.
- `PDF-VARIANT-DIGESTION.2d` (deterministic) — **DONE**: `table_is_noise` flags non-data NOISE tables (table
  of contents, list of tables/figures, revision history, section index) by general structure — dotted
  page-leaders, contents/revision caption-or-header, or rows mostly prefixed by a section number (no chip
  names). The VLM passes skip them (no wasted calls; CCIX has ~220 TOC tables) and `enrich` reports
  `tables_skipped_as_noise`. +1 hermetic test; full CI green.
- `PDF-VARIANT-DIGESTION.3a` (Lever B, prose SIGNALS) — **DONE**: `synthesize_signal_declarations_from_prose`
  gained the parenthetical-abbreviation form ("a serial data line (SDA)") on top of the pin appositive (`.2`).
  Guards (from live I2C runs): sparse-catalog FALLBACK gate (parenthetical runs only when <8 table signals —
  AXI etc. untouched, fixed a 1.000→0.857 AXI regression), uppercase-acronym gate (rejects "(resulting…)"),
  universal denylist (READ/WRITE/MODE). **I2C: 0 → 10 declared signals** (SDA/SCL + Hs SCLH/SDAH + USCL/USDA
  + ACK/NACK/DDC/SDR). Wire-based specs stay 1.000; +3 hermetic tests; full CI green. I2C PDF added to
  `corpus/`. KM `prose-signal-capture`.
- `PDF-VARIANT-DIGESTION.3b` (Lever B, prose ACTORS/AGENTS) — **DONE**: new `ProtocolActorRecord` surface +
  `extract_protocol_actors` capture agents a spec DEFINES in prose — "A <name> is the device which/that
  <capability>" and "considered a/the <name>" — grounding the agent model from prose, not only as a relation
  subject. `is_agent_noun` rejects function/structural words (general; admits vendor agents like SMMU). **I2C:
  2 actors — controller (def: "the device that initiates a data transfer … and generates the clock") +
  target.** Additive new surface (no eval impact; wire-based unaffected); +2 hermetic tests; full CI green.
  KM `prose-signal-capture`.

<!-- active-task-source-region:activity-13:end -->

