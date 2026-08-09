# PDF-VARIANT-DIGESTION — activities 10d–10e layouts

- Part ID: `activity-10d-10e-layouts`
- State: `legacy`

<!-- active-task-source-region:activity-10d-10e:start -->
· Status: **DONE `2026-06-11`** (probe → measured design → build → live per-item
verification; one live finding fixed and fixture-locked: the fused `Fields(Continued)`
caption). Spun from the `.10b` quantified residuals: **41 AMD tables /
184 rows** whose bit cells carry a byte-offset suffix (`31:28 +04` = bits 31:28 of the
dword at byte offset `+04` within the containing structure). The `.10b` STRICT cell parser
rejects the WHOLE table on any such cell BY DESIGN (capturing `31:28` while dropping the
`+04` would misplace the field — fabrication), so before this slice those tables were
honest residuals.
**Verification log (build):** lib tests 1560→1564 (4 new hermetic: the offset-cell parser
gates incl. every measured corpus rejection shape — `16: +04`, `13+`, `4 + (ITSnum × 2)`,
`15+HL:16`, `9+N`, `20+`, no-space `31:28+04`, bracketed, `04h`-suffixed; the
bracket-slice name grammar incl. the English-head admission, two-word/slash/boundary-less
/digit-less rejections; the framed single-letter grammar incl. frame requirement and
per-table uniqueness; the end-to-end dword-relative chain — same-dword + next-dword joins,
literal `byte_offset` on records, the offset-LESS row keeping `byte_offset: None`, opcode
value-row/Reserved/two-word/fresh-restart residuals; plus the `.10b` lock test updated to
the `.10d` contract and the fused-`(Continued)` label case); kg-bench **154/154**; full
`scripts/run_ci.sh` GREEN. **Old-vs-new dry-run parity over ALL 12 intact-bundle docs:
byte-IDENTICAL — zero deltas, not even the manifest** (the offset family rides the
EXISTING `message_fields.bit_position_table` strategy: same family, wider literal grammar
— a deliberate decision, no new extractor registered, so the corpus fingerprint vocabulary
is unchanged). **Live AMD (stub protocol, canonical untouched, re-verified on the final
post-fmt binary): 82 → 217 message fields, 15 → 30 containers, +135 fields of which 114
carry `byte_offset`, ZERO pre-existing records changed, every other surface
(registers 8, timing 62, constraints/relations 0) byte-identical.** Per-item eyeball of
all 135: 16 offset-bearing containers (`COMPLETION_WAIT` `f`/`i`/`s` at `+00`;
`ILLEGAL_DEV_TABLE_ENTRY` 13 fields `VCmd`→`Address[63:32]@31:0+12`; `IO_PAGE_FAULT` 16;
`PAGE_SERVICE_REQUEST` 11 incl. `PPRtag@9:0+04`; `INVALIDATE_IOTLB_PAGES` 10 incl.
`Maxpend[7:0]@31:24+00`), the table_0126 bare-`17` row lands as `US@17:17` with
`byte_offset` honestly ABSENT (never inferred from neighbors), and the D-form names
previously-residual rows on PURE `.10b` tables (+21: DTE `GuestID[15:0]`/`GDeviceID[15:0]`
— the documented `.10b` residual class; IRTE `Vector[8:0]`/`Destination[7:0]` English
heads under the bracket frame; page-table `A`/`D`/`G`/`U` single-letter bits).
`INVALIDATE_IOMMU_ALL` and `Guest_Event_Fault` rows recover no names → no records
(honest, probe-predicted). **Live finding (caught per-item, then fixed + tested):** the
`Table 39: PREFETCH_IOMMU_PAGES Fields(Continued)` caption fuses the marker to the
previous word (lost spacing) — the label grammar initially minted a bogus
`PREFETCH_IOMMU_PAGES Fields(Continued)` container; a shared `trim_continued_marker`
(applied to BOTH caption readers — container label and `.10c` register name) now strips
the fused form, and the fragment merges into `PREFETCH_IOMMU_PAGES` (7 fields).
**Probe-assertion CORRECTION (from code reading before building):** the probe checkpoint
overstated "the existing chain recovers ~0 on offset rows" — the existing
leading-identifier form (whose `identifier_shaped_token` trims a trailing `:`) already
recovers the multi-char colon/dot heads (`AttrV:`, `VCmd .`, CCIX `ESMEnable .`) once a
table passes the CELL gate; the offset tables yielded 0 purely because the gate rejected
them. The genuinely NEW grammar is therefore narrower than the probe's D/E/F framing:
**Form D bracket-slice** (`DeviceID[15:0] .` → verbatim name incl. the value-slice
qualifier — `Address[31:0]` and `Address[63:32]` stay distinct records; English heads
admitted by the bracket+boundary frame) and **framed SINGLE LETTERS** (`f:`/`U .` —
colon/dot frame required, per-table uniqueness via the count-key extension whose 1-char
keys are disjoint from the existing 2–40-char keys by construction). Consequently the
predicted `.10a` CCIX delta is ZERO (those rows were already named) — confirmed by the
sweep: CCIX outputs byte-identical. NVMe `Operation:` (Titlecase English, colon-only
frame) is rejected by the identifier-shape gate → NVMe byte-identical too, documented
honest residual.
**Build shape:** additive `MessageFieldRecord.byte_offset: Option<u32>` (serde-skipped;
old artifacts load unchanged); `parse_offset_suffixed_bit_position` beside the pure parser
(union grammar in the 2-col collector arm only — the 3-col family carries no offsets);
`BitLayoutRow.byte_offset`; chain adjacency gains the dword-relative branch — the
`(offset asc, bit desc)` lexicographic successor (same-dword `next_hi == prev_lo - 1`,
14 measured joins; next-dword `prev_lo == 0 → next_hi == 31, offset exactly +4`, 1
measured join), a `dword_rows_forward` order guard, BOTH boundary rows must carry offsets,
and the two position conventions never join each other; `recover_field_mnemonic` gains
forms D and framed-singles in the shared chain (register path included — corpus-measured
AMD-only, proven by the sweep). Golds on the 12 swept docs are locked by the byte-identity
(canonical artifacts untouched — the standing Pattern gauges survive by construction;
eval code untouched). **Honest residuals (quantified, unchanged class):** the 8
lost-caption capless chains (tables 0079, 0080+0081, 0088, 0093, 0116+0117, 0121, 0129,
0146 — the re-ingest lever); 0127/0128 (3-col conditional `Description, RX=0|RX=1`
headers) and 0132 (malformed `16: +04` cell) whole-table FAILs; 25 opcode value-rows
(`01h . COMPLETION_WAIT command number.`); two-word heads (`Store Address[31:3]`, `Type.`,
`Src .`, `Inval:`); name-less parsed rows. KM card
`offset-suffixed-dword-relative-bit-cells`; book `pipeline/evidenceir.md` subsection.
**The leaf's two recorded questions (decide from the probe, per-item):**
  1. **Derivation honesty:** the absolute bit position `offset*8 + bit` is DERIVABLE
     arithmetic from two document-stated numbers (the `EXTRACTION-GAP-FIX.4a` cumulative-
     tiling precedent: arithmetic on document-stated values is not fabrication) — but the
     `.10b` `bit_range` is documented as "the literal `(high, low)` position". Decide:
     derive absolute positions into `bit_range`, OR keep the literal dword-relative range
     plus a separate additive offset field (both document-stated, zero interpretation).
     The probe decides by measuring which reading the documents themselves ground (do
     sibling captioned/structure facts state absolute positions? do offsets tile?).
  2. **Typed home:** expected = the `.10b` structure surface
     (`message_field_records`, strategy family `message_fields.bit_position_table` or a
     sibling strategy) — same in-memory-structure class, same no-access/reset evidence;
     confirm against the 41 tables' captions per-item (any register-evidence captions
     route per the `.10c` per-table caption rule instead).
**PROBE DONE (`2026-06-11`, per-item over the persisted corpus — both questions answered):**
**(Q1 — derivation honesty: LITERAL capture wins.)** The description bracket-slices are
field-VALUE slices, NOT positions (`Store Data[63:32]` at `31:0 +12`; `DomainID[15:0]` at
`15:0 +04` — 54 of 66 bracket-slice rows MISMATCH `offset*8+bit`; the 12 matches are
value-aligned continuations like `Store Address[51:32]` at `19:0 +04`, which DO ground the
little-endian dword packing). So the typed capture is the document's literal statement:
dword-relative `bit_range` + a NEW additive `byte_offset: Option<u32>` (serde-skipped;
None on every existing strategy) — never an overwritten "absolute" range; the absolute
position stays consumer-derivable arithmetic. **(Q2 — typed home: the `.10b` structure
surface, confirmed.)** The rows describe 16-byte in-memory COMMAND/EVENT-LOG/PPR-LOG
entries (`COMPLETION_WAIT`, `IO_PAGE_FAULT Event Log Buffer Entry`…), zero access/reset
anywhere; captions are the `.10b` caption family verbatim (`Table NN: <NAME> Fields
(Continued)` → strip label + `(Continued)` + trailing `Fields`).
**Census (strict grammar `^\d+(:\d+)? \+\d+$`): corpus-safe — AMD-ONLY, 43 tables / 309
rows**; every non-AMD `+`-cell is SYMBOLIC (`13+ ITSnum`, `15+HL:16`, `9+N` — GIC-600 13,
NVMe 11, USB4 4, USB3.2 2 rows, all correctly rejected by the digits-only suffix). Offset
spellings exactly `{+00:90, +04:141, +08:54, +12:24}` — DECIMAL dword offsets (`+12` not
hex; semantics proven by the Store Data continuation). **Gate (`.10b` gate + union
pure|offset cell grammar, whole-table rejection stands): 40/43 PASS / 287 eligible rows**;
honest FAILs: `table_0127`/`table_0128` (3-col conditional-description headers
`Description, RX=0 | RX=1`) + `table_0132` (malformed `16: +04` cell — one bad eligible
cell rejects the table, the `.10b` rule). `table_0126` carries one offset-LESS `17` row
mid-table → that row records `byte_offset: None` (literal honest absence, never inferred
from neighbors). **Chains (offset-aware): adjacency = the (offset asc, bit desc)
lexicographic successor** — two measured forms: same-dword `next_hi == prev_lo - 1` (14
joins) and next-dword `prev_lo == 0 → next_hi == 31, offset increases` (1 join:
`INVALIDATE_IOTLB_PAGES` `(0,15,0)→(4,31,28)`); the capless-HEAD-adopts-captioned-tail
direction is the existing `.10b` DTE pattern (`table_0077` capless + `table_0078`
`COMPLETION_WAIT Fields (Continued)`). Walk result: **16 labeled chains / 30 tables / 217
rows extract; 8 lost-caption chains (10 tables, ~70 rows) stay honest residuals** (the
re-ingest lever; zero ambiguous adoptions — fresh structures restart at `(+00, 31:x)`,
never the successor of a mid-structure end).
**The name forms ARE the unlock** (existing mnemonic chain recovers ~0 on offset rows —
no record without a name, so `.10d` without them is an empty slice). Three description-fused
forms measured corpus-wide over every family-eligible description cell (`.10a` CCIX
bit-location + 2-col families): **Form D bracket-slice** `<Ident>[hi(:lo)]` + `.`/`:`
boundary — 95 fires, AMD-ONLY, every fire a genuine name (`GuestID[15:0]:`,
`Vector[8:0] .`); **Form E colon-gloss** `<Ident>: <gloss>` — 221 AMD + 1 NVMe
(`AttrV:`, `vImuEn:`, `f: flush queue`; NVMe `Operation:` genuine); **Form F ident-dot**
`<Ident> . <gloss>` — 65 AMD + 20 CCIX (`ESMEnable .`, `LinkReachTarget .` — currently
name-LESS `.10a` residuals, genuinely named). ZERO `Note:`-style/prose bleed in any
family. Gate decisions for the build (eyeball the full fire sets per-item before coding):
(a) under the bracket-slice frame the plain-English-word rejection is OVERRIDDEN
(`Address[31:12].` IS the field's name — 25 rows, all genuine; the frame is the
structural anchor the bare leading-identifier form lacked); (b) single-letter names
(`f`/`i`/`s`/`S`/`I` — 9 rows) accepted ONLY with the colon/dot frame; (c) `.10a`
per-table-uniqueness + mid-cell-bleed gates ported to all three forms. Expected yield:
**~16 new containers / ~73–107 named fields** on AMD offset rows (D 16 + E 34 + F 23
identifier-shaped, +25 frame-overridden, +9 singles; 67 Reserved skips, 25 opcode
value-rows (`01h . COMPLETION_WAIT command number.`) + 36 name-less = honest residuals).
**Measured cross-surface deltas (the shared-chain extension improves earlier slices —
document, per-item verify, re-measure golds):** `.10b` AMD plain rows gain D/E/F names
(the documented 82 fields GROW — `GDeviceID[15:0]:` class was an explicit `.10b`
residual); NVMe possibly +1 (`Operation:` — verify its table passes the gate); `.10a`
CCIX versions gain ~5–10 Form-F names each out of their honest residual pool; NVMe
register gold expected UNCHANGED (different surface — re-measure to prove). Build
verification plan: 12-doc parity (manifest-only delta), NVMe-216/AMD-82 re-proof with
documented deltas only, register golds + battery + kg-bench + full `scripts/run_ci.sh`,
stub-protocol live AMD run with per-item eyeball of all 16 chains. Honest boundary
unchanged: symbolic cells, two-word names (`Store Address` — 4 rows), value-rows, and
lost-caption chains stay residuals; `byte_offset` is recorded verbatim-literal, never
inferred.

**`PDF-VARIANT-DIGESTION.10e` — the `byte location | size (bytes) | register description`
placement-table family (CCIX-class).** · Status: **DONE `2026-06-11`** (probe `531c10cd` →
measured design → build → live per-item verification; one post-probe grammar decision made
during live verification, below). Spun from the `.10a` quantified residuals, which recorded the hypothesis
"register-AT-OFFSET placement maps → future lever: map them to register-map records".
**PROBE RESULT (per-item over the persisted corpus): the recorded hypothesis is
OVERTURNED — these are NOT register placement maps.** The family is **exactly 60 tables,
CCIX-ONLY (15 per version × 4), 236 raw rows** (census gate: effective `unknown`-kind
header carrying a `byte location` column + a `size`-led column + a `description`-containing
column; two signature variants `attributes`/`attribute` — the column is never required).
Every caption is a STRUCTURE caption (`CCIX PER Memory Error Type Structure`, `Cache/ATC/
Port/Link/Agent Internal Error Type Structure`, `Vendor-Specific Log Info`) — **0 of 60
captions carry the word register**; the rows describe byte-granular fields of in-memory/
in-log error-record structures (CPER-style), with `M/O` (mandatory/optional) record
vocabulary. Per the `.10c` caption-evidence rule the typed home is the **`.10b` structure
surface (`message_field_records`)** via a NEW third strategy
`message_fields.byte_location_table` — claiming MMIO register semantics would fabricate
(the `Attributes` RO/RsvdZ column does not outvote the document's own caption; FIELD.2's
access-column routing predates the stronger `.10c` per-table caption doctrine).
**Row grammar (measured per-item, 236 rows):** 228 eligible (first cell a decimal int —
the byte location); 8 wrapped-prose rows (non-int first cell: continuation text of the
PREVIOUS row's description) are row-level skips, NOT whole-table rejections (unlike the
`.10b` bit-cell rule: a wrapped row carries no placement data, so skipping cannot
misplace anything); sizes 200 int + 28 symbolic (`(indicated by VenLen)` variable-length
tails → width honestly absent); 24 `Reserved and Zero` padding skips.
**Capture shape (decided by precedent, recorded):** `byte_offset` = the literal byte
location (additive doc amendment: with `bit_range: None` the offset is the FIELD's own
byte offset within its container — the `.10d` dword-relative reading needs `bit_range:
Some`); `bit_width = size_bytes × 8` for plain-int size cells (exact unit arithmetic on a
document-stated value — the same class as `.10b`'s `high - low + 1`, and unlike the `.10d`
position derivation it cannot misrepresent), `None` on symbolic sizes; `bit_range` stays
`None` (no bit positions are stated — never derived). No schema change.
**Name forms (census over all 204 non-Reserved eligible rows — OTHER = 0):** paren+frame
12 (`Card or Channel Number (Chan) This field …` → the existing shared form yields the
document's own mnemonic `Chan`/`Mod`/`VenLen`); **head-before-definitional-frame 189**
(NEW, family-LOCAL): the field name is the text before the measured frame set `This
field/bit/value/register/structure …` / `This is …` / `All subsequent fields …`
(`Validation Bits`, `Operation Type`, `Memory Pool Generic Memory Type Capability`, `Row`,
`Set`, `Way` — multi-word English heads the bare leading-identifier form rightly rejects),
gated: ≤8 words, starts with a letter, and NO sentence period inside the head (the `.10c`
period-is-bleed precedent — kills exactly the 2 measured wrapped-bleed captures `All other
values are reserved. Cache Error Type|Operation Type`, which stay honest residuals rather
than minting a post-period grammar); bare-short-cell 3 (`Device`, `Memory Error Type` ×2 —
the description page-wrapped away; ≤6 words, no period, not Reserved-led). The form is
deliberately NOT added to the shared `recover_field_mnemonic` chain: the family gate
admits only these 60 tables corpus-wide (leak-proof by construction), and the shared
bare leading-identifier form would TRUNCATE measured heads whose first token is
identifier-shaped (`FRU ID`→`FRU`, `CCIX Message`→`CCIX`, `ATC Instance ID`→`ATC`), so
the family-local chain runs paren+frame → head-before-frame → bare-short and never the
bare-identifier form.
**Chains (byte-exact adjacency, measured over all 4 versions — zero gap/ambiguous cases):**
a capless table joins the open chain iff page distance ≤ 1 AND its first eligible offset
== the chain end (last eligible row's `offset + int size`); a symbolic-size tail closes
the chain (end unknowable → nothing adopts after it — measured: every symbolic size is a
genuine variable-length structure tail); fresh structures always restart at offset 0;
every captioned table starts at 0 (no head-adoption case exists in this family). 31 of 31
capless continuations join EXACT; chain label via the existing `.10b`
`bit_position_container_label` caption reader (no `Fields` suffix in this family → the
label is the full structure name verbatim); a chain with no captioned member yields
nothing. GUARD (measured-zero, future-doc honesty): a caption that grounds a register
identifier per the `.10c` caption grammar yields NOTHING for its chain (unmeasured
territory stays residual rather than mislabeled as a structure).
**Quantified honest residuals:** the lost-caption chains — Port `7-9` in ALL four versions
(caption lost in ingest; ~2 tables / ~7 rows each), plus r1.0a's ATC `7-8` + Log-Info
`7-12` and rev1.1's Cache `7-7` (the re-ingest lever, same class as `.10b`/`.10c`); the 2
period-bleed rows; the 8 wrapped-prose rows; 24 Reserved skips. Expected yield ≈ 40–50
fields / 4–6 containers per version (≈190 fields across the 4 CCIX docs).
**Build-safety facts (verified in code before building):** `is_bit_position_header` does
NOT match `byte location` (no current gate touches the family — the `.10a` negative-twin
test locks the register-surface exclusion); the field-titled reader needs a
fields-anchoring caption (these say `Structure`/`Log Info`) — the family is extracted by
NOTHING today; none of the 12 intact-bundle docs carry the family (census), so parity =
byte-identical everywhere with ONLY the manifest gaining the new strategy entry (the
`.10b`/`.10c` precedent); canonical CCIX EvidenceIRs are host-local-blocked → live
verification = the `.10a` stub-copy protocol, canonical untouched.
**Verification plan:** hermetic tests (gate incl. negative twins, name forms incl. the 2
bleed rejections + truncation-hazard cases, symbolic-size width absence, chain joins +
closed-tail + capless-chain-yields-nothing + register-caption guard); 12-doc parity
sweep (manifest-only delta); `.10b`/`.10d` NVMe-216/AMD-217 dry-run re-proof unchanged;
register golds + battery + kg-bench + full `scripts/run_ci.sh`; stub-protocol live run on
all 4 CCIX versions with per-item eyeball of every container/field; book + KM card.
**BUILD + VERIFICATION LOG (DONE `2026-06-11`):** lib tests 1565→1567 (2 new hermetic:
the full name-form grammar — paren-mnemonic incl. the bleed-recovery case, all three
truncation-hazard heads kept whole, both period-bleed refusals, frame-at-start, bare-short
accept + too-long reject, Reserved, letter-led; and the end-to-end surface test —
captioned head + byte-adjacent capless join, wrapped-prose row skip, symbolic-size tail
with honest `bit_width: None` closing the chain, capless fresh chain → nothing,
register-worded caption → nothing, kind-gate exclusion, `Attribute` singular variant,
manifest entry `message_fields.byte_location_table` produced=4); kg-bench **154/154**;
full `scripts/run_ci.sh` GREEN. **Implementation shape:** family-local collector
(`byte_location_layout_columns` header-position gate — `byte location` + `size`-led +
`description`-containing, distinct; attribute/M-O never required), row-level wrapped-prose
skips (deliberately NOT the `.10b` whole-table rule — a wrapped row states no placement,
skipping cannot misplace), `BitLayoutLabel`/`bit_layout_labels_agree`/
`bit_position_container_label` reuse with the `.10c` register-worded honesty guard,
byte-exact stitcher (`byte_location_chain_adjacent`), family-local
`byte_location_field_name` (NEVER the shared `recover_field_mnemonic` — truncation
hazard), `MessageFieldRecord` doc contracts amended additively (name multi-word note;
`byte_offset` dual reading disjoint on `bit_range`; `bit_width` byte-size×8 arithmetic).
**POST-PROBE GRAMMAR DECISION (made during live per-item verification):** r1.0a's wrapped
cells fuse the previous row's trailing enum sentence ahead of the row's own definition
(`All other values are reserved. Card or Channel Number (Chan) This field …`) — the
probe's PAREN regex had admitted that row, the build's stricter period gate initially
refused all 3 bleed rows. Decision: the trailing paren+frame anchor is trusted PAST bleed
(it is the row's own definition wherever the wrap put the prose — the exact trust the
shared mid-cell paren form grants), recovering `Chan@11` in r1.0a; the 2 head-only bleed
rows stay refused (no anchor separates bleed from name). Fixture-locked both ways.
**Live (stub protocol, canonical untouched, final post-fmt binary): rev2.0 0→45 fields /
6 containers; r1.0 47→92 (+45/6); r1.0a 50→86 (+36/4 — ATC + Log Info captions lost in
this version); rev1.1 51→86 (+35/5 — Cache caption lost); every pre-existing record
preserved bit-for-bit (old-vs-new record-set diff per doc); per-item eyeball of all 161
fields clean** (`Validation Bits@0/32b`, `FRU ID@4/8b`, `Chan@11/8b`, `Mod@12/16b`,
`CCIX Message@12/256b`, `Vendor-Specific Log Info@35` width-less, `VenLen@0/16b`); the
only differing top-level keys per doc = `message_field_records` + `extraction_manifest`.
**12-doc old-vs-new parity sweep: every extraction surface byte-identical on all 12
intact docs; the ONLY delta is the manifest's new `message_fields.byte_location_table`
entry (eligible, produced 0)** — golds locked by byte-identity (canonical artifacts +
eval code untouched). Residual reconciliation: 204 non-Reserved eligible rows = 161
extracted + ~40 in lost-caption chains (Port 7-9 ×4, ATC+LogInfo r1.0a, Cache rev1.1 —
the re-ingest lever) + 2 period-bleed + Chan recovered; 24 Reserved + 8 wrapped-prose
rows skipped by design. Book: `pipeline/evidenceir.md` `.10e` subsection + the `.10a`
closing corrected; KM card `byte-location-structure-field-extraction` + the `.10a` card
residual line corrected; README `.10e` bullet; RUST_CODEBASE_ANALYSIS third-strategy
note.

**`PDF-VARIANT-DIGESTION.10f` — the section-heading PROSE message-field family (DTI-class
<!-- active-task-source-region:activity-10d-10e:end -->

