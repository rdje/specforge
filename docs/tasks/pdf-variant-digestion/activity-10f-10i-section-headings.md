# PDF-VARIANT-DIGESTION — activities 10f–10i section headings

- Part ID: `activity-10f-10i-section-headings`
- State: `legacy`

<!-- active-task-source-region:activity-10f-10i:start -->
message protocols).** · Status: **DONE `2026-06-17`** (probe → scoping correction → measured
design → build → live per-item verification + old-vs-new parity).
Spun from `KG-ISF-COMPLETENESS.4`'s §spun-out gap: DTI `ihi0088` is a message protocol
(`DTI_TBU_TRANS_REQ` …) but carries **0 `message_field_records`**, so 30 field obligations
leak into `signal_constraints` with dotted/undeclared subjects (`DTI_TBU_TRANS_REQ.MMUV`,
`ATTR_OVR.MTCFG`). FIELD.4 was built to route exactly these, but only fires for
*catalog-declared* fields — and the `.10a`–`.10e` table readers see nothing because DTI's
field layout is **not in caption-anchored tables**.
**SCOPING CORRECTION (measurement-first, read-only over all 79 persisted `source_ir`):** the
`KG-ISF-COMPLETENESS.4` scoping said the field defs live in prose `list_item`s of the form
`"<Field>, bit [N]"`. **Re-measured on the artifact — they do NOT: DTI defines each message
field as its OWN section HEADING** of the form `<NAME>, bit [N]` / `<NAME>, bits [hi:lo]`
(`STAGES, bits [27:26]`, `SPD, bit [25]`, `M_MSG_TYPE, bits [3:0]`, `IMPLEMENTATION DEFINED,
bit [7]`), grouped under a dotted-numbered message container (`3.1.1 DTI_TBU_CONDIS_REQ`)
that carries a `Field descriptions` anchor sub-heading. `list_item` matches are ~0; the
`body_text` matches are descriptive prose (`Messages with bits [3:0] equal to 0xE …`) and
cross-references — exactly the noise the section-heading restriction excludes. This is a
CLEANER, more precise anchor than free prose.
**GENERALITY (section-heading form, all 79 docs):** the form appears as section headings in
**8 docs** — GIC `ihi0069` 612, SMMU `ihi0070` 434, DTI `ihi0088` 163, ARM-Debug-v6
`ihi0074` 138, CoreSight `ihi0029` 42, ACC `ihi0076` 5, + 2 singletons (1396 total). But the
**typed home varies by container** (the `.10c` caption-decides principle): DTI's container is
a MESSAGE (`DTI_TBU_CONDIS_REQ`, sub-structure `Source`/`Usage constraints`/`Flow control
result`), while GIC/SMMU/CoreSight/ACC/ARM-Debug containers are REGISTERS (`SMMU_IDR0` /
`AUTHSTATUS, … Register`, sub-structure `Purpose`/`Attributes`/`Accessing`).
**TYPED-HOME ROUTING (the `.10b`/`.10c`/`.10e` rule — register iff register-attribute
vocabulary; ADR 0006, no name list):** a field-def container routes to **`message_field_records`**
iff it is NOT a register — its caption has no `register` word AND its sub-heading run carries
no `Attributes`/`Accessing` marker. The REGISTER-routed containers (GIC/SMMU/…) are an
explicit honest residual deferred to a sibling leaf (`.10g`, section-heading register fields
→ the register surface) — NOT captured here, so no measured register gold is touched.
**Measured gate (each demonstrated per-item over the persisted corpus; ADR 0006):** (1)
field heading matches `<IDENT>, bit[s] [range]` with an identifier-shaped name (admits the
value-slice form `TOK_TRANS_REQ[11:8]` and multi-word ALL-CAPS `IMPLEMENTATION DEFINED`;
the `^…$` anchor + section-heading kind rejects descriptive prose and cross-refs); (2)
container = nearest preceding dotted-numbered heading whose leading token is identifier-shaped,
and is NOT a register (caption-`register` / `Attributes` / `Accessing`); (3) container carries
a `Field descriptions` anchor heading; (4) ≥2 field headings in the container (a real layout
has multiple fields — kills singleton coincidental matches). **Bit overlaps are KEPT, not
rejected:** DTI documents Manager-side and Subordinate-side views of the same position
(`M_MSG_TYPE[3:0]` / `S_MSG_TYPE[3:0]`) and multiple message variants in one container — a
no-overlap gate would wrongly drop legitimate dual-perspective fields; same-name duplicates
merge via the surface `(container, name)` dedup.
**Measured outcome (live, the built extractor over the persisted corpus): DTI 0 → 159 message
fields / 17 message containers**; **all other 78 docs fire 0 message fields** (GIC/SMMU/CoreSight/
ACC/ARM-Debug all route REG) → corpus-precise, **1/79 docs**. Recovered names match the leaked
obligation subjects (`MMUV`, `SEC_SID[1]`, `STAGES`, `M_MSG_TYPE`, `PARTID[3:0]`). Two
measured name-cleanliness gates were added from the live per-item eyeball (each ADR-0006
structural, no name list): a Docling spacing artifact `<ident> [slice]` is normalized to
`<ident>[slice]` so a field tokenized both ways merges to ONE record, and a name that is exactly
the bit-position UNIT word (`Bits, bit [5:4]`) is an unnamed/reserved range, NOT a mnemonic →
dropped (honest residual). The ≥2-field gate counts DISTINCT names.
**Build:** additive `extract_section_header_message_fields` + helpers (`parse_dotted_container_heading`,
`parse_section_header_field`, `normalize_section_field_name`, `is_section_header_field_name`,
`is_message_container_name`, `is_field_descriptions_anchor`, `caption_names_register`,
`is_register_attribute_heading`) in `ir/evidence.rs`, registered as the 4th strategy
`message_fields.section_header_field` in `message_field_surface`; reads `source_ir.document_sections`
(section headings in reading order), reuses `parse_pure_bit_position` + `bit_width_from_range`.
**FIELD.4 routing (honest):** this slice closes the RECOGNITION gap (the DTI field catalog goes
0 → 159). With `message_field_records` populated, the LLM-primary constraint extractor
(`extract-constraints-llm` — the default for live-NLP `converge`, `LLM-PRIMARY-PROMOTION.5`)
classifies a DTI field obligation as `EntityType::Field` → `message_field_constraints` instead of
`signal_constraints`. The deterministic Pattern `signal_constraints` surface does NOT consult the
catalog (by construction), so the currently-persisted leaked DTI constraints are corrected on the
next live-NLP DTI rebuild (its normalized bundle is host-local-blocked, re-provided like
`.10a`/`.10c`/`.10e`) — not retroactively in this slice.
**Verification log (`2026-06-17`):** lib tests **1664 → 1672** (7 new: DTI-shape positive,
register-container exclusion [caption-`register` + `Attributes`], anchor+≥2 gate, M/S overlap
kept, spacing-normalize + unit-word drop, `parse_section_header_field` forms/rejections,
`parse_dotted_container_heading` forms, surface field-id+manifest; + 1 `#[ignore]` corpus-sweep
local measurement). **Old-vs-new `evidence --dry-run` parity over 7 intact docs (APB/AHB/AXI/
AXI-Stream wire gold + NVMe 216 + AMD 217 message-field gold + ARM-Debug-v6 register-routing):
every surface byte-IDENTICAL, the ONLY delta is the manifest gaining `message_fields.section_header_field`
(eligible, produced 0)** — NVMe 216→216, AMD 217→217, ARM-Debug 0 (its 138 register-routed headings
correctly mint 0 message fields). **WIRE-BASED-100 provably orthogonal** (the 4 wire docs carry no
section-heading field form → 0 records, byte-identical). `kg-bench` **156/156**; full
`scripts/run_ci.sh` GREEN (fmt + clippy `-D warnings` + lib 1672 + rustdoc + mdBook). Book
`pipeline/evidenceir.md` `.10f` subsection; KM card `section-header-message-field-extraction`;
README `.10f` bullet; RUST_CODEBASE_ANALYSIS fourth-strategy note. The REGISTER-routed
section-heading field form (GIC 612 / SMMU 434 / CoreSight 42 / ACC 5 / ARM-Debug 138 → the
register surface) is the spun-out sibling lever `.10g` (honest residual; ~1230 register fields).

- ID: `PDF-VARIANT-DIGESTION.10g` · Status: `done` (`2026-06-17`, CODE) · Goal: **the REGISTER-routed twin of `.10f`** — recover the same
  `<NAME>, bit[s] [range]` section-heading field layouts, but for containers that ARE registers
  (caption-`register` OR an `Attributes`/`Accessing` sub-heading), emitting them to the **register
  surface** (`RegisterRecord` / `register_records`) instead of dropping them. AGNOSTIC (ADR 0006:
  universal section grammar, no chip-name list), and WITHOUT regressing the wire golds, the
  register golds (NVMe / CCIX / RISC-V Debug), DTI's `.10f` message-field surface, or `kg-bench`.
  **No-drift design:** factor `.10f`'s container-walk into ONE shared classifier
  (`scan_section_header_field_containers` → `{name, is_register, has_anchor, fields}`), so the
  message-vs-register routing is decided in exactly one place; `.10f` keeps the
  `!is_register`-routed containers, `.10g` keeps the `is_register`-routed ones (mirror of the
  `.10a` "one matcher, cannot drift" precedent).
  **Probe (`2026-06-17`, faithful re-derivation of the `.10f` predicates over all persisted
  `source_ir`, read-only, BEFORE coding):** register-routed containers (register + `Field
  descriptions` anchor + ≥2 distinct field headings) = **GIC `ihi0069` 73 / SMMU `ihi0070` 88 /
  CoreSight `ihi0029` 8 / ACC `ihi0076` 2 / ARM-Debug `ihi0074` 26 = 197 containers / ~1001
  fields**; **DTI fires 0 register / 18 message** (its containers are messages → routed away by
  `.10f`), and **NVMe / CCIX / RISC-V Debug / APB / AHB / AXI / AXI-Stream / AMD-IOMMU all fire 0
  register-routed containers** (they carry no section-heading field form — the `.10f` byte-identity
  proof, now confirmed for the register surface too).
  **PRECISION CRUX — duplicate short mnemonics (the decisive design finding):** ARM-Debug reuses
  `AUTHSTATUS` (×3) / `CLAIMSET` (×4) / `DEVARCH` (×2) / `IDR` (×2) / `CSW` (×2) and CoreSight
  reuses `AUTHSTATUS` (×3) across access-port blocks — the dotted heading carries only the SHORT
  name, not the block. The occurrences are a MIX of identical cross-references (`DEVARCH`/`IDR`),
  subset views (`AUTHSTATUS`), and GENUINELY DIFFERENT registers (`CSW` MEM-AP vs JTAG-AP have
  disjoint field sets). Letting the existing `consolidate_register_field_fragments` all-distinct
  merge run on them would CONFLATE two different registers into one fabricated mega-register, and
  the identical/subset dups would OVER-COUNT — both violate north-star bar #1 (every register real,
  complete AND exclusive). **DECISION (honest, ADR-0006-structural):** `.10g` emits a register only
  for a register-routed container whose name is UNIQUE within the document; a name reused across
  ≥2 register containers is structurally ambiguous → held as an honest residual (deferred to a
  future block-qualified lever), never over-counted or conflated. Affects only ARM-Debug (13
  containers / 5 names) + CoreSight (3 / 1 name); GIC 73 / SMMU 88 / ACC 2 are fully clean.
  **Collision with the existing register surface (the MEMORY precision crux):** a `.10g` register
  whose unique name matches an existing 0-field record (e.g. ARM-Debug `DPIDR`, minted name-only by
  the map/summary strategies) MERGES via the existing `consolidate_register_field_fragments`
  post-pass (0 fields are trivially distinct → safe → one enriched record, existing identity kept
  since `.10g` is the 4th strategy) — no double-count by construction.
  **Build plan:** new `extract_section_header_registers` + `SectionHeaderRegisterExtractor`
  (`name() = "registers.section_header_field"`), registered as the 4th register strategy in
  `register_record_surface` via `run_surface_concat`; each emitted `RegisterRecord` carries the
  dotted-heading name, fields with `(bits_high, bits_low, bit_width)` from the heading, and
  `offset_address` / `access_type` / `reset_value` / `description` HONESTLY absent (`None`) +
  `supporting_statement_ids` empty + `Medium` confidence (mirror of `.10c`). Acceptance:
  per-doc emit measured live; **DTI register_records byte-identical**, register/wire golds
  byte-identical, `.10f` message-fields byte-identical (one shared walk); hermetic positive +
  duplicate-name-residual + DTI-message-exclusion + gold-zero tests; full `scripts/run_ci.sh`
  GREEN + `kg-bench` 156/156; book `pipeline/evidenceir.md` + KM card + README/RUST/CHANGES/
  DEVELOPMENT_NOTES/LIVE_ACHIEVEMENT_STATUS refreshed.
  **Verification (`2026-06-17`):** built as the no-drift refactor — `.10f`'s walk factored into the shared
  `scan_section_header_field_containers` + `distinct_section_header_fields`, then `extract_section_header_registers`
  + `SectionHeaderRegisterExtractor` (`registers.section_header_field`) registered 4th in `register_record_surface`
  (`run_surface_concat`); a `type SectionHeaderRegisterCandidate` alias keeps the nested tuple clippy-clean.
  **Live (`extract_section_header_registers` corpus sweep over persisted `source_ir`): GIC `ihi0069` 73/468,
  SMMU `ihi0070` 88/381, CoreSight `ihi0029` 5/24, ACC `ihi0076` 2/4, ARM-Debug `ihi0074` 12/57 = 180 registers /
  934 fields across EXACTLY 5 docs**; every other doc 0; DTI 0 registers (message-routed) + 159 `.10f` message
  fields unchanged. **Full ARM-Debug `evidence --dry-run`: register_records 29→37, fields 186→243, +8 brand-new
  (ABORT/BASE/CFG/DEVID/IDCODE/MEMTYPE/PRIDR0/TARGETSEL), `DPIDR` enriched to 4 fields keeping its existing record
  id (`reg_table_0044_005`) via the consolidation merge, ZERO duplicate names** — the name-uniqueness gate correctly
  dropped the reused `AUTHSTATUS`/`CSW`/`IDR`/`CLAIMSET`/`DEVARCH`. **`git stash` baseline diff over 8 golds (NVMe
  42 reg/216 msg, AMD 217 msg, APB, AHB 3 reg, AXI 71 reg, AXI-Stream, CCIX r1.0 131 reg/92 msg, RISC-V Debug
  44 reg): register_records + message_field_records byte-IDENTICAL; the ONLY top-level delta is
  `extraction_manifest` gaining `registers.section_header_field` (eligible, produced 0)** → WIRE-BASED-100 +
  register golds provably orthogonal; `.10f` byte-identical. 5 new hermetic tests (positive register capture,
  message-container exclusion, duplicate-name residual, anchor+≥2 gate, surface+manifest) + 1 `#[ignore]` corpus
  sweep. `kg-bench` **156/156**; full `scripts/run_ci.sh` GREEN (fmt + clippy `-D warnings` + lib 1672 → **1677**
  + rustdoc + mdBook). KM `section-header-register-field-extraction`; book `pipeline/evidenceir.md` `.10g`. The
  block-ambiguous reused-mnemonic registers (ARM-Debug `AUTHSTATUS`/`CSW`/`IDR`/…, CoreSight `AUTHSTATUS`) are an
  honest residual — a future block-qualified lever can disambiguate + recover them. Commit: pending (this slice).
  **(`.10h` `2026-06-24` recovered the identical/nested sub-class — `AUTHSTATUS`/`DEVARCH`/`IDR` — by field-set
  containment; only the genuinely-different `CSW`/`CLAIMSET` remain residual.)**
- ID: `PDF-VARIANT-DIGESTION.10h` · Status: `done` (`2026-06-24`, measurement-first, CODE — GO) · Goal:
  **BLOCK-QUALIFIED register-mnemonic recovery — the `.10g` honest residual.** `.10g` emits a
  register-routed section-heading container only when its short mnemonic is UNIQUE in the document;
  a mnemonic reused across ≥2 register containers (the same `AUTHSTATUS`/`CSW`/`IDR`/`CLAIMSET`/
  `DEVARCH` documented once per access-port block, the dotted heading carrying only the short name) was
  dropped as an honest residual, because an all-distinct merge would either CONFLATE two
  genuinely-different registers (`CSW` MEM-AP vs JTAG-AP have disjoint field sets) or OVER-COUNT
  identical/subset cross-references (`DEVARCH`/`IDR`).
  **SHIPPED:** `extract_section_header_registers` (`ir/evidence.rs`) replaces the `.10g` drop-all-duplicates
  gate with a per-document FIELD-SET-CONTAINMENT resolver. For a mnemonic reused across ≥2 register-routed
  containers, the new pure helper `collapse_section_header_register_identity` returns the SINGLE maximal
  occurrence iff every occurrence's field set (by uppercased field name) is a subset of it — identical
  cross-references + nested views of one register, shown with more or fewer implemented fields — collapsing
  to ONE record carrying that fullest occurrence's REAL layout (never a fabricated union); disjoint /
  partially-overlapping sets (≥2 genuinely-different registers under one mnemonic — MEM-AP `CSW` vs JTAG-AP
  `CSW`) have no common superset → `None` → honest residual. +3 hermetic tests (nested-view collapse /
  identical-cross-ref collapse / disjoint residual) + the existing duplicate-name test re-scoped to the
  disjoint residual + 1 `#[ignore]` corpus probe (`section_header_register_block_probe`); lib 1718 → **1721**.
  **Measurement (probe-first, read-only over persisted `source_ir`):** the reused mnemonics live in EXACTLY
  2 docs — ARM-Debug `ihi0074` (`AUTHSTATUS`/`CLAIMSET`/`CSW`/`DEVARCH`/`IDR`, 13 containers) + CoreSight
  `ihi0029` (`AUTHSTATUS`, 3); every other corpus doc has zero reused register-routed containers. Heading
  levels are FLATTENED to L1 (so there is no ancestor-block heading), but the dotted-number hierarchy + the
  parent section title (`C2.6 MEM-AP register descriptions` / `C3.5 JTAG-AP register descriptions`) are
  intact. The per-occurrence field-set audit cleanly separates the three sub-classes: identical (`IDR`
  C1.4≡C2.6, `DEVARCH` C1.4≡C3.5), nested (`AUTHSTATUS` {2}⊂{3}⊂{4}⊂{5} across chapters), disjoint (`CSW`
  MEM-AP 11-field vs JTAG-AP 7-field).
  **Acceptance checklist (TOOLBOX.md):**
  - [x] ROOT CAUSE (WHY + WHERE): the `.10g` per-document name-uniqueness gate in
        `extract_section_header_registers` (`ir/evidence.rs`) DROPS every reused mnemonic — the
        `section_header_register_corpus_sweep` probe shows ARM-Debug 12/57, CoreSight 5/24 with the
        duplicates absent. WHY a single drop is wrong: it conflates the genuinely-different (`CSW`) and
        over-counts the identical (`IDR`); the `.10g` design deferred disambiguation to this lever.
  - [x] ADDRESSED (verified, measured per-item): the containment resolver recovers the safe sub-classes.
        `section_header_register_corpus_sweep` ARM-Debug 12 → **15** regs / 57 → **69** fields
        (+`AUTHSTATUS`/`DEVARCH`/`IDR`), CoreSight 5 → **6** / 24 → **29** (+`AUTHSTATUS`); GIC 73 / SMMU 88 /
        ACC 2 byte-identical. Full `evidence --dry-run` register_records: ARM-Debug 37 → **40**, CoreSight
        26 → **27**; each collapsed record carries the maximal field set (`AUTHSTATUS` = HID,NSID,NSNID,SID,SNID)
        and `CSW`/`CLAIMSET` stay residual.
  - [x] NO REGRESSION (orthogonal): a `git stash` baseline-vs-change full-`evidence` diff over 8 docs — CCIX
        r1.0, NVMe (register golds), AXI, AHB (wire golds), GIC, SMMU, ACC (section-header no-dup), DTI (`.10f`
        message-field) — is **byte-identical**; the only 2 changed docs ADD records with ZERO removals and all
        37 / 26 baseline records byte-identically preserved. `kg-bench` **156/156**; full `scripts/run_ci.sh`
        GREEN (lib **1721**, fmt + clippy `-D warnings` + rustdoc + mdBook). ADR-0006 (universal field-set
        containment, no chip-name list).
  **Narrowed residual:** the ≥2-distinct-identity class (`CSW` MEM-AP/JTAG-AP disjoint + `CLAIMSET` mixed pair/
  4-field) stays an honest residual — block-qualification needs a clean per-occurrence block name the flattened
  heading hierarchy does not provide (the dotted-parent number is available but cryptic; a future sub-lever may
  qualify it). Book `pipeline/evidenceir.md` `.10h`; KM `section-header-register-identity-collapse`. Commit:
  `70ac8faa`. **(Residual RECOVERED by `.10i` `2026-06-24`: the parent SECTION TITLE — not the cryptic number —
  is the clean block name via the universal `<NUM> <BLOCK> register descriptions` grammar; `CSW`/`CLAIMSET`
  block-qualified.)**

- ID: `PDF-VARIANT-DIGESTION.10i` · Status: `done` (`2026-06-24`, measurement-first, CODE — GO) · Goal:
  **BLOCK-QUALIFIED recovery of the genuinely-different register class — the `.10h` residual.** `.10h`
  collapses a reused section-heading register mnemonic to ONE record only when every occurrence's field
  set is a subset of one maximal occurrence (identical/nested views); the genuinely-different class —
  disjoint or partially-overlapping field sets under one mnemonic — stayed a fully-dropped honest residual
  because an all-distinct merge would conflate two different registers. The `.10h` leaf deferred the clean
  qualifier ("the dotted-parent number is available but cryptic"). **The decisive `.10i` measurement (the
  `.10h` `#[ignore]` block probe, read-only over persisted `source_ir`) is that the cryptic number RESOLVES
  to a clean human block name through the parent SECTION TITLE**: every reused register lives under a parent
  heading of the universal form `<dotted-num> <BLOCK> register descriptions` — `C2.6 MEM-AP register
  descriptions` → `MEM-AP`, `C3.5 JTAG-AP register descriptions` → `JTAG-AP`, `C1.4 AP Register Descriptions`
  → `AP` — while a parent with no block token (`D4.5 Register descriptions`) honestly yields none.
  **SHIPPED:** `extract_section_header_registers` (`ir/evidence.rs`) now threads each container's dotted
  number (`SectionHeaderFieldContainer.dotted`, additive — `.10f` ignores it) and, when `.10h` containment
  returns `None`, BLOCK-QUALIFIES each occurrence: the new pure helper `derive_register_block_name`
  (parent title must be exactly `<single-token BLOCK> register description(s)` after its dotted number)
  resolves the occurrence's `dotted_parent` to its block via a `by_number` (dotted-number → title) map
  over `document_sections`; each occurrence carrying a block is emitted as `<NAME>@<BLOCK>` (a no-block
  occurrence stays residual, and a block shared by ≥2 still-disjoint occurrences re-runs containment within
  the block, else residual). The rule is universal section grammar over the document's own block headings,
  NOT a chip-name list (ADR 0006); `@`/`-` are sanitized to a valid identifier downstream by the `.isf`
  emitter (`csw@mem-ap` → `csw_mem_ap`, kept distinct from `csw_jtag_ap`).
  **Measurement (probe-first, read-only over persisted `source_ir`):** the genuinely-different class lives
  in EXACTLY 1 doc — ARM-Debug `ihi0074`: `CSW` MEM-AP `{AddrInc,DbgSwEnable,DeviceEn,ERRNPASS,ERRSTOP,Mode,
  Prot,SDeviceEn,Size,TrInProg,Type}` (11, parent `C2.6 MEM-AP register descriptions`) vs JTAG-AP
  `{PORTCONNECTED,RFIFOCNT,SERACTV,SRSTCONNECTED,SRST_OUT,TRST_OUT,WFIFOCNT}` (7, parent `C3.5 JTAG-AP
  register descriptions`) — disjoint; `CLAIMSET` three identical `{Claim tag 0,Claim tag 1}` under AP
  (`C1.4`), MEM-AP (`C2.6`), JTAG-AP (`C3.5`) + one disjoint 4-field occurrence under `D4.5 Register
  descriptions` (no block token). CoreSight `ihi0029`'s only reused name (`AUTHSTATUS` ×3, all ⊆ the
  5-field `B2.3.1`) is containment-collapsible → handled by `.10h`, never reaches `.10i` → ihi0029 byte-identical.
  **Acceptance checklist (TOOLBOX.md):**
  - [x] ROOT CAUSE (WHY + WHERE): `extract_section_header_registers` drops the genuinely-different reused
        mnemonics because the `.10h` containment resolver returns `None` and there was no fallback; the block
        qualifier the document carries (the parent section title) was never threaded into the container scan.
  - [x] ADDRESSED (verified, measured per-item): `section_header_register_corpus_sweep` ARM-Debug
        15 → **20** regs / 69 → **93** fields (+`CSW@MEM-AP`{11}, `CSW@JTAG-AP`{7}, `CLAIMSET@AP`{2},
        `CLAIMSET@MEM-AP`{2}, `CLAIMSET@JTAG-AP`{2}); the no-block `D4.5` CLAIMSET stays residual; CoreSight
        unchanged at 6/29. Full `evidence` register_records ARM-Debug 40 → **45**.
  - [x] NO REGRESSION (orthogonal): a `git stash` baseline-vs-change full-`evidence` diff over the 8 docs the
        `.10h` checked (CCIX r1.0, NVMe, AXI, AHB, GIC, SMMU, ACC, DTI) + CoreSight `ihi0029` is byte-identical;
        ihi0074 ADDs 5 records with ZERO removals (all 40 baseline records preserved). ARM-Debug `.isf`
        re-emit passes FSMGen `--strict --check` with 0 new diagnostics (the block-qualified registers
        sanitize to distinct identifiers; FSMGen `--strict --check --json` → 0 diagnostics / 0 errors).
        `kg-bench` **156/156**; full `scripts/run_ci.sh` GREEN (lib 1721 → **1724**, +3 hermetic tests —
        block-qualify disjoint / skip no-block occurrence / `derive_register_block_name` grammar; fmt +
        clippy `-D warnings` + rustdoc + mdBook). ADR-0006 (universal `<NUM> <BLOCK> register descriptions`
        grammar, no chip-name list).
  **Narrowed residual:** a genuinely-different reused mnemonic whose parent heading carries NO block token
  (the `D4.5 Register descriptions` CLAIMSET) stays an honest residual — there is no clean qualifier to
  separate it from a same-named register, so emitting it would risk conflation. Book `pipeline/evidenceir.md`
  `.10i`; KM `section-header-register-block-qualification`. Commit: pending (this slice).

**`PDF-VARIANT-DIGESTION.11` — `validate` integration of the `message_field_*` surfaces.**
<!-- active-task-source-region:activity-10f-10i:end -->

