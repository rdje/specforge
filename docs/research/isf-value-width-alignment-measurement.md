# ISF value-literal width-alignment — measurement (`ISF-VALUE-WIDTH-EMIT.1`)

- Date: `2026-06-21`
- Scope: read-only over the 86 persisted `generated/adapters/isf/*/*.isf` + the affected
  `generated/intent_ir/*/intent_ir.json`, plus empirical FSMGen probes (Perl binary, RAM-safe — no
  cargo build). No code changed. This report is the authority for the `.2` emit slice.
- Tree: `ISF-VALUE-WIDTH-EMIT` (an `ISF-*-EMIT` emitter-fidelity tree, mirroring the closed
  `ISF-REGISTER-RESET-EMIT`). Surfaced by the `CORPUS-COVERAGE.2` re-ingest sweep as "Lever A".

## 1. The defect

The ISF emitter (`crates/specforge/src/ir/isf_ir.rs`) lowers a `(rule … (SIGNAL value))` clause by
copying the constraint/temporal value literal **verbatim** from the IntentIR
(`render_isf_control_expression` → `ControlExpressionRecord::Literal { literal } => literal.clone()`,
`isf_ir.rs:1493-1495`) and never reconciles the literal's bit-width against the target signal's declared
width. When the literal is wider than the signal's emitted `(width N)`, FSMGen's strict
`--check` rejects the whole `.isf`.

Live, on DTI (`generated/adapters/isf/ihi0088_g_2024_06_amba_dti_protocol_specification/channel.isf`):

```
(output ATST (width 1))          ; line 44 — declared 1-bit
...
(rule constraint_12
  (ATST 0B01))                   ; line 820 — a 2-bit literal driven onto a 1-bit signal
```

`perl subs/fsmgen/bin/fsmgen --strict --check --json channel.isf` →

```
success: false
Error generating HDL: [OperandContractValidationSupport.pm][validate_pre_generation_operand_contract()]
  - assignment to 'ATST' uses RHS '2'b1' with incompatible width 2 for LHS width 1; the current contract
    blocks implicit truncation and requires an explicit width-aligned source expression before generation
```

(plus three `[WARNING] Cannot truncate complex expression - leaving as-is`.)

## 2. The FSMGen contract (probed, pin `030f8c273`)

FSMGen determines a literal's width by its **notation digit count**, *not* its value, and requires the
RHS width to **equal** the LHS signal width (no implicit truncation/extension). Probed with minimal
hand-built `.isf` (RAM-safe):

| Signal `(width W)` | literal | FSMGen `--strict --check` | why |
|---|---|---|---|
| ATID `(width 7)` | `0x7D` | **FAIL** | `0x7D` = 2 hex digits = **8 notation-bits** ≠ 7 (value 125 fits 7 bits, but width is by digits) |
| ATID `(width 8)` | `0x7D` | **PASS** | 8 notation-bits == 8 |
| ATID `(width 7)` | `7'h7D` | **PASS** | explicit **width-cast** == LHS width |
| ATID `(width 7)` | `7'd125` | **PASS** | decimal width-cast also accepted |
| ARTAGOP `(width 2)` | `0b00` | **PASS** | 2 binary digits == 2 |
| ARTAGOP `(width 1)` | `0b00` | **FAIL** | 2 ≠ 1 |
| ATST `(width 1)` | `1'b1` | **PASS** | width-cast 1 == 1 |
| ATST `(width 2)` | `0B01` | **PASS** | 2 == 2 |
| SIG `(width 1)` | `0b11` | **FAIL** | 2 notation-bits ≠ 1 |
| SIG `(width 1)` | `1'b3` | **PASS** | FSMGen checks the **cast width** (`1'b…`), not value range |
| TLAST `(width 1)` | `1` (bare decimal) | **PASS** | bare/unsized literal fits any width |

**The contract the emitter must satisfy** is literally the one in the error message: emit *"an explicit
width-aligned source expression"* — a width-cast literal `W'<radix><digits>` whose declared cast width W
equals the signal's emitted width. A bare decimal is unsized and already safe.

## 3. Corpus prevalence + per-case classification

Over the 86 emitted `.isf`, the over-width value-literal class appears in **4 documents / 13 rule
clauses**. The decisive nuance: **all four signals are emitted at `(width 1)` even when the IntentIR
grounds a wider width** — the over-width is a *symptom*, and the faithful fix is **not** to truncate the
value:

| Doc | Signal | IntentIR width | `.isf` emits | literal | class |
|---|---|---|---|---|---|
| DTI `ihi0088_g` | ATST | `signal_records` all `None` → 1 | `width 1` | `0B01` ×3 | **mis-attribution** (upstream) |
| AXI+ACE `ihi0022_h_c` | ARTAGOP | `2` (3rd record) | `width 1` | `0b00` ×3 | width under-emitted; masked by `(port expr)` |
| AXI+ACE `ihi0022_h_c` | BTAGMATCH | `2` | `width 1` | `0b01`/`0b11`/`0b10` ×3 | width under-emitted; masked |
| AXI `ihi0022_l` (gold) | AWCMO | symbolic `AWCMO_WIDTH` | `width 1` | `0b000` ×1 | **parametric width** (residual) |
| Trace-bus `ihi0032_c` | ATID | `7` (3rd record) | `width 1` | `0x7D` ×3 | **width under-emitted** (clean lever) |

### Root cause #1 — grounded width dropped (the buildable lever)

The emitter dedups signals by first-seen name (`isf_ir.rs:696-700`: `if
seen_signal_names.contains(name) { continue; }`). ATID's `interfaces[].signal_records` are
`[{w:None},{w:None},{w:7}]` — the **first** (w=None→1) wins and the record carrying the grounded **width
7** is skipped. The `.2a.i` width-recovery only falls back to `actor_ports[].width_hint`, and ATID has
**no actor_port** (`actor_ports: []`) — so the width-7 living in a non-first *interface* record is lost.
ARTAGOP (width 2 in its 3rd record) hits the same gap. Emitting the grounded width makes the (correct)
value fit — *provided* the literal is also rendered width-aligned (probe: ATID `width 7` + `0x7D` still
FAILs; `width 7` + `7'd125` PASSes).

### Root cause #2 — value literal never width-aligned (the core lever)

Even with the grounded width emitted, the verbatim notation (`0x7D` = 8 bits, `0b00` = 2 bits, `0B01` =
2 bits) must be re-rendered as a width-cast `W'…` matching the signal width (§2). For ARTAGOP the
notation already matches (`0b00` is 2 bits, width 2) so emitting the grounded width alone suffices; for
ATID the notation (8) overshoots the grounded width (7) so it must be re-cast (`7'd125`).

### Spun out (not this tree)

- **DTI ATST — upstream mis-attribution.** `dyn_sigcon_0014.source_text` = *"…ATTR_OVR.MTCFG must be 0
  and **ATTR_OVR.SHCFG must be 0b01**…"* — the `0b01` belongs to **SHCFG**, and ATST is a *value of the
  FLOW field*, not the constrained signal. So `ATST must_be_value 0B01` is a mis-extracted constraint
  (wrong subject). The emitter width-alignment would render it FSMGen-valid (`1'b1`, value 1 fits width
  1, lossless), but the mis-attribution is an upstream extraction concern, recorded separately.
- **AXI `(port expr)` rule-assignment grammar.** Both AXI docs fail FSMGen **first** on
  `rule 'constraint_NN' assignment actions require '(port expr)'` (the known `ISF-RULE-CONFLICT` /
  constraint-assignment grammar class), with **0 truncate warnings** — so their width issue is *masked*
  and entangled. Only **DTI** and **TRACE** are clean, isolated value-width failures.

## 4. Wire-gold blast radius

The 4 WIRE-BASED-100 gold docs are APB (`ihi0024`), AHB (`ihi0033`), AXI (`ihi0022_l`), SWD. Of these,
**only AXI (`ihi0022_l`)** carries an over-width literal (AWCMO), and AXI already fails strict on the
orthogonal `(port expr)` error, so the width fix does **not** flip AXI's strict status. APB/AHB/SWD carry
no over-width literal → byte-identical. Critically, **WIRE-BASED-100 measures extraction F1**
(constraints/relations/temporal recall+precision), **not `.isf` bytes**, so re-rendering a value literal
is orthogonal to the gold by construction. AXI+ACE (`ihi0022_h_c`) and trace-bus (`ihi0032_c`) are not
gold docs.

## 5. Decision — GO

A real, FSMGen-confirmed, north-star (bar #6, faithful ISF lowering) emitter-fidelity defect. The
faithful fix is two coordinated emitter changes, never truncation:

1. **Width recovery completeness.** When collecting a signal's emitted width, prefer a single
   unambiguous concrete width grounded in **any** of the signal's interface `signal_records` *and*
   `actor_ports` (not just the first-seen record). Recovers ATID→7, ARTAGOP/BTAGMATCH→2. A conflict
   between grounded widths keeps the honest width-1 default — never a guess (the `.2a.i` discipline).
2. **Value-literal width-alignment.** When emitting a `(signal value)` clause, if the literal is a
   based/sized numeric whose notation width ≠ the signal's emitted width W, re-render it as an explicit
   width-cast `W'<radix><digits>` **iff the value fits** (`value < 2^W`). If `value ≥ 2^W` (a genuine
   over-value the signal width cannot hold), **residualize** the clause — an honest adapter residual
   (the `ISF-RULE-CONFLICT-RESIDUAL` pattern), never a truncated/fabricated value. Bare/unsized decimals
   are left untouched (already safe).

After the fix: ATID → `(output ATID (width 7))` + `(ATID 7'd125)` (PASS); ARTAGOP → `(width 2)` +
`(ARTAGOP 2'b00)` (PASS); AWCMO (parametric) → stays width 1, value 0 fits → `(AWCMO 1'b0)` (clause-valid;
AXI still fails on `(port expr)`); ATST → `(ATST 1'b1)` (clause-valid; mis-attribution spun out).

ADR-0006: numeric parsing + width arithmetic only — **no signal/value name list**. Universal across any
chip-spec PDF.

## 6. `.2` gates (the implementation slice)

- DTI + TRACE emitted `.isf` go OperandContract-FAIL → re-check `--strict --check` for the ATST/ATID
  clauses specifically (the *file* may still carry other unrelated diagnostics; the value-width class
  must clear).
- WIRE-BASED-100 (APB/AHB/AXI/SWD extraction F1) held 1.000 — orthogonal by construction (emitter-only,
  downstream of extraction; the gold is recall/precision, not `.isf` bytes).
- `kg-bench` 156/156; `scripts/run_ci.sh` GREEN; book `pipeline/isf-adapter.md` "Closed task trees"
  subsection on close (BOOK-METHOD-DOC).
- RAM: the impl needs a `cargo build` — defer until the host has headroom (this measurement was done on a
  6.3 GB host at ~16% free; Docling/heavy work is held).

See `[[register-reset-isf-emit]]` for the sibling `ISF-*-EMIT` precedent and
`[[isf-lowering-fidelity-gauge]]` for the orthogonal silent-drop gauge; governing north star is
`[[project_kg_isf_completeness]]` (bar #6).
