# FSMGen Feature Request: derived actor parameters for `.isf` interface widths

Title: sf-isf-derived-width-parameter — an actor parameter derived from other actor parameters (`(STRB_WIDTH (/ DATA_WIDTH 8))`) cannot size an interface port
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/derived-param.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/derived-param.isf
Failure class: rejected-input
FSMGen commit or capability-manifest producer commit: 5a44ad496c852b168c31d4021cfdb158001a47c7 (upstream HEAD, 2026-09-24); also reproduced on SPECFORGE's pins c0d8b668db2527108d1c23c20d184400c51efea6 and 54ac1c22d5314943027494f4c270f9c612e62704 (published origin/main) — see "Revision note" below
SPECFORGE version/commit: specforge 343bd7fa
Expected behavior: an actor scalar parameter whose default is a bounded integer expression over earlier actor scalar parameters, such as (STRB_WIDTH (/ DATA_WIDTH 8)), resolves to a positive integer, may size an interface port with (width STRB_WIDTH), and re-derives when the base parameter is overridden at instantiation; --strict --check --json reports success:true for sources/fsmgen-input/derived-param.isf.
Observed behavior: --strict --check --json reports success:false with "actor 'strobe_demo' parameter 'STRB_WIDTH' uses unsupported aggregate/list parameter leaf '/'"; the three other spellings of the same width (actor constant, inline width expression, ceiling-division parameter) are refused as well.
First failing command or API call: original (./bin/fsmgen --strict --check --json CASE)
Exit status or exception: 1 (observed/command-logs/original.exit); JSON diagnostic in observed/stdout/original.stdout
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **feature request** (FSMGen follows its current public contract; the contract lacks a capability).
SPECFORGE owner: `FSMGEN-REQUESTS.1` (bundle), `KG-ISF-COMPLETENESS.2a.vii.d` (SPECFORGE integration once shipped).
Local status: see `request.json` (`ready_to_forward` when this bundle was signed off).

## 1. What SPECFORGE needs

Hardware interface specifications describe **generic blocks**: one design that an integrator specialises
at instantiation by choosing parameter values. A data bus is `DATA_WIDTH` bits wide; its write strobe has
one bit per byte, so it is `DATA_WIDTH/8` bits wide; its parity-check signals are `ceil(ADDR_WIDTH/8)`
bits wide. The strobe's width is not an independent knob — it is a function of `DATA_WIDTH`, and a
design in which it could be set independently would admit invalid configurations.

FSMGen's `.isf` actor surface already expresses the first half of this: `(params (DATA_WIDTH 32))` with
`(output WDATA (width DATA_WIDTH))` elaborates, and SPECFORGE now emits exactly that for every signal a
specification sizes by a single parameter. The second half — a width **derived** from a parameter — has no
spelling in the current contract, so SPECFORGE must lower those signals to `(width 1)` and record the loss
as a residual.

## 2. Measured need (SPECFORGE corpus, 2026-09-24)

Across the three proof-carrying AMBA wire-protocol specifications SPECFORGE lowers (AXI `IHI0022L`,
APB `IHI0024E`, AHB `IHI0033C`), **56 interface signals** have a derived width. Every distinct form, with
the number of signals using it:

| Width as the specification writes it | Signals | Operators needed |
| --- | ---: | --- |
| `DATA_WIDTH/8` (and `DATA_WIDTH / 8`) | 9 | exact division |
| `ceil(X/8)` over `USER_DATA_WIDTH`, `USER_REQ_WIDTH`, `ADDR_WIDTH`, `USER_RESP_WIDTH`, `SSID_WIDTH`, `SID_WIDTH`, `MECID_WIDTH`, `LOOP_W_WIDTH`, `LOOP_R_WIDTH` | 26 | ceiling division |
| `ceil(DATA_WIDTH/64)`, `ceil(DATA_WIDTH/128)`, `ceil(DATA_WIDTH/512)` | 9 | ceiling division |
| `ceil(DATA_WIDTH/128)*4` | 2 | ceiling division, multiplication |
| `ceil((ACT_W_WIDTH+1)/8)`, `ceil((ACT_R_WIDTH+1)/8)` | 2 | addition, ceiling division |
| `ceil((ID_W_WIDTH + int(Unique_ID_Support))/8)` (and `ID_R`) | 4 | a 0/1 flag parameter, addition, ceiling division |
| `USER_DATA_WIDTH + USER_RESP_WIDTH` | 1 | addition |
| `clog2(Num_RP_AWW)`, `clog2(Num_RP_AR)` | 3 | ceiling log2 |

Re-derive: `python3 scripts/measure_parameterised_signal_widths.py --signals` in the SPECFORGE
repository (the `derived` rows of the three `live` chains).

## 3. Current public contract (why this is a feature request, not a bug)

- `docs/book/src/13a-actor-interface.md`, *Interface*: "An interface `(width ...)` may be a positive
  integer literal, an actor-local scalar parameter default, or a declared actor constant that resolves to
  a positive integer … arbitrary expressions fail closed." Unchanged between c0d8b668 and 5a44ad49.
- The same chapter, *Actor-Owned Storage*: "arbitrary storage dimension expressions … remain future
  generalizations."
- `docs/book/src/07-packages-and-sharing.md`: "Aggregate scalar leaves, parameters, runtime signals, and
  arbitrary expressions are not type-width symbols."
- **Precedent inside FSMGen:** `docs/book/src/04-symbols-types-and-imports.md` already accepts bounded
  operator expressions for `.fsm` `+params` — `(COUNT_PLUS_ONE (+ WIDTH 1))` with `+ - * / % & | ^` —
  and `docs/book/src/06-composition-advanced.md` keeps `(params (WIDTH_PLUS_ONE (+ WIDTH 1)))` semantic
  through instance emission ("the operands resolve before generation, then the current Verilog-family
  backend emits a normal instance parameter expression").
- **Width expressions already exist one level down.** `docs/book/src/04-symbols-types-and-imports.md`:
  "`+size` entries require one signal name plus one positive width expression or named type", and "for
  `+size` and composition `?ports`, width expressions also retain the broader already-shipped
  constant-expression surface". The capability manifest in this bundle
  (`observed/json/fsmgen-capability-manifest.json`) registers stable diagnostics for them:
  `FSMGEN_LANGUAGE_SIZE_DIVIDE_BY_ZERO`, `FSMGEN_LANGUAGE_SIZE_MODULO_BY_ZERO`,
  `FSMGEN_LANGUAGE_UNSUPPORTED_SIZE_OPERATOR`, `FSMGEN_LANGUAGE_SIZE_OPERATOR_ARITY`,
  `FSMGEN_LANGUAGE_UNKNOWN_SIZE_SYMBOL`, `FSMGEN_LANGUAGE_NON_POSITIVE_SIZE`.
- `docs/book/src/13k-isf-feature-support-matrix.md`, *Interface ports*: "arbitrary width expressions fail
  closed" at the `.isf` level; *Actor-owned scalar storage* likewise lists "width expressions".

So the request is to expose, at the `.isf` actor level, the width-expression and parameter-expression
surface FSMGen already implements for `.fsm`, not to invent a new evaluator.

FSMGen therefore behaves as documented; the capability is missing.

## 4. Reproduction

From the FSMGen repository root, with this directory unpacked there as
`fsmgen-issue-sf-isf-derived-width-parameter/`:

```bash
bash fsmgen-issue-sf-isf-derived-width-parameter/commands.sh
```

Every input is a standalone, hand-written actor (no SPECFORGE corpus content). Each differs from the
accepted baseline **only** in how the strobe width is stated:

| File | How `WSTRB`'s width is stated | Exit | Diagnostic (JSON `diagnostics[0].message`) |
| --- | --- | ---: | --- |
| `baseline-good.isf` | `(width 4)` — concrete, correct only for `DATA_WIDTH` 32 | 0 | — (`success:true`) |
| `derived-param.isf` (primary) | `(params … (STRB_WIDTH (/ DATA_WIDTH 8)))` + `(width STRB_WIDTH)` | 1 | `actor 'strobe_demo' parameter 'STRB_WIDTH' uses unsupported aggregate/list parameter leaf '/'` |
| `derived-constant.isf` | `(constants (STRB_WIDTH (/ DATA_WIDTH 8)))` + `(width STRB_WIDTH)` | 1 | `actor 'strobe_demo' constant 'STRB_WIDTH' requires a non-negative integer literal value or enum member reference` |
| `derived-inline.isf` | `(width (/ DATA_WIDTH 8))` | 1 | `interface port 'WSTRB' width must be a positive integer, actor constant, actor scalar parameter, or qualified package scalar constant` |
| `derived-ceil.isf` | `(ADDRCHK_WIDTH (/ (+ ADDR_WIDTH 7) 8))` — ceiling division by the integer identity | 1 | `actor 'check_demo' parameter 'ADDRCHK_WIDTH' uses unsupported aggregate/list parameter leaf '/'` |

Captured outputs: `observed/stdout/original.stdout` and `observed/stdout/<variant>-check-strict.stdout`,
exit codes in `observed/command-logs/*.exit`.

## 5. Requested capability

**R1 — derived actor parameters (required).** An `.isf` actor `(params …)` entry may take as its default a
bounded scalar integer expression, in the expression spelling `+params` already accepts, over integer
literals and actor scalar parameters declared **earlier** in the same block:

```lisp
(params
  (DATA_WIDTH 32)
  (STRB_WIDTH (/ DATA_WIDTH 8)))
(interface
  (output WDATA (width DATA_WIDTH))
  (output WSTRB (width STRB_WIDTH)))
```

- Width resolution evaluates the expression to an integer at elaboration; the existing rule that a width
  resolves to a **positive** integer still applies, with a diagnostic naming the parameter, its expression
  and the value it resolved to when it is not positive.
- The derived parameter stays **symbolic** in generated artifacts (`+params`, and the generated
  module's parameter list as `parameter STRB_WIDTH = DATA_WIDTH / 8`), so overriding `DATA_WIDTH` at
  instantiation re-derives `STRB_WIDTH`. A derived parameter is not itself an override target unless
  FSMGen decides otherwise; either way, please document which.
- Fail closed, with a diagnostic, on: a reference to an undeclared or later-declared parameter, a cycle,
  division or modulo by zero, a non-integer result, and runtime signals as operands.

**R2 — ceiling division (required).** 43 of the 56 signals need `ceil(a / b)`. Either a named operator
(for example `(ceil_div a b)`) or a documented guarantee that `(/ (+ a (- b 1)) b)` is supported with
truncating integer division for non-negative operands. `derived-ceil.isf` is the acceptance case for the
second option.

**R3 — ceiling log2 (required for 3 signals).** A `(clog2 x)` operator with the Verilog `$clog2` meaning
(`clog2(1) = 0`), since `clog2(Num_RP_AR)` sizes an index over `Num_RP_AR` resource planes.

**R4 — inline width expression (optional).** `(width (/ DATA_WIDTH 8))` as sugar for an anonymous
derived parameter. SPECFORGE does not need it if R1 ships; it is listed only because it is the spelling
authors try first (`derived-inline.isf`).

Out of scope: floating point, runtime-valued widths, and anything beyond the bounded expression domain
FSMGen already implements for `+params`.

## 6. Acceptance criteria (proposed, for FSMGen's regression tests)

1. `derived-param.isf` → `--strict --check --json` exit 0, `success:true`; `--emit-semantic-json` reports
   `WSTRB` width 4 with `DATA_WIDTH` at its default 32.
2. The same file generated with `-l sv` declares `STRB_WIDTH` as a parameter expression of `DATA_WIDTH`,
   and an instance overriding `DATA_WIDTH` to 64 elaborates `WSTRB` at 8 bits.
3. `derived-ceil.isf` (or its `ceil_div` spelling) → exit 0, `ADDRCHK` width 4 for `ADDR_WIDTH` 32, and 1 for
   `ADDR_WIDTH` 1.
4. `(clog2 8)` → 3, `(clog2 1)` → 0, `(clog2 5)` → 3.
5. Fail-closed cases each produce one diagnostic naming the parameter: `(X (/ DATA_WIDTH 0))`,
   `(X (- 1 DATA_WIDTH))` used as a width, `(A (+ B 1))` before `B` is declared, `(A B) (B A)`.
6. `baseline-good.isf` and every existing `.isf` fixture are unaffected.

## 7. Alternatives SPECFORGE considered and why they are not faithful

- **Emit the computed number** (`(width 4)`): correct only at the default and silently wrong after any
  override of `DATA_WIDTH`; it discards the dependency the specification states.
- **Declare an independent parameter** (`(params (STRB_WIDTH 4))`): elaborates, but lets an integrator set
  inconsistent values; it states a knob the specification does not have.
- **Compute widths in SPECFORGE**: SPECFORGE does not own elaboration or instantiation; the generic block
  would stop being generic.

Until this ships SPECFORGE keeps each derived-width signal at `(width 1)` and lists it, with its
expression, in the adapter residual `isf_signal_width_unlowered`. Nothing is fabricated.

## 8. Compatibility

Purely additive: every input accepted today keeps its meaning, and every input refused today with one of
the four diagnostics above either becomes accepted with the documented meaning or keeps failing closed.

## 9. Revision note

The bundle was captured with `bin/fsmgen-issue-bundle` from a `git archive` export of FSMGen
`5a44ad49` placed inside the SPECFORGE checkout. Because the export is not a git repository, the helper's
recorded head resolved to the enclosing SPECFORGE commit; `env.txt` and
`observed/command-logs/fsmgen-git-head.txt` were corrected by hand to `5a44ad49`, and the capability
manifest reports `producer.git_commit: unknown` for the same reason. That helper behaviour is reported
separately as `sf-tool-issue-bundle-foreign-git-head`. The strict-check results in section 4 were also
reproduced on the pinned `c0d8b668` binary.

## 10. Redactions and simplifications

- Absolute paths in captured outputs are replaced by `<FSMGEN_ROOT>` (the FSMGen export the helper ran
  in), `<SPECFORGE_ROOT>`, `<FSMGEN_CHECKOUT>` and `<HOME>`. No other byte of any captured output changed.
- `commands.sh` is the helper's script plus the SPECFORGE-added companion loop at its end, which runs the
  four other variants under `--strict --check --json`.

## Re-verification `2026-09-24` on the published tip

SPECFORGE moved its FSMGen pin to `54ac1c22d5314943027494f4c270f9c612e62704`, the published `origin/main`
(`FSMGEN-REFRESH-INTEGRATE-9`). The result above reproduces unchanged there. Note that `5a44ad49` and
`67c8b36d`, named above as upstream HEAD, were commits in an FSMGen checkout ahead of `origin` at capture time.
