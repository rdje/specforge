# Register bit-field ISF lowering — design + FSMGen-storage verification (DOC-INTENT-TAXONOMY.4a)

- Tree leaf: `DOC-INTENT-TAXONOMY.4a` (Gap A of the `.2` per-category ISF-completeness scorecard)
- Date: `2026-06-22`
- Type: **measurement / design**, read-only on code + **empirical** on the pinned FSMGen submodule
  (`subs/fsmgen` @ `030f8c273`). No SpecForge code change, no canonical artifact mutation → all golds +
  `kg-bench` orthogonal by construction. Outcome: a **verified FSMGen feature request**
  (`docs/FSMGEN_FEEDBACK.md`), per `[[feedback_isf_no_hacks]]` / `[[feedback_verify_fsmgen_before_fr]]`.
- Question: *the `.2` gauge measured Gap A — register **bit-fields** reach `.isf` zero times (12,638 fields /
  32 docs) while registers lower 1:1 to opaque `(storage (var … (width N)))`. WHERE is the loss, and is it a
  SpecForge carry/emit gap (buildable now) or a missing ISF abstraction (FSMGen FR)?*

## Method

Two grounded probes, both read-only:

1. **Code-path map** — trace a register's bit-fields from extraction → IntentIR → the emitted `.isf`, with
   exact `file:line`, to localize where the field substructure is lost.
2. **FSMGen ISF-storage capability** — read the pinned `subs/fsmgen` ISF spec + feature matrix + backlog to
   determine, empirically, whether the current ISF `(storage …)` grammar can declare a register's named
   bit-fields at all. (`[[feedback_verify_fsmgen_before_fr]]`: verify the submodule before claiming an
   ISF gap or filing an FR — never trust a commit subject or memory.)

## Finding 1 — the bit-field intent is fully captured and carried; it is dropped only at ISF emit

The field metadata is recovered in full and survives all the way into the canonical IntentIR — the loss is a
single emit-time discard, not an extraction or carry gap:

| Stage | Carrier | Bit-field substructure | Location |
|---|---|---|---|
| Extraction | `RegisterRecord.fields: Vec<RegisterFieldRecord>` | **FULL** — `field_name`, `bits_high`, `bits_low`, `bit_width`, `access_type`, `reset_value`, `description`, `enumerated_values` | `crates/specforge/src/ir/source.rs:397` (record) / `:414` (field) |
| IntentIR carry | `IntentIr.register_records: Vec<RegisterRecord>` | **FULL — unchanged clone** (`semantic_ir.register_records.clone()`) | `crates/specforge/src/ir/intent.rs:75` / build at `:193` |
| ISF emit | `IsfStorageVar { name, width, reset }` | **NONE** — only register name + width + optional composed reset | `crates/specforge/src/ir/isf_ir.rs:82` (struct) / `:834`–`852` (build) / `:391`–`402` (render) |

The discard is exactly at `isf_ir.rs:852`: the per-register loop reads `r.fields` only to compose a single
register-wide `(reset V)` (`classify_register_reset`, `isf_ir.rs:1326`, LSB-tiling the per-field resets), then
constructs `IsfStorageVar { name, width, reset }` — **every field's name, bit position, access type,
description, and enum encoding is dropped**. The rendered form is the opaque
`(var <register> (width N) [(reset V)])` (`isf_ir.rs:391`). This reproduces the `.2` live datum exactly: the
RISC-V IOMMU `.isf` storage block is `(var register_table_0033 (width 26))` with no field substructure,
though IntentIR carried 147 fields across its 33 registers.

**Conclusion of Finding 1:** there is **no SpecForge-side carry gap for Gap A** — the field map is already in
IntentIR. The loss is purely at the lowering boundary because the emitter has no ISF target to lower fields
into.

## Finding 2 — the current ISF has no field-structured-storage abstraction (empirically verified on `030f8c273`)

The ISF `(storage …)` grammar on the pinned submodule declares **opaque, width-only scalar state only**:

```lisp
(storage
  (var rd_ptr (width 2))
  (variable wr_ptr (width PTR_W))
  (bank data (width DATA_W) (depth DEPTH)))
```
(`subs/fsmgen/docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md` §8, ~L579–620: "`(var …)` and `(variable …)` declare
fixed-width actor-owned scalar state"; widths are positive integer literals / actor-local params / declared
constants.) There is **no construct to declare named bit-fields inside a storage variable** — no field at a
named bit range, no packed record/struct layout, no per-field access/reset.

ISF *does* ship runtime **field operations** — `(set-field NAME (bits HI LO) VALUE)`,
`(when-field …)` / `(unless-field …)`, `(extract WORD as FIELD…)`, `(assemble PART… as TARGET)`
(`subs/fsmgen/docs/book/src/13k-isf-feature-support-matrix.md` ~L354–407, shipped) — but these are
**read-modify-write operations on an opaque register declared `(var NAME (width N))` first**, not a static
field-map *declaration*. SpecForge does not extract register bit-fields as runtime manipulation behaviour; it
extracts them as the *declarative programming model* (which bits mean what, the per-field access/reset/enum).
Emitting `set-field`/`extract` to "represent" a static field map would fabricate runtime behaviour the
document never states — forbidden by the honest-residual doctrine. So the operational family is **not** a
faithful lowering target for Gap A.

The FSMGen backlog (`subs/fsmgen/docs/book/src/14-feature-backlog.md`) defers broader **aggregate** lowering
(records/arrays) and explicitly does **not** invent structs from partial use; it does **not** list named
bit-fields within a scalar register as a planned feature. So field-structured storage is **absent and not on
the published roadmap**.

**Conclusion of Finding 2:** ISF cannot represent a register's named bit-fields today. Gap A is a genuine
**missing ISF abstraction**, not an emitter bug.

## Decision — file a verified FSMGen feature request (no emitter hack)

Per `[[feedback_isf_no_hacks]]` (a missing abstraction → raise an ISF feature request, never hack the
emitter) and the `.2` prioritized program, the correct action is to **file a verified FSMGen FR** for a
declarative field-structured-storage ISF construct. The empirical verification above (Finding 2) satisfies
the `[[feedback_verify_fsmgen_before_fr]]` precondition. The FR is filed in `docs/FSMGEN_FEEDBACK.md`
(`## Feature request (2026-06-22) — declarative field-structured storage …`).

The three emitter-only alternatives were considered and **rejected as fabrication / loss**:

- *Per-field `(var REG_FIELD (width w))`* — fabricates N separate storage units for one register, loses the
  register grouping and the absolute bit positions, and collides with the register's own `(var REG …)`. Hack.
- *Runtime `(extract REG as FIELD…)` / `set-field`* — fabricates runtime behaviour the document never
  states (Finding 2). Dishonest.
- *Field names in comments* — comments are not intent and FSMGen ignores them; not synthesis.

There is genuinely no faithful emitter-only path; the gap is real and upstream. The owner's `2026-06-22`
context makes the timing right: FSMGen is adding a verification-oriented SV/UVM + VHDL lowering path and
anticipates new ISF abstractions (memory banks, single/dual-port memory modules) — a declarative
field-structured register is the same family.

## Gap A ↔ Gap B relationship

Gap B (message-field **structures**: 1,220 fields / 11 docs, with **no IntentIR carrier** — confirmed: zero
`message_field` matches in `intent.rs`) shares the SAME missing ISF abstraction family (a packed,
named-field layout). Both want "structured storage with named fields". Gap B additionally needs the
`Evidence→Intent` carrier that Gap A does not (Gap A's fields already reach IntentIR). The FR therefore
frames the abstraction generally enough to serve both (register field-map AND packet/structure layout),
while the cat-2/cat-1 Gap-B carrier stays a separate owned leaf (`.4b`).

## Follow-on leaves (recorded, not built here)

- `.4a.i` (CODE, after this FR): emit an explicit adapter **honest residual** `isf_register_fields_not_lowered`
  so the largest measurable intent-loss is surfaced in `residual_decisions` instead of silently dropped at
  `isf_ir.rs:852` (today only the *reset* drop is recorded, as `isf_storage_reset_not_lowered`). Requires the
  full task-acceptance checklist + `run_ci.sh` + FSMGen `--strict --check` 0-new-diagnostics.
- `.4a.ii` (CODE, gated on FSMGen): emit the field-structured storage once ISF gains the construct the FR
  asks for — the actual Gap A lowering.
- `.4b` (CODE): the Gap B `Evidence→Intent` `message_field_records` carrier, then the same FSMGen
  structure/packet construct.

## Honest caveats

- The 12,638 / 32-doc figure is the `.2` measured corpus count (`scripts/measure_isf_completeness.py`); this
  leaf changed no code and no canonical artifact, so that gauge is unchanged.
- "ISF cannot represent named bit-fields" is the verified state of the **pinned** `030f8c273`; if FSMGen
  ships the abstraction the FR requests, re-verify on the new pin before building `.4a.ii`.
- FSMGen `--strict --check --json` invocation for any future emitter verification:
  `./bin/fsmgen --strict --check --json <source>.isf` (`ISF_DOWNSTREAM_INTEGRATION_SPEC.md` ~L113–143).
</content>
</invoke>
