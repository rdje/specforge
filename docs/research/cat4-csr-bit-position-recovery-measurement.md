# Cat-4 RISC-V CSR bit-position recovery — feasibility measurement (`DOC-INTENT-TAXONOMY.4d.i`)

- Date: `2026-06-23`
- Leaf: `DOC-INTENT-TAXONOMY.4d.i` (measurement / decision packet — read-only, docs-only, **no Rust code**)
- Spun out of: `DOC-INTENT-TAXONOMY.4d` (cat-4 CSR lowering decision — the register gap is extraction recall)
- Reproducer: `scripts/measure_cat4_csr_bit_recovery.py` (deterministic, read-only)
- Related: `[[register-diagram-bit-recovery-via-tiling]]`, `[[register-field-table-defragmentation]]`,
  `[[cat4-isa-csr-lowering-decision]]`, `[[register-bit-field-isf-lowering-gap]]`

## Question

`.4d` resolved that cat-4 (CPU-ISA) CSRs **reuse** the existing ISF register/storage abstraction (no new
construct) and that their bit-fields lower automatically through `.4a.ii` **once located** — so the cat-4
register gap is *extraction recall*, not abstraction. `.4d.i` was spun out as "the genuine buildable cat-4
lever": recover RISC-V Debug's `0/179` unlocated field bit positions **deterministically** by parsing the
register bit-layout column/diagram into `bits_high`/`bits_low`, joining to the field-description table by name
(diagram-*shape* grammar, ADR 0006), and add a RISC-V-shaped register recogniser for AIA.

The `.4d.i` **pre-investigation** (`2026-06-23`, read-only) reported a feasibility verdict of *"deterministically
tractable (no VLM strictly needed) — the [Docling-flattened diagram] table carries the positions"*, while
flagging it intricate and regression-sensitive. **This leaf tests that verdict against the actual source data
and the human-reviewed bit gold before any code is written** (`[[feedback_scoring_rigor]]`: every claim
objectively measured + demonstrated per-item). The finding **overturns** the pre-investigation's optimism.

## Method

Read-only over the persisted RISC-V Debug bundle
(`generated/source_ir/1_0_risc_v_debug_specification/normalized/…md`, present — no re-ingest) and the
human-reviewed gold (`crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json`,
`dmcontrol` 14 fields / `dmstatus` 20 fields). Per the reproducer:

1. classify every register-with-address heading by what *modality* carries its bit layout (an `![Image]`
   diagram, a `Field|Description` table, and/or a Docling-flattened diagram **table**);
2. check, per-item against the gold, whether the flattened tables that exist actually carry the **correct**
   bit positions;
3. profile the AIA second sub-lever.

## Results

### Result 1 — the bit positions live in the IMAGE, not in any text table (53 of 56)

| modality next to a register heading | count (of 56 register-with-address headings) |
| --- | --- |
| `![Image]` bit-layout diagram | **53** |
| `Field \| Description \| Access \| Reset` table | 34 |
| Docling-**flattened** bit-diagram **table** | **7** (`dmcs2`, `dmstatus`, `dscratch1`, `tdata1`, `textra32`, `tinfo`, `tmexttrigger`) |

The bit layout is an **image** for the overwhelming majority. The diagram-as-text-table is the rare exception
(7/56), and — as Result 2 shows — even those are unreliable. This is exactly why `EXTRACTION-GAP-FIX.4` built a
**VLM** reader for this modality: the fact genuinely lives in a non-text modality
(`[[register-diagram-bit-recovery-via-tiling]]`).

### Result 2 — per-item gold check: the flattened tables that exist are garbled or symbolic

- **`dmcontrol`** (the cleanest register — 14 fields, **no reserved gaps**, the case `register_bits.rs` tiles
  to `14/14` exact gold via the VLM): its bit-layout diagram is captured **only** as
  `![Image](assets/picture-0019.png)`. **There is no flattened table to parse at all** — a deterministic
  text-table reader has nothing to read; the bits are in the image.
- **`dmstatus`** (20 fields, reserved gaps): a flattened table **exists but is garbled**. Decoding it against
  the gold:
  - its **explicit high-bit row** reads `17|16|16|15|14|13|12|12|11`, but the gold high bits are
    `ndmresetpending=24, stickyunavail=23, impebreak=22, allhavereset=19, anyhavereset=18` — the upper half is
    **off by ~8** (it places `ndmresetpending` at bit 16, gold 24);
  - a whole **middle band is dropped** — the 7 fields `allresumeack`(17)…`allrunning`(11) are absent from the
    diagram-table name rows (the PDF reader lost that band, the same loss `quality/extraction-eval.md`
    documents);
  - the upper-half *widths* (`7|1|1|1|1|2|1|1|1`) tile correctly from MSB while the upper-half explicit
    positions are wrong, and the lower-half explicit positions are correct — so **two contradictory decode
    rules** would be needed, with doubled cells (`ndmresetpending`×2, `allhavereset`×2) and two stacked
    half-rows in between;
  - the result: 13 diagram fields vs 20 field-table fields → the anti-fabrication **name-multiset gate**
    (`proposal_names_match_fields`) yields an honest residual, recovering nothing.
- **`tdata1`**: a flattened table exists but its positions are **symbolic XLEN-relative** (`XLEN-1`, `XLEN-5`;
  the `data` field width is literally `XLEN - 5`). RISC-V CSRs are XLEN-parameterized, so the bits are not
  concrete integers; resolving them deterministically would require assuming XLEN (32 vs 64), which fabricates
  the layout for the other XLEN.

### Result 3 — the tiling gates do not validate field ORDER (a deterministic-table reader is strictly more dangerous)

The proven recovery core (`ir/register_bits.rs`, `EXTRACTION-GAP-FIX.4`) gates on (a) the widths summing to a
standard register width and (b) the name multiset matching the field table — but it reconstructs positions from
**order + widths** and **does not** validate that the order is correct. The VLM reads fields in their visual
left-to-right diagram order, so that order is trustworthy. A Docling-flattened table, by contrast, **row-jumbles**
the diagram (the `dmstatus` doubled cells + two stacked halves prove it), so a deterministic table reader could
present a *wrong* field order whose widths still sum to 32 and whose names still match — passing **both** gates
while emitting **wrong bits**. So feeding the flattened table into the existing gates is not just low-yield, it
is a *new* fabrication path the VLM front-end does not have.

### Result 4 — the AIA second sub-lever is blocked + prose-bound

RISC-V AIA's normalized bundle is **ABSENT** (re-ingest is RAM/Docling-gated under `CORPUS-COVERAGE`), so its
table shapes cannot even be examined here; its persisted IntentIR carries **0 `register_records`** and its
IMSIC/APLIC CSR intent sits in **39 prose `conditional_rules`** (211 empty prose "interfaces"). A
RISC-V-shaped register *recogniser* therefore cannot be built or tested now, and the underlying intent is prose,
not a register table.

## Decision

**The `.4d.i` deterministic bit-position-recovery lever is NOT viable; cat-4 register bit recall stays an honest
residual.** Per-item, gold-checked:

1. The bit positions live in the **image** modality for 53/56 registers (incl. the cleanest gold register
   `dmcontrol`, image-only). A deterministic *text-table* parser has nothing to read for the vast majority.
2. The 7 flattened tables are **garbled** (`dmstatus`: off-by-8 + dropped band + contradictory decode rules) or
   **symbolic XLEN-relative** (`tdata1`). A deterministic parse would recover ~0 correct fields and/or
   **fabricate** wrong bits — a direct breach of the honesty guardrail (`[[feedback_isf_no_hacks]]`).
3. The tiling gates do not validate field order, so the flattened-table path is *strictly more dangerous* than
   the existing VLM front-end (Result 3) — and it modifies the shared register path that `.4a.ii`
   (24 docs / 6,570 emitted fields) depends on, so the regression risk is high for ~0 gain.
4. The **genuine lever already exists**: the VLM+tiling reader (`recover-register-bits` / `ir/register_bits.rs`)
   already covers all 44 de-fragmented registers and recovers `0` / residual `44` / **zero fabrication**, bound
   **purely by VLM read accuracy** — the local `qwen2.5vl:7b` is not precise enough on these dense diagrams
   (`[[register-diagram-bit-recovery-via-tiling]]`). The lever is a **sharper VLM read** (stronger/cloud model,
   image upscaling, voting, tighter prompt), **owned OUTSIDE the `.4` ISF-lowering program** — it is neither an
   ISF-abstraction gap nor a deterministic-extraction gap.
5. The AIA sub-lever is **blocked on RAM/Docling-gated re-ingest** (`CORPUS-COVERAGE`) and is **prose-bound**
   (CSR intent in `conditional_rules`), not a table/diagram parse.

**No Rust code, no FSMGen FR** — ISF already expresses register fields via `.4a.ii`; the gap is recall, the VLM
machinery exists, and a speculative parser would fabricate (`[[feedback_verify_fsmgen_before_fr]]` /
`[[feedback_isf_no_hacks]]`). This mirrors `.4c.i` (cat-3 topology: capture-recall-gated, no FR, honest
residual, lever owned outside `.4`) and the cat-2 structure-recall frontier — recorded as a cross-reference,
NOT minted as a `.4` ISF-lowering gap (`[[feedback_scoring_rigor]]`). With `.4d.i` resolved, the
`DOC-INTENT-TAXONOMY.4` actionable frontier is exhausted except the FSMGen-gated `.4b` (packet/flit structures).

## Genericity (ADR 0006)

The measurement keys off structural document shape — register-with-address headings, image-vs-table modality,
and the universal anti-fabrication tiling gates — and the human-reviewed bit gold; **no RISC-V register-name
list**. The decision is to NOT build a parser, so there is no runtime code to overfit.
