# Register-reset → ISF `(storage (var … (reset V)))` lowering — corpus measurement (`ISF-REGISTER-RESET-EMIT.1`)

Read-only measurement over the 36 persisted `generated/intent_ir/*/intent_ir.json` artifacts
(`2026-06-16`). No code. Decides the `.2` emit/residual policy and sizes the blast radius before
any emitter change (project measurement-first doctrine; this tree's Acceptance Criterion #1).

## Question

SpecForge extracts register-field `reset_value` and carries it to `IntentIr.register_records`, but
the ISF emitter renders `(var NAME (width W))` and drops it (`ISF-REGISTER-RESET-EMIT.0` baseline).
FSMGen's `(storage (var NAME (width N) [(reset V)]))` is `shipped` (`13k:42`/`13m`): `(reset V)`
optional, V a non-negative integer literal fitting the width, omission = all-0s (byte-identical),
over-width/non-integer **fails closed**. So: **is a faithful register reset groundable, for how many
registers, and what is the wire-doc blast radius?**

## Method

Per register, attempt to COMPOSE a register-level reset from its per-field resets by LSB-tiling (the
same discipline as `ir/register_bits.rs`): `V = OR over fields of (parse_int(reset_i) << bits_low_i)`.
A register is **strictly composable** iff EVERY field is (a) located (`bits_high`/`bits_low`, or
`bits_low`+`bit_width`, or a single bit), (b) has a `reset_value` parseable as a non-negative integer
(`dec` / `0x…` / `0b…` / `…h`), (c) whose value fits its own field width, and (d) no two fields
overlap. Anything else (missing/symbolic/partial-coverage/overlap) is an honest residual — never
composed. The composed `V` is then checked against the var width the emitter currently uses
(= the **max single-field extent**) and against the true register width (`size_bits`, else
`max(bits_high)+1`).

## Results

### Register inventory (19 of 36 docs carry registers; 2561 registers total)

Register-heavy docs: CoreSight SoC-600 TRMs (833 / 631 / 597), CCIX (131), Intel VT-d (103),
AXI (71), NVMe (42), RISC-V IOMMU (33), ARM Debug/SWD (29), eMMC (17), HBM2 (17), Wishbone (15),
GIC-600 (15), MMU-700 (13), TileLink (4/4), AHB (3), AXI+ACE (2), I2C (1). The other 17 docs carry
zero registers.

### Strict composability

| Metric | Count |
| --- | --- |
| registers total | 2561 |
| registers with ≥1 field carrying a `reset_value` | 2289 |
| **strictly composable** (located + parseable + field-fits + no-overlap) | **1508** |
| — composed reset FITS the current emit width (max-field-extent) | 1339 |
| — — of those, **V > 0** (the real `.isf` diff) | **446** |
| — — V == 0 (faithfully == FSMGen all-0s default → omit, no fact lost) | 893 |
| — composed reset NEEDS a wider width than the current emit | **169** |
| rejected for field overlap | 1 |

The 446 V>0 emittable resets are **entirely in the 3 CoreSight SoC-600 TRMs** (199 / 127 / 120) —
the register-map-heavy documents. (V==0 dominates the rest: most documented resets are 0, which the
FSMGen default already represents.)

### `reset_value` shape distribution (over the ~10k field resets)

`bin` 3065, `hex` 2405, `hexsuffix` 225, `dec` 73 → numeric/composable; the remainder is symbolic
and correctly excluded: `UNKNOWN` 469, partial-unknown `0x--------`-family ~600, `IMPLEMENTATION
DEFINED` 273, `-` 105, `X`/`Xh` 82, `Impl Spec` 96, `Configuration dependent` 14, `All zeros`/`All
ones` 24, enum-annotated `0b00 (Invalid)`-style ~20, `True`/`False` 9, etc. Symbolic resets are an
honest residual, never guessed (ADR-0006 — universal numeric parsing, no name list).

### Wire-doc blast radius — **ZERO** (the decisive finding)

All four wire docs (APB/AHB/AXI/SWD) emit **zero** `(reset V)`, so their `.isf` is **byte-identical**
and WIRE-BASED-100 / the wire-`.isf`-byte-identity gate hold trivially:

- **AXI** (`ihi0022_l`, 71 "registers"): these are encoding **pseudo-tables**
  (`Valid and Ready signals`, `Resource plane number properties`, `Shared credit properties`) whose
  fields have NO bit positions (`bits_high`/`bits_low`/`bit_width` all absent) and symbolic resets
  (`-`, `False`, `1`, `AxPROT[1]`, `0b00 (Non-secure)`) → never located → never composable.
- **AHB** (`ihi0033_c`, 3 registers): zero fields carry a `reset_value`.
- **APB** (`ihi0024_*`): zero registers.
- **SWD/ADI** (`ihi0074_a`, 29 registers): only 1 register has any field reset, none with full
  field coverage → not composable.

So the entire blast radius is the **register-heavy, non-wire** docs (CoreSight / VT-d / NVMe).

### Var-width interaction (→ spun out as `.3`)

The emitter's current storage-var width is the **max single field extent**, not the register width.
For 169 composable registers the composed reset needs MORE bits than that (e.g. CoreSight DPIDR
composes `V=0x1c013477` but the current width is `11` → emitting `(reset 0x1c013477)` at `(width 11)`
would be **over-width and FSMGen fails it closed**). The true register width (`size_bits`, present on
these docs, else `max(bits_high)+1`) is correct. Reconciling the var width is a **separate latent
width-derivation concern**, independently reviewable, so it is spun out as `ISF-REGISTER-RESET-EMIT.3`
(measured-justified, gated) rather than coupled into the reset-emit slice. For `.2`, the 169 are
honest residuals (omitted) until `.3` corrects the width.

## Decision — **GO**, with this policy

**`.2` (emit, purely additive):**
- Add `reset: Option<u64>` to `IsfStorageVar`; render `(var NAME (width W) (reset V))` only when set.
- Set it iff the register is **strictly composable**, the composed `V > 0`, AND `V` fits the current
  var width (max-field-extent). This is the **446-register, 3-doc** surface — purely additive, so
  register-bearing docs change ONLY by added `(reset V)` clauses (bar #4 exactly).
- `V == 0`: omit — the FSMGen all-0s default faithfully represents a documented 0 reset (no fact
  lost; not a residual). Keeps the diff minimal (avoids 893 redundant `(reset 0)` clauses).
- Composition = LSB-tiling (`V = OR (parse_int(reset_i) << bits_low_i)`), mirroring
  `ir/register_bits.rs`. Numeric parsing only (`dec`/`0x`/`0b`/`…h`), ADR-0006, no name list.
- Everything not emitted (symbolic / partial-coverage / overlap / over-width) is an honest residual,
  recorded on the adapter (reuse the `ResidualDecisionPacket` surface), never fabricated.

**`.3` (spun, measured-justified, gated):** reconcile the storage-var width to the true register
width (`size_bits` / `max(bits_high)+1`) so the 169 over-width composable resets become emittable;
review the width-only `.isf` delta on register docs; FSMGen `--strict --check` 0 new diagnostics.

**Gates carried to `.2`:** wire `.isf` byte-identical (proven 0 emissions here); register-doc `.isf`
changes ONLY by added `(reset V)`; FSMGen `--strict --check --json` 0 new diagnostics on every doc;
WIRE-BASED-100 1.000; `kg-bench` unchanged; `run_ci.sh` green.

## Reproduce

`python3` over `generated/intent_ir/*/intent_ir.json`: for each `register_records[].fields[]`, parse
`reset_value` (`dec`/`0x`/`0b`/`…h`), locate via `bits_high`/`bits_low`/`bit_width`, LSB-tile to a
register `V`, and bucket by composable / fits-current-width / V>0 / needs-wider-width / wire-doc.
(Full script in the `.1` session transcript; deterministic over the persisted corpus.)
