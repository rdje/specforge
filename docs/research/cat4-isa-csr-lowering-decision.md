# Cat-4 (CPU ISA / privileged architecture) ISF-lowering decision packet — `DOC-INTENT-TAXONOMY.4d`

- Tree leaf: `DOC-INTENT-TAXONOMY.4d`
- Date: `2026-06-23`
- Type: measurement + decision packet (read-only, docs-only — no Rust code, no canonical-artifact mutation)
- Reinforces: `[[project_kg_isf_completeness]]`, `[[project_doc_intent_taxonomy]]`,
  `[[feedback_verify_fsmgen_before_fr]]`, `[[feedback_isf_no_hacks]]`, `[[feedback_scoring_rigor]]`

## The question (the standing Open Question this packet resolves)

> Does category-4 (CPU ISA / privileged architecture) intent — CSR-fields, instructions, privilege modes,
> exceptions, memory-ordering — **map onto the existing register/storage ISF abstraction**, or does it need a
> **new ISF construct** (a verified FSMGen feature request)?

The `.2` ISF-completeness gauge scored cat-4 as **THIN** ("only the register-shaped surface lowers; bit-fields
0; no instruction/CSR/privilege/exception construct"). `.4a.ii` then shipped register **bit-field** lowering
(`(storage (var … (fields (field …))))`). This packet measures cat-4 specifically, on the now-current pipeline
and the now-current FSMGen pin, and decides the construct-vs-reuse question from evidence — never a guess.

## Method (read-only, reproducible)

The corpus has exactly **2** category-4 documents (the `.1` census; the corpus has no full ISA volumes yet):

| Doc key | Document | `.1` census bucket |
|---|---|---|
| `1_0_risc_v_debug_specification` | RISC-V Debug Specification 1.0 | reg/struct |
| `1_0_2025_03_12_risc_v_advanced_interrupt_architecture` | RISC-V Advanced Interrupt Architecture 1.0 | prose-only (reg 0, cond 39) |

(`1_0_1_2026_02_22_risc_v_iommu_architecture_specification` is **category 2**, register-IP — not cat-4.)

For each I profiled the persisted `evidence_ir.json` / `semantic_ir.json` / `intent_ir.json` surfaces
(`generated/<stage>/<key>/<stage>.json`), counted register_records and their fields, measured how many fields
are **located** (carry `bits_high`/`bits_low`, or `bits_low`+`bit_width`) — the exact `.4a.ii` admission gate —
and sampled the non-register intent surfaces. I then re-verified what the **current** FSMGen ISF
(`subs/fsmgen` pin `d327129b7`) declares for register/CSR/instruction/privilege intent.

## Measured evidence

### RISC-V Debug — registers captured, but every field is UNLOCATED

- `register_records = 44` (carried byte-identical evidence → semantic → intent). These are the Debug Module
  registers (`dmcontrol`, `dmstatus`, `hartinfo`, `abstractcs`, `command`, …) — i.e. the spec's memory-mapped
  **CSR block**. They were captured by the `field_table` register strategy (evidence manifest).
- `44 / 44` registers carry a field list; **179 fields total**. Each field carries `field_name`, `access_type`
  (`R` / `R/W` / `WARL` / `W1` / `WARZ` / …), `reset_value` (`-` / `Preset` / `0` / `3`), and a rich
  `description` — **good field-identity recall**.
- **`0 / 179` fields are located.** Not one field carries `bits_high`, `bits_low`, or `bit_width`. The register
  records also have `name`/`offset_address`/`size_bits` = `None`. So the bit *position* of each field — which
  lives in the register's bit-layout column/diagram — was **never parsed into the field record**.
- Consequence: under the `.4a.ii` gate (located fields only), **0 of Debug's 179 fields reach `.isf`**, even
  though the ISF field-structured-storage construct now exists and the field *names/access/reset* are in hand.
- Non-register intent (for completeness): `conditional_rules = 62`, `temporal_invariants = 216`,
  `behaviors = 784`, `transactions = 3`, `actor_signal_relations = 20`.

### RISC-V AIA — zero registers captured; the CSR/interrupt intent is in prose

- `register_records = 0` at every stage — despite AIA being **dense with CSRs and memory-mapped registers**
  (the IMSIC interrupt-file registers, the APLIC register map, the supervisor/machine AIA CSRs).
- Its register/interrupt-controller intent currently lands in **prose surfaces**: `conditional_rules = 39`
  (e.g. *"each RISC-V hart must have an Incoming MSI Controller (IMSIC)"*, *"the system will normally still
  contain an APLIC"*), `behaviors = 493`, `temporal_invariants = 182`.
- The `interfaces = 211` surface is **empty prose noise**: the sampled signal records are all `None`, and the
  doc carries `0` `signal_constraints` and `0` `actor_signal_relations` — confirming these are prose-derived
  shells, not real declared signals. (This is why `.1` filed AIA as `prose-only`.)
- No current register strategy matches AIA's layout. In particular the `.10g` section-heading register-field
  family (`<NAME>, bits [hi:lo]` under a dotted container) fires only on **ARM `ihiXXXX` architecture specs**;
  RISC-V's CSR/register tables use a different convention, so nothing fires.

### FSMGen ISF (pin `d327129b7`) — what it declares for ISA-shaped intent

- `(storage (var NAME (width N) [(reset V)] [(fields (field …))]))` is **explicitly the register-map / CSR
  construct**: `subs/fsmgen/docs/book/src/13a-actor-interface.md:419` is titled *"Storage reset values —
  register maps / CSRs"*, and `:468` *"Declarative Storage Fields"*. This is the exact construct cat-2 and
  cat-3 register maps lower into via `.4a.ii`.
- There is **no instruction / opcode / privilege-mode / exception / memory-ordering construct** anywhere in the
  ISF grammar (`13a`–`13i`, the feature matrix, the downstream-integration spec). ISF expresses *hardware*
  actor / signal / transaction / storage / temporal intent for FSMGen's HDL and (new) SV/UVM lowering — not
  software-visible ISA semantics.
- FSMGen's own listed future ISF directions (`13a-actor-interface.md:511`) are *"typed storage fields, banks,
  aggregate carriers, packet/flit layouts, access …"* — i.e. richer **storage/structure** abstractions. An
  instruction/ISA-semantics construct is **not** proposed or on the backlog.

## Decision

**Category-4 intent splits cleanly into three, and only one is a buildable SpecForge lever:**

1. **CSR / register intent → REUSE the existing ISF register/storage abstraction. No new ISF construct, no
   FSMGen FR.** CSRs are *structurally registers* — RISC-V Debug already captures its 44 CSRs as
   `register_records`, and FSMGen explicitly names `(storage (var … (fields …)))` the "register map / CSR"
   construct. The abstraction question is answered: **it maps; it does not need a new construct.** This also
   settles the cat-4 half of Gap A — cat-4 shares the register-field lowering `.4a.ii` already ships.

2. **The cat-4 register gap is EXTRACTION RECALL, not a missing ISF abstraction.** The fields don't lower
   because they aren't *located* (Debug: 0/179 fields carry bit positions; AIA: 0 registers captured at all),
   not because ISF can't express them. The doctrine-correct move is therefore an **extraction leaf**, not an
   FSMGen FR — spun out as **`.4d.i`: recover RISC-V CSR field bit positions** (parse the bit-layout
   column/diagram into `bits_high`/`bits_low` for the RISC-V register-table style, and add a RISC-V-shaped
   register recogniser for AIA). Once a field is located, it lowers automatically through `.4a.ii` — no
   emitter change. This is the genuine, measurable, north-star-advancing cat-4 lever.

3. **Instruction / privilege-mode / exception / memory-ordering intent → honest NON-TARGET for ISF
   synthesis.** These are software-visible ISA semantics, not synthesizable hardware actor/signal/storage/
   transaction intent; ISF has no construct for them and FSMGen lists none on its roadmap. Forcing them into
   ISF would fabricate (`[[feedback_isf_no_hacks]]`), and filing an FR for a construct FSMGen has not scoped
   would violate `[[feedback_verify_fsmgen_before_fr]]`. They are recorded as a **conditional-future**: a
   verified FSMGen FR becomes appropriate *only if* FSMGen's new SV/UVM verification path explicitly takes on
   ISA-model verification (instruction/exception coverage) — which is FSMGen's call to scope, not SpecForge's
   to assume. Until then this is the same honest non-target posture as cat-5 PHY: a near-empty `.isf` for the
   non-register ISA semantics is **correct**, not a miss.

**Net:** cat-4's THIN score is now explained, not just labelled. The lowerable part of cat-4 (CSRs) reuses the
existing register/storage road and the buildable work is **extraction recall** (`.4d.i`); the non-register ISA
semantics are an honest non-target pending an explicit FSMGen verification-path scope. No fabrication; no
speculative FR; no emitter hack.

## Genericity (ADR 0006)

The decision rests on structural evidence (register vs non-register intent shape; located vs unlocated fields)
and FSMGen's published grammar — **no chip/vendor/protocol-instance name list**. The spun-out `.4d.i` lever
must likewise key off structural register-table / bit-layout grammar (a RISC-V CSR layout *shape*), never a
list of RISC-V register names.

## Gates (this leaf)

- No Rust code, no canonical-artifact mutation → the wire golds / `kg-bench` / emitted `.isf` are
  byte-identical by construction (WIRE-BASED-100 orthogonal).
- `scripts/check_doctrines.sh` green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green;
  knowledge-map derive-and-diff in sync after adding the `[[cat4-isa-csr-lowering-decision]]` fact card.
- Objectively measured, per-item demonstrated (`[[feedback_scoring_rigor]]`): every count above is read
  directly off the persisted corpus and is reproducible from the `generated/<stage>/<key>/<stage>.json` files.
