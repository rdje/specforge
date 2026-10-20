# FSMGen Feature Request: a reduction, and a per-byte reduction over a parameterised width

Title: sf-isf-byte-group-reduction — `.isf` has no reduction operator, so a parity check bit per byte of a parameter-sized signal (`DATACHK`, one bit per byte of `DATA_WIDTH`) cannot be stated; only an unrolled XOR at fixed widths can
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/byte-parity-param-width.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/byte-parity-param-width.isf
Failure class: rejected-input
FSMGen commit or capability-manifest producer commit: 82f0a5661eb79061dbb874fcb45b1066c29051a5 (upstream HEAD, 2026-10-20); also reproduced on SPECFORGE's pin 54ac1c22d5314943027494f4c270f9c612e62704 — see "Revision note"
SPECFORGE version/commit: specforge 8d3c77721
Expected behavior: a rule may state, for an output DATACHK (width DATA_BYTES), that bit i is the odd parity of DATA[8i+7:8i] for a DATA (width DATA_WIDTH), and a one-operand reduction (^ DATA[7:0]) states the parity of a slice; --strict --check succeeds and the generated HDL computes one parity bit per byte at any parameter value
Observed behavior: --strict --check --json refuses both the parameterised form and the fixed-width slice form with "Malformed expression operator '^' with 1 operand(s). This active form requires at least 2 operands."
First failing command or API call: original (./bin/fsmgen --strict --check --json CASE)
Exit status or exception: 1 (observed/command-logs/original.exit); JSON diagnostic in observed/stdout/original.stdout
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **feature request** (FSMGen follows its current public contract; the contract lacks a capability).
SPECFORGE owner: `FSMGEN-REQUESTS.12`; ledger record `byte-group-reduction`.
Local status: see `request.json` (`signed_off`; waiting for the owner to forward it).

## 1. What SPECFORGE needs

AMBA 5 interfaces protect every signal with **odd parity** check signals. Each specification states it in its own
words (*Odd parity is used. Odd parity means that check signals are added to groups of signals on the interface and
driven such that there is always an odd number of asserted bits in that group*), and its check-signal tables give
each check signal the signals it covers, its width, its granularity and its enable. Two shapes follow:

- a **one-bit check** over a few control signals (*PCTRLCHK* over *PPROT*, *PWRITE*, *PNSE*): one inverted XOR of
  every covered bit, which `.isf` can state today when the covered widths are fixed;
- a **byte-granular check** over a data or address bus whose width is a parameter (*PADDRCHK*,
  `ceil(ADDR_WIDTH/8)` bits wide; *PWDATACHK*, `DATA_WIDTH/8`): bit *i* of the check is the odd parity of byte *i*
  of the bus. This cannot be stated: its bit count and each bit's byte both follow the parameter.

SPECFORGE lowers the first shape since `EXTRACTION-REPRESENTATION-GAPS.46.b` (48 rules, FSMGen-strict clean). The
second shape is what this request is for. Unrolling it at the parameters' defaults would bake one configuration into
a generic actor.

## 2. Measured need (SPECFORGE corpus, 2026-10-20)

SPECFORGE carries **153** check-signal records from AXI (123), AHB5 (16) and APB5 (14), every one `odd`;
**49** are byte-granular over a parameterised width (AXI 34, AHB5 8, APB5 7). Re-derive in a SPECFORGE checkout:

```bash
python3 - <<'PY'
import json, glob
for p in sorted(glob.glob('generated/adapters/isf/*/adapter.json')):
    a = json.load(open(p))
    for r in a['residual_decisions']:
        if r['packet_id'] == 'isf_parity_checks_not_lowered':
            print(p.split('/')[3][:24], r['why_unresolved'].split(': ', 1)[1].split('; ')[0][:80])
PY
```

## 3. Current public contract (why this is a feature request, not a bug)

- `docs/book/src/13k-isf-feature-support-matrix.md`, row *Rules and trigger fan-in*: a rule's actions are
  `(set TARGET EXPR)` and shorthand assignments; row *Data manipulation*: `select`, `max`, `min`, bit and field
  operations (`set-bit`, `set-field`, `when-field`, …); neither names a reduction or a per-group form.
- The observed diagnostic is the expression parser enforcing that `^` is binary: *"This active form requires at
  least 2 operands"*.

Both are the same at the pin and at upstream HEAD. **Related, not duplicated:** `sf-isf-derived-width-parameter`
asks for a port width derived from a parameter (`DATA_WIDTH/8`); this bundle sidesteps it with an independent
`DATA_BYTES` parameter so that it shows only the missing reduction.

## 4. Reproduction

Run from the FSMGen root: `bash <bundle>/commands.sh`.

| Input | Difference | `--strict --check` | Result |
| --- | --- | --- | --- |
| `baseline-good.isf` | `DATA` 16 bits; `DATACHK0`, `DATACHK1` each the inverted XOR of one byte, unrolled bit by bit | exit 0 | accepted |
| `reduction-over-slice.isf` | the same two checks as `(! (^ DATA[7:0]))` and `(! (^ DATA[15:8]))` | exit 1 | *Malformed expression operator '^' with 1 operand(s). This active form requires at least 2 operands.* |
| `byte-parity-param-width.isf` (primary) | `DATA (width DATA_WIDTH)`, `DATACHK (width DATA_BYTES)`, `(DATACHK (! (^ DATA)))` | exit 1 | the same diagnostic |

## 5. Requested capability

- **R1 (required) — unary reductions.** `(^ X)`, and its companions `(& X)` and `(| X)`, over a signal, a slice or
  a bit selection: a one-bit result. Failure cases: an operand that is not a vector fails closed naming it.
- **R2 (required) — a per-group reduction over a parameterised width.** A form that states, for an output of
  `ceil(W/N)` bits, that bit *i* is the reduction of bits `[N*i+N-1 : N*i]` of a `W`-bit operand, with `W` a
  parameter and `N` a positive literal (8 for byte parity); the last group may be shorter when `W` is not a multiple
  of `N`. Failure cases: a target width that does not match the operand's group count fails under `--strict`.
- **R3 (required) — lowering.** The generated `.fsm` and HDL compute the reduction at any parameter value, and the
  schedule and semantic reports name the form.
- **R4 (optional) — bitwise negation of a vector**, so that odd parity of every group is one `(! …)` (or a stated
  `(~ …)`) over the group vector rather than per bit.

## 6. Acceptance criteria (proposed, for FSMGen's regression tests)

1. `reduction-over-slice.isf` passes `--strict --check`; the generated HDL computes `DATACHK0` as the inverted XOR of
   `DATA[7:0]`.
2. `byte-parity-param-width.isf`, rewritten in the R2 form, passes; at `DATA_WIDTH = 16, DATA_BYTES = 2` the HDL
   computes each `DATACHK` bit from its byte, and at `DATA_WIDTH = 32, DATA_BYTES = 4` from its four bytes.
3. A per-group target whose width does not match the operand's group count fails under `--strict` naming both.
4. `baseline-good.isf` keeps its current result.

## 7. Alternatives SPECFORGE considered and why they are not faithful

- **Unroll at the parameters' defaults** (what `.46.b` does for fixed widths): for a parameterised bus it fixes one
  configuration, so a generic actor states the wrong parity at every other width.
- **One check output per byte at a fixed width**: invents a port list the specification does not have.
- **Leave the check undriven** (what SPECFORGE does today for these 49): the interface's parity is not modelled.

## 7a. What SPECFORGE does until this ships

The 49 byte-granular check signals stay ports driven by `val`, named by reason in one `isf_parity_checks_not_lowered`
residual per document; the one-bit checks over fixed-width signals are lowered as `(rule parity_<CHK> <enable>
(<CHK> (! (^ …))))`. Owner on the SPECFORGE side: `EXTRACTION-REPRESENTATION-GAPS.46.c`.

## 8. Compatibility

Additive: every currently accepted input keeps its meaning; `^`, `&` and `|` gain a one-operand form FSMGen
refuses today.

## 9. Revision note

Reproduced on FSMGen upstream HEAD `82f0a5661eb79061dbb874fcb45b1066c29051a5` (a read-only `git archive` export,
where `commands.sh` ran and generated `observed/`) and on SPECFORGE's pin
`54ac1c22d5314943027494f4c270f9c612e62704` (strict checks of the same three inputs: identical exit codes and
diagnostics). Each installation passed a known-good strict check (`baseline-good.isf`) before anything was compared.

## 10. Redactions and simplifications

- Absolute paths in captured outputs are replaced by `<FSMGEN_ROOT>` (the FSMGen export the helper ran in),
  `<SPECFORGE_ROOT>`, `<FSMGEN_CHECKOUT>`, `<HOME>` and `<TMP>`. No other byte of any captured output changed.
- `env.txt`'s *FSMGen git HEAD* and `observed/command-logs/fsmgen-git-head.txt` are corrected by hand: the helper
  recorded SPECFORGE's commit, the defect reported as `sf-tool-issue-bundle-foreign-git-head`.
- `commands.sh` is the helper's script plus the SPECFORGE-added companion loop at its end.
- The three inputs are hand-written; the AMBA quotations in §1 summarise the specifications' parity sections, and
  no corpus content is needed to reproduce.
