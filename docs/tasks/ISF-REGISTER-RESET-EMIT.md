# ISF-REGISTER-RESET-EMIT: lower extracted register reset values into the ISF `(storage (var … (reset V)))` surface

## Metadata

- Tree ID: `ISF-REGISTER-RESET-EMIT`
- Status: `done` (CLOSED `2026-06-16` — `.0`/`.1`/`.2`/`.3` all done; register reset values now reach the `.isf` at the true register width; wire docs byte-identical; FSMGen-strict-valid)
- Roadmap lane: `R6` (`.isf` adapter) × `R15`/`R16` (extraction quality, ISF-fidelity lens)
- Created: `2026-06-16`
- Last updated: `2026-06-16`
- Owner: repo-local workflow
- Parent context: promoted from the `FSMGEN-REFRESH-INTEGRATE-2.2` assessment (the ONE grounded
  new adopt candidate found in the `2026-06-16` FSMGen refresh — *"register/CSR reset values
  `(storage (var … (reset V)))` — SpecForge extracts register-field `reset_value` + has a
  `(storage)` emit surface not yet fed from it; measurement-first, WIRE-BASED-100-gated,
  ISF-strict-validated"*). Directly serves the `KG-ISF-COMPLETENESS` NORTH STAR bar #6: *every
  IntentIR surface element appears in the `.isf` or as an explicit residual* — a register reset
  value is an extracted IntentIR fact that is currently silently dropped at the ISF boundary.
  No FSMGen dependency (the construct is already `shipped`). Memory: `[[project_kg_isf_completeness]]`.

## The point (why this tree exists)

SpecForge already extracts register maps with per-field reset values, and the ISF emitter already
builds a `(storage …)` block from those registers — **but the reset value is thrown away at the
emit boundary.** FSMGen's contract makes the reset a first-class, optional storage-var property
designed for exactly this case (register maps / CSRs that power up at a default). Closing the gap
makes the emitted `.isf` carry the register's documented power-up value instead of silently
defaulting it to all-0s.

### Grounded baseline (read-only code grounding, `2026-06-16` — confirmed, to be quantified in `.1`)

- **Extraction (present):** `RegisterRecord` (`ir/source.rs:397`) carries
  `fields: Vec<RegisterFieldRecord>`; each `RegisterFieldRecord` (`ir/source.rs:414`) carries
  `bits_high`, `bits_low`, `bit_width`, and `reset_value: Option<String>` (the literal as written —
  `"0"`, `"0x1"`, `"0b00"`, or a symbolic token). The extractor populates it from register-field
  tables (`ir/evidence.rs:11509`, `:11838`; a regression test asserts a real recovered
  `reset_value == Some("0")` at `ir/evidence.rs:14758`).
- **Carry-through (present):** `IntentIr.register_records` is cloned straight from
  `SemanticIr.register_records` (`ir/intent.rs:193`), so the field-level `reset_value` reaches
  IntentIR intact.
- **ISF emit (the gap):** `IsfIr::from_intent_ir` builds one `IsfStorageVar { name, width }` per
  register (`ir/isf_ir.rs:730`) and renders it as `(var NAME (width W))` (`ir/isf_ir.rs:375`).
  **`IsfStorageVar` has no reset field (`ir/isf_ir.rs:81`); `reset_value` is never read → never
  lowered.** The var `width` is currently the **max single-field extent**, NOT the register width
  — a detail `.2` must reconcile for a composed multi-field reset to fit (measured in `.1`).
- **FSMGen target (authoritative, already `shipped` — no FR needed):**
  `(storage (var NAME (width N) [(reset V)]) …)` — feature-support matrix
  `subs/fsmgen/docs/book/src/13k-isf-feature-support-matrix.md:42` and
  `subs/fsmgen/docs/book/src/13m-local-variables.md:48-68`: `(reset V)` is OPTIONAL; `V` must be a
  **non-negative integer literal that fits in the width**; **omitting it = resets to all-0s
  (byte-identical to today's output)**; an **over-width or non-integer reset fails closed**. This
  exactly bounds what `.2` may emit: a clean in-width non-negative integer, else omit (honest
  residual, never fabricate).

## The checkable "register reset is ISF-faithful" bar (per doc)

A document's emitted `.isf` is register-reset-faithful when:
1. **Lowered when groundable** — a register whose documented reset is a clean, in-width,
   non-negative integer (directly, or composed from per-field resets tiled at their bit offsets)
   emits `(var NAME (width W) (reset V))` with the correct `V`.
2. **Honest residual otherwise** — a register with a missing, symbolic, partial, or over-width
   reset emits `(var NAME (width W))` with NO `(reset V)` (FSMGen then defaults it to all-0s),
   and the dropped/ungrounded reset is recorded as an explicit adapter residual rather than
   fabricated. No invented values; ADR-0006 (universal numeric parsing, no chip-name list).
3. **Strict-valid** — every emitted `.isf` still passes `subs/fsmgen/bin/fsmgen --strict --check
   --json` (a non-integer / over-width `(reset V)` would fail closed, so the gate enforces #1/#2).
4. **No regression** — documents with no register reset to lower emit BYTE-IDENTICAL `.isf` to
   today; register-bearing docs change ONLY by the added `(reset V)` clauses (and any
   measured-justified, gated var-width reconciliation needed for the reset to fit).

**Hard gate (non-negotiable):** the wire docs (APB/AHB/AXI/SWD) stay at WIRE-BASED-100 and their
emitted `.isf` stays byte-identical UNLESS they carry register maps (measured in `.1`; expected:
they are signal/handshake-centric and carry few/none, so the blast radius is the register-heavy
docs — CCIX / NVMe / GIC / CoreSight / etc.). Universal grammar only, no name lists (ADR 0006).

## Non-Goals

- NOT inventing a register-level reset where the document grounds none (honest residual instead).
- NOT changing register EXTRACTION (the `reset_value` field already exists and is populated); this
  tree is purely the IntentIR→ISF lowering of an already-captured fact (plus the minimal var-width
  reconciliation a composed reset needs).
- NOT FSMGen-side work — `(reset V)` is already `shipped`; this tree raises no FR.
- NOT bank/aggregate/per-element reset (FSMGen fails a per-element bank `(reset V)` closed) — scalar
  storage vars only.

## Acceptance Criteria

- `.1` quantifies the corpus reset surface (how many docs/registers carry reset values; numeric vs
  symbolic; full vs partial field coverage; var-width-vs-register-width interaction; wire-doc blast
  radius) read-only before any code, per the project's measurement-first doctrine.
- `.2` implements the lowering to signoff quality: composed/clean reset emitted, ungrounded reset
  an explicit residual, ADR-0006-safe, with focused unit tests.
- Gates for `.2`: ISF round-trip 0 new `--strict --check` diagnostics on every wire doc + the
  register-bearing docs; WIRE-BASED-100 held 1.000; `kg-bench` unchanged; `run_ci.sh` green;
  non-register docs byte-identical `.isf`.
- Live docs + the mdBook (the `BOOK-METHOD-DOC` close-rule on the tree's closing leaf) updated.
- Each completed leaf committed through `COMMIT.md`.

## Task Tree

- ID: `ISF-REGISTER-RESET-EMIT` · Status: `done` (CLOSED `2026-06-16`) · Children: `.0` (ownership/
  scoping), `.1` (corpus measurement), `.2` (emit `(reset V)`), `.3` (var-width reconciliation) — all done
- ID: `ISF-REGISTER-RESET-EMIT.0` · Status: `done` (`2026-06-16`, docs-only ownership/scoping) ·
  Goal: own the adopt candidate promoted from `FSMGEN-REFRESH-INTEGRATE-2.2`; record the
  read-only-grounded baseline (the exact gap: `reset_value` extracted + carried but dropped at
  `ir/isf_ir.rs:730/375`; FSMGen `(reset V)` already `shipped` per `13k:42`/`13m`); define the
  checkable 4-point bar; set the slice sequence. No code (own before touching). Memory
  `[[project_kg_isf_completeness]]`.
- ID: `ISF-REGISTER-RESET-EMIT.1` · Status: `done` (`2026-06-16`, measurement-first, read-only,
  docs-only; report `docs/research/register-reset-emit-measurement.md`; KM card
  `[[register-reset-isf-emit]]`) · Goal: **corpus measurement — is a faithful register reset
  groundable, and where? → GO.** Measured over all 36 persisted `intent_ir.json`: 19/36 docs carry
  registers (2561 total); **1508 strictly composable** (every field located via
  `bits_high`/`bits_low`/`bit_width` + `reset_value` parseable as a non-neg int fitting its field
  width + no overlap; LSB-tiled to a register `V`). 1339 fit the current emit width; of those **446
  have V>0** (the real `.isf` diff — **entirely the 3 CoreSight SoC-600 TRMs**, 199/127/120), 893 are
  V==0 (= the FSMGen all-0s default → omit). **Wire-doc blast radius = ZERO** (APB/AHB/AXI/SWD emit
  no reset → `.isf` byte-identical → WIRE-BASED-100 trivially held; AXI's 71 "registers" are
  positionless encoding pseudo-tables with symbolic resets). **169 composable registers need a wider
  var width** than the current max-field-extent (e.g. CoreSight DPIDR composes `0x1c013477` but the
  current width is 11 → over-width, FSMGen fails closed) — the latent width-derivation bug, spun out
  as `.3`. Symbolic/partial/over-width = honest residuals (ADR-0006, numeric parsing only, no name
  list). **Decision: GO** with the `.2` policy below + the `.3` spin-out. No code.
  Verification: `passed` (read-only) · Commit: `718495e8`-successor (this commit)
- ID: `ISF-REGISTER-RESET-EMIT.2` · Status: `done` (`2026-06-16`, first code slice; measurement-first;
  ALL gates green) · Goal: **emit `(reset V)` — purely additive.** **LANDED:** `IsfStorageVar` gains
  `reset: Option<u64>` (`ir/isf_ir.rs`); `from_intent_ir` classifies each register via the new
  `classify_register_reset` (LSB-tiling `V = OR(parse_reset_literal(reset_i) << bits_low_i)`,
  strict-composable gate: every field located + parseable non-neg int fitting its field width + no
  overlap, ≤64-bit) → `Emit(V>0, fits width)` / `DefaultZero` / `DeferredWidth` / `NotLowerable` /
  `NoReset`; `render` emits `(var NAME (width W) (reset V))` only on `Emit`, else the byte-identical
  `(var NAME (width W))`. Dropped resets (`NotLowerable`+`DeferredWidth`) are surfaced as ONE
  proportionate adapter residual `isf_storage_reset_not_lowered` (`storage_reset_residuals()` →
  `adapters.rs` residual_decisions). ADR-0006 numeric parsing only, no name list. **Live-verified
  (release binary):** AXI `.isf` **byte-identical** (0 emissions; its 71 pseudo-table resets → 1 honest
  residual); CoreSight SoC-600 `.isf` gains exactly **120** `(reset V)` (the measured V>0), the
  normalize-out diff proves the ONLY change is added `(reset V)` (no width/other delta), FSMGen
  `--strict --check --json` = **success / 0 errors / 0 diagnostics** (old also 0 → **0 new**). **Gates
  ALL GREEN:** ADR-0006 ✓; **WIRE-BASED-100 1.000** (constraint ×4 + relation document-level + temporal
  ×3, orthogonal — eval doesn't invoke the adapter) ✓; ISF round-trip 0 new strict diagnostics ✓;
  wire-`.isf` byte-identical ✓; register-doc `.isf` changes ONLY by added `(reset V)` ✓; `kg-bench`
  156/156 ✓; `run_ci.sh` GREEN (lib **1649** pass / 2 ignored, **+4 tests**: parse/classify/render/
  residual) ✓. Book: `pipeline/isf-adapter.md` gained "Register reset values". Verification: `passed` ·
  Commit: this commit
- ID: `ISF-REGISTER-RESET-EMIT.3` · Status: `done` (`2026-06-16`, measurement-first; closes the tree;
  ALL gates green) · Goal: **reconcile the storage-var width to the true register width.** **LANDED:**
  new `register_var_width(r)` (`ir/isf_ir.rs`) = `size_bits ⊔ max(bits_high)+1` (declared width, never
  below the highest located field bit so no field truncates), 32 fallback when neither is available;
  replaces the prior max-single-field-extent width in the storage build. With the var at register
  width, the previously-over-width composable resets now fit → `classify_register_reset` returns
  `Emit` for them. **Measured (read-only, before coding):** 1045 of 2108 register vars change width
  (true register width), **0 in wire docs** (APB/AHB/AXI/SWD register vars are unlocated/absent →
  fallback unchanged), ~155 over-width composable resets become emittable. **Live-verified (release):**
  AXI `.isf` **byte-identical** to the original pre-`.2` baseline (wire docs 0-change); CoreSight
  SoC-600 storage resets **120 → 183** at corrected widths (DPIDR `(width 32) (reset 469841015)`),
  FSMGen `--strict --check --json` **success / 0 errors / 0 diagnostics**; NVMe/HBM2 keep their
  PRE-EXISTING strict diagnostics (NVMe `isf_conflicting_rule_writes` on `ELEN`; HBM2 `+enums`
  `REPAIR_LANE_8` literal) which are rule/enum-lowering issues unrelated to storage — **0 NEW
  diagnostics** (the change only touches `(var …)`). **Gates ALL GREEN:** ADR-0006 ✓; WIRE-BASED-100
  1.000 (constraint ×4 + temporal ×3, orthogonal) ✓; wire-`.isf` byte-identical ✓; FSMGen 0-new ✓;
  `kg-bench` 156/156 ✓; `run_ci.sh` GREEN (lib **1651** / 2 ignored, **+2 tests**: `register_var_width`,
  over-width-now-emits) ✓. Book: `pipeline/isf-adapter.md` "Closed task trees" subsection added (close-
  rule). Verification: `passed` · Commit: this commit

## Current Frontier

**Empty — tree CLOSED `2026-06-16`** (`.0`/`.1`/`.2`/`.3` all done). Register reset values now reach
the `.isf` as `(storage (var … (reset V)))` at the true register width: wire docs (APB/AHB/AXI/SWD)
byte-identical, register-heavy docs gain the documented power-up values (CoreSight SoC-600 183 resets),
every emitted `.isf` FSMGen-strict-valid (0 new diagnostics), WIRE-BASED-100 + `kg-bench` unchanged.
Honest residual for symbolic/partial/unfit resets; nothing fabricated. Out of scope (honest residuals,
not regressions): NVMe rule-write conflict + HBM2 enum-literal strict diagnostics (pre-existing
rule/enum-lowering issues, unrelated to register reset).

## Decisions

- `2026-06-16` (`.0`): **Promote the `FSMGEN-REFRESH-INTEGRATE-2.2` adopt candidate into its own
  tree** (the "captured + owned, execution on promotion" pattern). Picked by PNT after the two
  active north-star trees reached a deferred/parked frontier: `KG-ISF-TRANSACTIONS` body-emission is
  PARKED pending FSMGen + owner steer (`.2i`/`.2j` done), and `KG-ISF-COMPLETENESS` frontier is
  `.1b.ii`/`.1b.iv` (both deferred-with-trigger) + a broad `.2+`. This tree is the clean eligible
  build frontier: bounded, no FSMGen dependency, directly advances the ISF-fidelity north star.
- `2026-06-16` (`.0`): **Measurement-first sequencing** — `.1` (read-only corpus measurement)
  precedes `.2` (code). Acceptance Criterion #1 of every code-bearing slice in this repo is a
  read-only measurement; the var-width-vs-register-width interaction and the numeric/symbolic reset
  distribution are real unknowns that decide the emit/residual policy and the blast radius, so they
  are measured before any emitter change.
- `2026-06-16` (`.0`): **Honest-residual policy, FSMGen-contract-bounded** — emit `(reset V)` ONLY
  for a clean in-width non-negative integer (`13k:42`/`13m`: over-width/non-integer fails closed,
  omission = all-0s default = byte-identical); a missing/symbolic/partial reset emits no `(reset V)`
  and is an explicit adapter residual. Never fabricate a power-up value. ADR-0006 (universal numeric
  parsing, no name list).
- `2026-06-16` (`.1`): **GO.** The reset surface is real (1508 composable; 446 V>0) and the wire-doc
  blast radius is ZERO (proven: APB/AHB/AXI/SWD compose no register reset). **Split reset-emit (`.2`,
  purely additive over the 446 fits-current V>0 registers) from var-width reconciliation (`.3`, the
  169 over-width registers)** — they are independently reviewable concerns (the width is a separate
  latent bug; the splitting rule forbids conflating them), and `.2` stays a clean additive diff that
  satisfies bar #4 exactly.
- `2026-06-16` (`.1`): **`V == 0` is omitted, not emitted** — the FSMGen all-0s default faithfully
  represents a documented 0 reset (identical lowered hardware; no fact lost), so emitting 893
  `(reset 0)` clauses would be redundant noise. The `.isf` diff is the 446 V>0 resets only.

## Open Questions

- (for `.1`) Does any wire doc (APB/AHB/AXI/SWD) carry `register_records`? If not, the WIRE-BASED-100
  + wire-`.isf`-byte-identity gates hold trivially and the blast radius is exactly the register-heavy
  docs. — does NOT block `.1` (it is the first thing `.1` measures).
- (for `.1`/`.2`) Must the var width be reconciled from max-field-extent to the true register width
  (`size_bits` / `max(bits_high)+1`) for a composed reset to fit, and does that width change alter
  any existing `.isf`? If so, `.2` gates that change separately. — measured in `.1`; does not block.
- (for `.1`) Are field reset values predominantly per-field partials (→ compose) or is there often a
  single full-width field whose reset is the register reset (→ direct)? Decides the composition
  complexity. — measured in `.1`; does not block.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-16` | `ISF-REGISTER-RESET-EMIT.0` | read-only code grounding (`reset_value` extracted `ir/source.rs:432` + carried `ir/intent.rs:193` but dropped at `ir/isf_ir.rs:730/375`); FSMGen `(reset V)` confirmed `shipped` (`13k:42`, `13m:48-68`); fsmgen binary present (`subs/fsmgen/bin/fsmgen`); `scripts/check_memory_architecture.sh` green | `passed` (docs-only; no code) |
| `2026-06-16` | `ISF-REGISTER-RESET-EMIT.1` | read-only corpus measurement over 36 `intent_ir.json` (1508 composable, 446 V>0 in 3 CoreSight TRMs, 1339 fits-current, 169 over-width, wire-doc blast radius = 0); report + KM card written; `scripts/check_memory_architecture.sh` + `knowledge-map` green | `passed` (docs-only; no code) |
| `2026-06-16` | `ISF-REGISTER-RESET-EMIT.2` | code: `IsfStorageVar.reset` + `classify_register_reset`/`parse_reset_literal`/`register_field_extent` + render + `storage_reset_residuals` adapter wiring + 4 unit tests. Live (release): AXI `.isf` byte-identical, CoreSight gains 120 `(reset V)` (only-reset diff proven), FSMGen `--strict --check` success/0 errors/0 diagnostics (old 0 → 0 new). WIRE-BASED-100 constraint ×4 + relation + temporal ×3 = 1.000; `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1649 / 2 ignored, +4 tests) | `passed` |
| `2026-06-16` | `ISF-REGISTER-RESET-EMIT.3` | code: `register_var_width` (`size_bits ⊔ max(bits_high)+1`) replaces max-field-extent in the storage build + 2 unit tests. Measured 1045/2108 vars change width (0 in wire docs). Live (release): AXI byte-identical to pre-`.2` baseline, CoreSight 120→183 resets at corrected widths, FSMGen `--strict --check` success/0/0; NVMe/HBM2 keep PRE-EXISTING rule/enum diagnostics (0 new). WIRE-BASED-100 constraint ×4 + temporal ×3 = 1.000; `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1651 / 2 ignored, +2 tests); book close-rule subsection added | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `ISF-REGISTER-RESET-EMIT.0` | `ISF-REGISTER-RESET-EMIT.0 — own register-reset→ISF lowering candidate (docs-only ownership/scoping)` (`718495e8`) | done |
| `ISF-REGISTER-RESET-EMIT.1` | `ISF-REGISTER-RESET-EMIT.1 — corpus measurement: GO (446 V>0 resets, wire blast radius 0, 169 width-spun .3)` (`cacdb158`) | done; docs-only |
| `ISF-REGISTER-RESET-EMIT.2` | `ISF-REGISTER-RESET-EMIT.2 — emit register reset values as ISF (storage (var … (reset V)))` (`2a18c32e`) | done; first code slice |
| `ISF-REGISTER-RESET-EMIT.3` | `ISF-REGISTER-RESET-EMIT.3 — reconcile storage-var width to the true register width (closes tree)` | this commit; closes tree |

## Changelog

- `2026-06-16`: Created task tree. `.0` ownership/scoping DONE (docs-only): promoted the
  `FSMGEN-REFRESH-INTEGRATE-2.2` adopt candidate; recorded the read-only-grounded baseline (the
  exact ISF-emit gap + the authoritative FSMGen `(reset V)` contract); defined the checkable 4-point
  bar + the honest-residual policy; set the `.1` (measurement) → `.2` (code) sequence. No code.
- `2026-06-16`: `.1` corpus measurement DONE (read-only, docs-only) — **GO.** 1508 strictly-composable
  registers corpus-wide; 446 emit a real V>0 reset (all 3 CoreSight SoC-600 TRMs); **wire-doc blast
  radius ZERO** (APB/AHB/AXI/SWD compose none → `.isf` byte-identical); 169 over-width composable
  registers spun to `.3` (var-width reconciliation); symbolic/partial/over-width/V==0 are honest
  residuals/defaults. Locked the `.2` purely-additive policy + the `.3` spin-out. Report
  `docs/research/register-reset-emit-measurement.md`; KM card `register-reset-isf-emit`. No code.
- `2026-06-16`: `.2` emit DONE (first code slice) — `IsfStorageVar.reset` + `classify_register_reset`
  (LSB-tiling, strict-composable gate) + `parse_reset_literal` + render `(var … (reset V))` +
  `storage_reset_residuals` honest-summary surface (`ir/isf_ir.rs`, `ir/adapters.rs`); +4 unit tests.
  Live (release binary): AXI `.isf` byte-identical, CoreSight SoC-600 gains 120 `(reset V)`
  (normalize-out diff proves only-reset change), FSMGen `--strict --check` success / 0 errors /
  0 diagnostics (old also 0 → 0 new). WIRE-BASED-100 1.000 (constraint ×4 + relation + temporal ×3,
  orthogonal); `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1649, +4 tests). Book
  `pipeline/isf-adapter.md` "Register reset values" added. Only `.3` (var-width) remains.
- `2026-06-16`: `.3` var-width reconciliation DONE — **tree CLOSED.** `register_var_width`
  (`size_bits ⊔ max(bits_high)+1`, never truncating a field) replaces the max-field-extent width in the
  storage build (`ir/isf_ir.rs`); the over-width composable resets now emit at the true register width.
  Measured 1045/2108 vars change width, 0 in wire docs. Live (release): AXI byte-identical to the
  pre-`.2` baseline, CoreSight SoC-600 120→183 resets at corrected widths, FSMGen `--strict --check`
  success/0/0; NVMe/HBM2 keep their pre-existing rule-conflict/enum-literal diagnostics (0 new — the
  change only touches `(var …)`). WIRE-BASED-100 1.000; `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1651,
  +2 tests). Book `pipeline/isf-adapter.md` gained the "Closed task trees" `ISF-REGISTER-RESET-EMIT`
  subsection (BOOK-METHOD-DOC close-rule). The bar is met: every composable register reset reaches the
  `.isf` at its true width; symbolic/partial/unfit resets are honest residuals; nothing fabricated.
